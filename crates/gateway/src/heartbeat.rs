use std::time::Duration;

use futures_util::SinkExt;
use tokio::{
    sync::{
        mpsc,
        watch,
    },
    time,
};
use tracing::{debug, warn};

use crate::connection::WsSink;
use crate::opcodes::Opcode;

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
/// The task sends opcode 1 every `interval` milliseconds, using the latest
/// sequence number broadcast on `seq_rx`. If a `HeartbeatMessage::Ack` is not
/// received between two consecutive heartbeats the connection is considered
/// lost and a `GatewayError::HeartbeatTimeout` is logged and propagated.
///
/// Returns a [`mpsc::Sender`] through which the event loop sends
/// `HeartbeatMessage` control signals to the task.
pub fn spawn(
    interval_ms: u64,
    seq_rx: watch::Receiver<Option<u64>>,
    sink: WsSink,
) -> mpsc::Sender<HeartbeatMessage> {
    let (tx, mut rx) = mpsc::channel::<HeartbeatMessage>(8);

    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(interval_ms));
        // Discard the first tick which fires immediately.
        interval.tick().await;

        let mut acked = true;
        let mut sink = sink;

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if !acked {
                        warn!("gateway heartbeat was not acknowledged — connection is lost");
                        // The connection loop will notice the sink is gone and reconnect.
                        break;
                    }

                    let seq = *seq_rx.borrow();
                    let payload = build_heartbeat(seq);
                    debug!(seq = ?seq, "sending heartbeat");

                    if sink.send(payload).await.is_err() {
                        warn!("heartbeat sink closed — stopping heartbeat task");
                        break;
                    }

                    acked = false;
                }

                msg = rx.recv() => {
                    match msg {
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

/// Build a serialised heartbeat payload (opcode 1) with the given sequence number.
fn build_heartbeat(seq: Option<u64>) -> tokio_tungstenite::tungstenite::Message {
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
