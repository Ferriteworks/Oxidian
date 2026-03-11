//! DAVE (Dave's Awesome Voice Encryption) key management.
//!
//! DAVE is Discord's end-to-end encryption protocol for voice, built on the
//! MLS (Messaging Layer Security) standard.  Key distribution is handled
//! through the voice WebSocket using opcodes 14–24.
//!
//! **TODO**: Full MLS key schedule and ratchet implementation.

/// A set of DAVE per-sender encryption keys.
///
/// Each participant in the voice channel has a unique key set that is
/// distributed via MLS key packages and commit/welcome messages.
#[derive(Debug, Clone)]
pub struct KeySet {
    // TODO: Add MLS group state and per-sender keys.
    pub user_id: u64,
    pub key_id: u32,
}
