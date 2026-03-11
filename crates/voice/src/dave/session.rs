//! DAVE session — combines protocol state with per-sender key sets.
//!
//! A `DaveSession` is created when a voice channel uses E2EE and is updated
//! as members join or leave and as new MLS epochs are committed.
//!
//! ## Current status
//!
//! The session tracks which DAVE phase the connection is in and stores
//! placeholder key sets per SSRC.  Full MLS group management and AEAD key
//! derivation will be added in a future release.

use super::{keys::SenderKeySet, protocol::DaveState};

/// Manages DAVE E2EE state for a single voice channel session.
#[derive(Debug)]
pub struct DaveSession {
    /// Protocol negotiation state machine.
    pub state: DaveState,
    /// Per-sender key sets, indexed by SSRC.
    pub sender_keys: Vec<SenderKeySet>,
}

impl DaveSession {
    /// Create an inactive DAVE session (DAVE not yet negotiated).
    pub fn new() -> Self {
        Self {
            state: DaveState::new(),
            sender_keys: Vec::new(),
        }
    }

    /// Whether DAVE E2EE is currently active.
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    /// Get the key set for a given SSRC, if one exists.
    pub fn get_sender_key(&self, ssrc: u32) -> Option<&SenderKeySet> {
        self.sender_keys.iter().find(|k| k.ssrc == ssrc)
    }

    /// Register a new sender SSRC with a placeholder key set.
    pub fn register_sender(&mut self, ssrc: u32) {
        if !self.sender_keys.iter().any(|k| k.ssrc == ssrc) {
            self.sender_keys.push(SenderKeySet::placeholder(ssrc));
        }
    }

    /// Remove a sender by SSRC (e.g. when they leave the voice channel).
    pub fn remove_sender(&mut self, ssrc: u32) {
        self.sender_keys.retain(|k| k.ssrc != ssrc);
    }
}

impl Default for DaveSession {
    fn default() -> Self {
        Self::new()
    }
}
