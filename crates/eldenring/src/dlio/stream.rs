use std::ptr::NonNull;

use vtable_rs::VPtr;

use crate::dlkr::DLAllocatorBase;
use shared::OwnedPtr;

use super::{DLFileOperatorContainer, DLFileSeekDirection, DLIOResult};

#[vtable_rs::vtable]
pub trait DLInputStreamVmt {
    // Sets the status of last operation.
    fn set_last_error(&mut self, status: DLIOResult);

    fn destructor(&mut self, param_2: u32);

    /// Returns the status of last operation.
    fn get_status(&self) -> DLIOResult;

    /// Reads length amount of bytes from the stream into output. Returns -1 if the read failed,
    /// returns read amount of bytes otherwise.
    /// WARNING: Even though the function takes length as usize, it will cast it to u32 and panic if length exceeds 2GB.
    fn read_bytes(&mut self, output: *mut u8, length: usize) -> i32;

    /// Indicates if there's bytes left for reading.
    fn has_bytes_left(&self) -> bool;

    /// Indicates the amount of bytes left in the reader.
    fn get_bytes_left(&self) -> usize;

    /// Skips count amount of bytes, returns the amount of bytes skipped. Will be less than count if
    /// position + count exceeds the streams length.
    /// WARNING: Even though the function takes count as usize, it will cast it to u32 and panic if count exceeds 2GB.
    fn skip_bytes(&mut self, count: usize) -> usize;

    /// Closes the stream causing read functions to stop yielding bytes.
    fn close_stream(&mut self);

    /// Returns true if the stream is open.
    fn stream_open(&self) -> bool;

    /// Returns the size of one sector for disk this stream is reading from.
    /// Implemented only for DLFileInputStream and calls same method on the underlying MicrosoftDiskFileOperator
    fn get_disk_sector_size(&self) -> u32;

    /// Calls get_disk_sector_size2 on underlying file operator in DLFileInputStream.
    /// Underlying file operator's method implemented only for MicrosoftDiskFileOperator and it just calls to get_disk_sector_size, hence
    /// the name.
    fn get_disk_sector_size_2(&self) -> u32;

    /// Calls query_async_status on underlying file operator in DLFileInputStream
    /// On any other stream writes 0 and last read bytes count to args
    /// Returns true if operation was successful
    fn query_async_status(
        &mut self,
        bytes_remaining: &mut usize,
        bytes_transferred: Option<&mut usize>,
    ) -> bool;

    /// On DLFileInputStream calls read_bytes_async on the underlying file operator.
    /// On DLMemoryInputStream calls read_bytes on itself and populates last_read_bytes
    /// with the amount of bytes read.
    fn read_bytes_async(&mut self, output: *mut u8, length: usize) -> bool;
}

#[vtable_rs::vtable]
pub trait DLSeekableInputStreamVmt: DLInputStreamVmt {
    fn seek(&mut self, offset: usize, mode: DLFileSeekDirection) -> bool;

    /// Returns the current position in the stream (from the start).
    fn current_position(&self) -> usize;

    /// Checks if the position has reached the end of the stream.
    fn reached_end(&self) -> bool;
}

#[repr(C)]
pub struct DLFileInputStream {
    pub vftable: VPtr<dyn DLSeekableInputStreamVmt, Self>,
    /// Container that used to operate on the disk.
    /// Stores pointer to MicrosoftDiskFileOperator.
    pub file_operator_container: NonNull<DLFileOperatorContainer>,
    /// Status of the latest operation.
    pub status: DLIOResult,
}

#[repr(C)]
pub struct DLMemoryInputStream {
    /// Allocator used for this stream.
    pub vftable: VPtr<dyn DLSeekableInputStreamVmt, Self>,
    /// Capacity of the stream.
    pub capacity: usize,
    /// Pointer to the start of the stream data.
    pub data: *mut u8,
    /// Current position in the stream.
    pub current_position: usize,
    /// Amount of bytes read last time.
    /// Used to emulate the behavior of ReadAsync and return the amount of bytes read.
    pub last_read_bytes: u32,
    /// Indicates if the stream is open.
    pub is_open: bool,
    // _pad25: [u8; 3],
    pub status: DLIOResult,
    // _pad2c: [u8; 4],
}

#[repr(C)]
/// Input stream used as base in all decompress streams and DLBufferedInputStream.
pub struct PseudoAsyncInputStream {
    pub vftable: VPtr<dyn DLSeekableInputStreamVmt, Self>,
    /// Amount of bytes read last time.
    /// Used to emulate the behavior of ReadAsync and return the amount of bytes read.
    pub last_read_bytes: u32,
    // _pad: [u8; 4],
}

