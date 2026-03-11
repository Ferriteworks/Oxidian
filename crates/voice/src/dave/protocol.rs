//! DAVE protocol state machine.
//!
//! Handles the MLS-based key exchange that precedes E2EE audio in DAVE-enabled
//! voice channels.  Discord signals a DAVE transition via voice opcodes 14–16
//! before the new encryption epoch is active.
//!
//! **TODO**: Full MLS handshake implementation.

/// The current phase of a DAVE protocol negotiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DaveProtocol {
    /// No DAVE negotiation in progress; using standard RTP encryption.
    Inactive,
    /// Discord has signalled a transition; waiting for all members to be ready.
    PendingTransition { epoch: u32 },
    /// All members are ready; DAVE encryption is active for this epoch.
    Active { epoch: u32 },
}
