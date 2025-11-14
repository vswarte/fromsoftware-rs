use std::io;
use vtable_rs::VPtr;

use super::DLIOResult;

#[vtable_rs::vtable]
pub trait DLMemoryInputStreamVmt {
    /// Sets the status of the last operation to [status].
    fn set_last_error(&mut self, status: DLIOResult);

    fn destructor(&mut self, param_2: u8);

    /// Returns the status of the last operation.
    fn get_last_error(&self) -> DLIOResult;

    /// Reads up to [length] bytes from the underlying target to [data]. Returns
    /// the number of bytes read successfully.
    fn read_bytes(&mut self, data: *mut u8, length: usize) -> usize;

    /// Whether or not there are bytes remaining in the stream.
    fn has_bytes_left(&self) -> bool;

    /// The number of bytes left in the reader.
    fn get_bytes_left(&self) -> usize;

    /// Skips count amount of bytes, returns the amount of bytes skipped. Will
    /// be less than count if position + count exceeds the stream's length.
    ///
    /// WARNING: Even though the function takes count as usize, it will cast it
    /// to u32 and panic if it exceeds 2GB.
    fn skip_bytes(&mut self, count: isize) -> isize;
}

#[repr(C)]
pub struct DLMemoryInputStream {
    pub vftable: VPtr<dyn DLMemoryInputStreamVmt, Self>,

    /// The number of bytes in the stream.
    pub capacity: usize,

    /// The stream's data.
    pub data: *const u8,

    /// The current position in the stream.
    pub current_position: usize,

    _unk20: u32,

    /// Whether the stream is open.
    pub is_open: bool,

    _pad25: [u8; 3],

    /// THe result of the last operation.
    pub status: DLIOResult,
}

impl DLMemoryInputStream {
    /// Safely moves the cursor to [difference] relative to [base].
    fn seek_relative(&mut self, difference: i64, base: usize) -> io::Result<u64> {
        if !difference.is_negative() {
            Ok(self.current_position as u64)
        } else if let Some(new) = base.checked_add_signed(difference.try_into_io()?) {
            let new = std::cmp::min(new, self.capacity);
            self.current_position = new;
            Ok(new as u64)
        } else {
            Err(io::Error::from(io::ErrorKind::InvalidInput))
        }
    }
}

impl io::Read for DLMemoryInputStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let result: usize =
            (self.vftable.read_bytes)(self, buf.as_mut_ptr(), buf.len()).try_into_io()?;
        (self.vftable.get_last_error)(self).as_io_result()?;
        Ok(result)
    }
}

impl io::Seek for DLMemoryInputStream {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        if !self.is_open {
            return Err(io::Error::from(io::ErrorKind::BrokenPipe));
        }

        match pos {
            io::SeekFrom::Start(pos) => {
                let new = std::cmp::min(pos.try_into_io()?, self.capacity);
                self.current_position = new;
                Ok(new as u64)
            }
            io::SeekFrom::End(difference) => self.seek_relative(difference, self.capacity),
            io::SeekFrom::Current(difference) => {
                self.seek_relative(difference, self.current_position)
            }
        }
    }

    fn stream_position(&mut self) -> io::Result<u64> {
        if !self.is_open {
            Err(io::Error::from(io::ErrorKind::BrokenPipe))
        } else {
            Ok(self.current_position as u64)
        }
    }
}

trait TryIntoIoExt<T>: Sized {
    /// Like try_into, but wraps any errors in [io::Error].
    fn try_into_io(self) -> io::Result<T>;
}

impl<T, U> TryIntoIoExt<T> for U
where
    U: TryInto<T>,
    U::Error: std::error::Error + Send + Sync + 'static,
{
    fn try_into_io(self) -> io::Result<T> {
        self.try_into().map_err(io::Error::other)
    }
}

#[vtable_rs::vtable]
pub trait DLMemoryOutputStreamVmt {
    fn unk00(&mut self);
    fn unk08(&mut self);
    fn unk0a(&mut self);

    /// Writes [length] bytes of [data] to the underlying target. Returns the
    /// number of bytes written successfully.
    fn write(&mut self, data: *const u8, length: usize) -> usize;
}

#[repr(C)]
pub struct DLMemoryOutputStream {
    pub vftable: VPtr<dyn DLMemoryOutputStreamVmt, Self>,
}

impl io::Write for DLMemoryOutputStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok((self.vftable.write)(self, buf.as_ptr(), buf.len()))
    }

    fn flush(&mut self) -> io::Result<()> {
        // There's probably a built-in flush method but I haven't found it yet
        Ok(())
    }
}
