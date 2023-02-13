use core::fmt::{self, Write};

use super::Color;

/// The structure represents a ANSI color by suffix and prefix.
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct StaticColor {
    prefix: &'static str,
    suffix: &'static str,
}

impl StaticColor {
    /// Constructs a new instance with suffix and prefix.
    ///
    /// They are not checked so you should make sure you provide correct ANSI.
    /// Otherwise you may want to use [`TryFrom`].
    ///
    /// [`TryFrom`]: std::convert::TryFrom
    pub const fn new(prefix: &'static str, suffix: &'static str) -> Self {
        Self { prefix, suffix }
    }
}

impl StaticColor {
    /// Gets a reference to a prefix.
    pub fn get_prefix(&self) -> &str {
        self.prefix
    }

    /// Gets a reference to a suffix.
    pub fn get_suffix(&self) -> &str {
        self.suffix
    }
}

impl Color for StaticColor {
    fn fmt_prefix<W: Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str(self.prefix)
    }

    fn fmt_suffix<W: Write>(&self, f: &mut W) -> fmt::Result {
        f.write_str(self.suffix)
    }
}
