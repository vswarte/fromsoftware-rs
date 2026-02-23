use std::{ffi::CStr, ptr::NonNull, slice};

use crate::fd4::FD4ResCapHolder;
use crate::param::ParamDef;
use shared::{OwnedPtr, Subclass};

use super::{FD4ResCap, FD4ResRep};

// Under most circumstances, it would make the most sense to consider
// [FD4ParamRepository] to be the owner of the [FD4ParamResCap]s it contains.
// However, it's substantially less efficient to look up a given parameter
// through [FD4ParamRepository] than it is through more specific repositories
// like [SoloParamRepository]. Other repositories assemble a mapping from
// parameter "indexes" to the actual locations of those parameters in memory
// which is robust against the parameters being reordered on disk;
// [FD4ParamRepository] does not. Because of this, it's possible to do an O(1)
// lookup through individual repositories while any lookup here must be O(n).
//
// To mitigate this, we expose APIs that act like [FD4ParamRepository] is *not*
// the owner of its data, in the sense that all access to that data is `unsafe`
// on the condition that users have no other mutable references (or immutable
// references in the case of mutable access) to param data elsewhere in their
// program. This allows us to avoid the `unsafe` qualifier for access through
// individual parameter repositories, because they're guaranteed to have the
// only references on the Rust side. (We're insulated from references on the C++
// side by the safety requirements for accessing any of these globals in the
// first place.)
//
// It's important to note as well that this solution relies on all the
// individual parameter repositories having access to a disjoint set of
// parameters. If any of them overlap (in another patch or another game),
// they'll have the same soundness issues between one another and we may need to
// consider a different approach to solving that problem.
#[repr(C)]
#[shared::singleton("FD4ParamRepository")]
#[derive(Subclass)]
#[subclass(base = FD4ResRep, base = FD4ResCap)]
pub struct FD4ParamRepository {
    /// Resource repository holding the actual param data.
    pub res_rep: FD4ResRep,
    res_cap_holder: FD4ResCapHolder<FD4ParamResCap>,
    allocator: usize,
}

impl FD4ParamRepository {
    /// Returns this repository's collection of [FD4ParamResCap]s.
    ///
    /// ## Safety
    ///
    /// This accesses data that `fromsoftware-rs` considers to be owned by
    /// individual parameter repositories such as [`SoloParamRepository`]. The
    /// caller must guarantee that there are no mutable references to *any*
    /// parameter data anywhere in the program before calling this.
    ///
    /// [`SoloParamRepository`]: crate::cs::SoloParamRepository
    pub unsafe fn res_cap_holder(&self) -> &FD4ResCapHolder<FD4ParamResCap> {
        &self.res_cap_holder
    }

    /// Returns a mutable reference to this repository's collection of
    /// [FD4ParamResCap]s.
    ///
    /// ## Safety
    ///
    /// This accesses data that `fromsoftware-rs` considers to be owned by
    /// individual parameter repositories such as [`SoloParamRepository`]. The
    /// caller must guarantee that there are no other references to *any* parameter
    /// data anywhere in the program before calling this.
    ///
    /// [`SoloParamRepository`]: crate::cs::SoloParamRepository
    pub unsafe fn res_cap_holder_mut(&mut self) -> &mut FD4ResCapHolder<FD4ParamResCap> {
        &mut self.res_cap_holder
    }

    /// Returns the first parameter in the repository whose struct type is `P`,
    /// or `None` if there is no such parameter. This should never return `None`
    /// for a vanilla game, because the only parameters this library defines are
    /// ones that are found in the game.
    ///
    /// ## Safety
    ///
    /// This accesses data that `fromsoftware-rs` considers to be owned by
    /// individual parameter repositories such as [`SoloParamRepository`]. The
    /// caller must guarantee that there are no mutable references to *any*
    /// parameter data anywhere in the program before calling this.
    ///
    /// [`SoloParamRepository`]: crate::cs::SoloParamRepository
    pub unsafe fn get<P: ParamDef>(&self, id: u32) -> Option<&P> {
        unsafe { self.get_rescap::<P>() }.and_then(|entry| unsafe { entry.get::<P>(id) })
    }

    /// Returns a mutable reference to the first parameter in the repository
    /// whose struct type is `P`, or `None` if there is no such parameter. This
    /// should never return `None` for a vanilla game, because the only
    /// parameters this library defines are ones that are found in the game.
    ///
    /// ## Safety
    ///
    /// This accesses data that `fromsoftware-rs` considers to be owned by
    /// individual parameter repositories such as [`SoloParamRepository`]. The
    /// caller must guarantee that there are no other references (mutable or
    /// immutable) to *any* parameter data anywhere in the program before
    /// calling this.
    ///
    /// [`SoloParamRepository`]: crate::cs::SoloParamRepository
    pub unsafe fn get_mut<P: ParamDef>(&mut self, id: u32) -> Option<&mut P> {
        unsafe { self.get_rescap_mut::<P>() }.and_then(|entry| unsafe { entry.get_mut::<P>(id) })
    }

