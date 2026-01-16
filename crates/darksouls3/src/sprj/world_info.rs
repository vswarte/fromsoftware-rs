use std::{mem::MaybeUninit, ptr::NonNull, slice};

use bitfield::bitfield;

use shared::UnknownStruct;

#[repr(C)]
/// Source of name: RTTI
pub struct WorldInfo {
    _vftable: usize,

    /// The number of defined entries in
    /// [world_area_info](Self::world_area_info).
    ///
    /// Use [Self::area_info] to access this safely.
    pub world_area_info_len: u32,

    /// A pointer to the beginning of [world_area_info](Self::world_area_info).
    ///
    /// Use [Self::area_info] to access this safely.
    pub world_area_info_list_ptr: NonNull<WorldAreaInfo>,

    /// The number of defined entries in
    /// [world_block_info](Self::world_block_info).
    ///
    /// Use [Self::block_info] to access this safely.
    pub world_block_info_len: u32,

    /// A pointer to the beginning of
    /// [world_block_info](Self::world_block_info).
    ///
    /// These are always an initial sublist of
    /// [world_block_info](Self::world_block_info).
    ///
    /// Use [Self::block_info] to access this safely.
    pub world_block_info_list_ptr: NonNull<WorldBlockInfo>,

    _unk28: u8,

    /// The pool of [WorldAreaInfo]s. Only the first
    /// [world_area_info_len](Self::world_area_info_len) are initialized.
    ///
    /// Use [Self::area_info] to access this safely.
    pub world_area_info: [MaybeUninit<WorldAreaInfo>; 0x14],

    /// The pool of [WorldBlockInfo]s. Only the first
    /// [world_block_info_len](Self::world_block_info_len) are initialized.
    ///
    /// Use [Self::block_info] to access this safely.
    pub world_block_info: [MaybeUninit<WorldBlockInfo>; 0x20],

    _unk1290: u64,
}

impl WorldInfo {
    /// The currently initialized area infos.
    pub fn area_info(&self) -> &[WorldAreaInfo] {
        unsafe {
            slice::from_raw_parts(
                self.world_area_info_list_ptr.as_ptr(),
                self.world_area_info_len as usize,
            )
        }
    }

    /// The mutable currently initialized area infos.
    pub fn area_info_mut(&mut self) -> &mut [WorldAreaInfo] {
        unsafe {
            slice::from_raw_parts_mut(
                self.world_area_info_list_ptr.as_mut(),
                self.world_area_info_len as usize,
            )
        }
    }

    /// The currently initialized block infos.
    pub fn block_info(&self) -> &[WorldBlockInfo] {
        unsafe {
            slice::from_raw_parts(
                self.world_block_info_list_ptr.as_ptr(),
                self.world_block_info_len as usize,
            )
        }
    }

    /// The mutable currently initialized block infos.
    pub fn block_info_mut(&mut self) -> &mut [WorldBlockInfo] {
        unsafe {
            slice::from_raw_parts_mut(
                self.world_block_info_list_ptr.as_mut(),
                self.world_block_info_len as usize,
            )
        }
    }
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldAreaInfo {
    _vftable: usize,
    _pad08: [u8; 3],

    /// The area's numeric identifier.
    ///
    /// This is corresponds to the `XX00000` digits in an event flag.
    pub area_number: u8,

    /// The [WorldInfo] instance that owns this area.
    pub owner: NonNull<WorldInfo>,

    _unk18: u32,
    _unk1c: u32,

    /// The length of the [block_info](Self::block_info) array.
    pub block_info_length: u32,

    /// The block infos for this [WorldAreaInfo].
    pub block_info: NonNull<WorldBlockInfo>,

    _unk30: u8,
}

impl WorldAreaInfo {
    /// The block infos for this [WorldAreaInfo].
    pub fn block_info(&self) -> &[WorldBlockInfo] {
        unsafe { slice::from_raw_parts(self.block_info.as_ptr(), self.block_info_length as usize) }
    }

