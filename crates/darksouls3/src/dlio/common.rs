use std::fmt::Debug;
use std::io;

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum DLIOResult {
    DirNotEmpty = -17,
    OutOfMemory = -13,
    DiskFull = -12,
    NotStreamed = -9,
    AlreadyOpen = -6,
    IsNotOpen = -5,
    NotFound = -4,
    AccessDenied = -3,
    OperationUnsupported = -2,
    Invalid = -1,
    Success = 0,
    NoMoreFiles = 1,
}

impl DLIOResult {
    /// Converts this into an equivalent [io::Result].
    pub fn as_io_result(&self) -> io::Result<()> {
        use io::ErrorKind::*;
        let kind = match self {
            DLIOResult::DirNotEmpty => DirectoryNotEmpty,
            DLIOResult::OutOfMemory => OutOfMemory,
            DLIOResult::DiskFull => StorageFull,
            DLIOResult::IsNotOpen => BrokenPipe,
            DLIOResult::NotFound => NotFound,
            DLIOResult::AccessDenied => PermissionDenied,
            DLIOResult::OperationUnsupported => Unsupported,
            DLIOResult::Invalid => InvalidInput,
            // The following mappings are guesses as to the intended semantics
            DLIOResult::AlreadyOpen => PermissionDenied,
            DLIOResult::NotStreamed => NotConnected,
            _ => return Ok(()),
        };
        Err(io::Error::from(kind))
    }
}
