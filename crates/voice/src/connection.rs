//! Discord voice gateway connection.

use std::sync::{
    atomic::{AtomicI64, AtomicU32, Ordering},
    Arc,
};
use std::time::Duration;

use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Key, Nonce,
};
use futures_util::{SinkExt, StreamExt};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, info, warn};

use oxidian_core::{
    error::{Error as OxidianError, VoiceError},
    snowflake::Snowflake,
};


type WsSink = futures_util::stream::SplitSink<
    tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    Message,
>;

type WsStream = futures_util::stream::SplitStream<
    tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
>;


struct VoiceReady {
    ssrc: u32,
    ip: String,
    port: u16,
    modes: Vec<String>,
}


/// A live Discord voice connection.
///
/// Created by [`VoiceConnection::connect`] after both `VOICE_STATE_UPDATE` and
/// `VOICE_SERVER_UPDATE` events have been received.
pub struct VoiceConnection {
    /// The guild this voice connection is for.
    pub guild_id: Snowflake,
    /// SSRC assigned by Discord for this connection.
    pub ssrc: u32,

    udp: Arc<UdpSocket>,
    secret_key: [u8; 32],
    nonce_counter: Arc<AtomicU32>,
    sequence: Arc<AtomicU32>,
    timestamp: Arc<AtomicU32>,
    /// Sending on this channel stops the voice heartbeat task.
    write_tx: mpsc::Sender<Message>,
    _heartbeat_stop: mpsc::Sender<()>,
}

impl VoiceConnection {
    /// Connect to a Discord voice server.
    pub async fn connect(
        endpoint: &str,
        server_id: Snowflake,
        user_id: Snowflake,
        session_id: &str,
        token: &str,
    ) -> Result<Self, OxidianError> {
        let url = format!("wss://{endpoint}/?v=8");
        info!(url = %url, "connecting to voice WebSocket");

        let (ws, _) = connect_async(&url)
            .await
            .map_err(|e| VoiceError::Connection(e.to_string()))?;
        let (sink, mut stream) = ws.split();

        // Set up the write channel.
        let (write_tx, write_rx) = mpsc::channel::<Message>(64);
        tokio::spawn(write_loop(sink, write_rx));

        // 1. Receive Hello (op 8) → heartbeat interval.
        let hello = recv_op(&mut stream, 8).await?;
        let interval_ms = hello["heartbeat_interval"]
            .as_f64()
            .unwrap_or(30_000.0) as u64;
        debug!(interval_ms, "received voice Hello");

        // 2. Send Identify (op 0) — must be the first message we send.
        //    max_dave_protocol_version: 1 — required on servers that enforce
        //    E2EE/DAVE.  We handle the DAVE transition handshake in the
        let identify_payload = serde_json::json!({
            "op": 0u8,
            "d": {
                "server_id":                server_id.to_string(),
                "user_id":                  user_id.to_string(),
                "session_id":               session_id,
                "token":                    token,
                "max_dave_protocol_version": 1u8,
            }
        });
        info!(payload = %identify_payload, "sending voice Identify");
        send_json(&write_tx, identify_payload).await?;
        debug!("sent voice Identify");

        // 3. Start heartbeat AFTER identify to avoid sending op 3 before op 0
        //    (tokio::time::interval fires the first tick immediately).
        //    Voice gateway v8 heartbeats: {"op":3,"d":{"t":timestamp,"seq_ack":N}}
        let seq_ack = Arc::new(AtomicI64::new(-1));
        let hb_stop = spawn_heartbeat(interval_ms, write_tx.clone(), Arc::clone(&seq_ack));

        // 4. Receive Ready (op 2).
        let ready_data = recv_op(&mut stream, 2).await?;
        let ready = VoiceReady {
            ssrc: ready_data["ssrc"].as_u64().unwrap_or(0) as u32,
            ip:   ready_data["ip"].as_str().unwrap_or("").to_owned(),
            port: ready_data["port"].as_u64().unwrap_or(0) as u16,
            modes: ready_data["modes"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(str::to_owned)).collect())
                .unwrap_or_default(),
        };
        info!(ssrc = ready.ssrc, ip = %ready.ip, port = ready.port, "voice Ready");

        // 5. UDP IP discovery.
        let udp = UdpSocket::bind("0.0.0.0:0")
            .await
            .map_err(|e| VoiceError::Udp(e.to_string()))?;
        udp.connect(format!("{}:{}", ready.ip, ready.port))
            .await
            .map_err(|e| VoiceError::Udp(e.to_string()))?;

