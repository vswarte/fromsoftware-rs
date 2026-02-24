use std::{ffi::CStr, iter, mem::ManuallyDrop, ops::Drop, ptr::NonNull, slice};

use bitfield::bitfield;

use super::FD4ResCap;
use crate::param::ParamDef;
use shared::{OwnedPtr, Subclass};

#[repr(C)]
#[derive(Subclass)]
pub struct FD4ParamResCap {
    pub res_cap: FD4ResCap,

    /// The size in bytes of the [ParamFile] pointed at by [Self::data].
    pub size: u64,

    /// The raw row data for this param resource.
    pub data: OwnedPtr<ParamFile>,
}

impl FD4ParamResCap {
    /// Returns the name of the struct that this parameter uses.
    ///
    /// This corresponds to [`ParamDef::NAME`] (typically written in all-caps
    /// snake case) rather than the parameter name (typically written in camel
    /// case).
    ///
    /// [`ParamDef::NAME`]: crate::param::ParamDef::NAME
    pub fn struct_name(&self) -> &str {
        self.data.struct_name()
    }

    /// Returns the row in this parameter with the given `id`, interpreted as a
    /// `P`.
    ///
    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn get<P: ParamDef>(&self, id: u32) -> Option<&P> {
        unsafe { self.data.get_row_by_id(id) }
    }

    /// Returns the mutable row in this parameter with the given `id`,
    /// interpreted as a `P`.
    ///
    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn get_mut<P: ParamDef>(&mut self, id: u32) -> Option<&mut P> {
        unsafe { self.data.get_row_by_id_mut(id) }
    }
}

/// Runtime metadata prepended at offset -0x10 from the param file.
#[repr(C)]
struct ParamFileMetadata {
    /// The unaligned offset from the beginning of the parameter file to the end
    /// of its name string (or its [RowDescriptor] array if the name is stored
    /// inline).
    after_name_offset: u32,

    _un08: u64,
}

/// An entry in the runtime lookup table to look up rows by ID.
///
/// This table is sorted by ID, enabling O(log n) lookups.
#[repr(C)]
struct RowLookupEntry {
    param_id: u32,
    index: u32,
}

/// A row descriptor that makes Row descriptor for param files.
///
/// The actual size depends on the offset type:
/// - 32-bit offsets: 12 bytes (id + data_offset + name_offset)
/// - 64-bit offsets: 24 bytes (id + pad + data_offset + name_offset)
#[repr(C)]
struct RowDescriptor<T: Into<u64> + Copy> {
    /// The parameter ID of the row this describes.
    id: u32,

    data_offset: T,

    /// The offset between the beginning of the [ParamFile] and its struct name.
    /// This is the same for all descriptors.
    name_offset: T,
}

impl<T: Into<u64> + Copy> RowDescriptor<T> {
    /// The offset between the beginning of the [ParamFile] and the [ParamDef]
    /// data this descriptor refers to.
    pub fn data_offset(&self) -> usize {
        self.data_offset.into() as usize
    }
}

/// The oldest parameter format Sekiro supports, used for only one of its
/// parameters that we know of. This stores the parameter name inline within the
/// file's data.
// Memory layout:
// ```text
// [ParamFileMetadata]            <- file_ptr-0x10
// [ParamFile]                    <- file_ptr ([FD4ParamResCap::file] points here)
// [RowDescriptor * row_count]
// [aligned padding to 0x10]
// [ParamDef * row_count]         <- file_ptr + RowDescriptor::data_offset
// [RowLookupEntry * row_count]   <- lookup table for param indexes, sorted by param ID
// ```
#[repr(C)]
struct ParamFileV2 {
    _unk00: [u8; 0x8],
    paramdef_version: u16,
    row_count: u16,

    /// The struct's name, stored inline.
    struct_name: [u8; 0x20],

    /// Flags indicating the structure of this file.
    flags: ParamFileFlags,
}

/// The newest parameter format Sekiro supports, used for almost all of its
/// parameters. This has some extra metadata and stores the parameter name
/// outside the struct header.
// Memory layout:
// ```text
// [ParamFileMetadata]            <- file_ptr-0x10
// [ParamFile]                    <- file_ptr ([FD4ParamResCap::file] points here)
// [RowDescriptor * row_count]
// [char...]                      <- struct name
// [aligned padding to 0x10]
// [ParamDef * row_count]         <- file_ptr + RowDescriptor::data_offset
// [RowLookupEntry * row_count]   <- lookup table for param indexes, sorted by param ID
// ```
#[repr(C)]
struct ParamFileV5 {
    _aligned_offset_after_param_data: u32,
    _unk04: u32,
    paramdef_version: u16,
    row_count: u16,
    _unk0c: u32,

    /// The offset in bytes between the beginning of this struct and the
    /// beginning of the [u8] array that contains its name.
    struct_name_offset: u32,

