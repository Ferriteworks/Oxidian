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
//! Handles the MLS key exchange lifecycle that precedes E2EE media in
//! DAVE-enabled voice channels. Discord drives this over voice gateway opcodes
//! `20..=28`.

use base64::{engine::general_purpose, Engine as _};

/// DAVE gateway opcode: prepare transition.
pub const OP_DAVE_PREPARE_TRANSITION: u64 = 20;
/// DAVE gateway opcode: MLS external sender.
pub const OP_DAVE_MLS_EXTERNAL_SENDER: u64 = 21;
/// DAVE gateway opcode: MLS key package.
pub const OP_DAVE_MLS_KEY_PACKAGE: u64 = 22;
/// DAVE gateway opcode: MLS proposals.
pub const OP_DAVE_MLS_PROPOSALS: u64 = 23;
/// DAVE gateway opcode: MLS commit + welcome.
pub const OP_DAVE_MLS_COMMIT_WELCOME: u64 = 24;
/// DAVE gateway opcode: transition ready (client -> server).
pub const OP_DAVE_TRANSITION_READY: u64 = 25;
/// DAVE gateway opcode: execute transition.
pub const OP_DAVE_EXECUTE_TRANSITION: u64 = 28;

/// A parsed DAVE event emitted while processing voice gateway payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DaveEvent {
    /// Server began a new DAVE epoch transition.
    PrepareTransition {
        transition_id: u64,
        protocol_version: u64,
    },
    /// Server provided the MLS external sender payload.
    MlsExternalSender { bytes: Vec<u8> },
    /// Server provided an MLS key package payload.
    MlsKeyPackage { bytes: Vec<u8> },
    /// Server provided MLS proposals payload.
    MlsProposals { bytes: Vec<u8> },
    /// Server provided MLS commit/welcome payload.
    MlsCommitWelcome { bytes: Vec<u8> },
    /// Server activated a transition/epoch.
    ExecuteTransition { transition_id: u64 },
}

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
#[derive(Debug, Clone)]
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

    /// Apply a raw DAVE gateway payload (`op`, `d`) and update local state.
    ///
    /// Returns a typed [`DaveEvent`] when the opcode is DAVE-related and valid.
    pub fn apply_gateway_payload(
        &mut self,
        op: u64,
        d: &serde_json::Value,
    ) -> Option<DaveEvent> {
        match op {
            OP_DAVE_PREPARE_TRANSITION => {
                let transition_id = d.get("transition_id")?.as_u64()?;
                let protocol_version = d
                    .get("protocol_version")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1);
                self.prepare_transition(transition_id, protocol_version);
                Some(DaveEvent::PrepareTransition {
                    transition_id,
                    protocol_version,
                })
            }
            OP_DAVE_MLS_EXTERNAL_SENDER => {
                let bytes = decode_payload_bytes(d)?;
                self.set_external_sender(bytes.clone());
                Some(DaveEvent::MlsExternalSender { bytes })
            }
            OP_DAVE_MLS_KEY_PACKAGE => {
                let bytes = decode_payload_bytes(d)?;
                self.add_key_package(bytes.clone());
                Some(DaveEvent::MlsKeyPackage { bytes })
            }
            OP_DAVE_MLS_PROPOSALS => {
                let bytes = decode_payload_bytes(d)?;
                self.add_proposals(bytes.clone());
                Some(DaveEvent::MlsProposals { bytes })
            }
            OP_DAVE_MLS_COMMIT_WELCOME => {
                let bytes = decode_payload_bytes(d)?;
                self.set_commit_welcome(bytes.clone());
                Some(DaveEvent::MlsCommitWelcome { bytes })
            }
            OP_DAVE_EXECUTE_TRANSITION => {
                let transition_id = d.get("transition_id")?.as_u64()?;
                self.execute_transition(transition_id);
                Some(DaveEvent::ExecuteTransition { transition_id })
            }
            _ => None,
        }
    }
}

impl Default for DaveState {
    fn default() -> Self {
        Self::new()
    }
}

/// Build the op 25 payload acknowledging a DAVE transition.
pub fn transition_ready_payload(transition_id: u64) -> serde_json::Value {
    serde_json::json!({
        "op": OP_DAVE_TRANSITION_READY,
        "d": {
            "transition_id": transition_id,
        }
    })
}

/// Decode DAVE byte payloads from voice gateway `d` fields.
///
/// Discord can encode these as:
/// - a raw byte array (`[1,2,3,...]`)
/// - a string (base64/base64url; UTF-8 fallback)
/// - an object wrapper containing one of `data`, `bytes`, or `payload`
pub fn decode_payload_bytes(value: &serde_json::Value) -> Option<Vec<u8>> {
    match value {
        serde_json::Value::Array(arr) => {
            let mut out = Vec::with_capacity(arr.len());
            for v in arr {
                out.push(v.as_u64()? as u8);
            }
            Some(out)
        }
        serde_json::Value::String(s) => {
            if let Ok(bytes) = general_purpose::STANDARD.decode(s.as_bytes()) {
                return Some(bytes);
            }
            if let Ok(bytes) = general_purpose::URL_SAFE.decode(s.as_bytes()) {
                return Some(bytes);
            }
            if let Ok(bytes) = general_purpose::URL_SAFE_NO_PAD.decode(s.as_bytes()) {
                return Some(bytes);
            }
            // If it's not valid base64/base64url, keep literal UTF-8 bytes.
            Some(s.as_bytes().to_vec())
        }
        serde_json::Value::Object(map) => decode_payload_bytes(
            map.get("data")
                .or_else(|| map.get("bytes"))
                .or_else(|| map.get("payload"))?,
        ),
        _ => None,
    }
}
