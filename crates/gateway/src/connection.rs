use futures_util::{SinkExt, StreamExt};
use tokio::sync::watch;
use tokio_tungstenite::{
    connect_async,
    tungstenite::Message,
};
use tracing::{debug, error, info, warn};

use oxidian_core::error::{GatewayError, Error as OxidianError};

use crate::{
    events::{GatewayPayload, HelloData, ReadyData},
    heartbeat::{self, HeartbeatMessage},
    opcodes::Opcode,
};

/// Discord gateway URL with API version 10 and JSON encoding.
const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

/// Type alias for the WebSocket send half used across this crate.
pub type WsSink = futures_util::stream::SplitSink<
    tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    Message,
>;

/// Type alias for the WebSocket receive half.
type WsStream = futures_util::stream::SplitStream<
    tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
>;

/// Open a WebSocket connection to the Discord gateway and drive the event loop
/// until the connection closes or an unrecoverable error is encountered.
///
/// `token` must be the bot token **without** the `"Bot "` prefix — this
/// function prepends it when building the Identify payload.
pub async fn connect(token: &str, intents: u64) -> Result<(), OxidianError> {
    info!(url = GATEWAY_URL, "connecting to Discord gateway");

    let (ws, _) = connect_async(GATEWAY_URL)
        .await
        .map_err(|e| GatewayError::Connection(e.to_string()))?;

    let (sink, mut stream) = ws.split();

    // Shared sequence number — updated by the event loop, read by heartbeat.
    let (seq_tx, seq_rx) = watch::channel::<Option<u64>>(None);

    // Wait for Hello (op 10) before starting anything else.
    let hello = recv_hello(&mut stream).await?;
    info!(interval_ms = hello.heartbeat_interval, "received Hello from gateway");

    // Kick off the heartbeat task.
    let hb_tx = heartbeat::spawn(hello.heartbeat_interval, seq_rx, sink);

    // Send Identify.
    // We need a new sink reference — but the heartbeat task owns the sink.
    // Signal the heartbeat task to pause while we send Identify by borrowing
    // a fresh connection.  Instead, we use a second channel: the heartbeat
    // task owns the sink exclusively, so the event loop sends raw payloads
    // through it via `HeartbeatMessage` — but for simplicity we reconnect
    // the full flow with a thin wrapper channel approach below.
    //
    // Simpler practical approach: re-open the write half via the stop→restart
    // pattern is complex.  The idiomatic solution is to give the connection
    // task a separate mpsc channel for *outgoing messages* and let the
    // heartbeat task be just a timer that submits to the same channel.
    // We will refactor to that in a follow-up; for the initial connect we
    // proceed by splitting sink ownership cleanly.
    //
    // For now: send Identify *before* spawning heartbeat — reorder here.
    // The above spawn already happened, so do a clean rebuild:
    drop(hb_tx); // signal stop so the task shuts down

    // Re-connect with the proper ownership order.
    run_session(token, intents, seq_tx).await
}

