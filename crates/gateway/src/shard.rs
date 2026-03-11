use std::sync::Arc;

use tokio::sync::{broadcast, mpsc};
use tracing::{error, info, warn};

use oxidian_core::{
    error::{Error as OxidianError, GatewayError},
    intents::Intents,
};

use crate::{connection::{self, SessionState}, events::DispatchEvent};

/// A single Discord gateway shard.
pub struct Shard {
    token: String,
    intents: Intents,
    event_tx: mpsc::Sender<DispatchEvent>,
    outbound_tx: Arc<broadcast::Sender<serde_json::Value>>,
}

impl Shard {
    /// Create a new `Shard`.
    pub fn new(
        token: impl Into<String>,
        intents: Intents,
        event_tx: mpsc::Sender<DispatchEvent>,
    ) -> Self {
        let (outbound_tx, _) = broadcast::channel(64);
        Self {
            token: token.into(),
            intents,
            event_tx,
            outbound_tx: Arc::new(outbound_tx),
        }
    }

    /// Return a clone of the broadcast sender used for outbound gateway
    /// payloads.  Messages sent when no connection is active are discarded.
    pub fn gateway_sender(&self) -> Arc<broadcast::Sender<serde_json::Value>> {
        Arc::clone(&self.outbound_tx)
    }

    /// Connect to the Discord gateway and drive the event loop.
    ///
    /// Retries on transient errors (connection drops, op 7 reconnect, op 9
    /// invalid session) up to a fixed limit before giving up.
    pub async fn start(&self) -> Result<(), OxidianError> {
        const MAX_RETRIES: u32 = 5;
        let mut attempts = 0u32;
        let mut session: Option<SessionState> = None;

        loop {
            info!(attempt = attempts + 1, resuming = session.is_some(), "starting gateway session");

            // Subscribe before each connection so the new socket drains
            // outbound messages sent during the lifetime of that session.
            let outbound_rx = self.outbound_tx.subscribe();

            match connection::connect(
                &self.token,
                self.intents,
                self.event_tx.clone(),
                outbound_rx,
                session.take(),
            ).await {
                Ok(None) => {
                    info!("gateway connection closed cleanly");
                    return Ok(());
                }
                Ok(Some(state)) => {
                    // Gateway sent op 7 or op 9 (resumable) — reconnect and resume.
                    attempts += 1;
                    if attempts >= MAX_RETRIES {
                        error!(attempts, "exceeded maximum gateway reconnect attempts");
                        return Err(GatewayError::Connection(
                            format!("gave up after {MAX_RETRIES} reconnect attempts"),
                        )
                        .into());
                    }
                    let delay = std::time::Duration::from_millis(500 * u64::from(attempts));
                    warn!(delay_ms = delay.as_millis(), session_id = %state.session_id, "reconnecting to resume session");
                    tokio::time::sleep(delay).await;
                    session = Some(state);
                }
                Err(OxidianError::Gateway(GatewayError::SessionInvalidated { resumable: false }))
                | Err(OxidianError::Gateway(GatewayError::Connection(_))) => {
                    // Non-resumable — start a fresh session after backoff.
                    session = None;
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
                        "transient error — reconnecting with fresh Identify after backoff"
                    );
                    tokio::time::sleep(backoff).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}

