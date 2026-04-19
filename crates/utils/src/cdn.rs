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

//! Discord CDN URL builders.
//!
//! All functions return fully-formed `https://cdn.discordapp.com/…` URLs.
//! Pass `size` as a power of two between 16 and 4096 to append `?size=N`.

const CDN: &str = "https://cdn.discordapp.com";

/// Image format for CDN assets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageFormat {
    /// PNG (lossless, supports transparency).
    #[default]
    Png,
    /// JPEG (lossy, no transparency).
    Jpg,
    /// WebP (modern, smaller file size).
    WebP,
    /// GIF (animated). Only valid for animated assets (hashes starting with `a_`).
    Gif,
}

impl ImageFormat {
    fn ext(self) -> &'static str {
        match self {
            Self::Png => "png",
            Self::Jpg => "jpg",
            Self::WebP => "webp",
            Self::Gif => "gif",
        }
    }
}

fn append_size(url: String, size: Option<u16>) -> String {
    match size {
        Some(s) => format!("{url}?size={s}"),
        None => url,
    }
}

/// User avatar URL.
///
/// If `hash` is `None` the default avatar is returned (no format or size applies).
/// Animated hashes start with `a_`: pass [`ImageFormat::Gif`] or the correct
/// ext is chosen automatically.
pub fn user_avatar(
    user_id: u64,
    hash: Option<&str>,
    format: ImageFormat,
    size: Option<u16>,
) -> String {
    match hash {
        Some(h) => {
            let ext = if h.starts_with("a_") && format == ImageFormat::Gif {
                "gif"
            } else {
                format.ext()
            };
            append_size(format!("{CDN}/avatars/{user_id}/{h}.{ext}"), size)
        }
        None => {
            // Default avatar index is (user_id >> 22) % 6 for new username system.
            let index = (user_id >> 22) % 6;
            format!("{CDN}/embed/avatars/{index}.png")
        }
    }
}

/// Guild member avatar URL (server-specific avatar).
pub fn member_avatar(
    guild_id: u64,
    user_id: u64,
    hash: &str,
    format: ImageFormat,
    size: Option<u16>,
) -> String {
    let ext = if hash.starts_with("a_") && format == ImageFormat::Gif {
        "gif"
    } else {
        format.ext()
    };
    append_size(
        format!("{CDN}/guilds/{guild_id}/users/{user_id}/avatars/{hash}.{ext}"),
        size,
    )
}

/// Guild icon URL.
pub fn guild_icon(
    guild_id: u64,
    hash: &str,
    format: ImageFormat,
    size: Option<u16>,
) -> String {
    let ext = if hash.starts_with("a_") && format == ImageFormat::Gif {
        "gif"
    } else {
        format.ext()
    };
    append_size(format!("{CDN}/icons/{guild_id}/{hash}.{ext}"), size)
}

/// Guild banner URL.
pub fn guild_banner(
    guild_id: u64,
    hash: &str,
    format: ImageFormat,
    size: Option<u16>,
) -> String {
    let ext = if hash.starts_with("a_") && format == ImageFormat::Gif {
        "gif"
    } else {
        format.ext()
    };
    append_size(format!("{CDN}/banners/{guild_id}/{hash}.{ext}"), size)
}

/// Guild splash (invite background) URL.
pub fn guild_splash(guild_id: u64, hash: &str, size: Option<u16>) -> String {
    append_size(format!("{CDN}/splashes/{guild_id}/{hash}.png"), size)
}

/// Guild discovery splash URL.
pub fn guild_discovery_splash(guild_id: u64, hash: &str, size: Option<u16>) -> String {
    append_size(
        format!("{CDN}/discovery-splashes/{guild_id}/{hash}.png"),
        size,
    )
}

/// Custom emoji URL.
pub fn emoji(emoji_id: u64, animated: bool) -> String {
    let ext = if animated { "gif" } else { "png" };
    format!("{CDN}/emojis/{emoji_id}.{ext}")
}

/// Application (bot) icon URL.
pub fn application_icon(
    application_id: u64,
    hash: &str,
    format: ImageFormat,
    size: Option<u16>,
) -> String {
    append_size(
        format!("{CDN}/app-icons/{application_id}/{hash}.{}", format.ext()),
        size,
    )
}

/// Application asset URL.
pub fn application_asset(
    application_id: u64,
    asset_id: &str,
    format: ImageFormat,
    size: Option<u16>,
) -> String {
    append_size(
        format!(
            "{CDN}/app-assets/{application_id}/{asset_id}.{}",
            format.ext()
        ),
        size,
    )
}

/// Guild sticker URL.
pub fn sticker(sticker_id: u64) -> String {
    // Stickers are served as PNG (static) or JSON (Lottie): PNG is the safe default.
    format!("{CDN}/stickers/{sticker_id}.png")
}

/// User banner URL.
pub fn user_banner(
    user_id: u64,
    hash: &str,
    format: ImageFormat,
    size: Option<u16>,
) -> String {
    let ext = if hash.starts_with("a_") && format == ImageFormat::Gif {
        "gif"
    } else {
        format.ext()
    };
    append_size(format!("{CDN}/banners/{user_id}/{hash}.{ext}"), size)
}

/// Role icon URL.
pub fn role_icon(
    role_id: u64,
    hash: &str,
    format: ImageFormat,
    size: Option<u16>,
) -> String {
    append_size(
        format!("{CDN}/role-icons/{role_id}/{hash}.{}", format.ext()),
        size,
    )
}

/// Scheduled event cover image URL.
pub fn event_cover(
    event_id: u64,
    hash: &str,
    format: ImageFormat,
    size: Option<u16>,
) -> String {
    append_size(
        format!("{CDN}/guild-events/{event_id}/{hash}.{}", format.ext()),
        size,
    )
}
