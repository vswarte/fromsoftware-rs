use std::ptr::NonNull;

use windows::core::PCSTR;

use crate::param::ParamDef;
use shared::OwnedPtr;

use super::resource::FD4ResCap;
use super::FD4ResRep;

#[repr(C)]
#[dlrf::singleton("FD4ParamRepository")]
pub struct FD4ParamRepository {
    /// Resource repository holding the actual param data.
    pub res_rep: FD4ResRep<FD4ParamResCap>,
    allocator: usize,
}

impl FD4ParamRepository {
    pub fn get<T: ParamDef>(&self, id: u32) -> Option<&T> {
        let file_header = self
            .res_rep
            .res_cap_holder
            .entries()
            .find(|e| e.data.name().as_str().eq(T::NAME))?;

        // SAFETY: we shouldn't run into invalid casts because of the code gen dictating T::NAME.
        unsafe { file_header.data.get(id) }
    }

    pub fn get_mut<T: ParamDef>(&mut self, id: u32) -> Option<&mut T> {
        let file_header = self
            .res_rep
            .res_cap_holder
            .entries_mut()
            .find(|e| e.data.name().as_str().eq(T::NAME))?;

        // SAFETY: we shouldn't run into invalid casts because of the code gen dictating T::NAME.
        unsafe { file_header.data.get_mut(id) }
    }
}

#[repr(C)]
pub struct FD4ParamResCap {
    pub inner: FD4ResCap<Self>,
    /// Size of data at pointer.
    pub size: u64,
    /// Raw row data for this param file.
    pub data: OwnedPtr<ParamData>,
}

impl AsRef<FD4ResCap<Self>> for FD4ParamResCap {
    fn as_ref(&self) -> &FD4ResCap<Self> {
        &self.inner
    }
}

impl AsMut<FD4ResCap<Self>> for FD4ParamResCap {
    fn as_mut(&mut self) -> &mut FD4ResCap<Self> {
        &mut self.inner
    }
}

#[repr(C)]
pub struct ParamData {
    pub header: ParamFileHeader,
}

impl ParamData {
    pub fn name(&self) -> String {
        unsafe {
            PCSTR(
                (self as *const _ as usize + self.header.param_type.name_offset as usize)
                    as *const _,
            )
            .to_string()
            .unwrap()
        }
    }

    fn row_descriptors(&self) -> &[ParamRowDescriptor] {
        unsafe {
            // The row descriptors are right after the header.
            std::slice::from_raw_parts(
                (self as *const _ as usize + size_of::<ParamFileHeader>()) as *const _,
                self.header.row_count as usize,
            )
        }
    }

    /// Retrieve a param entry by its ID and type.
    ///
    /// # Safety
    /// Caller has to ensure that the param type passed to T is valid for the param file.
    pub unsafe fn get<T: Sized>(&self, id: u32) -> Option<&T> {
        let offset = self
            .row_descriptors()
            .iter()
            .find(|r| r.id == id)?
            .data_offset;

        unsafe { Some(&*((self as *const _ as usize + offset) as *const T)) }
    }

    /// Retrieve a param entry mutable by its ID and type.
    ///
    /// # Safety
    /// Caller has to ensure that the param type passed to T is valid for the param file.
    pub unsafe fn get_mut<T: Sized>(&mut self, id: u32) -> Option<&mut T> {
        let offset = self
            .row_descriptors()
            .iter()
            .find(|r| r.id == id)?
            .data_offset;

        unsafe { Some(&mut *((self as *const _ as usize + offset) as *mut T)) }
    }
}

#[repr(C)]
pub struct ParamFileHeader {
    strings_offset: u32,
    short_data_offset: u16,
    unk6: u16,
    pub paramdef_version: u16,
    pub row_count: u16,
    param_type: ParamTypeDescriptor,
    endianness: u8,
    flags_2d: u8,
    flags_2e: u8,
    pub paramdef_format_version: u8,
    unk30: [u8; 0x10],
}

#[repr(C)]
pub struct ParamTypeDescriptor {
    unk0: u32,
    name_offset: u32,
    /// String will be stored here if it fits.
    _pad8: [u8; 24],
}

#[repr(C)]
struct ParamRowDescriptor {
    id: u32,
    _pad4: u32,
    data_offset: usize,
    name_offset: usize,
}
