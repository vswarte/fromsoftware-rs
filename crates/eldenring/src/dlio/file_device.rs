use std::{
    fmt::Display,
    io::{Cursor, Read, Seek, SeekFrom},
    ptr::NonNull,
};

use vtable_rs::VPtr;

use crate::{
    dlio::DLIOResult,
    dlkr::{DLAllocatorBase, DLPlainLightMutex},
    dltx::DLString,
    dlut::DLDateTime,
    Vector,
};
use shared::OwnedPtr;

use super::{DLFileSeekDirection, OpenFileMode};

#[repr(u32)]
pub enum DLFileDeviceDriveType {
    Unknown = 0x0,
    CdRom = 0x1,
    Default = 0x2,
}

#[repr(C)]
pub struct DLFileDeviceBase {
    pub vftable: VPtr<dyn DLFileDeviceVmt, Self>,
    unk8: bool,
    // _pad9: [u8; 3],
    ref_count: u32,
    pub mutex: DLPlainLightMutex,
}

/// Represents a source of files.
#[vtable_rs::vtable]
pub trait DLFileDeviceVmt {
    fn destructor(&mut self);

    /// Returns pointer to DLFileOperator
    fn get_file_operator(
        &mut self,
        path_dlstring: &DLString,
        path_u16: *const u16,
        operator_container: &mut DLFileOperatorContainer,
        allocator: &mut DLAllocatorBase,
        is_temp_file: bool,
    ) -> *const DLFileOperatorBase;

    fn file_enumerator(&self) -> *const u8;

    fn get_drive_type(&self, path: *const u16) -> DLFileDeviceDriveType {
        DLFileDeviceDriveType::Default
    }

    /// Set true on DLEncryptedBinderLightFileDevice.
    /// Will use additional processing logic in DLEncryptedBinderLightUtility by creating
    /// a DLEncryptedBinderLightObject and passing FileOperator to second vftable entry of it.
    fn is_encrypted(&self) -> bool {
        false
    }
}

#[repr(C)]
pub struct DLFileEnumeratorSPIBase {
    pub vftable: VPtr<dyn DLFileEnumeratorSPIVmt, Self>,
}

/// Used to locate files from a given device.
#[vtable_rs::vtable]
pub trait DLFileEnumeratorSPIVmt {
    fn destructor(&mut self);

    /// Starts a search using the given search parameter. Writes a handle to search_handle that
    /// can be passed to the rest of the methods. Also writes the first found path to the
    /// found parameter. No results have been found if the found output parameter is empty.
    fn start_search(&mut self, search: &[u16], search_handle: &mut u64, found: &mut DLString);

    /// Stops a search using the search_handle.
    fn close_search(&self, search_handle: &u64);

    /// Attempts to find the next file that matches the search specified when creating the
    /// search_handle. If found is empty after calling no further results have been found.
    fn search_next(&mut self, search_handle: &mut u64, found: &mut DLString);
}

/// Represents a remote file abstracting away the storage.
#[vtable_rs::vtable]
pub trait DLFileOperatorVmt {
    fn destructor(&mut self);

    /// Copies the data from the source DLFileOperator into itself.
    fn copy_from(&mut self, source: &DLFileOperatorBase) -> bool;

    /// Sets the path for the file operator.
    /// Additionally changes the ioState.
    /// See AdapterFileOperator::set_path for implementation details.
    fn set_path(&mut self, path: &DLString, param_3: bool, param_4: bool) -> bool;

    /// Duplicate of set_path, believed to be for other DLString variants.
    fn set_path_other_1(&mut self, path: &DLString, param_3: bool, param_4: bool) -> bool;

    /// Duplicate of set_path, believed to be for other DLString variants.
    fn set_path_other_2(&mut self, path: &DLString, param_3: bool, param_4: bool) -> bool;

    /// Seems to be a function to reset ioState or toggle specific bits on it.
    /// Same happens in all set_path variants.
    /// See AdapterFileOperator::set_path for implementation details.
    fn set_state(&mut self, param_2: bool, param_3: bool) -> bool;

    /// Clears file info like path, or calls to close.
    fn clear_file_info(&mut self) -> bool;

    /// Returns a pointer to the virtual disk operator.
    fn get_virtual_disk_operator(&self) -> *const DLFileOperatorBase;

