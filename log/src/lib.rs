pub mod ffi;

use std::{
    ffi::CString,
    ops::{Deref, DerefMut, Index, IndexMut},
};

pub use slog2_types::RegisterFlags;
pub use slog2_types::Verbosity;

#[macro_export]
macro_rules! log_verbosity_default_buffer {
    ($code:expr, $level:expr, $($arg:tt)+) => {
        if let Some(buffer) = $crate::Buffer::get_default_buffer() {
            buffer.log($code, $level, format_args!($($arg)+));
        }
    };
}

#[macro_export]
macro_rules! log_verbosity_specifc_buffer {
    ($buffer:expr, $code:expr, $level:expr, $($arg:tt)+) => {
        $buffer.log($code, $level, format_args!($($arg)+));
    };
}

#[macro_export]
macro_rules! log {
    ($severity:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        $crate::log_verbosity_default_buffer!(0, $severity, $fmt $(, $($arg)+)?);
    };

    ($severity:expr, buffer = $buffer:expr, $($rest:tt)+) => {
        $crate::log!($severity, @internal Some($buffer), None, $($rest)+);
    };
    ($severity:expr, code = $code:expr, $($rest:tt)+) => {
        $crate::log!($severity, @internal None, Some($code), $($rest)+);
    };

    ($severity:expr, @internal $buffer:expr, $code:expr, buffer = $b:expr, $($rest:tt)+) => {
        $crate::log!($severity, @internal Some($b), $code, $($rest)+);
    };
    ($severity:expr, @internal $buffer:expr, $code:expr, code = $c:expr, $($rest:tt)+) => {
        $crate::log!($severity, @internal $buffer, Some($c), $($rest)+);
    };
    ($severity:expr, @internal $buffer:expr, $code:expr, $fmt:literal $(, $($arg:tt)+)?) => {
        let buffer : Option<$crate::Buffer> = $buffer;
        match (buffer, $code) {
            (Some(buf), Some(c)) => $crate::log_verbosity_specifc_buffer!(buf, c, $severity, $fmt $(, $($arg)+)?),
            (Some(buf), None) => $crate::log_verbosity_specifc_buffer!(buf, 0, $severity, $fmt $(, $($arg)+)?),
            (None, Some(c)) => $crate::log_verbosity_default_buffer!(c, $severity, $fmt $(, $($arg)+)?),
            (None, None) => $crate::log_verbosity_default_buffer!(0, $severity, $fmt $(, $($arg)+)?),
        }
    };
}

#[macro_export]
macro_rules! shutdown {
    ( $($arg:tt)+ ) => {
        $crate::log!($crate::Verbosity::Shutdown, $($arg)+);
    };
}

#[macro_export]
macro_rules! critical {
    ( $($arg:tt)+ ) => {
        $crate::log!($crate::Verbosity::Critical, $($arg)+);
    };
}

#[macro_export]
macro_rules! error {
    ( $($arg:tt)+ ) => {
        $crate::log!($crate::Verbosity::Error, $($arg)+);
    };
}

#[macro_export]
macro_rules! warning {
    ( $($arg:tt)+ ) => {
        $crate::log!($crate::Verbosity::Warning, $($arg)+);
    };
}

#[macro_export]
macro_rules! notice {
    ( $($arg:tt)+ ) => {
        $crate::log!($crate::Verbosity::Notice, $($arg)+);
    };
}

#[macro_export]
macro_rules! info {
    ( $($arg:tt)+ ) => {
        $crate::log!($crate::Verbosity::Info, $($arg)+);
    };
}

#[macro_export]
macro_rules! debug1 {
    ( $($arg:tt)+ ) => {
        $crate::log!($crate::Verbosity::Debug1, $($arg)+);
    };
}

