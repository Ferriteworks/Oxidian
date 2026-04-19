// MIT License
//
// Copyright (c) 2026 Ferriteworks organization and its rightful owners.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::time::Duration;

use tokio::{
    sync::{mpsc, watch},
    time,
};
use tracing::{debug, warn};

use crate::opcodes::Opcode;

pub type WsMessageTx = mpsc::Sender<tokio_tungstenite::tungstenite::Message>;

#[derive(Debug)]
pub enum HeartbeatMessage {
    Ack,
    Stop,
}

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
                        warn!("gateway heartbeat not acknowledged: connection presumed lost");
                        break;
                    }

                    let seq = *seq_rx.borrow();
                    let msg = build_heartbeat(seq);
                    debug!(seq = ?seq, "sending heartbeat");

                    if write_tx.send(msg).await.is_err() {
                        warn!("heartbeat write channel closed: stopping heartbeat task");
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
        None => serde_json::json!({ "op": Opcode::Heartbeat as u8, "d": null }),
    };
    tokio_tungstenite::tungstenite::Message::Text(
        serde_json::to_string(&payload)
            .unwrap_or_else(|_| r#"{"op":1,"d":null}"#.to_owned())
            .into(),
    )
}
