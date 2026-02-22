//! Integration test that runs the main.rs demo.
//! Run with: `cargo test --test integration_test`

use urlogger::{LogLevel, log};

#[test]
fn main_rs_demo() {
    log(LogLevel::Trace, "Hello, world!");
    log(LogLevel::Debug, "Hello, world!");
    log(LogLevel::Info, "Hello, world!");
    log(LogLevel::Warn, "Hello, world!");
    log(LogLevel::Error, "Hello, world!");
}
