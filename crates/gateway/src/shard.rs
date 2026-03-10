use tokio::sync::mpsc;
use tracing::{error, info, warn};

use oxidian_core::error::{Error as OxidianError, GatewayError};

use crate::{connection, events::DispatchEvent};

/// A single Discord gateway shard.
///
/// Each shard maintains one WebSocket connection to the Discord gateway.
/// Create one with [`Shard::new`] and call [`Shard::start`] to connect.
/// Parsed dispatch events are forwarded to the caller via the `event_tx` channel.
pub struct Shard {
    token: String,
    intents: u64,
    /// Channel sender — parsed events are sent here for the caller to handle.
    event_tx: mpsc::Sender<DispatchEvent>,
}

impl Shard {
    /// Create a new `Shard`.
    ///
    /// All parsed gateway dispatch events will be sent on `event_tx`.
    /// The receiver side is typically owned by a `Bot` which dispatches them
    /// to user-defined handlers.
    pub fn new(
        token: impl Into<String>,
        intents: u64,
        event_tx: mpsc::Sender<DispatchEvent>,
    ) -> Self {
        Self { token: token.into(), intents, event_tx }
    }

    /// Connect to the Discord gateway and drive the event loop.
    ///
    /// Retries on transient errors (connection drops, op 7 reconnect, op 9
    /// invalid session) up to a fixed limit before giving up.
    pub async fn start(&self) -> Result<(), OxidianError> {
        const MAX_RETRIES: u32 = 5;
        let mut attempts = 0u32;

        loop {
            info!(attempt = attempts + 1, "starting gateway session");

            match connection::connect(&self.token, self.intents, self.event_tx.clone()).await {
                Ok(()) => {
                    info!("gateway connection closed cleanly");
                    return Ok(());
                }
                Err(OxidianError::Gateway(GatewayError::SessionInvalidated { resumable: false }))
                | Err(OxidianError::Gateway(GatewayError::Connection(_))) => {
                    attempts += 1;
                    if attempts >= MAX_RETRIES {
                        error!(attempts, "exceeded maximum gateway reconnect attempts");
                        return Err(GatewayError::Connection(
                            format!("gave up after {MAX_RETRIES} reconnect attempts"),
                        )
                        .into());
                    }
                    let backoff = std::time::Duration::from_secs(2u64.pow(attempts.min(6)));
                    warn!(
                        backoff_secs = backoff.as_secs(),
                        "transient error — reconnecting after backoff"
                    );
                    tokio::time::sleep(backoff).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}

