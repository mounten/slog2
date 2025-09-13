# slog2-parse

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/slog2_parse.svg)](https://crates.io/crates/slog2_parse)
[![Docs.rs](https://docs.rs/slog2_parse/badge.svg)](https://docs.rs/slog2_parse)

A crate that wraps methods defined in `libslog2parse` in an idomatic rust interface. Also exposes all unsafe c ffi functions via a `ffi` module.

`libslog2parse` is used to to read logging buffers from the [slogger2](https://www.qnx.com/developers/docs/8.0/com.qnx.doc.neutrino.utilities/topic/s/slogger2.html) system logger daemon. 

See [examples](./examples/) for how to use this crate.

