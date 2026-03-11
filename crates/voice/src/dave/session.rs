//! DAVE session — combines protocol state with per-sender key sets.
//!
//! A `DaveSession` is created when a voice channel uses E2EE and is updated
//! as members join or leave and as new MLS epochs are committed.
//!
//! **TODO**: Full MLS group management and AEAD key derivation.

use super::{keys::KeySet, protocol::DaveProtocol};

/// Manages DAVE E2EE state for a single voice channel session.
#[derive(Debug, Clone)]
pub struct DaveSession {
    pub protocol: DaveProtocol,
    pub keys: Vec<KeySet>,
}

impl DaveSession {
    /// Create an inactive DAVE session (DAVE not yet negotiated).
    pub fn inactive() -> Self {
        Self {
            protocol: DaveProtocol::Inactive,
            keys: Vec::new(),
        }
    }

    /// Whether DAVE E2EE is currently active.
    pub fn is_active(&self) -> bool {
        matches!(self.protocol, DaveProtocol::Active { .. })
    }
}
