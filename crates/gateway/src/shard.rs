use tracing::{error, info, warn};

use oxidian_core::error::{Error as OxidianError, GatewayError};

use crate::connection;

/// A single Discord gateway shard.
///
/// Each shard maintains one WebSocket connection to the Discord gateway and
/// handles its share of guild events as determined by Discord's sharding
/// algorithm.  Right now the implementation is a thin wrapper around the
/// connection loop; session tracking and RESUME will be added in a follow-up.
pub struct Shard {
    /// The bot token (without the `"Bot "` prefix).
    token: String,
    /// The [Gateway Intents](https://discord.com/developers/docs/topics/gateway#gateway-intents)
    /// bitmask for this shard.
    intents: u64,
}

impl Shard {
    /// Create a new `Shard` with the given token and intents bitmask.
    pub fn new(token: impl Into<String>, intents: u64) -> Self {
        Self {
            token: token.into(),
            intents,
        }
    }

    /// Connect to the Discord gateway and drive the event loop.
    ///
    /// This method blocks (async) until the connection is closed or an
    /// unrecoverable error occurs.  Transient errors (reconnects, invalid
    /// sessions) are handled internally up to a retry limit.
    pub async fn start(&self) -> Result<(), OxidianError> {
        const MAX_RETRIES: u32 = 5;
        let mut attempts = 0u32;

        loop {
            info!(attempt = attempts + 1, "starting gateway session");

            match connection::connect(&self.token, self.intents).await {
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
                        ).into());
                    }
                    let backoff = std::time::Duration::from_secs(2u64.pow(attempts.min(6)));
                    warn!(backoff_secs = backoff.as_secs(), "reconnecting after backoff");
                    tokio::time::sleep(backoff).await;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
