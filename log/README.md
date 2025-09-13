# slog2

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/slog2.svg)](https://crates.io/crates/slog2)
[![Docs.rs](https://docs.rs/slog2/badge.svg)](https://docs.rs/slog2)

A crate that wraps methods defined in `libslog2` in an idomatic rust interface. Also exposes all unsafe c ffi functions via a `ffi` module.

`libslog2` is used to to register and write to logging buffers via the [slogger2](https://www.qnx.com/developers/docs/8.0/com.qnx.doc.neutrino.utilities/topic/s/slogger2.html) system logger daemon.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
slog2 = "0.1"
```

## Example

For more examples [examples](./examples/) for how to use this crate.

Register a simple logger buffer:

```rust 
// Create config
let mut config = slog2::BufferSetConfig::default();
config.set_name("myprogramm")?;
config.config_name("default")?;
config.set_num_pages(7);

// Register and get handle.
let [handle] = config.register(None)?;

// Pass handle as buffer.
slog2::info!(buffer=handle, "Hello World {}", 42);
slog2::info!(buffer=handle, code=1000, "Hello World {}", 42);
```

Set a default buffer:

```rust
// Set a default buffer.
slog2::Buffer::set_default_buffer(Some(buffer_handle));

// Omitting the buffer argument will use the default buffer.
slog2::info!("Hello World {}", 42);
slog2::info!(code=1000, "Hello World {}", 42);

```
## Target Support 

This crate was tested for targets `aarch64-unknown-nto-qnx800, x86_64-pc-nto-qnx800` with the [rustc](https://www.qnx.com/developers/docs/8.0/com.qnx.doc.neutrino.utilities/topic/r/rust-host.html) that is shipped in the [qnxsoftwarecenter](https://www.qnx.com/download/group.html?programid=29178). 
