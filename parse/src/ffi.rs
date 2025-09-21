#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_uchar, c_ulonglong, c_void};

use std::ffi::c_int;
use std::ffi::c_uint;
use std::ffi::c_ushort;

pub const SLOG2_PARSE_MAX_NAME_SIZE: usize = 64;

pub const SLOG2_PARSE_FLAGS_DYNAMIC: c_uint = 0x00000001;
pub const SLOG2_PARSE_FLAGS_CURRENT: c_uint = 0x00000002;

pub const SLOG2_PACKET_FLAGS_FIRSTPACKET: c_uint = 0x00000001;
pub const SLOG2_PACKET_FLAGS_MONOTONIC: c_uint = 0x00000002;

pub type slog2_log_t = *mut ::core::ffi::c_void;

#[repr(i32)]
pub enum slog2_packet_data_type_t {
    SLOG2_TYPE_ASCII_STRING,
    SLOG2_TYPE_BINARY,
    SLOG2_TYPE_UNSYNC,
    SLOG2_TYPE_ONLINE,
}

#[repr(C)]
pub struct slog2_log_info_t {
    pub size: c_uint,
    pub num_buffers: c_uint,
    pub owner_pid: c_uint,
    pub buffer_set_name: *const c_char,
    pub verbosity_level: c_uchar,
}

pub const SLOG2_LOG_INFO_INIT: slog2_log_info_t = slog2_log_info_t {
    size: core::mem::size_of::<slog2_log_info_t>() as c_uint,
    num_buffers: 0,
    owner_pid: 0,
    buffer_set_name: core::ptr::null(),
    verbosity_level: 0,
};

#[repr(C)]
pub struct slog2_buffer_info_t {
    pub size: c_uint,
    pub buffer_size: c_uint,
    pub buffer_name: *const c_char,
}

pub const SLOG2_BUFFER_INFO_INIT: slog2_buffer_info_t = slog2_buffer_info_t {
    size: core::mem::size_of::<slog2_buffer_info_t>() as c_uint,
    buffer_size: 0,
    buffer_name: core::ptr::null(),
};

#[repr(C)]
pub struct slog2_packet_info_t {
    pub size: c_uint,
    pub sequence_number: c_ushort,
    pub data_size: c_ushort,
    pub timestamp: c_ulonglong,
    pub data_type: slog2_packet_data_type_t,
    pub thread_id: c_ushort,
    pub code: c_ushort,
    pub severity: c_uchar,
    pub file_name: [c_char; 2 * SLOG2_PARSE_MAX_NAME_SIZE],
    pub buffer_name: [c_char; SLOG2_PARSE_MAX_NAME_SIZE],
    pub owner_pid: c_uint,
    pub flags: c_uint,
    pub register_flags: c_uint,
}

pub const SLOG2_PACKET_INFO_INIT: slog2_packet_info_t = slog2_packet_info_t {
    size: core::mem::size_of::<slog2_packet_info_t>() as c_uint,
    sequence_number: 0,
    data_size: 0,
    timestamp: 0,
    data_type: slog2_packet_data_type_t::SLOG2_TYPE_ASCII_STRING,
    thread_id: 0,
    code: 0,
    severity: 0,
    file_name: [0; 2 * SLOG2_PARSE_MAX_NAME_SIZE],
    buffer_name: [0; SLOG2_PARSE_MAX_NAME_SIZE],
    owner_pid: 0,
    flags: 0,
    register_flags: 0,
};

pub type slog2_packet_callback = Option<
    unsafe extern "C" fn(
        info: *mut slog2_packet_info_t,
        payload: *mut core::ffi::c_void,
        param: *mut core::ffi::c_void,
    ) -> c_int,
>;

#[link(name = "slog2parse")]
unsafe extern "C" {
    pub fn slog2_open_log(filename: *const c_char) -> slog2_log_t;

    pub fn slog2_close_log(log: slog2_log_t);

    pub fn slog2_get_log_info(log: slog2_log_t, log_info: *mut slog2_log_info_t) -> c_int;

    pub fn slog2_packet_cmp(
        packet1: *mut slog2_packet_info_t,
        packet2: *mut slog2_packet_info_t,
    ) -> c_int;

    pub fn slog2_get_buffer_info(
        log: slog2_log_t,
        buffer_index: c_int,
        buffer_info: *mut slog2_buffer_info_t,
    ) -> c_int;

    pub fn slog2_parse_static_buffer(
        log: slog2_log_t,
        buffer_index: c_int,
        packet_info: *mut slog2_packet_info_t,
        callback: slog2_packet_callback,
        param: *mut c_void,
    ) -> c_int;

    pub fn slog2_parse_dynamic_buffer(
        log: slog2_log_t,
        buffer_index: c_int,
        packet_info: *mut slog2_packet_info_t,
        callback: slog2_packet_callback,
        param: *mut c_void,
    ) -> c_int;

    pub fn slog2_parse_all(
        flags: c_uint,
        directory_path: *mut c_char,
        match_list: *mut c_char,
        packet_info: *mut slog2_packet_info_t,
        callback: slog2_packet_callback,
        param: *mut c_void,
    ) -> c_int;

    pub fn slog2_parse_settings(flags: c_uint) -> c_int;
}

pub const SLOG2_PARSE_SETTINGS_VERBOSE: c_int = 1 << 0;
