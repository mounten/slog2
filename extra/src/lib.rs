pub mod ffi;

use std::{ffi::CString, fs::File, os::fd::AsRawFd};

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct DumpFlags: u32 {
        const DUMP_LOGS_ALL = ffi::SLOG2_DUMP_LOGS_ALL;
    }
}

pub fn dump_logs_to_file(file: &File, flags: Option<DumpFlags>) -> std::io::Result<()> {
    let fd = file.as_raw_fd();
    let mode = CString::new("w").unwrap();
    let c_file: *mut libc::FILE = unsafe { libc::fdopen(fd, mode.as_ptr()) };
    let result = unsafe {
        ffi::slog2_dump_logs_to_file(c_file, flags.map(|flags| flags.bits()).unwrap_or_default())
    };
    if result == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}
