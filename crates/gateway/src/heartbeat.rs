use std::time::Duration;

use tokio::{
    sync::{mpsc, watch},
    time,
};
use tracing::{debug, warn};

use crate::opcodes::Opcode;

/// Sender half of the write channel — used to push raw WebSocket messages
/// to the write task that owns the WsSink.
pub type WsMessageTx = mpsc::Sender<tokio_tungstenite::tungstenite::Message>;

/// Message type sent over the heartbeat control channel.
#[derive(Debug)]
pub enum HeartbeatMessage {
    /// The gateway acknowledged our last heartbeat (opcode 11).
    Ack,
    /// The heartbeat task should stop cleanly.
    Stop,
}

/// Spawn the heartbeat loop as an independent Tokio task.
///
/// Messages are written to the gateway via `write_tx`, a channel whose
/// receiver is owned by the write task.  The returned sender lets the event
/// loop send `Ack` / `Stop` control signals.
pub fn spawn(
    interval_ms: u64,
    seq_rx: watch::Receiver<Option<u64>>,
    write_tx: WsMessageTx,
) -> mpsc::Sender<HeartbeatMessage> {
    let (tx, mut rx) = mpsc::channel::<HeartbeatMessage>(8);

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(interval_ms));
        // Discard the first tick which fires immediately.
        interval.tick().await;

        let mut acked = true;

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if !acked {
                        warn!("gateway heartbeat not acknowledged — connection presumed lost");
                        break;
                    }

                    let seq = *seq_rx.borrow();
                    let msg = build_heartbeat(seq);
                    debug!(seq = ?seq, "sending heartbeat");

                    if write_tx.send(msg).await.is_err() {
                        warn!("heartbeat write channel closed — stopping heartbeat task");
                        break;
                    }

                    acked = false;
                }

                ctrl = rx.recv() => {
                    match ctrl {
                        Some(HeartbeatMessage::Ack) => {
                            debug!("heartbeat acknowledged by gateway");
                            acked = true;
                        }
                        Some(HeartbeatMessage::Stop) | None => {
                            debug!("heartbeat task stopping cleanly");
                            break;
                        }
                    }
                }
            }
        }
    });

    tx
}

/// Build an opcode-1 heartbeat payload with the current sequence number.
/// `pub(crate)` so `connection.rs` can call it for immediate heartbeat requests.
pub(crate) fn build_heartbeat(
    seq: Option<u64>,
) -> tokio_tungstenite::tungstenite::Message {
    let payload = match seq {
        Some(s) => serde_json::json!({ "op": Opcode::Heartbeat as u8, "d": s }),
        None    => serde_json::json!({ "op": Opcode::Heartbeat as u8, "d": null }),
    };
    tokio_tungstenite::tungstenite::Message::Text(
        serde_json::to_string(&payload)
            .unwrap_or_else(|_| r#"{"op":1,"d":null}"#.to_owned())
            .into(),
    )
}

