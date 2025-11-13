use std::fmt::{Debug, Display};
use std::io;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
/// Determines the starting position for passed-in offsets.
pub enum DLFileSeekDirection {
    /// Seek from start of stream.
    Head = 0x0,
    /// Seek from current position.
    Current = 0x1,
    /// Seek from end of stream.
    Tail = 0x2,
}

#[repr(i32)]
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
    pub fn to_io_result(&self) -> io::Result<()> {
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

#[repr(C)]
#[derive(Clone, Copy)]
pub struct OpenFileMode(pub u32);

impl OpenFileMode {
    pub fn read_only(&self) -> bool {
        self.0 & 0x1 != 0
    }
    pub fn write(&self) -> bool {
        self.0 & 0x2 != 0
    }
    pub fn append(&self) -> bool {
        self.0 & 0x4 != 0
    }
    pub fn detailed_errors(&self) -> bool {
        self.0 & 0x8 != 0
    }
    pub fn async_io(&self) -> bool {
        self.0 & 0x10 != 0
    }
    pub fn unk5(&self) -> bool {
        self.0 & 0x20 != 0
    }
    pub fn no_sharing(&self) -> bool {
        self.0 & 0x40 != 0
    }
    pub fn unk7(&self) -> bool {
        self.0 & 0x80 != 0
    }
}

impl Debug for OpenFileMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OpenFileMode {{ read_only: {}, write: {}, append: {}, detailed_errors: {}, async_io: {}, unk5: {}, no_sharing: {}, unk7: {} }}",
            self.read_only(),
            self.write(),
            self.append(),
            self.detailed_errors(),
            self.async_io(),
            self.unk5(),
            self.no_sharing(),
            self.unk7()
        )
    }
}