    /// Binds the file operator to a device image.
    fn bind_device_image(
        &mut self,
        image_spi: &DLFileDeviceImageSPIBase,
    ) -> *const DLFileDeviceImageSPIBase;

    /// Checks that path exists and is a file that can be read.
    fn is_readable(&mut self) -> bool;

    /// Checks if the file is writable.
    fn is_writable(&mut self) -> bool;

    /// Returns last access time in FS's proprietary format.
    fn last_access_time(&self, ptr: *const DLDateTime) -> *const DLDateTime;

    /// Returns last modify time in FS's proprietary format.
    fn last_modify_time(&self, ptr: *const DLDateTime) -> *const DLDateTime;

    /// Returns the size of the file in bytes.
    fn file_size(&mut self) -> usize;

    /// Returns the amount of bytes left to read.
    fn get_read_size(&mut self) -> usize;

    fn get_write_size(&self) -> usize;

    /// Truncates the file to the current position.
    fn set_eof(&mut self);

    /// Checks if the end of the file has been reached.
    /// by checking
    fn is_eof(&self) -> bool;

    fn is_directory(&self) -> bool;

    fn is_open(&self) -> bool;

    fn open(&mut self, open_mode: OpenFileMode) -> bool;

    /// Closes the file and releases os file handle or closes underlying readers depending on the implementation.
    /// Returns true if the file was closed successfully.
    fn close(&mut self) -> bool;

    fn set_read_only(&mut self, is_open: bool) -> bool;

    fn seek(&mut self, is_stream: bool, offset: i64, seek_mode: DLFileSeekDirection) -> bool;

    fn cursor_position(&self) -> usize;

    /// # Safety
    ///
    /// Caller must ensure that `output` is a pointer to valid memory.
    /// Returns amount of bytes read.
    /// It can be less than length if the file is smaller or, value is bigger than u32::MAX because of WinAPI limitations.
    unsafe fn read(&mut self, output: *mut u8, length: usize) -> i32;

    fn write(&mut self, input: *const u8, length: usize) -> usize;

    fn get_async_block_size(&self) -> usize;

    fn get_async_buffer_alignment_size(&self) -> usize;

    /// # Safety
    ///
    /// Caller must ensure that `output` is a pointer to valid memory.
    unsafe fn start_async_read(&mut self, output: *mut u8, length: usize) -> bool;

    fn start_async_write(&mut self, input: *const u8, length: usize) -> bool;

    fn query_async_status(
        &mut self,
        bytes_remaining: &mut usize,
        bytes_transferred: Option<&mut usize>,
    ) -> bool;

    fn get_open_mode(&self) -> OpenFileMode;

    /// Can delete both files and directories.
    fn delete(&mut self) -> bool;

    /// Called by DLFileOutputStream::Close as part of flush > truncate > close chain.
    /// Not implemented in all file operators.
    fn flush(&mut self);

    fn populate_file_info(&mut self) -> bool;

    fn unk2(&mut self) -> bool;

    fn rename_w(&mut self, path: *const u16) -> bool;

    fn rename(&mut self, path: *const u8) -> bool;

    fn create_directory(&mut self) -> bool;
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct DLFileOperatorIOState(pub u32);

impl DLFileOperatorIOState {
    pub fn is_open(&self) -> bool {
        self.0 & 0x1 != 0
    }
    pub fn unk1(&self) -> bool {
        self.0 & 0x2 != 0
    }
    pub fn unk2(&self) -> bool {
        self.0 & 0x4 != 0
    }
}

impl Display for DLFileOperatorIOState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DLFileOperatorIOState {{ is_open: {}, unk1: {}, unk2: {} }}",
            self.is_open(),
            self.unk1(),
            self.unk2()
        )
    }
}