    /// The mutable block infos for this [WorldAreaInfo].
    pub fn block_info_mut(&mut self) -> &mut [WorldBlockInfo] {
        unsafe {
            slice::from_raw_parts_mut(self.block_info.as_mut(), self.block_info_length as usize)
        }
    }
}

bitfield! {
    /// An ID that contains information about the block's event locations.
    #[derive(Copy, Clone)]
    pub struct BlockId(u32);
    impl Debug;

    /// The event group that this block belongs to.
    pub u8, group, _: 23, 16;

    /// The area number that this block belongs to.
    pub u8, area, _: 31, 24;
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldBlockInfo {
    _vftable: usize,

    /// The block ID that indicates which event flags this block refers to.
    pub block_id: BlockId,

    /// The [WorldInfo] instance that owns this area.
    pub owner: NonNull<WorldInfo>,

    /// The [WorldAreaInfo] that contains this block.
    pub world_area_info: Option<NonNull<WorldAreaInfo>>,

    /// The index of this in [WorldInfo.world_block_info].
    ///
    /// This is also used as the index of this block's events in
    /// [EventWorld.blocks].
    pub world_block_index: u32,

    _unk24: u32,
    _msb_res_cap: usize,
    _btab_file_cap: usize,
    _btl_file_cap: usize,
    _btpb_file_cap: usize,
    _breakobj_file_cap: usize,
    _pre_map_decal_file_cap: usize,
    _unk58: usize,
    _unk60: u8,
    _pad61: [u8; 3],
    _unk64: u8,
    _unk68: u32,
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldInfoOwner {
    pub super_world_info: WorldInfo,
    _unk8: u64,

    /// The number of defined entries in [world_area_res](Self::world_area_res).
    ///
    /// Use [Self::area_res] to access this safely.
    pub world_area_res_len: u32,

    /// A pointer to the beginning of [world_area_res](Self::world_area_res).
    ///
    /// Use [Self::area_res] to access this safely.
    pub world_area_res_list_ptr: NonNull<WorldAreaRes>,

    _unk12b0: u32,
    _unk12b4: u32,

    /// The number of defined entries in
    /// [world_block_res](Self::world_block_res).
    ///
    /// Use [Self::block_res] to access this safely.
    pub world_block_res_len: u32,

    /// A pointer to the beginning of [world_block_res](Self::world_block_res).
    ///
    /// Use [Self::block_res] to access this safely.
    pub world_block_res_list_ptr: NonNull<WorldBlockRes>,

    _unk12c8: u64,
    _unk12d0: u64,
    _unk12d8: u64,

    /// The pool of [WorldAreaRes]es. Only the first
    /// [world_area_res_len](Self::world_area_res_len) are initialized.
    ///
    /// Use [Self::area_res] to access this safely.
    pub world_area_res: [MaybeUninit<WorldAreaRes>; 0x14],

    /// The pool of [WorldBlockRes]es. Only the first
    /// [world_block_res_len](Self::world_block_res_len) are initialized.
    ///
    /// Use [Self::block_res] to access this safely.
    pub world_block_res: [MaybeUninit<WorldBlockRes>; 0x20],

    _unkae80: u64,
    _unkae88: u64,
}

impl WorldRes {
    pub fn area_res(&self) -> &[WorldAreaRes] {
        unsafe {
            slice::from_raw_parts(
                self.world_area_res_list_ptr.as_ptr(),
                self.world_area_res_len as usize,
            )
        }
    }

    pub fn area_res_mut(&mut self) -> &mut [WorldAreaRes] {
        unsafe {
            slice::from_raw_parts_mut(
                self.world_area_res_list_ptr.as_mut(),
                self.world_area_res_len as usize,
            )
        }
    }

    pub fn block_res(&self) -> &[WorldBlockRes] {
        unsafe {
            slice::from_raw_parts(
                self.world_block_res_list_ptr.as_ptr(),
                self.world_block_res_len as usize,
            )
        }
    }

    pub fn block_res_mut(&mut self) -> &mut [WorldBlockRes] {
        unsafe {
            slice::from_raw_parts_mut(
                self.world_block_res_list_ptr.as_mut(),
                self.world_block_res_len as usize,
            )
        }
    }
}

// WorldRes doesn't add any additional fields.
pub type WorldRes = WorldInfoOwner;

// Source of name: RTTI
pub type WorldAreaRes = UnknownStruct<0x108>;

// Source of name: RTTI
pub type WorldBlockRes = UnknownStruct<0x438>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x38, size_of::<WorldAreaInfo>());
        assert_eq!(0x70, size_of::<WorldBlockInfo>());
        assert_eq!(0x1298, size_of::<WorldInfo>());
        assert_eq!(0xae90, size_of::<WorldInfoOwner>());
    }
}
