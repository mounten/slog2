# slog2-extra

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/slog2.svg)](https://crates.io/crates/slog2)
[![Docs.rs](https://docs.rs/slog2/badge.svg)](https://docs.rs/slog2)

A crate that wraps methods defined in `libslog2-extra` in an idiomatic rust interface. Also exposes all unsafe c ffi functions via a `ffi` module.

`libslog2-extra` is used to dump/obfuscate/hash logging buffers data from the [slogger2](https://www.qnx.com/developers/docs/8.0/com.qnx.doc.neutrino.utilities/topic/s/slogger2.html) system logger daemon.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
slog2-extra = "0.1"
```

## Example

Dump all logs to a file:

```rust 
let file = File::create("dumped_logs.log").expect("Couldn't create file");
dump_logs_to_file(&file, Some(DumpFlags::DUMP_LOGS_ALL)).expect("Couldn't dump logs to file");
```
## Target Support 

This crate was tested for targets `aarch64-unknown-nto-qnx800, x86_64-pc-nto-qnx800` with the [rustc](https://www.qnx.com/developers/docs/8.0/com.qnx.doc.neutrino.utilities/topic/r/rust-host.html) that is shipped in the [qnxsoftwarecenter](https://www.qnx.com/download/group.html?programid=29178). 
