//! Opus encoding and decoding for Discord voice.
//!
//! Discord requires all voice audio to be encoded as Opus at 48 kHz stereo
//! in 20 ms frames (960 samples per channel per frame).
//!
//! # Example
//!
//! ```rust,ignore
//! use oxidian_voice::opus::{OpusEncoder, OpusDecoder};
//!
//! let mut encoder = OpusEncoder::new()?;
//! let pcm: Vec<i16> = vec![0i16; 960 * 2]; // 20ms of silence, stereo
//! let opus_frame = encoder.encode(&pcm)?;
//!
//! let mut decoder = OpusDecoder::new()?;
//! let decoded = decoder.decode(&opus_frame)?;
//! ```

use std::convert::TryInto;

use oxidian_core::error::{Error as OxidianError, VoiceError};

/// Discord voice always uses 48 kHz.
pub const SAMPLE_RATE: u32 = 48_000;

/// Discord expects 20 ms Opus frames.
pub const FRAME_DURATION_MS: u32 = 20;

/// Samples per channel in one 20 ms frame at 48 kHz.
pub const FRAME_SIZE: usize =
    (SAMPLE_RATE as usize * FRAME_DURATION_MS as usize) / 1000;

/// Discord voice uses stereo audio.
pub const CHANNELS: u32 = 2;

/// An Opus encoder configured for Discord voice.
///
/// Produces 20 ms Opus frames at 48 kHz stereo, suitable for passing
/// directly to [`VoiceConnection::send_audio`](crate::VoiceConnection::send_audio).
pub struct OpusEncoder {
    inner: audiopus::coder::Encoder,
}

impl OpusEncoder {
    /// Create a new Opus encoder with Discord's required settings
    /// (48 kHz, stereo, VoIP application profile).
    pub fn new() -> Result<Self, OxidianError> {
        let encoder = audiopus::coder::Encoder::new(
            audiopus::SampleRate::Hz48000,
            audiopus::Channels::Stereo,
            audiopus::Application::Voip,
        )
        .map_err(|e| {
            VoiceError::Connection(format!("failed to create Opus encoder: {e}"))
        })?;

        Ok(Self { inner: encoder })
    }

    /// Set the bitrate in bits per second.
    ///
    /// Discord typically uses 64–128 kbps for voice. The default is
    /// determined by libopus (usually ~64 kbps for VoIP).
    pub fn set_bitrate(&mut self, bitrate: i32) -> Result<(), OxidianError> {
        self.inner
            .set_bitrate(audiopus::Bitrate::BitsPerSecond(bitrate))
            .map_err(|e| {
                VoiceError::Connection(format!("failed to set Opus bitrate: {e}"))
            })?;
        Ok(())
    }

    /// Encode a 20 ms frame of interleaved stereo PCM (i16 samples)
    /// into an Opus packet.
    ///
    /// `pcm` must contain exactly [`FRAME_SIZE`] × [`CHANNELS`] = 1920
    /// samples (960 per channel for 20 ms at 48 kHz).
    ///
    /// Returns the Opus-encoded bytes.
    pub fn encode(&mut self, pcm: &[i16]) -> Result<Vec<u8>, OxidianError> {
        let expected = FRAME_SIZE * CHANNELS as usize;
        if pcm.len() != expected {
            return Err(VoiceError::Connection(format!(
                "PCM frame must be {expected} samples (got {})",
                pcm.len()
            ))
            .into());
        }

        // Max Opus frame size recommended by the spec.
        let mut output = vec![0u8; 4000];
        let len = self
            .inner
            .encode(pcm, &mut output)
            .map_err(|e| VoiceError::Connection(format!("Opus encode failed: {e}")))?;
        output.truncate(len);
        Ok(output)
    }

