pub mod ffi;

use std::{
    ffi::{CStr, CString, c_char},
    os::unix::ffi::OsStrExt,
    path::Path,
};

pub use slog2_types::RegisterFlags;
pub use slog2_types::Verbosity;

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ParseFlags: u32 {
        const DYNAMIC = ffi::SLOG2_PARSE_FLAGS_DYNAMIC;
        const CURRENT = ffi::SLOG2_PARSE_FLAGS_CURRENT;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct PacketFlags: u32 {
        const FIRSTPACKET = ffi::SLOG2_PACKET_FLAGS_FIRSTPACKET;
        const MONOTONIC = ffi::SLOG2_PACKET_FLAGS_MONOTONIC;
    }
}

#[derive(Clone, Copy)]
pub struct PacketInfo<'a> {
    info: &'a ffi::slog2_packet_info_t,
    payload: &'a CStr,
}

impl<'a> PacketInfo<'a> {
    pub(crate) fn from_callback_data(
        info: &'a ffi::slog2_packet_info_t,
        payload: &'a CStr,
    ) -> PacketInfo<'a> {
        Self { info, payload }
    }

    pub fn sequence_number(&self) -> u16 {
        self.info.sequence_number
    }

    pub fn size(&self) -> u16 {
        self.info.data_size
    }

    pub fn timestamp_raw(&self) -> u64 {
        self.info.timestamp
    }

    pub fn timestamp(&self) -> chrono::NaiveDateTime {
        chrono::DateTime::from_timestamp_nanos(self.timestamp_raw() as i64).naive_local()
    }

    pub fn thread_id(&self) -> u16 {
        self.info.thread_id
    }

    pub fn severity(&self) -> Verbosity {
        Verbosity::from_u8(self.info.severity)
    }

    pub fn file_name(&self) -> Result<&str, std::str::Utf8Error> {
        unsafe { CStr::from_ptr(self.info.file_name.as_ptr()).to_str() }
    }

    pub fn buffer_name(&self) -> Result<&str, std::str::Utf8Error> {
        unsafe { CStr::from_ptr(self.info.buffer_name.as_ptr()).to_str() }
    }

    pub fn owner_pid(&self) -> u32 {
        self.info.owner_pid
    }

    pub fn flags(&self) -> Option<PacketFlags> {
        PacketFlags::from_bits(self.info.flags)
    }

    pub fn register_flags(&self) -> Option<RegisterFlags> {
        RegisterFlags::from_bits(self.info.register_flags)
    }

    pub fn message(&self) -> Result<&str, std::str::Utf8Error> {
        self.payload.to_str()
    }
}

impl<'a> std::fmt::Debug for PacketInfo<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PacketInfo")
            .field("sequence_number", &self.sequence_number())
            .field("size", &self.size())
            .field("timestamp", &self.timestamp())
            .field("thread_id", &self.thread_id())
            .field("severity", &self.severity())
            .field("file_name", &self.file_name().ok())
            .field("buffer_name", &self.buffer_name().ok())
            .field("owner_pid", &self.owner_pid())
            .field("flags", &self.flags())
            .field("register_flags", &self.register_flags())
            .field("message", &self.message().ok())
            .finish()
    }
}

pub struct LogInfo(ffi::slog2_log_info_t);

impl LogInfo {
    pub fn num_buffers(&self) -> usize {
        self.0.num_buffers as usize
    }

    pub fn owner_pid(&self) -> u32 {
        self.0.owner_pid
    }

    pub fn buffer_name(&self) -> Result<&str, std::str::Utf8Error> {
        unsafe { CStr::from_ptr(self.0.buffer_set_name).to_str() }
    }

    pub fn verbosity(&self) -> Verbosity {
        Verbosity::from_u8(self.0.verbosity_level)
    }
}

impl std::fmt::Debug for LogInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogInfo")
            .field("num_buffers", &self.num_buffers())
            .field("owner_pid", &self.owner_pid())
            .field("buffer_name", &self.buffer_name().ok())
            .field("verbosity", &self.verbosity())
            .finish()
    }
}

pub struct BufferInfo(ffi::slog2_buffer_info_t);

impl BufferInfo {
    pub fn buffer_size(&self) -> u32 {
        self.0.buffer_size
    }

    pub fn buffer_name(&self) -> Result<&str, std::str::Utf8Error> {
        unsafe { CStr::from_ptr(self.0.buffer_name).to_str() }
    }
}

impl std::fmt::Debug for BufferInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferInfo")
            .field("buffer_size", &self.buffer_size())
            .field("buffer_name", &self.buffer_name().ok())
            .finish()
    }
}

pub struct BufferInfoIterator<'a> {
    log_file: &'a LogFile,
    num_buffers: usize,
    current_index: usize,
}

impl<'a> BufferInfoIterator<'a> {
    pub fn new(log_file: &'a LogFile, num_buffers: usize) -> BufferInfoIterator<'a> {
        BufferInfoIterator {
            num_buffers,
            current_index: 0,
            log_file: log_file,
        }
    }
}

impl<'a> Iterator for BufferInfoIterator<'a> {
    type Item = Option<BufferInfo>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.num_buffers {
            None
        } else {
            let buffer = self.log_file.buffer_info(self.current_index as i32).ok();
            self.current_index += 1;
            Some(buffer)
        }
    }
}

