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

//! Discord message components (v1 interactive + v2 layout).
//!
//! V1 components are interactive elements: buttons, select menus, and text inputs.
//! V2 components add layout primitives: sections, containers, separators, etc.

pub mod v1;
pub mod v2;

// Re-export frequently used v1 types.
pub use v1::{
    ActionRow, Button, ButtonStyle, ChannelSelect, MentionableSelect, PartialEmoji,
    RoleSelect, SelectDefaultValue, SelectOption, StringSelect, TextInput,
    TextInputStyle, UserSelect,
};

// Re-export frequently used v2 types.
pub use v2::{
    Container, FileComponent, MediaGallery, MediaGalleryItem, Section, Separator,
    TextDisplay, Thumbnail, UnfurlMedia,
};