        let (our_ip, our_port) = ip_discovery(&udp, ready.ssrc).await?;
        debug!(our_ip = %our_ip, our_port, "UDP IP discovery complete");

        // 6. Choose encryption mode (prefer aead_aes256_gcm_rtpsize).
        let mode = choose_mode(&ready.modes)?;
        debug!(mode = %mode, "selected voice encryption mode");

        // 7. Send Select Protocol (op 1).
        send_json(
            &write_tx,
            serde_json::json!({
                "op": 1u8,
                "d": {
                    "protocol": "udp",
                    "data": {
                        "address": our_ip,
                        "port":    our_port,
                        "mode":    mode,
                    }
                }
            }),
        )
        .await?;

        // 8. Receive Session Description (op 4).
        let session_data = recv_op(&mut stream, 4).await?;
        let secret_key_vec: Vec<u8> = session_data["secret_key"]
            .as_array()
            .ok_or_else(|| VoiceError::Connection("no secret_key in session description".into()))?
            .iter()
            .filter_map(|v| v.as_u64().map(|b| b as u8))
            .collect();

        let secret_key: [u8; 32] = secret_key_vec
            .try_into()
            .map_err(|_| VoiceError::Connection("secret_key must be 32 bytes".into()))?;

        info!("voice session established");

        // Spawn background event loop (handles HeartbeatAck, seq_ack, DAVE transitions).
        tokio::spawn(voice_event_loop(stream, write_tx.clone(), Arc::clone(&seq_ack)));

        Ok(Self {
            guild_id: server_id,
            ssrc: ready.ssrc,
            udp: Arc::new(udp),
            secret_key,
            nonce_counter: Arc::new(AtomicU32::new(0)),
            sequence: Arc::new(AtomicU32::new(0)),
            timestamp: Arc::new(AtomicU32::new(0)),
            write_tx,
            _heartbeat_stop: hb_stop,
        })
    }

    /// Set the speaking state (must be sent before transmitting audio).
    ///
    /// `speaking` — `true` to start speaking, `false` to stop.
    pub async fn speak(&self, speaking: bool) -> Result<(), OxidianError> {
        send_json(
            &self.write_tx,
            serde_json::json!({
                "op": 5u8,
                "d": {
                    "speaking": if speaking { 1u8 } else { 0u8 },
                    "delay":    0,
                    "ssrc":     self.ssrc,
                }
            }),
        )
        .await
    }

    /// Send a pre-encoded 20 ms Opus audio frame over UDP.
    ///
    /// The frame is encrypted with `aead_aes256_gcm_rtpsize` before
    /// transmission.  Call [`Self::speak`] with `true` before the first call.
    pub async fn send_audio(&self, opus_data: &[u8]) -> Result<(), OxidianError> {
        let seq = self.sequence.fetch_add(1, Ordering::Relaxed) as u16;
        let ts  = self.timestamp.fetch_add(960, Ordering::Relaxed);  // 20 ms at 48 kHz
        let nonce = self.nonce_counter.fetch_add(1, Ordering::Relaxed);

        let rtp_header = build_rtp_header(seq, ts, self.ssrc);
        let packet = encrypt_audio(&rtp_header, opus_data, &self.secret_key, nonce)?;

        self.udp
            .send(&packet)
            .await
            .map(|_| ())
            .map_err(|e| VoiceError::Udp(e.to_string()).into())
    }

    /// Disconnect from the voice channel and close the WebSocket.
    pub async fn disconnect(self) -> Result<(), OxidianError> {
        // Sending the stop signal drops _heartbeat_stop which closes the channel.
        // The write channel close will propagate to the socket.
        drop(self._heartbeat_stop);
        Ok(())
    }
}


async fn write_loop(mut sink: WsSink, mut rx: mpsc::Receiver<Message>) {
    while let Some(msg) = rx.recv().await {
        if sink.send(msg).await.is_err() {
            break;
        }
    }
}

async fn send_json(
    write_tx: &mpsc::Sender<Message>,
    value: serde_json::Value,
) -> Result<(), OxidianError> {
    let text = serde_json::to_string(&value).map_err(OxidianError::Serialization)?;
    write_tx
        .send(Message::Text(text.into()))
        .await
        .map_err(|_| VoiceError::Connection("voice write channel closed".into()).into())
}

