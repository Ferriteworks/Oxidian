//! Discord voice connection for Oxidian.
//!
//! # Quick start
//!
//! ```rust,ignore
//! use oxidian_voice::connection::VoiceConnection;
//!
//! // After receiving VOICE_STATE_UPDATE + VOICE_SERVER_UPDATE:
//! let vc = VoiceConnection::connect(
//!     &server.endpoint.unwrap(),
//!     server.guild_id,
//!     bot_user_id,
//!     &state.session_id,
//!     &server.token,
//! )
//! .await?;
//!
//! vc.speak(true).await?;
//! // send Opus frames ...
//! vc.speak(false).await?;
//! vc.disconnect().await?;
//! ```

pub mod connection;
pub mod dave;

pub use connection::VoiceConnection;