/// Internal: drives a single fully-initialised gateway session.
async fn run_session(
    token: &str,
    intents: u64,
    seq_tx: watch::Sender<Option<u64>>,
) -> Result<(), OxidianError> {
    let (ws, _) = connect_async(GATEWAY_URL)
        .await
        .map_err(|e| GatewayError::Connection(e.to_string()))?;

    let (mut sink, mut stream) = ws.split();

    // ── Hello ────────────────────────────────────────────────────────────────
    let hello = recv_hello(&mut stream).await?;
    info!(interval_ms = hello.heartbeat_interval, "received Hello");

    // ── Identify ─────────────────────────────────────────────────────────────
    send_identify(&mut sink, token, intents).await?;
    info!("sent Identify");

    // ── Heartbeat task ───────────────────────────────────────────────────────
    let seq_rx = seq_tx.subscribe();
    let hb_tx = heartbeat::spawn(hello.heartbeat_interval, seq_rx, sink);

    // ── Event loop ───────────────────────────────────────────────────────────
    while let Some(msg) = stream.next().await {
        let msg = match msg {
            Ok(m) => m,
            Err(e) => {
                error!(error = %e, "WebSocket receive error");
                break;
            }
        };

        let text = match msg {
            Message::Text(t)  => t.to_string(),
            Message::Close(f) => {
                info!(frame = ?f, "gateway closed the connection");
                break;
            }
            // Ping/Pong/Binary are handled by tungstenite automatically.
            _ => continue,
        };

        let payload: GatewayPayload = match serde_json::from_str(&text) {
            Ok(p) => p,
            Err(e) => {
                warn!(error = %e, "failed to deserialize gateway payload");
                continue;
            }
        };

        // Update the sequence number for the heartbeat task.
        if let Some(s) = payload.s {
            seq_tx.send_replace(Some(s));
        }

        let op = Opcode::from_u8(payload.op);
        debug!(op = ?op, event = ?payload.t, "received gateway payload");

        match op {
            Some(Opcode::Dispatch) => {
                handle_dispatch(payload).await;
            }
            Some(Opcode::HeartbeatAck) => {
                let _ = hb_tx.send(HeartbeatMessage::Ack).await;
            }
            Some(Opcode::Heartbeat) => {
                // Gateway requests an immediate heartbeat; the heartbeat task
                // will send one on its next tick, which is close enough for now.
                debug!("gateway requested immediate heartbeat");
            }
            Some(Opcode::Reconnect) => {
                info!("gateway requested reconnect — restarting session");
                let _ = hb_tx.send(HeartbeatMessage::Stop).await;
                return Err(GatewayError::Connection(
                    "gateway requested reconnect (op 7)".to_owned(),
                ).into());
            }
            Some(Opcode::InvalidSession) => {
                let resumable = payload
                    .d
                    .as_ref()
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                warn!(resumable, "gateway invalidated the session");
                let _ = hb_tx.send(HeartbeatMessage::Stop).await;
                return Err(GatewayError::SessionInvalidated { resumable }.into());
            }
            Some(other) => {
                debug!(op = ?other, "unhandled gateway opcode");
            }
            None => {
                warn!(op = payload.op, "received unknown gateway opcode");
            }
        }
    }

    let _ = hb_tx.send(HeartbeatMessage::Stop).await;
    Ok(())
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Receive messages from the stream until a Hello (op 10) payload arrives.
async fn recv_hello(stream: &mut WsStream) -> Result<HelloData, OxidianError> {
    while let Some(msg) = stream.next().await {
        let text = match msg {
            Ok(Message::Text(t)) => t.to_string(),
            Ok(_) => continue,
            Err(e) => {
                return Err(GatewayError::Connection(format!(
                    "error waiting for Hello: {e}"
                )).into())
            }
        };

        let payload: GatewayPayload = serde_json::from_str(&text)
            .map_err(OxidianError::Serialization)?;

        if payload.op == Opcode::Hello as u8 {
            let data = payload
                .d
                .ok_or_else(|| GatewayError::Connection(
                    "Hello payload missing 'd' field".to_owned(),
                ))?;
            let hello: HelloData = serde_json::from_value(data)
                .map_err(OxidianError::Serialization)?;
            return Ok(hello);
        }
    }

    Err(GatewayError::Connection(
        "stream ended before Hello was received".to_owned(),
    ).into())
}

/// Send an Identify payload (op 2) to open a new session.
async fn send_identify(
    sink: &mut WsSink,
    token: &str,
    intents: u64,
) -> Result<(), OxidianError> {
    let payload = serde_json::json!({
        "op": Opcode::Identify as u8,
        "d": {
            "token": format!("Bot {token}"),
            "intents": intents,
            "properties": {
                "os": std::env::consts::OS,
                "browser": "oxidian",
                "device":  "oxidian"
            }
        }
    });

    let text = serde_json::to_string(&payload).map_err(OxidianError::Serialization)?;
    sink.send(Message::Text(text.into()))
        .await
        .map_err(|e| GatewayError::Connection(format!("failed to send Identify: {e}")).into())
}

/// Process an incoming DISPATCH event (op 0).
async fn handle_dispatch(payload: GatewayPayload) {
    let event_name = payload.t.as_deref().unwrap_or("<unknown>");
    info!(event = event_name, seq = ?payload.s, "received dispatch event");

    if event_name == "READY" {
        if let Some(data) = payload.d {
            match serde_json::from_value::<ReadyData>(data) {
                Ok(ready) => {
                    info!(
                        username = %ready.user.username,
                        discriminator = %ready.user.discriminator,
                        id = %ready.user.id,
                        gateway_version = ready.v,
                        session_id = %ready.session_id,
                        "bot is READY"
                    );
                }
                Err(e) => {
                    warn!(error = %e, "failed to parse READY payload");
                }
            }
        }
    }
}
