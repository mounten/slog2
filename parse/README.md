# slog2-parse

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/slog2_parse.svg)](https://crates.io/crates/slog2_parse)
[![Docs.rs](https://docs.rs/slog2_parse/badge.svg)](https://docs.rs/slog2_parse)

A crate that wraps methods defined in `libslog2parse` in an idiomatic rust interface. Also exposes all unsafe c ffi functions via a `ffi` module.

`libslog2parse` is used to to read logging buffers from the [slogger2](https://www.qnx.com/developers/docs/8.0/com.qnx.doc.neutrino.utilities/topic/s/slogger2.html) system logger daemon. 

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
slog2-parse = "0.1"
```

## Example

For more examples [examples](./examples/) for how to use this crate.

Register a callback to parse all existing and incoming log messages for all buffers:

```rust
parse_all(Some(ParseFlags::DYNAMIC), None, None, |info| {
    println!("{:?}", info);
    Ok(())
})?;
```

## Target Support 

This crate was tested for targets `aarch64-unknown-nto-qnx800, x86_64-pc-nto-qnx800` with the [rustc](https://www.qnx.com/developers/docs/8.0/com.qnx.doc.neutrino.utilities/topic/r/rust-host.html) that is shipped in the [qnxsoftwarecenter](https://www.qnx.com/download/group.html?programid=29178). 