#[repr(C)]
pub struct DLFileOperatorBase<T: DLFileOperatorVmt = AdapterFileOperator<Cursor<Vec<u8>>>> {
    pub vftable: VPtr<dyn DLFileOperatorVmt, T>,
    /// Allocator passed to constructor, used for all memory operations
    pub allocator: NonNull<DLAllocatorBase>,
    /// Result of latest operation involving this file operator
    pub result: DLIOResult,
    // _pad14: u32,
    /// File operator container, that owns this file operator.
    pub owning_operator_container: NonNull<DLFileOperatorContainer>,
    /// Bitfield that tracks the state of the file operator.
    /// bit 0: 0x1 - file is open
    /// bit 1 - Unk, changed by vftable.set_path
    /// bit 2: Unk, changed by vftable.set_path
    pub io_state: DLFileOperatorIOState,
    // _pad24: u32,
    /// File device that created this file operator.
    pub owning_file_device: NonNull<DLFileDeviceBase>,
    /// Virtual or real path, this operator is bound to.
    pub path: DLString,
}

impl<T> DLFileOperatorBase<T>
where
    T: DLFileOperatorVmt,
{
    pub fn new(
        vftable: VPtr<dyn DLFileOperatorVmt, T>,
        allocator: &DLAllocatorBase,
        path: &DLString,
        operator_container: &DLFileOperatorContainer,
        file_device: &DLFileDeviceBase,
    ) -> Self {
        Self {
            vftable,
            allocator: NonNull::from(allocator),
            result: DLIOResult::Success,
            owning_operator_container: NonNull::from(operator_container),
            io_state: DLFileOperatorIOState::default(),
            owning_file_device: NonNull::from(file_device),
            path: DLString::copy(allocator, path).expect("Failed to copy DLString"),
        }
    }
}

#[vtable_rs::vtable]
pub trait DLFileDeviceImageSPIVmt {
    fn destructor(&mut self);
}

#[repr(C)]
pub struct DLFileDeviceImageSPIBase {
    pub vftable: VPtr<dyn DLFileDeviceImageSPIVmt, Self>,
}

#[repr(C)]
pub struct BndEntry {
    pub name: DLString,
    pub device: NonNull<DLFileDeviceBase>,
    pub file_size: u64,
}

#[repr(C)]
pub struct DLFileOperatorContainer {
    allocator: NonNull<DLAllocatorBase>,
    read_file_operator: OwnedPtr<DLFileOperatorBase>,
    write_file_operator: OwnedPtr<DLFileOperatorBase>,
    flags: u32,
}

#[repr(C)]
pub struct DLFileDeviceManager {
    pub devices: Vector<NonNull<DLFileDeviceBase>>,
    pub service_providers: Vector<NonNull<DLFileDeviceImageSPIBase>>,
    pub msvc_file_device: OwnedPtr<DLFileDeviceBase>,
    pub virtual_roots: Vector<[DLString; 2]>,
    pub bnd3_files: Vector<BndEntry>,
    pub bnd4_files: Vector<BndEntry>,
    pub bnd3_service_provider: OwnedPtr<DLFileDeviceImageSPIBase>,
    pub bnd4_service_provider: OwnedPtr<DLFileDeviceImageSPIBase>,
    pub mutex: DLPlainLightMutex,
}

impl DLFileDeviceVmt for DLFileDeviceBase {
    extern "C" fn destructor(&mut self) {
        (self.vftable.destructor)(self);
    }

    extern "C" fn get_file_operator(
        &mut self,
        name_dlstring: &DLString,
        name_u16: *const u16,
        operator_container: &mut DLFileOperatorContainer,
        allocator: &mut DLAllocatorBase,
        is_temp_file: bool,
    ) -> *const DLFileOperatorBase {
        (self.vftable.get_file_operator)(
            self,
            name_dlstring,
            name_u16,
            operator_container,
            allocator,
            is_temp_file,
        )
    }

    extern "C" fn file_enumerator(&self) -> *const u8 {
        (self.vftable.file_enumerator)(self)
    }

    extern "C" fn get_drive_type(&self, path: *const u16) -> DLFileDeviceDriveType {
        (self.vftable.get_drive_type)(self, path)
    }

    extern "C" fn is_encrypted(&self) -> bool {
        (self.vftable.is_encrypted)(self)
    }
}

#[repr(C)]
pub struct AdapterFileOperator<R>
where
    R: Read + Seek + 'static,
{
    pub base: DLFileOperatorBase,
    buffer: R,
}

