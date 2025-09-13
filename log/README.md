# slog2

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/slog2.svg)](https://crates.io/crates/slog2)
[![Docs.rs](https://docs.rs/slog2/badge.svg)](https://docs.rs/slog2)

A crate that wraps methods defined in `libslog2` in an idomatic rust interface. Also exposes all unsafe c ffi functions via a `ffi` module.

`libslog2` is used to to register and write to logging buffers via the [slogger2](https://www.qnx.com/developers/docs/8.0/com.qnx.doc.neutrino.utilities/topic/s/slogger2.html) system logger daemon.

See [examples](./examples/) for how to use this crate.

