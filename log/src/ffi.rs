#![allow(non_camel_case_types)]

use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_short;
use std::ffi::c_uchar;
use std::ffi::c_uint;

#[macro_export]
macro_rules! slogf {
    ($buffer:expr, $code:expr, $level:expr, $($arg:tt)+) => {{
        let formatted = format!($($arg)+);
        let c_str = std::ffi::CString::new(formatted)
            .expect("Failed to create CString");
        unsafe { slog2c($buffer, $code, $level as c_uchar, c_str.as_ptr()) }
    }};
}

pub const SLOG2_MAX_BUFFERS: usize = 4;

pub const SLOG2_DUMP_LOGS_ALL: c_int = 1 << 0;

#[repr(C)]
pub struct slog2_buffer_meta {
    _private: [c_uchar; 0], // makes it zero-sized and opaque
}

pub type slog2_buffer_t = *mut slog2_buffer_meta;

#[repr(C)]
pub struct slog2_buffer_config_t {
    pub buffer_name: *const c_char,
    pub num_pages: c_int,
}

#[repr(C)]
pub struct slog2_buffer_set_config_t {
    pub num_buffers: c_int,
    pub buffer_set_name: *const c_char,
    pub verbosity_level: c_uchar,
    pub buffer_config: [slog2_buffer_config_t; SLOG2_MAX_BUFFERS],
    pub max_retries: c_uint,
}

#[repr(u16)] // match the C enum underlying type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum slog2_fa_t {
    UnsignSLOG2_FA_UNSIGNEDed = 0,
    SLOG2_FA_SIGNED = 1 << 14,
    SLOG2_FA_STRING = 2 << 14,
    SLOG2_FA_STAR = 3 << 14,
}

#[link(name = "slog2")]
unsafe extern "C" {
    pub fn slog2c(
        buffer: slog2_buffer_t,
        code: c_short,
        severity: c_uchar,
        data: *const c_char,
    ) -> c_int;

    pub fn slog2_set_verbosity(buffer: slog2_buffer_t, verbosity: c_uchar) -> c_int;

    pub fn slog2_get_verbosity(buffer: slog2_buffer_t) -> c_uchar;

    pub fn slog2_register(
        config: *const slog2_buffer_set_config_t,
        handles: *mut slog2_buffer_t,
        flags: c_uint,
    ) -> c_int;

    pub fn slog2_set_default_buffer(buffer: slog2_buffer_t) -> slog2_buffer_t;

    pub fn slog2_reset() -> c_int;

    pub fn slog2_find_buffer(
        buffer_name: *const c_char,
        buffer_set_name: *const c_char,
    ) -> slog2_buffer_t;
}