    unsafe fn get_rescap<P: ParamDef>(&self) -> Option<&FD4ParamResCap> {
        self.res_cap_holder
            .entries()
            .find(|e| e.struct_name() == P::NAME)
    }

    unsafe fn get_rescap_mut<P: ParamDef>(&mut self) -> Option<&mut FD4ParamResCap> {
        self.res_cap_holder
            .entries_mut()
            .find(|e| e.struct_name() == P::NAME)
    }
}

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

    /// The number of rows in the parameter file.
    row_count: u32,

    _reserved: u64,
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

/// An in-memory representation of a file that contains all the data for a
/// single parameter type.
// Memory layout:
// ```text
// [ParamFileMetadata]            <- file_ptr-0x10
// [ParamFile]                    <- file_ptr ([FD4ParamResCap::file] points here)
// [RowDescriptor * row_count]
// [char...]                      <- struct name, if [ParamFile::has_offset_param_type] is true
// [aligned padding to 0x10]
// [ParamDef * row_count]         <- file_ptr + RowDescriptor::data_offset
// [RowLookupEntry * row_count]   <- lookup table for param indexes, sorted by param ID
// ```
#[repr(C)]
pub struct ParamFile {
    strings_offset: u32,
    short_data_offset: u16,
    _unk06: u16,
    paramdef_version: u16,
    row_count: u16,
    struct_name: ParamStructName,
    big_endian: u8,
    format_2d: u8,
    format_2e: u8,
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
        if self.has_offset_param_type() {
            self.read_offset_struct_name()
        } else {
            self.read_inline_struct_name()
        }
    }

    /// The revision of this paramdef struct type.
    pub const fn paramdef_version(&self) -> u16 {
        self.paramdef_version
    }

    /// The number of rows this file contains.
    pub const fn row_count(&self) -> usize {
        self.row_count as usize
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
    pub unsafe fn rows<'a, P: ParamDef + 'a>(&'a self) -> impl Iterator<Item = &'a P> + 'a {
        self.lookup_table()
            .iter()
            .map(|l| unsafe { self.get_row_by_index(l.index as usize).unwrap() })
    }

    /// Returns an iterator over each mutable row in this file, in parameter ID order.
    ///
    /// # Safety
    ///
    /// Type `P` must match the actual row data structure for this param file.
    pub unsafe fn rows_mut<'a, P: ParamDef + 'a>(
        &'a mut self,
    ) -> impl Iterator<Item = &'a mut P> + 'a {
        // We have to do this more manually to avoid having a reference to the
        // `lookup_table` slice coexisting with the mutable reference returned
        // by the iterator.
        struct Iter<'a, P: ParamDef + 'a> {
            file: NonNull<ParamFile>,
            ptr: *const RowLookupEntry,
            end: *const RowLookupEntry,
            _marker: std::marker::PhantomData<&'a mut P>,
        }

        impl<'a, P: ParamDef + 'a> Iterator for Iter<'a, P> {
            type Item = &'a mut P;

            fn next(&mut self) -> Option<Self::Item> {
                if self.ptr == self.end {
                    None
                } else {
                    unsafe {
                        let result = self
                            .file
                            .as_mut()
                            .get_row_by_index_mut(self.ptr.as_ref().unwrap().index as usize)
                            .unwrap();
                        self.ptr = self.ptr.add(1);
                        Some(result)
                    }
                }
            }
        }

        let range = self.lookup_table().as_ptr_range();
        Iter {
            file: NonNull::from_ref(self),
            ptr: range.start,
            end: range.end,
            _marker: std::marker::PhantomData,
        }
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
                self.row_count as usize,
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

    fn read_inline_struct_name(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.struct_name.inline.as_ptr() as *const i8)
                .to_str()
                .unwrap_or("")
        }
    }

    fn read_offset_struct_name(&self) -> &str {
        unsafe {
            let offset = self.struct_name.offset.value;
            if offset == 0 {
                return "";
            }
            CStr::from_ptr(self.offset::<i8>(offset as usize).as_ptr())
                .to_str()
                .unwrap_or("")
        }
    }

    /// Returns this param file's row descriptor list. This uses the same
    /// indices as the paramter data.
    const fn row_descriptors<T: Into<u64> + Copy>(&self) -> &[RowDescriptor<T>] {
        let offset = size_of::<ParamFile>() + if self.has_extended_header() { 0x10 } else { 0 };
        unsafe {
            slice::from_raw_parts(
                self.offset::<RowDescriptor<T>>(offset).as_ptr(),
                self.row_count as usize,
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

        if self.is_64_bit() {
            self.row_descriptors::<u64>()
                .get(row_index)
                .map(|d| d.data_offset())
        } else {
            self.row_descriptors::<u32>()
                .get(row_index)
                .map(|d| d.data_offset())
        }
    }

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
union ParamStructName {
    inline: [u8; 0x20],
    offset: ParamStructNameOffset,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ParamStructNameOffset {
    _pad: u32,
    value: u32,
    _reserved: [u32; 6],
}
