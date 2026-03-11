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

//! DAVE protocol state machine.
//!
//! Handles the MLS-based key exchange lifecycle that precedes E2EE audio in
//! DAVE-enabled voice channels.  Discord drives the protocol via voice
//! gateway opcodes 20–28.
//!
//! ## Lifecycle
//!
//! 1. **Prepare Transition** (op 20) — Discord signals that a new encryption
//!    epoch is being negotiated.  The client acknowledges with op 25.
//! 2. **MLS Proposals / Commits** (ops 21–24) — Discord distributes MLS key
//!    material.  Currently logged; full MLS group ratchet is a TODO.
//! 3. **Execute Transition** (op 28) — Discord activates the new epoch.
//!    All subsequent audio must use the new key material.

/// The current phase of a DAVE protocol negotiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DavePhase {
    /// No DAVE negotiation in progress; using standard RTP encryption.
    Inactive,
    /// Discord has signalled a transition (op 20); we have acknowledged (op 25)
    /// and are waiting for the execute transition (op 28).
    PendingTransition {
        /// The transition ID assigned by Discord.
        transition_id: u64,
        /// The DAVE protocol version being negotiated.
        protocol_version: u64,
    },
    /// DAVE E2EE is active for this epoch.
    Active {
        /// The current epoch number.
        epoch: u64,
        /// The DAVE protocol version in use.
        protocol_version: u64,
    },
}

/// Tracks DAVE protocol state for a single voice connection.
#[derive(Debug)]
pub struct DaveState {
    /// Current protocol phase.
    pub phase: DavePhase,
    /// MLS external sender public key (op 21), if received.
    pub external_sender: Option<Vec<u8>>,
    /// MLS key packages received (op 22).
    pub key_packages: Vec<Vec<u8>>,
    /// MLS proposals received (op 23).
    pub proposals: Vec<Vec<u8>>,
    /// MLS commit + welcome (op 24).
    pub commit_welcome: Option<Vec<u8>>,
}

impl DaveState {
    /// Create a new inactive DAVE state.
    pub fn new() -> Self {
        Self {
            phase: DavePhase::Inactive,
            external_sender: None,
            key_packages: Vec::new(),
            proposals: Vec::new(),
            commit_welcome: None,
        }
    }

    /// Whether DAVE E2EE is currently active.
    pub fn is_active(&self) -> bool {
        matches!(self.phase, DavePhase::Active { .. })
    }

    /// Handle a Prepare Transition (op 20).
    ///
    /// Returns the transition ID that should be sent back in op 25.
    pub fn prepare_transition(
        &mut self,
        transition_id: u64,
        protocol_version: u64,
    ) -> u64 {
        self.phase = DavePhase::PendingTransition {
            transition_id,
            protocol_version,
        };
        // Clear stale MLS state from the previous epoch.
        self.key_packages.clear();
        self.proposals.clear();
        self.commit_welcome = None;
        transition_id
    }

    /// Handle an Execute Transition (op 28).
    ///
    /// Moves to the Active phase.  The epoch counter is derived from the
    /// transition.
    pub fn execute_transition(&mut self, transition_id: u64) {
        let protocol_version = match self.phase {
            DavePhase::PendingTransition {
                protocol_version, ..
            } => protocol_version,
            _ => 1,
        };
        self.phase = DavePhase::Active {
            epoch: transition_id,
            protocol_version,
        };
    }

    /// Store an MLS external sender (op 21).
    pub fn set_external_sender(&mut self, data: Vec<u8>) {
        self.external_sender = Some(data);
    }

    /// Store an MLS key package (op 22).
    pub fn add_key_package(&mut self, data: Vec<u8>) {
        self.key_packages.push(data);
    }

    /// Store MLS proposals (op 23).
    pub fn add_proposals(&mut self, data: Vec<u8>) {
        self.proposals.push(data);
    }

    /// Store an MLS commit + welcome (op 24).
    pub fn set_commit_welcome(&mut self, data: Vec<u8>) {
        self.commit_welcome = Some(data);
    }
}

impl Default for DaveState {
    fn default() -> Self {
        Self::new()
    }
}
