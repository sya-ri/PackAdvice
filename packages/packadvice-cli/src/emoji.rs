// From terminal-supports-emoji, but false for Unix other than Mac.
// This is because even if it supports UTF8, it may not support emoji.
// https://github.com/mainrs/terminal-supports-emoji-rs/blob/1ead98a8372dd85946576e4447ed9d40b36f00db/src/lib.rs

use atty::Stream;
use std::fmt;

#[cfg(windows)]
fn platform_supports_emoji() -> bool {
    std::env::var("WT_SESSION").is_ok()
}

#[cfg(target_os = "macos")]
fn platform_supports_emoji() -> bool {
    true
}

#[cfg(all(unix, not(target_os = "macos")))]
fn platform_supports_emoji() -> bool {
    false
}

#[cfg(all(not(unix), not(windows)))]
fn platform_supports_emoji() -> bool {
    false
}

fn enable_emoji() -> bool {
    platform_supports_emoji() && atty::is(Stream::Stdout)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Emoji<'a>(pub &'a str, pub &'a str);

impl<'a> Emoji<'a> {
    pub const fn new(emoji: &'a str, fallback: &'a str) -> Self {
        Self(emoji, fallback)
    }

    pub fn string(self) -> &'a str {
        if enable_emoji() {
            self.0
        } else {
            self.1
        }
    }
}

impl fmt::Display for Emoji<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}
