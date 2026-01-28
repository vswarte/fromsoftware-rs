use std::ffi::CStr;

use crate::fd4::FD4ResCapHolder;
use crate::param::ParamDef;
use shared::{OwnedPtr, Subclass};

use super::FD4ResRep;
use super::resource::FD4ResCap;

use param_layout::{ParamLayout, ParamLayout32, ParamLayout64};

#[repr(C)]
#[shared::singleton("FD4ParamRepository")]
#[derive(Subclass)]
#[subclass(base = FD4ResRep, base = FD4ResCap)]
pub struct FD4ParamRepository {
    /// Resource repository holding the actual param data.
    pub res_rep: FD4ResRep,
    pub res_cap_holder: FD4ResCapHolder<FD4ParamResCap>,
    allocator: usize,
}

impl FD4ParamRepository {
    pub fn get<P: ParamDef>(&self, id: u32) -> Option<&P> {
        self.get_rescap::<P>()
            .and_then(|entry| unsafe { entry.get::<P>(id) })
    }

    pub fn get_mut<P: ParamDef>(&mut self, id: u32) -> Option<&mut P> {
        self.get_rescap_mut::<P>()
            .and_then(|entry| unsafe { entry.get_mut::<P>(id) })
    }

    fn get_rescap<P: ParamDef>(&self) -> Option<&FD4ParamResCap> {
        self.res_cap_holder
            .entries()
            .find(|e| e.param_name() == P::NAME)
    }

    fn get_rescap_mut<P: ParamDef>(&mut self) -> Option<&mut FD4ParamResCap> {
        self.res_cap_holder
            .entries_mut()
            .find(|e| e.param_name() == P::NAME)
    }
}

#[repr(C)]
#[derive(Subclass)]
pub struct FD4ParamResCap {
    inner: FD4ResCap,
    /// Size of data at pointer.
    size: u64,
    /// Raw row data for this param file.
    pub data: OwnedPtr<ParamFile>,
}

impl FD4ParamResCap {
    pub fn param_name(&self) -> &str {
        self.data.param_name()
    }

    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn get<P: ParamDef>(&self, id: u32) -> Option<&P> {
        unsafe { self.data.get_row_by_id(id) }
    }

    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn get_mut<P: ParamDef>(&mut self, id: u32) -> Option<&mut P> {
        unsafe { self.data.get_row_by_id_mut(id) }
    }
}

/// Runtime metadata prepended at offset -0x10 from the param file.
///
/// Memory layout:
/// ```text
/// [ParamHeaderMetadata]          <- file_ptr-0x10
/// [ParamFile]                    <- file_ptr (FD4ParamResCap.file points here)
/// [row data...]
/// [aligned padding to 0x10]
/// [RowLookupEntry * row_count]   <- sorted by param ID lookup table
/// ```
#[repr(C)]
pub struct ParamHeaderMetadata {
    file_size: u32,
    row_count: u32,
    _reserved: u64,
}

impl ParamHeaderMetadata {
    const ALIGNMENT: u32 = 0x10;
    const SIZE: usize = size_of::<Self>();

    fn lookup_table(&self) -> &[RowLookupEntry] {
        let aligned_file_size = self.file_size.next_multiple_of(Self::ALIGNMENT) as usize;

        unsafe {
            let file_start = (self as *const Self).add(1) as *const u8;
            std::slice::from_raw_parts(
                file_start.add(aligned_file_size) as *const RowLookupEntry,
                self.row_count as usize,
            )
        }
    }

    pub fn find_index(&self, param_id: u32) -> Option<usize> {
        let table = self.lookup_table();
        let target_index = self
            .lookup_table()
            .binary_search_by(|entry| entry.param_id.cmp(&param_id))
            .ok()?;
        Some(table[target_index].index as usize)
    }
}

/// Entry in the runtime lookup table for O(log n) access by row ID.
#[repr(C)]
struct RowLookupEntry {
    param_id: u32,
    index: u32,
}

/// Row descriptor for param files.
///
/// The actual size depends on the offset type:
/// - 32-bit offsets: 12 bytes (id + data_offset + name_offset)
/// - 64-bit offsets: 24 bytes (id + pad + data_offset + name_offset)
#[repr(C)]
pub struct RowDescriptor<O: ParamLayout> {
    pub id: u32,
    pub data_offset: O::FileOffsetType,
    pub name_offset: O::FileOffsetType,
}

impl<O: ParamLayout> RowDescriptor<O> {
    pub fn data_offset(&self) -> usize {
        self.data_offset.into() as usize
    }
}

/// Param file accessor that handles format variations at runtime.
#[repr(C)]
pub struct ParamFile {
    header: ParamHeader,
}

impl ParamFile {
    pub fn param_name(&self) -> &str {
        if self.header.has_offset_param_type() {
            self.read_offset_param_name()
        } else {
            self.read_inline_param_name()
        }
    }

    pub const fn row_count(&self) -> usize {
        self.header.row_count as usize
    }