pub struct LogFile(ffi::slog2_log_t);

impl LogFile {
    pub fn open(filename: &str) -> Option<Self> {
        let filename = CString::new(filename).ok()?;
        let result = unsafe { ffi::slog2_open_log(filename.as_ptr()) };
        if result.is_null() {
            None
        } else {
            Some(LogFile(result))
        }
    }

    pub fn info(&self) -> Result<LogInfo, i32> {
        let mut info = ffi::SLOG2_LOG_INFO_INIT;
        let result = unsafe { ffi::slog2_get_log_info(self.0, &mut info) };
        if result == 0 {
            Ok(LogInfo(info))
        } else {
            Err(result)
        }
    }

    pub fn buffer_info(&self, buffer_index: i32) -> Result<BufferInfo, i32> {
        let mut info = ffi::SLOG2_BUFFER_INFO_INIT;
        let result = unsafe { ffi::slog2_get_buffer_info(self.0, buffer_index, &mut info) };
        if result == 0 {
            Ok(BufferInfo(info))
        } else {
            Err(result)
        }
    }

    pub fn parse_static<F>(&self, buffer_index: i32, callback: F) -> Result<(), i32>
    where
        F: FnMut(PacketInfo) -> Result<(), i32>,
    {
        let mut callback_box = Box::new(callback);
        let param = &mut *callback_box as *mut _ as *mut core::ffi::c_void;
        let mut packet_info = ffi::SLOG2_PACKET_INFO_INIT;
        let result = unsafe {
            ffi::slog2_parse_dynamic_buffer(
                self.0,
                buffer_index,
                &mut packet_info,
                Some(parse_trampoline::<F>),
                param,
            )
        };
        if result == 0 { Ok(()) } else { Err(result) }
    }

    pub fn parse_dynamic<F>(&self, buffer_index: i32, callback: F) -> Result<(), i32>
    where
        F: FnMut(PacketInfo) -> Result<(), i32>,
    {
        let mut callback_box = Box::new(callback);
        let param = &mut *callback_box as *mut _ as *mut core::ffi::c_void;
        let mut packet_info = ffi::SLOG2_PACKET_INFO_INIT;
        let result = unsafe {
            ffi::slog2_parse_dynamic_buffer(
                self.0,
                buffer_index,
                &mut packet_info,
                Some(parse_trampoline::<F>),
                param,
            )
        };
        if result == 0 { Ok(()) } else { Err(result) }
    }

    pub(crate) fn buffer_info_iter(&self) -> Result<BufferInfoIterator, i32> {
        Ok(BufferInfoIterator::new(self, self.info()?.num_buffers()))
    }
}

impl<'a> IntoIterator for &'a LogFile {
    type Item = Option<BufferInfo>;
    type IntoIter = BufferInfoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        // Since we can't return an error we return an iterator with zero elements
        self.buffer_info_iter().unwrap_or(BufferInfoIterator {
            log_file: self,
            num_buffers: 0,
            current_index: 0,
        })
    }
}

impl Drop for LogFile {
    fn drop(&mut self) {
        unsafe { ffi::slog2_close_log(self.0) };
    }
}

pub fn parse_all<F>(
    flags: Option<ParseFlags>,
    directory: Option<&Path>,
    match_list: Option<&str>,
    callback: F,
) -> Result<(), i32>
where
    F: FnMut(PacketInfo) -> Result<(), i32>,
{
    let directory = directory
        .map(|p| CString::new(p.as_os_str().as_bytes()).expect("Path contains interior NUL"));
    let directory_ptr = directory
        .as_ref()
        .map_or(std::ptr::null_mut(), |cstr| cstr.as_ptr() as *mut c_char);

    let match_list = match_list.map(|s| CString::new(s).expect("Path contains interior NUL"));
    let match_list_ptr = match_list
        .as_ref()
        .map_or(std::ptr::null_mut(), |cstr| cstr.as_ptr() as *mut c_char);

    let mut callback_box = Box::new(callback);
    let param = &mut *callback_box as *mut _ as *mut core::ffi::c_void;
    let mut packet_info = ffi::SLOG2_PACKET_INFO_INIT;
    let result = unsafe {
        ffi::slog2_parse_all(
            flags.map(|flags| flags.bits()).unwrap_or_default(),
            directory_ptr,
            match_list_ptr,
            &mut packet_info,
            Some(parse_trampoline::<F>),
            param,
        )
    };
    if result == 0 { Ok(()) } else { Err(result) }
}

extern "C" fn parse_trampoline<F>(
    info: *mut ffi::slog2_packet_info_t,
    payload: *mut core::ffi::c_void,
    param: *mut core::ffi::c_void,
) -> i32
where
    F: FnMut(PacketInfo) -> Result<(), i32>,
{
    let closure: &mut F = unsafe { &mut *(param as *mut F) };
    let payload = unsafe { CStr::from_ptr(payload as *mut c_char) };
    let info = PacketInfo::from_callback_data(unsafe { &*info }, payload);
    if let Err(res) = closure(info) { res } else { 0 }
}
