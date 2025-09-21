use std::ffi::{c_uchar, c_uint};

pub const SLOG2_SHUTDOWN: c_uchar = 0;
pub const SLOG2_CRITICAL: c_uchar = 1;
pub const SLOG2_ERROR: c_uchar = 2;
pub const SLOG2_WARNING: c_uchar = 3;
pub const SLOG2_NOTICE: c_uchar = 4;
pub const SLOG2_INFO: c_uchar = 5;
pub const SLOG2_DEBUG1: c_uchar = 6;
pub const SLOG2_DEBUG2: c_uchar = 7;

pub const SLOG2_INVALID_VERBOSITY: c_uchar = u8::MAX;
pub const SLOG2_NO_LOG_VERBOSITY: c_uchar = u8::MAX - 1;

pub const SLOG2_ALLOC_TYPE_SHMEM: c_uint = 1 << 0;
pub const SLOG2_TRY_REUSE_BUFFER_SET: c_uint = 1 << 1;
pub const SLOG2_DISCARD_NEWLINE: c_uint = 1 << 2;
pub const SLOG2_HINT_SKIP_BUFFER_0: c_uint = 1 << 3;
pub const SLOG2_HINT_SKIP_BUFFER_1: c_uint = 1 << 4;
pub const SLOG2_HINT_SKIP_BUFFER_2: c_uint = 1 << 5;
pub const SLOG2_HINT_SKIP_BUFFER_3: c_uint = 1 << 6;
pub const SLOG2_ALLOC_TYPE_PHYSICAL: c_uint = 1 << 7;
pub const SLOG2_LIMIT_RETRIES: c_uint = 1 << 8;
pub const SLOG2_QUIET: c_uint = 1 << 9;
pub const SLOG2_DYNAMIC_VERBOSITY: c_uint = 1 << 10;
