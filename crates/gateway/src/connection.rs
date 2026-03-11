use futures_util::{SinkExt, StreamExt};
use tokio::sync::{broadcast, mpsc, watch};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, error, info, warn};

use oxidian_core::{
    error::{Error as OxidianError, GatewayError},
    models::{guild::Guild, interaction::Interaction, message::Message as DiscordMessage},
};

use crate::{
    events::{
        DispatchEvent, GatewayPayload, HelloData, MessageDeleteData, ReadyData,
        UnavailableGuild, VoiceServerUpdateData, VoiceStateUpdateData,
    },
    heartbeat::{self, HeartbeatMessage, WsMessageTx},
    opcodes::Opcode,
};

const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

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

/// Open a WebSocket connection to the Discord gateway and drive the event loop
/// until the connection closes or an unrecoverable error is encountered.
///
/// Parsed dispatch events are forwarded on `event_tx`. Returns when the
/// connection closes cleanly or on an error that the caller should handle
/// (e.g. by reconnecting with exponential back-off via [`crate::Shard`]).
pub async fn connect(
    token: &str,
    intents: u64,
    event_tx: mpsc::Sender<DispatchEvent>,
    mut outbound_rx: broadcast::Receiver<serde_json::Value>,
) -> Result<(), OxidianError> {
    info!(url = GATEWAY_URL, "connecting to Discord gateway");

    let (ws, _) = connect_async(GATEWAY_URL)
        .await
        .map_err(|e| GatewayError::Connection(e.to_string()))?;

    let (sink, mut stream) = ws.split();

    // Sequence number — written by the event loop, read by the heartbeat task.
    let (seq_tx, seq_rx) = watch::channel::<Option<u64>>(None);


    let hello = recv_hello(&mut stream).await?;
    info!(interval_ms = hello.heartbeat_interval, "received Hello from gateway");

    // all outgoing messages are funneled through this channel
    let (write_tx, write_rx) = mpsc::channel::<Message>(64);
    tokio::spawn(write_loop(sink, write_rx));

    // forward broadcast messages (e.g. VoiceStateUpdate) to the write channel
    {
        let write_tx_out = write_tx.clone();
        tokio::spawn(async move {
            loop {
                match outbound_rx.recv().await {
                    Ok(value) => {
                        if let Ok(text) = serde_json::to_string(&value) {
                            if write_tx_out.send(Message::Text(text.into())).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        });
    }

    send_identify(&write_tx, token, intents).await?;
    info!("sent Identify");

    let hb_tx = heartbeat::spawn(hello.heartbeat_interval, seq_rx, write_tx.clone());

    while let Some(result) = stream.next().await {
        let msg = match result {
            Ok(m) => m,
            Err(e) => {
                error!(error = %e, "WebSocket receive error");
                break;
            }
        };

        let text = match msg {
            Message::Text(t) => t.to_string(),
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
                warn!(error = %e, "failed to deserialize gateway payload — skipping");
                continue;
            }
        };

        if let Some(s) = payload.s {
            seq_tx.send_replace(Some(s));
        }

        let op = Opcode::from_u8(payload.op);
        debug!(op = ?op, event = ?payload.t, "received gateway payload");

        match op {
            Some(Opcode::Dispatch) => {
                if let Some(event) = parse_dispatch(payload) {
                    // Drop the event if the receiver is gone (bot is shutting down).
                    if event_tx.send(event).await.is_err() {
                        break;
                    }
                }
            }
            Some(Opcode::HeartbeatAck) => {
                let _ = hb_tx.send(HeartbeatMessage::Ack).await;
            }
            Some(Opcode::Heartbeat) => {
                // Gateway requests an immediate heartbeat.
                debug!("gateway requested immediate heartbeat");
                let seq = *seq_tx.borrow();
                let msg = heartbeat::build_heartbeat(seq);
                let _ = write_tx.send(msg).await;
            }
            Some(Opcode::Reconnect) => {
                info!("gateway requested reconnect — restarting session");
                let _ = hb_tx.send(HeartbeatMessage::Stop).await;
                return Err(GatewayError::Connection(
                    "gateway requested reconnect (op 7)".to_owned(),
                )
                .into());
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


async fn write_loop(mut sink: WsSink, mut rx: mpsc::Receiver<Message>) {
    while let Some(msg) = rx.recv().await {
        if sink.send(msg).await.is_err() {
            break;
        }
    }
}


async fn recv_hello(stream: &mut WsStream) -> Result<HelloData, OxidianError> {
    while let Some(msg) = stream.next().await {
        let text = match msg {
            Ok(Message::Text(t)) => t.to_string(),
            Ok(_) => continue,
            Err(e) => {
                return Err(GatewayError::Connection(format!(
                    "error waiting for Hello: {e}"
                ))
                .into())
            }
        };

        let payload: GatewayPayload =
            serde_json::from_str(&text).map_err(OxidianError::Serialization)?;

        if payload.op == Opcode::Hello as u8 {
            let data = payload.d.ok_or_else(|| {
                GatewayError::Connection("Hello payload missing 'd' field".to_owned())
            })?;
            return serde_json::from_value(data).map_err(OxidianError::Serialization);
        }
    }

    Err(GatewayError::Connection(
        "stream ended before Hello was received".to_owned(),
    )
    .into())
}

/// Send an Identify payload (op 2) through the write channel.
async fn send_identify(
    write_tx: &WsMessageTx,
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
    write_tx
        .send(Message::Text(text.into()))
        .await
        .map_err(|_| {
            GatewayError::Connection(
                "write channel closed before Identify was sent".to_owned(),
            )
            .into()
        })
}

/// Parse a raw dispatch payload into a typed [`DispatchEvent`].
fn parse_dispatch(payload: GatewayPayload) -> Option<DispatchEvent> {
    let name = payload.t.as_deref().unwrap_or("");
    let data = payload.d.unwrap_or(serde_json::Value::Null);

    match name {
        "READY" => match serde_json::from_value::<ReadyData>(data) {
            Ok(ready) => {
                info!(
                    username = %ready.user.username,
                    id = %ready.user.id,
                    gateway_version = ready.v,
                    session_id = %ready.session_id,
                    "bot is READY"
                );
                Some(DispatchEvent::Ready(ready))
            }
            Err(e) => {
                warn!(error = %e, "failed to parse READY payload");
                None
            }
        },
        "MESSAGE_CREATE" => match serde_json::from_value::<DiscordMessage>(data) {
            Ok(msg) => {
                debug!(
                    channel = %msg.channel_id,
                    author  = %msg.author.username,
                    "MESSAGE_CREATE"
                );
                Some(DispatchEvent::MessageCreate(msg))
            }
            Err(e) => {
                warn!(error = %e, "failed to parse MESSAGE_CREATE payload");
                None
            }
        },
        "MESSAGE_DELETE" => match serde_json::from_value::<MessageDeleteData>(data) {
            Ok(d) => Some(DispatchEvent::MessageDelete(d)),
            Err(e) => {
                warn!(error = %e, "failed to parse MESSAGE_DELETE payload");
                None
            }
        },
        "GUILD_CREATE" => match serde_json::from_value::<Guild>(data) {
            Ok(guild) => {
                debug!(guild = %guild.name, id = %guild.id, "GUILD_CREATE");
                Some(DispatchEvent::GuildCreate(guild))
            }
            Err(e) => {
                warn!(error = %e, "failed to parse GUILD_CREATE payload");
                None
            }
        },
        "GUILD_UPDATE" => match serde_json::from_value::<Guild>(data) {
            Ok(guild) => Some(DispatchEvent::GuildUpdate(guild)),
            Err(e) => {
                warn!(error = %e, "failed to parse GUILD_UPDATE payload");
                None
            }
        },
        "GUILD_DELETE" => match serde_json::from_value::<UnavailableGuild>(data) {
            Ok(g) => Some(DispatchEvent::GuildDelete(g)),
            Err(e) => {
                warn!(error = %e, "failed to parse GUILD_DELETE payload");
                None
            }
        },
        "INTERACTION_CREATE" => match serde_json::from_value::<Interaction>(data) {
            Ok(interaction) => {
                debug!(id = %interaction.id, "INTERACTION_CREATE");
                Some(DispatchEvent::InteractionCreate(interaction))
            }
            Err(e) => {
                warn!(error = %e, "failed to parse INTERACTION_CREATE payload");
                None
            }
        },
        "VOICE_STATE_UPDATE" => {
            debug!(raw = %data, "raw VOICE_STATE_UPDATE");
            match serde_json::from_value::<VoiceStateUpdateData>(data) {
                Ok(state) => Some(DispatchEvent::VoiceStateUpdate(state)),
                Err(e) => {
                    warn!(error = %e, "failed to parse VOICE_STATE_UPDATE payload");
                    None
                }
            }
        },
        "VOICE_SERVER_UPDATE" => {
            debug!(raw = %data, "raw VOICE_SERVER_UPDATE");
            match serde_json::from_value::<VoiceServerUpdateData>(data) {
                Ok(server) => Some(DispatchEvent::VoiceServerUpdate(server)),
                Err(e) => {
                    warn!(error = %e, "failed to parse VOICE_SERVER_UPDATE payload");
                    None
                }
            }
        },
        other => {
            debug!(event = other, "unhandled dispatch event type");
            Some(DispatchEvent::Unknown {
                name: other.to_owned(),
                data,
            })
        }
    }
}

