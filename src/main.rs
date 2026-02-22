//! Demo: logs at all levels. Use `RUST_LOG=info` to filter.

use urlogger::{LogLevel, log};

fn main() {
    log(LogLevel::Trace, "Hello, world!");
    log(LogLevel::Debug, "Hello, world!");
    log(LogLevel::Info, "Hello, world!");
    log(LogLevel::Warn, "Hello, world!");
    log(LogLevel::Error, "Hello, world!");
}