async fn recv_op(
    stream: &mut WsStream,
    expected_op: u64,
) -> Result<serde_json::Value, OxidianError> {
    loop {
        let msg = tokio::time::timeout(Duration::from_secs(10), stream.next())
            .await
            .map_err(|_| {
                VoiceError::Connection(format!("timed out waiting for voice op {expected_op}"))
            })?;
        let msg = match msg {
            Some(m) => m,
            None => break,
        };
        let text = match msg {
            Ok(Message::Text(t)) => t.to_string(),
            Ok(Message::Close(frame)) => {
                let reason = frame
                    .map(|f| format!("code={} reason={}", f.code, f.reason))
                    .unwrap_or_else(|| "no close frame".into());
                return Err(VoiceError::Connection(format!(
                    "voice WebSocket closed by server ({reason})"
                ))
                .into());
            }
            Ok(_) => continue,
            Err(e) => return Err(VoiceError::Connection(e.to_string()).into()),
        };

        let payload: serde_json::Value =
            serde_json::from_str(&text).map_err(OxidianError::Serialization)?;

        if payload["op"].as_u64() == Some(expected_op) {
            return Ok(payload["d"].clone());
        }
        // Skip other opcodes (e.g. HeartbeatAck) while waiting.
        debug!(op = ?payload["op"], "skipping voice opcode while waiting for {}", expected_op);
    }
    Err(VoiceError::Connection(format!(
        "stream ended before op {expected_op} was received"
    ))
    .into())
}

/// Discover our external IP and port via Discord's UDP IP discovery protocol.
async fn ip_discovery(udp: &UdpSocket, ssrc: u32) -> Result<(String, u16), OxidianError> {
    // Request packet: type(2) | length(2) | ssrc(4) | 66 zero bytes = 74 bytes.
    let mut packet = [0u8; 74];
    packet[0] = 0x00;
    packet[1] = 0x01; // request
    let length: u16 = 70;
    packet[2..4].copy_from_slice(&length.to_be_bytes());
    packet[4..8].copy_from_slice(&ssrc.to_be_bytes());

    udp.send(&packet)
        .await
        .map_err(|e| VoiceError::Udp(e.to_string()))?;

    let mut buf = [0u8; 74];
    let n = tokio::time::timeout(Duration::from_secs(5), udp.recv(&mut buf))
        .await
        .map_err(|_| VoiceError::Udp("IP discovery timed out".into()))?
        .map_err(|e| VoiceError::Udp(e.to_string()))?;

    if n < 74 {
        return Err(VoiceError::Udp(format!(
            "IP discovery response too short: {n} bytes"
        ))
        .into());
    }

    // Response: type(2) | length(2) | ssrc(4) | ip(64, null-terminated) | port(2)
    let ip_bytes = &buf[8..72];
    let null_pos = ip_bytes.iter().position(|&b| b == 0).unwrap_or(64);
    let ip = String::from_utf8_lossy(&ip_bytes[..null_pos]).into_owned();
    let port = u16::from_be_bytes([buf[72], buf[73]]);

    Ok((ip, port))
}

/// Choose the best supported encryption mode.
fn choose_mode(available: &[String]) -> Result<String, OxidianError> {
    // Preference order: AES-256-GCM (current) > XChaCha20 > legacy XSalsa20.
    let preferred = [
        "aead_aes256_gcm_rtpsize",
        "aead_xchacha20_poly1305_rtpsize",
        "xsalsa20_poly1305_lite_rtpsize",
        "xsalsa20_poly1305_lite",
        "xsalsa20_poly1305_suffix",
        "xsalsa20_poly1305",
    ];
    for pref in preferred {
        if available.iter().any(|m| m == pref) {
            return Ok(pref.to_owned());
        }
    }
    Err(VoiceError::Connection(format!(
        "no supported encryption mode in: {available:?}"
    ))
    .into())
}

/// Build a 12-byte RTP header.
fn build_rtp_header(sequence: u16, timestamp: u32, ssrc: u32) -> [u8; 12] {
    let mut h = [0u8; 12];
    h[0] = 0x80; // V=2, P=0, X=0, CC=0
    h[1] = 0x78; // M=0, PT=120 (Opus)
    h[2..4].copy_from_slice(&sequence.to_be_bytes());
    h[4..8].copy_from_slice(&timestamp.to_be_bytes());
    h[8..12].copy_from_slice(&ssrc.to_be_bytes());
    h
}

