#![allow(non_camel_case_types)]

use std::ffi::c_char;

#[macro_export]
macro_rules! slogf {
    ($buffer:expr, $code:expr, $level:expr, $($arg:tt)+) => {{
        let formatted = format!($($arg)+);
        let c_str = std::ffi::CString::new(formatted)
            .expect("Failed to create CString");
        unsafe { slog2c($buffer, $code, $level as u8, c_str.as_ptr()) }
    }};
}

pub const SLOG2_MAX_BUFFERS: usize = 4;

pub const SLOG2_DUMP_LOGS_ALL: i32 = 1 << 0;

#[repr(C)]
pub struct slog2_buffer_meta {
    _private: [u8; 0], // makes it zero-sized and opaque
}

pub type slog2_buffer_t = *mut slog2_buffer_meta;

#[repr(C)]
pub struct slog2_buffer_config_t {
    pub buffer_name: *const c_char,
    pub num_pages: i32,
}

#[repr(C)]
pub struct slog2_buffer_set_config_t {
    pub num_buffers: i32,
    pub buffer_set_name: *const c_char,
    pub verbosity_level: u8,
    pub buffer_config: [slog2_buffer_config_t; SLOG2_MAX_BUFFERS],
    pub max_retries: u32,
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
    pub fn slog2c(buffer: slog2_buffer_t, code: i16, severity: u8, data: *const c_char) -> i32;

    pub fn slog2_set_verbosity(buffer: slog2_buffer_t, verbosity: u8) -> i32;

    pub fn slog2_get_verbosity(buffer: slog2_buffer_t) -> u8;

    pub fn slog2_register(
        config: *const slog2_buffer_set_config_t,
        handles: *mut slog2_buffer_t,
        flags: u32,
    ) -> i32;

    pub fn slog2_set_default_buffer(buffer: slog2_buffer_t) -> slog2_buffer_t;

    pub fn slog2_reset() -> i32;

    pub fn slog2_find_buffer(
        buffer_name: *const c_char,
        buffer_set_name: *const c_char,
    ) -> slog2_buffer_t;
}
