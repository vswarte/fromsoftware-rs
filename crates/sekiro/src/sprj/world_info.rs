use std::{mem::MaybeUninit, ptr::NonNull, slice};

use bitfield::bitfield;

use shared::*;

#[repr(C)]
#[derive(Superclass)]
#[superclass(children(WorldRes))]
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

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub is_lock: bool,

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

    /// The currently initialized area infos and their corresponding block
    /// infos.
    pub fn area_and_block_info(&self) -> impl Iterator<Item = (&WorldAreaInfo, &[WorldBlockInfo])> {
        self.area_info().iter().map(|area| {
            // Safety: We know there isn't a mutable reference to the block
            // info because it's owned by this WorldInfo to which we have an
            // immutable reference.
            (area, unsafe {
                slice::from_raw_parts(area.block_info.as_ptr(), area.block_info_length as usize)
            })
        })
    }

    /// The mutable currently initialized area infos and their corresponding
    /// block infos.
    pub fn area_and_block_info_mut(
        &mut self,
    ) -> impl Iterator<Item = (&mut WorldAreaInfo, &mut [WorldBlockInfo])> {
        self.area_info_mut().iter_mut().map(|area| {
            // Safety: We know there aren't any other references to the block
            // info because it's owned by this WorldInfo to which we have a
            // mutable reference.
            let blocks = unsafe {
                slice::from_raw_parts_mut(area.block_info.as_mut(), area.block_info_length as usize)
            };
            (area, blocks)
        })
    }
}

#[repr(C)]
#[derive(Subclass)]
/// Source of name: RTTI
pub struct WorldRes {
    pub world_info: WorldInfo,

    /// The number of defined entries in [world_area_res](Self::world_area_res).
    ///
    /// Use [Self::area_res] to access this safely.
    pub world_area_res_len: u32,

    /// A pointer to the beginning of [world_area_res](Self::world_area_res).
    ///
    /// Use [Self::area_res] to access this safely.
    pub world_area_res_list_ptr: NonNull<WorldAreaRes>,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub remaining_time_to_activation: u32,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub time_between_activations: u32,

    /// The number of defined entries in
    /// [world_block_res](Self::world_block_res).
    ///
    /// Use [Self::block_res] to access this safely.
    pub world_block_res_len: u32,

    /// A pointer to the beginning of [world_block_res](Self::world_block_res).
    ///
    /// Use [Self::block_res] to access this safely.
    pub world_block_res_list_ptr: NonNull<WorldBlockRes>,

    _unk1ab8: u32,
    _unk1abc: u8,
    _unk1ac0: u64,
    _unk1ac8: u16,
    _unk1aca: u8,
    _unk1acb: u8,
    _unk1acc: u8,
    _unk1ad0: u32,
    _unk1ad4: [u8; 0xc],

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

// Source of name: RTTI
pub type WorldAreaRes = UnknownStruct<0x108>;

// Source of name: RTTI
pub type WorldBlockRes = UnknownStruct<0x458>;

#[repr(C)]
/// Source of name: RTTI
pub struct WorldAreaInfo {
    _vftable: usize,
    _unk08: u16,
    _unk0a: u8,

    /// The area's numeric identifier.
    ///
    /// This is corresponds to the `XX00000` digits in an event flag.
    pub area_number: u8,

    /// The [WorldInfo] instance that owns this area.
    pub owner: NonNull<WorldInfo>,

    /// The index of this area in [WorldInfo::world_area_info].
    pub world_area_index: u32,

    /// The index of this area's first block in [WorldInfo::world_block_info].
    pub world_block_index: u32,

    /// The length of the [block_info](Self::block_info) array.
    pub block_info_length: u32,

    /// The block infos for this [WorldAreaInfo].
    pub block_info: NonNull<WorldBlockInfo>,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub is_lock: bool,
}

bitfield! {
    /// An ID that contains information about the block's event locations.
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
    pub world_area_info: NonNull<WorldAreaInfo>,

    /// The index of this in [WorldInfo.world_block_info].
    pub world_block_index: u32,

    _unk24: u8,
    _unk25: u8,
    _unk26: u8,
    _unk27: u8,

    /// The index of the corresponding area in [WorldInfo::world_area_info].
    pub area_block_index: u32,

    _unk30: u64,
    _unk38: u64,
    _unk40: u64,
    _unk48: [u64; 0x6],
    _unk78: u64,
    _unk80: u64,
    _unk88: u64,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub is_lock: bool,

    _unk91: [u8; 3],

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub ceremony_id: u8,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub current_time_zone: i32,

    /// Seems to be unused.
    _ceremony_id: i32,

    /// Seems to be unused.
    pub debug_time_to_id_change: i32,

    _unka4: i32,
    _unka8: f32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x38, size_of::<WorldAreaInfo>());
        assert_eq!(0xb0, size_of::<WorldBlockInfo>());
        assert_eq!(0x1a90, size_of::<WorldInfo>());
        assert_eq!(0xba80, size_of::<WorldRes>());
    }
}