/// Encrypt an Opus frame with `aead_aes256_gcm_rtpsize`.
///
/// Packet layout: RTP header (12) || ciphertext+tag (n+16) || nonce (4).
fn encrypt_audio(
    rtp_header: &[u8; 12],
    opus_data: &[u8],
    secret_key: &[u8; 32],
    nonce: u32,
) -> Result<Vec<u8>, OxidianError> {
    // Nonce: 4-byte big-endian counter padded to 12 bytes.
    let mut nonce_bytes = [0u8; 12];
    nonce_bytes[..4].copy_from_slice(&nonce.to_be_bytes());

    let key = Key::<Aes256Gcm>::from_slice(secret_key);
    let cipher = Aes256Gcm::new(key);
    let nonce_val = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(
            nonce_val,
            Payload { msg: opus_data, aad: rtp_header },
        )
        .map_err(|_| VoiceError::Connection("AES-256-GCM encryption failed".into()))?;

    // Final packet: header || ciphertext (includes 16-byte tag) || nonce (4 bytes).
    let mut packet = Vec::with_capacity(12 + ciphertext.len() + 4);
    packet.extend_from_slice(rtp_header);
    packet.extend_from_slice(&ciphertext);
    packet.extend_from_slice(&nonce.to_be_bytes());

    Ok(packet)
}


fn spawn_heartbeat(
    interval_ms: u64,
    write_tx: mpsc::Sender<Message>,
    seq_ack: Arc<AtomicI64>,
) -> mpsc::Sender<()> {
    let (stop_tx, mut stop_rx) = mpsc::channel::<()>(1);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let t = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .map(|d| d.as_millis() as u64)
                        .unwrap_or(0);
                    let current_seq = seq_ack.load(Ordering::Relaxed);
                    let payload = serde_json::json!({
                        "op": 3u8,
                        "d": {
                            "t": t,
                            "seq_ack": current_seq,
                        }
                    });
                    if let Ok(text) = serde_json::to_string(&payload) {
                        if write_tx.send(Message::Text(text.into())).await.is_err() {
                            break;
                        }
                    }
                }
                _ = stop_rx.recv() => break,
            }
        }
    });
    stop_tx
}


async fn voice_event_loop(
    mut stream: WsStream,
    write_tx: mpsc::Sender<Message>,
    seq_ack: Arc<AtomicI64>,
) {
    while let Some(msg) = stream.next().await {
        let text = match msg {
            Ok(Message::Text(t)) => t.to_string(),
            Ok(Message::Close(_)) => {
                info!("voice WebSocket closed");
                break;
            }
            Ok(_) => continue,
            Err(e) => {
                warn!(error = %e, "voice WebSocket error");
                break;
            }
        };

        let payload: serde_json::Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => { warn!(error = %e, "failed to parse voice payload"); continue; }
        };

        // Update seq_ack from every message (voice gateway v8 protocol).
        if let Some(seq) = payload.get("seq").and_then(|v| v.as_i64()) {
            seq_ack.store(seq, Ordering::Relaxed);
        }

        match payload["op"].as_u64() {
            Some(6) => debug!("voice HeartbeatAck"),
            Some(9) => info!("voice session Resumed"),
            Some(13) => debug!("voice client disconnect"),

            // DAVE_PREPARE_TRANSITION: acknowledge with DAVE_TRANSITION_READY (op 25)
            Some(20) => {
                let transition_id = payload["d"]["transition_id"].as_u64().unwrap_or(0);
                let protocol_version = payload["d"]["protocol_version"].as_u64().unwrap_or(1);
                info!(transition_id, protocol_version, "DAVE prepare transition");

                let ready = serde_json::json!({
                    "op": 25u8,
                    "d": {
                        "transition_id": transition_id,
                    }
                });
                if let Ok(text) = serde_json::to_string(&ready) {
                    let _ = write_tx.send(Message::Text(text.into())).await;
                }
                debug!(transition_id, "sent DAVE transition ready");
            }

            Some(21) => debug!("DAVE MLS external sender received"),
            Some(22) => debug!("DAVE MLS key package"),
            Some(23) => debug!("DAVE MLS proposals received"),
            Some(24) => debug!("DAVE MLS commit welcome"),
            Some(28) => {
                let transition_id = payload["d"]["transition_id"].as_u64().unwrap_or(0);
                info!(transition_id, "DAVE execute transition (new epoch active)");
            }

            Some(op) => debug!(op, "unhandled voice opcode"),
            None => {}
        }
    }
}

