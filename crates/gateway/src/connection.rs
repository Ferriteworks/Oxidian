use futures_util::{SinkExt, StreamExt};
use tokio::sync::{broadcast, mpsc, watch};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tracing::{debug, error, info, warn};

use oxidian_core::{
    error::{Error as OxidianError, GatewayError},
    intents::Intents,
    models::{
        channel::Channel, guild::Guild, interaction::Interaction,
        message::Message as DiscordMessage,
    },
};

use crate::{
    events::{
        DispatchEvent, GatewayPayload, GuildBanData, GuildMemberAddData,
        GuildMemberRemoveData, GuildRoleData, GuildRoleDeleteData, HelloData,
        MessageDeleteBulkData, MessageDeleteData, ReactionData, ReactionRemoveAllData,
        ReactionRemoveEmojiData, ReadyData, TypingStartData, UnavailableGuild,
        VoiceServerUpdateData, VoiceStateUpdateData,
    },
    heartbeat::{self, HeartbeatMessage, WsMessageTx},
    opcodes::Opcode,
};

const GATEWAY_URL: &str = "wss://gateway.discord.gg/?v=10&encoding=json";

/// Enough information to resume a disconnected gateway session.
///
/// Returned by [`connect`] when the connection closes in a resumable state
/// (op 7 Reconnect or op 9 InvalidSession with `d = true`).  Pass it back
/// into the next [`connect`] call to send a Resume instead of Identify.
#[derive(Debug, Clone)]
pub struct SessionState {
    /// The session ID received in the `READY` payload.
    pub session_id: String,
    /// The resume URL received in the `READY` payload.
    pub resume_gateway_url: String,
    /// The last sequence number processed in this session.
    pub last_seq: Option<u64>,
}

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
/// - Pass `session = None` to start a fresh session (sends Identify).
/// - Pass `session = Some(state)` to attempt a resume (connects to the
///   resume URL and sends Resume/op 6 instead of Identify).
///
/// Returns `Ok(Some(state))` when the connection closes in a resumable state
/// (op 7 or op 9 resumable).  The caller should reconnect and pass the
/// returned state back in.
///
/// Returns `Ok(None)` on a clean close.
/// Returns `Err` on a non-resumable invalidation or a fatal error.
pub async fn connect(
    token: &str,
    intents: Intents,
    event_tx: mpsc::Sender<DispatchEvent>,
    mut outbound_rx: broadcast::Receiver<serde_json::Value>,
    session: Option<SessionState>,
) -> Result<Option<SessionState>, OxidianError> {
    let gateway_url = match session.as_ref() {
        Some(s) => format!("{}/?v=10&encoding=json", s.resume_gateway_url),
        None => GATEWAY_URL.to_owned(),
    };
    info!(url = %gateway_url, "connecting to Discord gateway");

    let (ws, _) = connect_async(gateway_url.as_str())
        .await
        .map_err(|e| GatewayError::Connection(e.to_string()))?;

    let (sink, mut stream) = ws.split();

    // Sequence number — written by the event loop, read by the heartbeat task.
    let (seq_tx, seq_rx) = watch::channel::<Option<u64>>(None);

    // Track session identifiers so we can build a resume payload if needed.
    let mut current_session: Option<(String, String)> = session
        .as_ref()
        .map(|s| (s.session_id.clone(), s.resume_gateway_url.clone()));

    let hello = recv_hello(&mut stream).await?;
    info!(
        interval_ms = hello.heartbeat_interval,
        "received Hello from gateway"
    );

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
                            if write_tx_out
                                .send(Message::Text(text.into()))
                                .await
                                .is_err()
                            {
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

    if let Some(ref s) = session {
        send_resume(&write_tx, token, &s.session_id, s.last_seq).await?;
        info!(session_id = %s.session_id, "sent Resume");
    } else {
        send_identify(&write_tx, token, intents).await?;
        info!("sent Identify");
    }

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
                    // Capture session identifiers from READY (fresh connect or
                    // failed resume that triggered a new session).
                    if let DispatchEvent::Ready(ref ready) = event {
                        current_session = Some((
                            ready.session_id.clone(),
                            ready.resume_gateway_url.clone(),
                        ));
                        info!(session_id = %ready.session_id, "session established");
                    }
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
                info!("gateway requested reconnect (op 7) — will attempt resume");
                let _ = hb_tx.send(HeartbeatMessage::Stop).await;
                let state = current_session.map(|(session_id, resume_gateway_url)| {
                    SessionState {
                        session_id,
                        resume_gateway_url,
                        last_seq: *seq_tx.borrow(),
                    }
                });
                return Ok(state);
            }
            Some(Opcode::InvalidSession) => {
                let resumable = payload
                    .d
                    .as_ref()
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                warn!(resumable, "gateway invalidated the session");
                let _ = hb_tx.send(HeartbeatMessage::Stop).await;
                if resumable {
                    let state =
                        current_session.map(|(session_id, resume_gateway_url)| {
                            SessionState {
                                session_id,
                                resume_gateway_url,
                                last_seq: *seq_tx.borrow(),
                            }
                        });
                    return Ok(state);
                } else {
                    return Err(
                        GatewayError::SessionInvalidated { resumable: false }.into()
                    );
                }
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
    Ok(None)
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

    Err(
        GatewayError::Connection("stream ended before Hello was received".to_owned())
            .into(),
    )
}

/// Send an Identify payload (op 2) through the write channel.
async fn send_identify(
    write_tx: &WsMessageTx,
    token: &str,
    intents: Intents,
) -> Result<(), OxidianError> {
    let payload = serde_json::json!({
        "op": Opcode::Identify as u8,
        "d": {
            "token": format!("Bot {token}"),
            "intents": intents.bits(),
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

/// Send a Resume payload (op 6) through the write channel.
async fn send_resume(
    write_tx: &WsMessageTx,
    token: &str,
    session_id: &str,
    seq: Option<u64>,
) -> Result<(), OxidianError> {
    let payload = serde_json::json!({
        "op": Opcode::Resume as u8,
        "d": {
            "token": format!("Bot {token}"),
            "session_id": session_id,
            "seq": seq,
        }
    });
    let text = serde_json::to_string(&payload).map_err(OxidianError::Serialization)?;
    write_tx
        .send(Message::Text(text.into()))
        .await
        .map_err(|_| {
            GatewayError::Connection(
                "write channel closed before Resume was sent".to_owned(),
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
        "RESUMED" => {
            info!("session resumed successfully");
            Some(DispatchEvent::Resumed)
        }
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
        "MESSAGE_UPDATE" => match serde_json::from_value::<DiscordMessage>(data) {
            Ok(msg) => Some(DispatchEvent::MessageUpdate(msg)),
            Err(e) => {
                warn!(error = %e, "failed to parse MESSAGE_UPDATE payload");
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
        "MESSAGE_DELETE_BULK" => {
            match serde_json::from_value::<MessageDeleteBulkData>(data) {
                Ok(d) => Some(DispatchEvent::MessageDeleteBulk(d)),
                Err(e) => {
                    warn!(error = %e, "failed to parse MESSAGE_DELETE_BULK payload");
                    None
                }
            }
        }
        "MESSAGE_REACTION_ADD" => match serde_json::from_value::<ReactionData>(data) {
            Ok(d) => Some(DispatchEvent::MessageReactionAdd(d)),
            Err(e) => {
                warn!(error = %e, "failed to parse MESSAGE_REACTION_ADD payload");
                None
            }
        },
        "MESSAGE_REACTION_REMOVE" => match serde_json::from_value::<ReactionData>(data)
        {
            Ok(d) => Some(DispatchEvent::MessageReactionRemove(d)),
            Err(e) => {
                warn!(error = %e, "failed to parse MESSAGE_REACTION_REMOVE payload");
                None
            }
        },
        "MESSAGE_REACTION_REMOVE_ALL" => {
            match serde_json::from_value::<ReactionRemoveAllData>(data) {
                Ok(d) => Some(DispatchEvent::MessageReactionRemoveAll(d)),
                Err(e) => {
                    warn!(error = %e, "failed to parse MESSAGE_REACTION_REMOVE_ALL payload");
                    None
                }
            }
        }
        "MESSAGE_REACTION_REMOVE_EMOJI" => {
            match serde_json::from_value::<ReactionRemoveEmojiData>(data) {
                Ok(d) => Some(DispatchEvent::MessageReactionRemoveEmoji(d)),
                Err(e) => {
                    warn!(error = %e, "failed to parse MESSAGE_REACTION_REMOVE_EMOJI payload");
                    None
                }
            }
        }
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
        "GUILD_MEMBER_ADD" => {
            match serde_json::from_value::<GuildMemberAddData>(data) {
                Ok(d) => Some(DispatchEvent::GuildMemberAdd(d)),
                Err(e) => {
                    warn!(error = %e, "failed to parse GUILD_MEMBER_ADD payload");
                    None
                }
            }
        }
        "GUILD_MEMBER_REMOVE" => {
            match serde_json::from_value::<GuildMemberRemoveData>(data) {
                Ok(d) => Some(DispatchEvent::GuildMemberRemove(d)),
                Err(e) => {
                    warn!(error = %e, "failed to parse GUILD_MEMBER_REMOVE payload");
                    None
                }
            }
        }
        "GUILD_BAN_ADD" => match serde_json::from_value::<GuildBanData>(data) {
            Ok(d) => Some(DispatchEvent::GuildBanAdd(d)),
            Err(e) => {
                warn!(error = %e, "failed to parse GUILD_BAN_ADD payload");
                None
            }
        },
        "GUILD_BAN_REMOVE" => match serde_json::from_value::<GuildBanData>(data) {
            Ok(d) => Some(DispatchEvent::GuildBanRemove(d)),
            Err(e) => {
                warn!(error = %e, "failed to parse GUILD_BAN_REMOVE payload");
                None
            }
        },
        "GUILD_ROLE_CREATE" => match serde_json::from_value::<GuildRoleData>(data) {
            Ok(d) => Some(DispatchEvent::GuildRoleCreate(d)),
            Err(e) => {
                warn!(error = %e, "failed to parse GUILD_ROLE_CREATE payload");
                None
            }
        },
        "GUILD_ROLE_UPDATE" => match serde_json::from_value::<GuildRoleData>(data) {
            Ok(d) => Some(DispatchEvent::GuildRoleUpdate(d)),
            Err(e) => {
                warn!(error = %e, "failed to parse GUILD_ROLE_UPDATE payload");
                None
            }
        },
        "GUILD_ROLE_DELETE" => {
            match serde_json::from_value::<GuildRoleDeleteData>(data) {
                Ok(d) => Some(DispatchEvent::GuildRoleDelete(d)),
                Err(e) => {
                    warn!(error = %e, "failed to parse GUILD_ROLE_DELETE payload");
                    None
                }
            }
        }
        "CHANNEL_CREATE" => match serde_json::from_value::<Channel>(data) {
            Ok(c) => Some(DispatchEvent::ChannelCreate(c)),
            Err(e) => {
                warn!(error = %e, "failed to parse CHANNEL_CREATE payload");
                None
            }
        },
        "CHANNEL_UPDATE" => match serde_json::from_value::<Channel>(data) {
            Ok(c) => Some(DispatchEvent::ChannelUpdate(c)),
            Err(e) => {
                warn!(error = %e, "failed to parse CHANNEL_UPDATE payload");
                None
            }
        },
        "CHANNEL_DELETE" => match serde_json::from_value::<Channel>(data) {
            Ok(c) => Some(DispatchEvent::ChannelDelete(c)),
            Err(e) => {
                warn!(error = %e, "failed to parse CHANNEL_DELETE payload");
                None
            }
        },
        "TYPING_START" => match serde_json::from_value::<TypingStartData>(data) {
            Ok(d) => Some(DispatchEvent::TypingStart(d)),
            Err(e) => {
                warn!(error = %e, "failed to parse TYPING_START payload");
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
        }
        "VOICE_SERVER_UPDATE" => {
            debug!(raw = %data, "raw VOICE_SERVER_UPDATE");
            match serde_json::from_value::<VoiceServerUpdateData>(data) {
                Ok(server) => Some(DispatchEvent::VoiceServerUpdate(server)),
                Err(e) => {
                    warn!(error = %e, "failed to parse VOICE_SERVER_UPDATE payload");
                    None
                }
            }
        }
        other => {
            debug!(event = other, "unhandled dispatch event type");
            Some(DispatchEvent::Unknown {
                name: other.to_owned(),
                data,
            })
        }
    }
}