impl<R> AdapterFileOperator<R>
where
    R: Read + Seek + 'static,
{
    pub fn new(
        allocator: &DLAllocatorBase,
        path: &DLString,
        operator_container: &DLFileOperatorContainer,
        file_device: &DLFileDeviceBase,
        buffer: R,
    ) -> Self {
        Self {
            base: DLFileOperatorBase::new(
                Default::default(),
                allocator,
                path,
                operator_container,
                file_device,
            ),
            buffer,
        }
    }
}

impl<R> Display for AdapterFileOperator<R>
where
    R: Read + Seek + 'static,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AdapterFileOperator({})", self.base.path)
    }
}

impl<R> DLFileOperatorVmt for AdapterFileOperator<R>
where
    R: Read + Seek + 'static,
{
    extern "C" fn destructor(&mut self) {
        tracing::debug!("{self}::destructor()");
    }

    extern "C" fn copy_from(&mut self, source: &DLFileOperatorBase) -> bool {
        tracing::debug!("{self}::copy_from()");
        unimplemented!()
    }

    extern "C" fn set_path(&mut self, path: &DLString, param_3: bool, param_4: bool) -> bool {
        tracing::debug!(
            "{self}::set_path({}, {}, {})",
            path.to_string(),
            param_3,
            param_4
        );

        self.base.io_state.0 &= 0xfffffff9;
        self.base.io_state.0 |= ((((param_4 as u32 & 1) * 2) | (param_3 as u32 & 1)) * 2);

        true
    }

    extern "C" fn set_path_other_1(
        &mut self,
        path: &DLString,
        param_3: bool,
        param_4: bool,
    ) -> bool {
        tracing::debug!(
            "{self}::set_path_other_1({}, {}, {})",
            path.to_string(),
            param_3,
            param_4
        );
        unimplemented!()
    }

    extern "C" fn set_path_other_2(
        &mut self,
        path: &DLString,
        param_3: bool,
        param_4: bool,
    ) -> bool {
        tracing::debug!(
            "{self}::set_path_other_2({}, {}, {})",
            path.to_string(),
            param_3,
            param_4
        );
        unimplemented!()
    }

    extern "C" fn set_state(&mut self, param_2: bool, param_3: bool) -> bool {
        tracing::debug!("{self}::set_state({}, {})", param_2, param_3);

        self.base.io_state.0 &= 0xfffffff9;
        self.base.io_state.0 |= ((((param_3 as u32 & 1) * 2) | (param_2 as u32 & 1)) * 2);

        true
    }

    extern "C" fn clear_file_info(&mut self) -> bool {
        tracing::debug!("{self}::clear_file_info()");
        unimplemented!()
    }

    extern "C" fn get_virtual_disk_operator(&self) -> *const DLFileOperatorBase {
        tracing::debug!("{self}::get_virtual_disk_operator()");
        unimplemented!()
    }

    extern "C" fn bind_device_image(
        &mut self,
        image_spi: &DLFileDeviceImageSPIBase,
    ) -> *const DLFileDeviceImageSPIBase {
        tracing::debug!("{self}::bind_device_image()");
        unimplemented!()
    }

    extern "C" fn is_readable(&mut self) -> bool {
        tracing::debug!("{self}::is_readable()");
        unimplemented!()
    }
    extern "C" fn is_writable(&mut self) -> bool {
        tracing::debug!("{self}::is_writable()");
        unimplemented!()
    }

    extern "C" fn last_access_time(&self, ptr: *const DLDateTime) -> *const DLDateTime {
        tracing::debug!("{self}::last_access_time()");
        unimplemented!()
    }

    extern "C" fn last_modify_time(&self, ptr: *const DLDateTime) -> *const DLDateTime {
        tracing::debug!("{self}::last_modify_time()");
        unimplemented!()
    }

    extern "C" fn file_size(&mut self) -> usize {
        let current = self.buffer.stream_position().unwrap();
        let end = self.buffer.seek(SeekFrom::End(0)).unwrap() as usize;
        self.buffer.seek(SeekFrom::Start(current));
        tracing::debug!("{self}::file_size() -> {end}");
        end
    }

    extern "C" fn get_read_size(&mut self) -> usize {
        tracing::debug!("{self}::get_read_size()");
        unimplemented!()
    }

    extern "C" fn get_write_size(&self) -> usize {
        tracing::debug!("{self}::get_write_size()");
        unimplemented!()
    }

    extern "C" fn set_eof(&mut self) {
        tracing::debug!("{self}::set_eof()");
        unimplemented!()
    }

    extern "C" fn is_eof(&self) -> bool {
        tracing::debug!("{self}::is_eof()");
        unimplemented!()
    }

    extern "C" fn is_directory(&self) -> bool {
        tracing::debug!("{self}::is_directory()");
        unimplemented!()
    }

    extern "C" fn is_open(&self) -> bool {
        tracing::debug!("{self}::is_open()");
        true
    }

    extern "C" fn open(&mut self, open_mode: OpenFileMode) -> bool {
        tracing::debug!("{self}::open({:?})", open_mode);
        true
    }

    extern "C" fn close(&mut self) -> bool {
        tracing::debug!("{self}::close()");
        true
    }

    extern "C" fn set_read_only(&mut self, is_open: bool) -> bool {
        tracing::debug!("{self}::set_read_only({})", is_open);
        unimplemented!()
    }

    extern "C" fn seek(
        &mut self,
        is_stream: bool,
        offset: i64,
        seek_mode: DLFileSeekDirection,
    ) -> bool {
        tracing::debug!("{self}::seek({}, {}, {:?})", is_stream, offset, seek_mode);
        unimplemented!()
    }

    extern "C" fn cursor_position(&self) -> usize {
        tracing::debug!("{self}::cursor_position()");
        unimplemented!()
    }

    unsafe extern "C" fn read(&mut self, output: *mut u8, length: usize) -> i32 {
        tracing::debug!("{self}::read({:x?}, {})", output, length);
        let mut buffer = vec![0x0u8; length];
        self.buffer.read_exact(&mut buffer).unwrap();

        unsafe { std::ptr::copy_nonoverlapping(buffer.as_ptr(), output, length) };
        self.base.result = DLIOResult::Success;
        length as i32
    }

    extern "C" fn write(&mut self, input: *const u8, length: usize) -> usize {
        tracing::debug!("{self}::write_file({:x?}, {})", input, length);
        unimplemented!()
    }

    extern "C" fn get_async_block_size(&self) -> usize {
        tracing::debug!("{self}::get_async_block_size()");
        unimplemented!()
    }
    extern "C" fn get_async_buffer_alignment_size(&self) -> usize {
        tracing::debug!("{self}::get_async_buffer_alignment_size()");
        unimplemented!()
    }

    unsafe extern "C" fn start_async_read(&mut self, output: *mut u8, length: usize) -> bool {
        tracing::debug!("{self}::start_async_read({:x?}, {})", output, length);
        unimplemented!()
    }
    extern "C" fn start_async_write(&mut self, input: *const u8, length: usize) -> bool {
        tracing::debug!("{self}::start_async_write({:x?}, {})", input, length);
        unimplemented!()
    }
    extern "C" fn query_async_status(
        &mut self,
        bytes_remaining: &mut usize,
        bytes_transferred: Option<&mut usize>,
    ) -> bool {
        tracing::debug!("{self}::query_async_status()");
        unimplemented!()
    }
    extern "C" fn get_open_mode(&self) -> OpenFileMode {
        tracing::debug!("{self}::get_open_mode()");
        unimplemented!()
    }

    extern "C" fn delete(&mut self) -> bool {
        tracing::debug!("{self}::delete()");
        unimplemented!()
    }

    extern "C" fn flush(&mut self) {
        tracing::debug!("{self}::flush()");
        unimplemented!()
    }

    extern "C" fn populate_file_info(&mut self) -> bool {
        tracing::debug!("{self}::populate_file_info()");
        unimplemented!()
    }

    extern "C" fn unk2(&mut self) -> bool {
        tracing::debug!("{self}::unk2()");
        unimplemented!()
    }

    extern "C" fn rename_w(&mut self, path: *const u16) -> bool {
        tracing::debug!("{self}::rename_w({:x?})", path);
        unimplemented!()
    }

    extern "C" fn rename(&mut self, path: *const u8) -> bool {
        tracing::debug!("{self}::rename({:x?})", path);
        unimplemented!()
    }

    extern "C" fn create_directory(&mut self) -> bool {
        tracing::debug!("{self}::create_directory()");
        unimplemented!()
    }
}
