//! Lightweight colored logger: `log(level, message)` with `RUST_LOG` filtering.

mod color;
mod logger;
mod types;

pub use logger::{enabled, log};
pub use types::LogLevel;

/// Logs a message at the given level only if `RUST_LOG` allows it. Formatting is lazy:
/// `format!(...)` runs only when the level is enabled, avoiding allocation when filtered out.
///
/// # Example
/// ```
/// use urlogger::{log, LogLevel};
/// log!(LogLevel::Info, "user {} did {}", 42, "login");
/// ```
#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        if $crate::enabled($level) {
            $crate::log($level, &format!($($arg)*));
        }
    };
}
