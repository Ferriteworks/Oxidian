//! DAVE (Discord Audio/Video Encryption) key management.
//!
//! DAVE is Discord's end-to-end encryption protocol for voice, built on the
//! MLS (Messaging Layer Security) standard.  Key distribution is handled
//! through the voice WebSocket using opcodes 20–28.
//!
//! ## Current status
//!
//! The MLS group ratchet and per-sender key derivation are not yet
//! implemented.  The voice connection currently uses the standard
//! `aead_aes256_gcm_rtpsize` encryption with the server-provided secret key
//! from the Session Description (op 4).
//!
//! When DAVE is fully implemented, per-sender keys will be derived from the
//! MLS group state and used for an additional layer of encryption on top of
//! the standard RTP encryption.

/// A set of DAVE per-sender encryption keys for one voice participant.
///
/// Each participant has a unique key set distributed via MLS key packages
/// and commit/welcome messages.
#[derive(Debug, Clone)]
pub struct SenderKeySet {
    /// The SSRC of the sender this key set belongs to.
    pub ssrc: u32,
    /// The current key generation / epoch.
    pub generation: u64,
    /// The raw key material (derived from MLS; empty until MLS is implemented).
    pub key_material: Vec<u8>,
}

impl SenderKeySet {
    /// Create a placeholder key set (no key material until MLS is implemented).
    pub fn placeholder(ssrc: u32) -> Self {
        Self {
            ssrc,
            generation: 0,
            key_material: Vec::new(),
        }
    }
}