#[repr(C)]
/// Buffered input stream that reads data from the source stream into a buffer.
pub struct DLBufferedInputStream {
    pub base: PseudoAsyncInputStream,
    /// Source file stream to read from.
    pub source_stream: Option<NonNull<DLFileInputStream>>,
    /// Set to true if the source file stream reached the end of the file.
    pub is_eof: bool,
    // _pad19: [u8; 7],
    /// Buffer used to read data from the source stream.
    pub buffer: DLStreamBuffer,
}

#[vtable_rs::vtable]
pub trait DLOutputStreamVmt {
    /// Sets the status of last operation.
    fn set_last_error(&mut self, status: DLIOResult);

    fn destructor(&mut self, param_2: u32);

    /// Returns the status of last operation.
    fn get_last_error(&self) -> DLIOResult;

    /// Writes length amount of bytes from input to the stream. Returns 0 if the write failed,
    fn write(&mut self, input: *const u8, length: usize) -> usize;

    /// Returns amount of bytes that can be written to the stream.
    /// On DLFileOutputStream calls max_non_streamed_bytes on the underlying file operator.
    fn get_write_size(&self) -> usize;

    /// Closes the stream.
    /// On DLFileOutputStream calls flush, truncate and close on the underlying file operator.
    /// On DLMemoryOutputStream deallocates underlying stream buffer.
    fn close(&mut self);

    /// Flushes the stream.
    /// Implemented only for DLFileOutputStream, but because it calls for flush on MicrosoftDiskFileOperator
    /// and flush is not implemented there, it does nothing.
    fn flush(&mut self);

    /// Returns true if the stream is open.
    fn is_open(&self) -> bool;

    /// Returns the size of one sector for disk this stream is writing to.
    /// Implemented only for DLFileOutputStream and calls same method on the underlying MicrosoftDiskFileOperator which returns sector size of current disk.
    /// Probably used for other platforms.
    fn get_async_block_size(&self) -> u32;

    /// Returns alignment size for async writes.
    /// Underlying MicrosoftDiskFileOperator just calls get_async_block_size and returns the result.
    fn get_async_buffer_alignment_size(&self) -> u32;

    /// Calls query_async_status on underlying file operator in DLFileOutputStream
    /// On MemoryOutputStream writes 0 and last written bytes count to args
    /// Returns true if operation was successful
    fn query_async_status(
        &mut self,
        bytes_remaining: &mut usize,
        bytes_transferred: Option<&mut usize>,
    ) -> bool;

    /// # Safety
    ///
    /// The caller must ensure that the input is valid and the length is correct.
    /// On DLFileOutputStream calls start_async_write on the underlying file operator.
    /// On DLMemoryOutputStream calls write_bytes on itself and populates last_bytes_written
    /// with the amount of bytes written.
    unsafe fn start_async_write(&mut self, input: *const u8, length: usize) -> bool;
}

#[vtable_rs::vtable]
pub trait DLSeekableOutputStreamVmt: DLOutputStreamVmt {
    fn seek(&mut self, offset: usize, mode: DLFileSeekDirection) -> bool;

    fn set_auto_flush(&mut self, auto_flush: bool);

    fn pad_to_align(&mut self, alignment: usize, padding_byte: u8) -> bool;
}

#[repr(C)]
pub struct DLFileOutputStream {
    pub vftable: VPtr<dyn DLSeekableOutputStreamVmt, Self>,
    /// Container that used to operate on the disk.
    /// Stores pointer to MicrosoftDiskFileOperator.
    pub file_operator_container: NonNull<DLFileOperatorContainer>,
    /// Controls should the stream be flushed and truncated on close.
    pub set_eof_on_close: bool,
    // _pad11: [u8; 3],
    /// Status of the latest operation.
    pub status: DLIOResult,
}

#[repr(C)]
pub struct DLStreamBuffer {
    /// Allocator used for this stream.
    pub allocator: OwnedPtr<DLAllocatorBase>,
    /// Pointer to the start of the buffer data.
    pub data: *mut u8,
    /// Capacity of the buffer.
    pub capacity: usize,
    /// Controls buffer growth behavior.
    pub auto_resize: bool,
    // _pad19: [u8; 7],
    /// Offset of which the data starts.
    /// Can be not zero if the stream is aligned to a certain size.
    pub data_offset: usize,
    /// Current position in the stream.
    pub current_position: usize,
    /// Max ever used position in the stream.
    pub max_position: usize,
    /// Size to grow the buffer by when it runs out of space.
    pub growth_factor: usize,
}

#[repr(C)]
pub struct DLMemoryOutputStream {
    pub vftable: VPtr<dyn DLSeekableOutputStreamVmt, Self>,
    /// Underlying stream buffer.
    pub stream_buffer: DLStreamBuffer,
    /// Count of bytes written last time.
    /// Used to emulate the behavior of WriteAsync and return the amount of bytes written.
    pub last_bytes_written: usize,
    /// Status of latest operation.
    pub status: DLIOResult,
    // _pad: [u8; 4],
}