#[macro_export]
macro_rules! debug12 {
    ( $($arg:tt)+ ) => {
        $crate::log!($crate::Verbosity::Debug2, $($arg)+);
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Buffer(ffi::slog2_buffer_t);

impl Buffer {
    pub const USE_DEFAULT: Self = Buffer(std::ptr::null_mut());

    pub fn reset() -> std::io::Result<()> {
        let result = unsafe { ffi::slog2_reset() };
        if result == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn set_default_buffer(buffer: Option<Self>) -> Option<Self> {
        let passed_buffer: *mut ffi::slog2_buffer_meta = match buffer {
            Some(buffer) => buffer.0,
            None => std::ptr::null_mut(),
        };
        let default_buffer: *mut ffi::slog2_buffer_meta =
            unsafe { ffi::slog2_set_default_buffer(passed_buffer) };
        if default_buffer.is_null() {
            None
        } else {
            Some(Buffer(default_buffer))
        }
    }

    pub fn get_default_buffer() -> Option<Self> {
        let default_buffer =
            unsafe { ffi::slog2_set_default_buffer(-1isize as ffi::slog2_buffer_t) }; // pass -1 as buffer to get default buffer
        if default_buffer.is_null() {
            None
        } else {
            Some(Buffer(default_buffer))
        }
    }

    pub fn find_buffer(buffer_name: &str, buffer_set_name: &str) -> Option<Self> {
        let buffer_name = CString::new(buffer_name).ok()?;
        let buffer_set_name = CString::new(buffer_set_name).ok()?;
        let buffer =
            unsafe { ffi::slog2_find_buffer(buffer_name.as_ptr(), buffer_set_name.as_ptr()) };
        if buffer.is_null() {
            None
        } else {
            Some(Buffer(buffer))
        }
    }

    pub fn set_verbosity(&self, verbosity: Verbosity) -> std::io::Result<()> {
        let result = unsafe { ffi::slog2_set_verbosity(self.0, verbosity as u8) };
        if result == -1 {
            Err(std::io::Error::other("no success"))
        } else {
            Ok(())
        }
    }

    pub fn get_verbosity(&self) -> Verbosity {
        let verbosity = unsafe { ffi::slog2_get_verbosity(self.0) };
        Verbosity::from_u8(verbosity)
    }

    pub fn log(&self, code: i16, verbosity: Verbosity, args: std::fmt::Arguments) {
        let formatted = format!("{}", args);
        let c_str = std::ffi::CString::new(formatted).expect("Failed to create CString");
        unsafe { ffi::slog2c(self.0, code, verbosity as u8, c_str.as_ptr()) };
    }
}

#[derive(Debug, Default)]
pub struct BufferConfig {
    buffer_name: CString,
    num_pages: i32,
}

impl BufferConfig {
    pub fn buffer_name(&mut self, name: &str) -> Result<(), std::ffi::NulError> {
        self.buffer_name = CString::new(name)?;
        Ok(())
    }

    pub fn set_num_pages(&mut self, pages: i32) {
        self.num_pages = pages;
    }
}

#[derive(Debug, Default)]
pub struct BufferSetConfig<const S: usize = 1> {
    buffer_set_name: CString,
    verbosity_level: Verbosity,
    buffer_config: [BufferConfig; ffi::SLOG2_MAX_BUFFERS],
    #[allow(unused)]
    max_retries: u32,
}

impl<const S: usize> BufferSetConfig<S> {
    pub fn buffer_set_name(&mut self, name: &str) -> Result<(), std::ffi::NulError> {
        self.buffer_set_name = CString::new(name)?;
        Ok(())
    }

    pub fn set_verbosity(&mut self, verbosity_level: Verbosity) {
        self.verbosity_level = verbosity_level;
    }

    pub fn register(&self, flags: Option<RegisterFlags>) -> std::io::Result<[Buffer; S]> {
        let mut buffer_handle: [ffi::slog2_buffer_t; S] = unsafe { std::mem::zeroed() };
        let mut buffer_config: ffi::slog2_buffer_set_config_t = unsafe { std::mem::zeroed() };

        buffer_config.buffer_set_name = self.buffer_set_name.as_ptr();
        buffer_config.num_buffers = S as i32;
        buffer_config.verbosity_level = self.verbosity_level as u8;

        for (ffi_buffer_config, buffer_config) in buffer_config
            .buffer_config
            .iter_mut()
            .zip(self.buffer_config.iter())
        {
            ffi_buffer_config.buffer_name = buffer_config.buffer_name.as_ptr();
            ffi_buffer_config.num_pages = buffer_config.num_pages;
        }

        let result = unsafe {
            ffi::slog2_register(
                &buffer_config,
                buffer_handle.as_mut_ptr(),
                flags.map(|flags| flags.bits()).unwrap_or_default(),
            )
        };
        if result == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(buffer_handle.map(Buffer))
        }
    }
}

impl<const S: usize> Index<usize> for BufferSetConfig<S> {
    type Output = BufferConfig;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buffer_config[index]
    }
}

impl<const S: usize> IndexMut<usize> for BufferSetConfig<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buffer_config[index]
    }
}

impl Deref for BufferSetConfig<1> {
    type Target = BufferConfig;

    fn deref(&self) -> &Self::Target {
        &self.buffer_config[0]
    }
}

impl DerefMut for BufferSetConfig<1> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer_config[0]
    }
}
