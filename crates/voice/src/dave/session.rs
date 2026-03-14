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

use super::{
    keys::SenderKeySet,
    protocol::{DaveEvent, DaveState},
};

/// Read-only snapshot of a [`DaveSession`].
#[derive(Debug, Clone)]
pub struct DaveSessionSnapshot {
    /// Protocol negotiation state machine.
    pub state: DaveState,
    /// Per-sender key sets, indexed by SSRC.
    pub sender_keys: Vec<SenderKeySet>,
}

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

    /// Apply one raw voice gateway payload into the DAVE state machine.
    pub fn apply_gateway_payload(
        &mut self,
        op: u64,
        d: &serde_json::Value,
    ) -> Option<DaveEvent> {
        self.state.apply_gateway_payload(op, d)
    }

    /// Return an owned snapshot of the current DAVE session.
    pub fn snapshot(&self) -> DaveSessionSnapshot {
        DaveSessionSnapshot {
            state: self.state.clone(),
            sender_keys: self.sender_keys.clone(),
        }
    }

    /// Whether we have received at least one MLS commit/welcome payload.
    pub fn has_commit_welcome(&self) -> bool {
        self.state.commit_welcome.is_some()
    }

    /// Return the latest commit/welcome payload, if available.
    pub fn latest_commit_welcome(&self) -> Option<&[u8]> {
        self.state.commit_welcome.as_deref()
    }

    /// Return the number of cached key package payloads.
    pub fn key_package_count(&self) -> usize {
        self.state.key_packages.len()
    }

    /// Return the number of cached proposal payloads.
    pub fn proposal_count(&self) -> usize {
        self.state.proposals.len()
    }
}

impl Default for DaveSession {
    fn default() -> Self {
        Self::new()
    }
}