    /// Encode a 20 ms frame of interleaved stereo PCM (f32 samples)
    /// into an Opus packet.
    ///
    /// `pcm` must contain exactly [`FRAME_SIZE`] × [`CHANNELS`] = 1920
    /// samples. Each sample should be in the range `[-1.0, 1.0]`.
    pub fn encode_float(&mut self, pcm: &[f32]) -> Result<Vec<u8>, OxidianError> {
        let expected = FRAME_SIZE * CHANNELS as usize;
        if pcm.len() != expected {
            return Err(VoiceError::Connection(format!(
                "PCM frame must be {expected} samples (got {})",
                pcm.len()
            ))
            .into());
        }

        let mut output = vec![0u8; 4000];
        let len = self.inner.encode_float(pcm, &mut output).map_err(|e| {
            VoiceError::Connection(format!("Opus encode_float failed: {e}"))
        })?;
        output.truncate(len);
        Ok(output)
    }
}

/// An Opus decoder configured for Discord voice.
///
/// Decodes incoming 20 ms Opus frames (from other participants) back
/// to PCM audio.
pub struct OpusDecoder {
    inner: audiopus::coder::Decoder,
}

impl OpusDecoder {
    /// Create a new Opus decoder (48 kHz, stereo).
    pub fn new() -> Result<Self, OxidianError> {
        let decoder = audiopus::coder::Decoder::new(
            audiopus::SampleRate::Hz48000,
            audiopus::Channels::Stereo,
        )
        .map_err(|e| {
            VoiceError::Connection(format!("failed to create Opus decoder: {e}"))
        })?;

        Ok(Self { inner: decoder })
    }

    /// Decode an Opus packet into interleaved stereo i16 PCM samples.
    ///
    /// Returns a buffer of [`FRAME_SIZE`] × [`CHANNELS`] = 1920 samples.
    pub fn decode(&mut self, opus_data: &[u8]) -> Result<Vec<i16>, OxidianError> {
        let mut output = vec![0i16; FRAME_SIZE * CHANNELS as usize];
        let packet: audiopus::packet::Packet<'_> = opus_data
            .try_into()
            .map_err(|e| VoiceError::Connection(format!("invalid Opus packet: {e}")))?;
        let signals: audiopus::MutSignals<'_, i16> = (&mut output)
            .try_into()
            .map_err(|e| VoiceError::Connection(format!("signal buffer error: {e}")))?;
        let samples = self
            .inner
            .decode(Some(packet), signals, false)
            .map_err(|e| VoiceError::Connection(format!("Opus decode failed: {e}")))?;
        output.truncate(samples * CHANNELS as usize);
        Ok(output)
    }

    /// Decode an Opus packet into interleaved stereo f32 PCM samples.
    ///
    /// Returns a buffer of [`FRAME_SIZE`] × [`CHANNELS`] = 1920 samples.
    pub fn decode_float(&mut self, opus_data: &[u8]) -> Result<Vec<f32>, OxidianError> {
        let mut output = vec![0f32; FRAME_SIZE * CHANNELS as usize];
        let packet: audiopus::packet::Packet<'_> = opus_data
            .try_into()
            .map_err(|e| VoiceError::Connection(format!("invalid Opus packet: {e}")))?;
        let signals: audiopus::MutSignals<'_, f32> = (&mut output)
            .try_into()
            .map_err(|e| VoiceError::Connection(format!("signal buffer error: {e}")))?;
        let samples = self
            .inner
            .decode_float(Some(packet), signals, false)
            .map_err(|e| {
                VoiceError::Connection(format!("Opus decode_float failed: {e}"))
            })?;
        output.truncate(samples * CHANNELS as usize);
        Ok(output)
    }

    /// Generate a frame of Packet Loss Concealment (PLC) audio.
    ///
    /// Call this when an expected Opus frame was not received (e.g. packet
    /// loss on UDP) to ask the decoder to produce a smooth interpolation.
    pub fn decode_loss(&mut self) -> Result<Vec<i16>, OxidianError> {
        let mut output = vec![0i16; FRAME_SIZE * CHANNELS as usize];
        let signals: audiopus::MutSignals<'_, i16> = (&mut output)
            .try_into()
            .map_err(|e| VoiceError::Connection(format!("signal buffer error: {e}")))?;
        let samples = self.inner.decode(None, signals, false).map_err(|e| {
            VoiceError::Connection(format!("Opus PLC decode failed: {e}"))
        })?;
        output.truncate(samples * CHANNELS as usize);
        Ok(output)
    }
}
