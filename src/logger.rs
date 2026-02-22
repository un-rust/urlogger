//! Logger formatting with colored output. Filter by `RUST_LOG` (default: show all).

use std::sync::OnceLock;

use crate::color;
use crate::color::codes::{BOLD, RESET};
use crate::types::LogLevel;

static MIN_LEVEL: OnceLock<LogLevel> = OnceLock::new();

fn min_level() -> LogLevel {
    *MIN_LEVEL.get_or_init(|| {
        std::env::var("RUST_LOG")
            .ok()
            .and_then(|s| parse_log_level(&s))
            .unwrap_or(LogLevel::Trace)
    })
}

/// Returns whether the given level is enabled (i.e. passes `RUST_LOG` filtering).
#[inline]
pub fn enabled(level: LogLevel) -> bool {
    level <= min_level()
}

fn parse_log_level(s: &str) -> Option<LogLevel> {
    if s.eq_ignore_ascii_case("error") {
        Some(LogLevel::Error)
    } else if s.eq_ignore_ascii_case("warn") || s.eq_ignore_ascii_case("warning") {
        Some(LogLevel::Warn)
    } else if s.eq_ignore_ascii_case("info") {
        Some(LogLevel::Info)
    } else if s.eq_ignore_ascii_case("debug") {
        Some(LogLevel::Debug)
    } else if s.eq_ignore_ascii_case("trace") {
        Some(LogLevel::Trace)
    } else {
        None
    }
}

/// Logs a message at the given level. Filtering follows `RUST_LOG`.
pub fn log(level: LogLevel, message: &str) {
    let min = min_level();
    // Show if level severity >= min (lower enum discriminant = more severe)
    if level <= min {
        let (start, end) = color::level_color(level);
        let level_str = level.as_str();
        println!(
            "{}{}{:6}{} {}{}{}",
            BOLD, start, level_str, end, BOLD, message, RESET
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LogLevel;

    #[test]
    fn parse_log_level_accepts_lowercase() {
        assert_eq!(parse_log_level("error"), Some(LogLevel::Error));
        assert_eq!(parse_log_level("warn"), Some(LogLevel::Warn));
        assert_eq!(parse_log_level("warning"), Some(LogLevel::Warn));
        assert_eq!(parse_log_level("info"), Some(LogLevel::Info));
        assert_eq!(parse_log_level("debug"), Some(LogLevel::Debug));
        assert_eq!(parse_log_level("trace"), Some(LogLevel::Trace));
    }

    #[test]
    fn parse_log_level_accepts_any_case() {
        assert_eq!(parse_log_level("ERROR"), Some(LogLevel::Error));
        assert_eq!(parse_log_level("Info"), Some(LogLevel::Info));
    }

    #[test]
    fn parse_log_level_unknown_returns_none() {
        assert_eq!(parse_log_level(""), None);
        assert_eq!(parse_log_level("unknown"), None);
        assert_eq!(parse_log_level("infoo"), None);
    }

    #[test]
    fn log_does_not_panic_for_each_level() {
        for level in [
            LogLevel::Error,
            LogLevel::Warn,
            LogLevel::Info,
            LogLevel::Debug,
            LogLevel::Trace,
        ] {
            log(level, "test message");
        }
    }
}
