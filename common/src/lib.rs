pub mod ffi;

use bitflags::bitflags;

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    Shutdown = ffi::SLOG2_SHUTDOWN,
    Critical = ffi::SLOG2_CRITICAL,
    Error = ffi::SLOG2_ERROR,
    Warning = ffi::SLOG2_WARNING,
    Notice = ffi::SLOG2_NOTICE,
    #[default]
    Info = ffi::SLOG2_INFO,
    Debug1 = ffi::SLOG2_DEBUG1,
    Debug2 = ffi::SLOG2_DEBUG2,

    Invalid = ffi::SLOG2_INVALID_VERBOSITY,
    Nothing = ffi::SLOG2_NO_LOG_VERBOSITY,
}

impl Verbosity {
    pub fn from_u8(value: u8) -> Self {
        match value {
            ffi::SLOG2_SHUTDOWN => Self::Shutdown,
            ffi::SLOG2_CRITICAL => Self::Critical,
            ffi::SLOG2_ERROR => Self::Error,
            ffi::SLOG2_WARNING => Self::Warning,
            ffi::SLOG2_NOTICE => Self::Notice,
            ffi::SLOG2_INFO => Self::Info,
            ffi::SLOG2_DEBUG1 => Self::Debug1,
            ffi::SLOG2_DEBUG2 => Self::Debug2,
            ffi::SLOG2_INVALID_VERBOSITY => Self::Invalid,
            ffi::SLOG2_NO_LOG_VERBOSITY => Self::Nothing,
            _ => unreachable!(),
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct RegisterFlags: u32 {
        const ALLOC_TYPE_SHMEM = ffi::SLOG2_ALLOC_TYPE_SHMEM;
        const TRY_REUSE_BUFFER_SET = ffi::SLOG2_TRY_REUSE_BUFFER_SET;
        const DISCARD_NEWLINE = ffi::SLOG2_DISCARD_NEWLINE;
        const HINT_SKIP_BUFFER_0 = ffi::SLOG2_HINT_SKIP_BUFFER_0;
        const HINT_SKIP_BUFFER_1 = ffi::SLOG2_HINT_SKIP_BUFFER_1;
        const HINT_SKIP_BUFFER_2 = ffi::SLOG2_HINT_SKIP_BUFFER_2;
        const HINT_SKIP_BUFFER_3 = ffi::SLOG2_HINT_SKIP_BUFFER_3;
        const ALLOC_TYPE_PHYSICA = ffi::SLOG2_ALLOC_TYPE_PHYSICAL;
        const LIMIT_RETRIES = ffi::SLOG2_LIMIT_RETRIES;
        const QUIET = ffi::SLOG2_QUIET;
        const DYNAMIC_VERBOSITY = ffi::SLOG2_DYNAMIC_VERBOSITY;
    }
}
