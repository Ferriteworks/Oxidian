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
