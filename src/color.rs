//! ANSI color and style utilities for terminal output.

use crate::types::LogLevel;

/// ANSI escape codes (VT100/xterm).
pub mod codes {
    /// Bold text attribute (SGR 1)
    pub const BOLD: &str = "\x1b[1m";
    /// Reset all attributes (SGR 0)
    pub const RESET: &str = "\x1b[0m";
    /// Red foreground (SGR 31)
    pub const RED: &str = "\x1b[31m";
    /// Yellow foreground (SGR 33)
    pub const YELLOW: &str = "\x1b[33m";
    /// Cyan foreground (SGR 36)
    pub const CYAN: &str = "\x1b[36m";
    /// Magenta foreground (SGR 35)
    pub const MAGENTA: &str = "\x1b[35m";
    /// Bright black / dark gray (SGR 90)
    pub const GRAY: &str = "\x1b[90m";
}

use codes::{CYAN, GRAY, MAGENTA, RED, RESET, YELLOW};

/// Returns (start, reset) ANSI codes for the given log level.
#[inline]
pub fn level_color(level: LogLevel) -> (&'static str, &'static str) {
    let start = match level {
        LogLevel::Error => RED,
        LogLevel::Warn => YELLOW,
        LogLevel::Info => CYAN,
        LogLevel::Debug => MAGENTA,
        LogLevel::Trace => GRAY,
    };
    (start, RESET)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LogLevel;

    #[test]
    fn level_color_returns_reset_as_second() {
        for level in [
            LogLevel::Error,
            LogLevel::Warn,
            LogLevel::Info,
            LogLevel::Debug,
            LogLevel::Trace,
        ] {
            let (start, end) = level_color(level);
            assert!(!start.is_empty());
            assert_eq!(end, super::codes::RESET);
        }
    }

    #[test]
    fn level_color_maps_each_level_to_different_code() {
        assert_eq!(level_color(LogLevel::Error).0, super::codes::RED);
        assert_eq!(level_color(LogLevel::Warn).0, super::codes::YELLOW);
        assert_eq!(level_color(LogLevel::Info).0, super::codes::CYAN);
        assert_eq!(level_color(LogLevel::Debug).0, super::codes::MAGENTA);
        assert_eq!(level_color(LogLevel::Trace).0, super::codes::GRAY);
    }
}
