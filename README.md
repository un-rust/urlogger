# urlogger

<!-- automdrs:badges showCrateVersion="true" showCrateDownloads="true" showCrateDocs="true" showCommitActivity="true" showRepoStars="true" -->
![Crates.io Version](https://img.shields.io/crates/v/urlogger)
![Crates.io Total Downloads](https://img.shields.io/crates/d/urlogger)
![docs.rs](https://img.shields.io/docsrs/urlogger)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/un-rust/urlogger)
![GitHub Repo stars](https://img.shields.io/github/stars/un-rust/urlogger)
<!-- /automdrs -->

<!-- automdrs:description -->

A template for a new Rust project

<!-- /automdrs -->

**[Full documentation →](https://docs.rs/urlogger/)**

## Quick start

<!-- automdrs:cargo-add -->

```sh
cargo add urlogger
```

<!-- /automdrs -->

## Usage

<!-- automdrs:file src="./src/main.rs" -->
```rust
//! Demo: logs at all levels. Use `RUST_LOG=info` to filter.

use urlogger::{LogLevel, log};

fn main() {
    log(LogLevel::Trace, "Hello, world!");
    log(LogLevel::Debug, "Hello, world!");
    log(LogLevel::Info, "Hello, world!");
    log(LogLevel::Warn, "Hello, world!");
    log(LogLevel::Error, "Hello, world!");
}
```
<!-- /automdrs -->

## License

<!-- automdrs:contributors author="UnRUST" license="Apache-2.0" -->
Published under the [Apache-2.0](./LICENSE) license.
Made by [@UnRUST](https://github.com/un-rust) 💛
<br><br>
<a href="https://github.com/un-rust/urlogger/graphs/contributors">
<img src="https://contrib.rocks/image?repo=un-rust/urlogger" />
</a>
<!-- /automdrs -->

<!-- automdrs:with-automdrs -->

---

_🛠️ auto updated with [automd-rs](https://github.com/betterhyq/automd-rs)_

<!-- /automdrs -->