    _unk18: u64,
    _unk20: u64,
    _unk28: u32,

    /// Flags indicating the structure of this file.
    flags: ParamFileFlags,

    /// The offset in bytes between the beginning of this struct and the
    /// beginning of the first parameter struct's data.
    data_offset: u32,
}

/// An in-memory representation of a file that contains all the data for a
/// single parameter type.
// The discriminator for a param file is in [ParamFileFlags::file_version].
pub union ParamFile {
    v2: ManuallyDrop<ParamFileV2>,
    v5: ManuallyDrop<ParamFileV5>,
}

impl ParamFile {
    /// The alignment used for the beginning of the [RowLookupEntry] table.
    const LOOKUP_TABLE_ALIGNMENT: u32 = 0x10;

    /// Returns the name of the struct that this parameter uses.
    ///
    /// This corresponds to [`ParamDef::NAME`] (typically written in all-caps
    /// snake case) rather than the parameter name (typically written in camel
    /// case).
    ///
    /// [`ParamDef::NAME`]: crate::param::ParamDef::NAME
    pub fn struct_name(&self) -> &str {
        match self.as_enum() {
            ParamFileType::V2(file) => CStr::from_bytes_until_nul(&file.struct_name)
                .ok()
                .and_then(|s| s.to_str().ok())
                .unwrap_or(""),
            ParamFileType::V5(file) => {
                let offset = file.struct_name_offset;
                if offset == 0 {
                    ""
                } else {
                    unsafe {
                        CStr::from_ptr(self.offset::<i8>(offset as usize).as_ptr())
                            .to_str()
                            .unwrap_or("")
                    }
                }
            }
        }
    }

    /// The revision of this paramdef struct type.
    pub fn paramdef_version(&self) -> u16 {
        // This has the same positioning across both union types.
        unsafe { self.v5.paramdef_version }
    }

    /// The number of rows this file contains.
    pub fn row_count(&self) -> usize {
        // This has the same positioning across both union types.
        unsafe { self.v5.row_count as usize }
    }

    /// Returns the row in this file with the given `id`, if one exists.
    ///
    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn get_row_by_id<P: ParamDef>(&self, id: u32) -> Option<&P> {
        unsafe { self.get_row_by_index(self.find_index(id)?) }
    }

    /// Returns the mutable row in this file with the given `id`, if one exists.
    ///
    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn get_row_by_id_mut<P: ParamDef>(&mut self, id: u32) -> Option<&mut P> {
        unsafe { self.get_row_by_index_mut(self.find_index(id)?) }
    }

    /// Returns the row in this file at the given `index`, if one exists.
    ///
    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn get_row_by_index<P: ParamDef>(&self, row_index: usize) -> Option<&P> {
        let data_offset = self.row_data_offset(row_index)?;
        Some(unsafe { self.offset::<P>(data_offset).as_ref() })
    }

    /// Returns the mutable row in this file at the given `index`, if one exists.
    ///
    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn get_row_by_index_mut<P: ParamDef>(&mut self, row_index: usize) -> Option<&mut P> {
        let data_offset = self.row_data_offset(row_index)?;
        Some(unsafe { self.offset::<P>(data_offset).as_mut() })
    }

