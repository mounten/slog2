#![allow(non_camel_case_types)]

use std::ffi::c_char;
use std::ffi::c_uint;

use std::ffi::c_int;

pub const SLOG2_DUMP_LOGS_ALL: c_uint = 1 << 0;

#[repr(i32)]
pub enum slog2_hash_expiry_t {
    SLOG2_HASH_EXPIRY_ONE_DAY,
    SLOG2_HASH_EXPIRY_ONE_WEEK,
    SLOG2_HASH_EXPIRY_ONE_MONTH,
    SLOG2_HASH_EXPIRY_NUM,
}

#[repr(i32)]
pub enum slog2_obf_t {
    SLOG2_OBF_FILEPATH,
    SLOG2_OBF_NUM,
}

#[link(name = "slog2-extra")]
unsafe extern "C" {
    pub fn slog2_dump_logs_to_file(file: *mut libc::FILE, flags: c_uint) -> c_int;

    pub fn slog2_hash(
        expiry: slog2_hash_expiry_t,
        input: *const c_char,
        ouput_size: usize,
        output_hash: *mut c_char,
    ) -> c_int;

    pub fn slog2_obfuscate(
        obf_type: slog2_obf_t,
        flags: c_uint,
        input: *const c_char,
        size: usize,
        output: *mut c_char,
    ) -> c_int;
}
