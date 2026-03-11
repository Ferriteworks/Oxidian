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
pub mod opus;

pub use connection::VoiceConnection;
pub use opus::{OpusDecoder, OpusEncoder};