    /// Returns an iterator over each row in this file, in parameter ID order.
    ///
    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn rows<'a, P: ParamDef + 'a>(&'a self) -> impl Iterator<Item = (u32, &'a P)> + 'a {
        self.lookup_table()
            .iter()
            .map(|l| unsafe { (l.index, self.get_row_by_index(l.index as usize).unwrap()) })
    }

    /// Returns an iterator over each mutable row in this file, in parameter ID order.
    ///
    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn rows_mut<'a, P: ParamDef + 'a>(
        &'a mut self,
    ) -> impl Iterator<Item = (u32, &'a mut P)> + 'a {
        // We have to do this more manually to avoid having a reference to the
        // `lookup_table` slice coexisting with the mutable reference returned
        // by the iterator.
        let mut file = NonNull::from_ref(self);
        let range = self.lookup_table().as_ptr_range();
        let mut ptr = range.start;
        let end = range.end;

        iter::from_fn(move || {
            if ptr == end {
                return None;
            }

            // Safety: We know `ptr` is valid because the iterator holds a
            // reference to `self` and thus to its lookup table, and `ptr` can't
            // be `end` at this point. We know `file` is valid because of that
            // same reference.
            unsafe {
                let index = ptr.as_ref().unwrap().index;
                let result = file.as_mut().get_row_by_index_mut(index as usize).unwrap();
                ptr = ptr.add(1);
                Some((index, result))
            }
        })
    }

    /// Returns the index of the parameter row with the given `id`.
    pub fn find_index(&self, id: u32) -> Option<usize> {
        let table = self.lookup_table();
        let target_index = self
            .lookup_table()
            .binary_search_by(|entry| entry.param_id.cmp(&id))
            .ok()?;
        Some(table[target_index].index as usize)
    }

    /// Returns a reference to the lookup table used to efficiently map
    /// parameter IDs to indices.
    fn lookup_table(&self) -> &[RowLookupEntry] {
        let aligned_file_size =
            self.metadata()
                .after_name_offset
                .next_multiple_of(Self::LOOKUP_TABLE_ALIGNMENT) as usize;

        unsafe {
            slice::from_raw_parts(
                self.offset::<RowLookupEntry>(aligned_file_size).as_ptr(),
                self.row_count(),
            )
        }
    }

    /// Returns the metadata that's stored in memory before this file.
    const fn metadata(&self) -> &ParamFileMetadata {
        unsafe {
            let metadata_ptr = (self as *const Self).byte_sub(size_of::<ParamFileMetadata>())
                as *const ParamFileMetadata;
            &*metadata_ptr
        }
    }

    /// Returns a pointer to [offset] bytes after the beginning of this struct.
    ///
    /// ## Safety
    ///
    /// The `offset` must be in range of [isize] and the resulting addition must
    /// not overflow the address space.
    const unsafe fn offset<T>(&self, offset: usize) -> NonNull<T> {
        unsafe { NonNull::from_ref(self).cast::<u8>().add(offset).cast::<T>() }
    }

    /// Returns this param file's row descriptor list. This uses the same
    /// indices as the paramter data.
    fn row_descriptors<T: Into<u64> + Copy>(&self) -> &[RowDescriptor<T>] {
        let offset = size_of::<ParamFile>()
            + match self.as_enum() {
                ParamFileType::V2(_) => 0,
                ParamFileType::V5(_) => 0x10,
            };
        unsafe {
            slice::from_raw_parts(
                self.offset::<RowDescriptor<T>>(offset).as_ptr(),
                self.row_count(),
            )
        }
    }

    /// Returns the offset from the beginning of this paramter file to the data
    /// of the row at `row_index`.
    ///
    /// Returns `None` if `row_index` is out-of-range for this file.
    fn row_data_offset(&self, row_index: usize) -> Option<usize> {
        if row_index >= self.row_count() {
            return None;
        }

        match self.as_enum() {
            ParamFileType::V2(_) => self
                .row_descriptors::<u32>()
                .get(row_index)
                .map(|d| d.data_offset()),
            ParamFileType::V5(_) => {
                debug_assert!(
                    self.flags().offset_64() && self.flags().offset_64_v5(),
                    "Expected all v5 param files to use 64-bit RowDescriptor offsets",
                );

                self.row_descriptors::<u64>()
                    .get(row_index)
                    .map(|d| d.data_offset())
            }
        }
    }

    /// Reeturns a type-safe representation of this as an enum.
    ///
    /// Panics if this has a parameter type we don't recognize.
    fn as_enum(&self) -> ParamFileType<'_> {
        unsafe {
            match self.flags().file_version() {
                2 => ParamFileType::V2(&self.v2),
                5 => ParamFileType::V5(&self.v5),
                n => panic!("Unexpected ParamFile version {n}"),
            }
        }
    }

    /// The flags indicating details about how the file is structured.
    fn flags(&self) -> ParamFileFlags {
        // This has the same positioning across both union types.
        unsafe { self.v5.flags }
    }
}

impl Drop for ParamFile {
    fn drop(&mut self) {
        unsafe {
            match self.flags().file_version() {
                2 => ManuallyDrop::drop(&mut self.v2),
                5 => ManuallyDrop::drop(&mut self.v5),
                n => panic!("Unexpected ParamFile version {n}"),
            }
        }
    }
}

/// The type-safe enum equivalent of the  [ParamFile] union type.
enum ParamFileType<'a> {
    V2(&'a ParamFileV2),
    V5(&'a ParamFileV5),
}

bitfield! {
    /// Configuration that indicates the specific layout of the parameter file
    /// in memory.
    #[derive(Clone, Copy, PartialEq, Eq)]
    struct ParamFileFlags(u32);
    impl Debug;

    /// For files with [Self::file_version] 4 and 5, this indicates that the
    /// param file's [RowDescriptor]s uses [u64] offsets.
    ///
    /// For version 5, [offset_64_v5] must *also* be true for the descriptor to
    /// use [u64] offsets.
    ///
    /// In practice, this is always true.
    _, offset_64, _: 17;

    /// For files with [Self::file_version] 5, this **and** [offset_64] must
    /// both be true for the param file's [RowDescriptor]s to use [u64] offsets.
    ///
    /// In practice, this is always true.
    _, offset_64_v5, _: 15;

    /// The version of the parameter file format used for this parameter. Sekiro
    /// only supports versions between 2 and 5, inclusive.
    pub u8, file_version, _: 14, 8;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x38, size_of::<ParamFile>());
    }
}