    pub const fn paramdef_version(&self) -> u16 {
        self.header.paramdef_data_version
    }

    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn get_row_by_id<P: ParamDef>(&self, id: u32) -> Option<&P> {
        let row_index = self.metadata().find_index(id)?;
        let data_offset = self.row_data_offset(row_index)?;
        Some(unsafe { &*(self.as_ptr().add(data_offset) as *const P) })
    }

    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn get_row_by_id_mut<P: ParamDef>(&mut self, id: u32) -> Option<&mut P> {
        let row_index = self.metadata().find_index(id)?;
        let data_offset = self.row_data_offset(row_index)?;
        Some(unsafe { &mut *(self.as_ptr().add(data_offset) as *mut P) })
    }

    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn get_by_row_index<P: ParamDef>(&self, row_index: usize) -> Option<&P> {
        let data_offset = self.row_data_offset(row_index)?;
        Some(unsafe { &*(self.as_ptr().add(data_offset) as *const P) })
    }

    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn get_by_row_index_mut<P: ParamDef>(&mut self, row_index: usize) -> Option<&mut P> {
        let data_offset = self.row_data_offset(row_index)?;
        Some(unsafe { &mut *(self.as_ptr().add(data_offset) as *mut P) })
    }

    pub const fn metadata(&self) -> &ParamHeaderMetadata {
        unsafe {
            let metadata_ptr = (self as *const Self).byte_sub(ParamHeaderMetadata::SIZE)
                as *const ParamHeaderMetadata;
            &*metadata_ptr
        }
    }

    fn as_ptr(&self) -> *const u8 {
        self as *const Self as *const u8
    }

    fn read_inline_param_name(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.header.param_name.inline.as_ptr() as *const i8)
                .to_str()
                .unwrap_or("")
        }
    }

    fn read_offset_param_name(&self) -> &str {
        unsafe {
            let offset = self.header.param_name.offset.value;
            if offset == 0 {
                return "";
            }
            CStr::from_ptr(self.as_ptr().add(offset as usize) as *const i8)
                .to_str()
                .unwrap_or("")
        }
    }

    /// Extended header offset is 0x40 bytes, otherwise 0x30
    const fn row_descriptors_offset(&self) -> usize {
        const HEADER_SIZE: usize = size_of::<ParamHeader>();
        if self.header.has_extended_header() {
            return HEADER_SIZE + 0x10;
        };
        HEADER_SIZE
    }

    fn row_data_offset(&self, row_index: usize) -> Option<usize> {
        if row_index >= self.row_count() {
            return None;
        }
        let base = self.row_descriptors_offset();

        unsafe {
            if self.header.is_64_bit() {
                let desc = &*(self
                    .as_ptr()
                    .add(base + row_index * size_of::<RowDescriptor<ParamLayout64>>())
                    as *const RowDescriptor<ParamLayout64>);
                Some(desc.data_offset())
            } else {
                let desc = &*(self
                    .as_ptr()
                    .add(base + row_index * size_of::<RowDescriptor<ParamLayout32>>())
                    as *const RowDescriptor<ParamLayout32>);
                Some(desc.data_offset())
            }
        }
    }
}

#[repr(C)]
struct ParamHeader {
    strings_offset: u32,
    short_data_offset: u16,
    unk_06: u16,
    paramdef_data_version: u16,
    row_count: u16,
    param_name: ParamTypeName,
    big_endian: u8,
    format_2d: u8,
    format_2e: u8,
}

impl ParamHeader {
    /// Whether param type is stored as an offset (bit 7 of format_2d).
    const fn has_offset_param_type(&self) -> bool {
        (self.format_2d & 0x80) != 0
    }

    /// Whether the file uses 64-bit offsets (bit 2 of format_2d).
    const fn is_64_bit(&self) -> bool {
        (self.format_2d & 0x04) != 0
    }

    /// Whether the header has the extended 16-byte section.
    const fn has_extended_header(&self) -> bool {
        self.is_64_bit() || ((self.format_2d & 0x01) != 0 && (self.format_2d & 0x02) != 0)
    }
}

#[repr(C)]
union ParamTypeName {
    inline: [u8; 0x20],
    offset: ParamTypeNameOffset,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ParamTypeNameOffset {
    _pad: u32,
    value: u32,
    _reserved: [u32; 6],
}

/// Traits for compile-time encoding of param file layout variations.
mod param_layout {
    use std::mem::size_of;

    /// Trait for offset size variations in param files.
    ///
    /// # Safety
    ///
    /// Implementors must assure that the associated `FileOffsetType` correctly
    /// represents param file row descriptor offset sizes.
    pub unsafe trait ParamLayout: Copy {
        type FileOffsetType: Into<u64> + Copy;

        fn is_64_bit() -> bool {
            size_of::<Self::FileOffsetType>() == 8
        }
    }

    /// Marker for 32-bit offsets (12-byte row descriptors).
    #[derive(Clone, Copy)]
    pub struct ParamLayout32;

    unsafe impl ParamLayout for ParamLayout32 {
        type FileOffsetType = u32;
    }

    /// Marker for 64-bit offsets (24-byte row descriptors).
    #[derive(Clone, Copy)]
    pub struct ParamLayout64;

    unsafe impl ParamLayout for ParamLayout64 {
        type FileOffsetType = u64;
    }
}
