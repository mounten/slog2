pub const SLOG2_SHUTDOWN: u8 = 0;
pub const SLOG2_CRITICAL: u8 = 1;
pub const SLOG2_ERROR: u8 = 2;
pub const SLOG2_WARNING: u8 = 3;
pub const SLOG2_NOTICE: u8 = 4;
pub const SLOG2_INFO: u8 = 5;
pub const SLOG2_DEBUG1: u8 = 6;
pub const SLOG2_DEBUG2: u8 = 7;

pub const SLOG2_INVALID_VERBOSITY: u8 = u8::MAX;
pub const SLOG2_NO_LOG_VERBOSITY: u8 = u8::MAX - 1;

pub const SLOG2_ALLOC_TYPE_SHMEM: u32 = 1 << 0;
pub const SLOG2_TRY_REUSE_BUFFER_SET: u32 = 1 << 1;
pub const SLOG2_DISCARD_NEWLINE: u32 = 1 << 2;
pub const SLOG2_HINT_SKIP_BUFFER_0: u32 = 1 << 3;
pub const SLOG2_HINT_SKIP_BUFFER_1: u32 = 1 << 4;
pub const SLOG2_HINT_SKIP_BUFFER_2: u32 = 1 << 5;
pub const SLOG2_HINT_SKIP_BUFFER_3: u32 = 1 << 6;
pub const SLOG2_ALLOC_TYPE_PHYSICAL: u32 = 1 << 7;
pub const SLOG2_LIMIT_RETRIES: u32 = 1 << 8;
pub const SLOG2_QUIET: u32 = 1 << 9;
pub const SLOG2_DYNAMIC_VERBOSITY: u32 = 1 << 10;
