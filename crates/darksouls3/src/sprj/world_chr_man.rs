use std::{mem::MaybeUninit, ptr::NonNull, slice};

use shared::{OwnedPtr, Subclass, UnknownStruct, empty::*};

use super::{ChrIns, PlayerIns, ReplayGhostIns, WorldAreaInfo, WorldBlockChr, WorldInfoOwner};
use crate::CxxVec;

#[repr(C)]
#[shared::singleton("WorldChrMan")]
/// Source of name: RTTI
pub struct WorldChrMan {
    vtable: usize,
    pub world_info_owner: NonNull<WorldInfoOwner>,

    /// The number of defined [WorldAreaChr]s.
    ///
    /// Use [Self::area_chrs] to access these safely.
    pub world_area_chr_len: u32,

    /// A pointer to the beginning of [world_area_chr](Self::world_area_chr).
    ///
    /// Use [Self::area_chrs] to access these safely.
    pub world_area_chr_ptr: NonNull<WorldAreaChr>,

    /// The number of defined [WorldBlockChr]s. These aren't necessarily
    /// contiguous at the beginning of [world_block_chr](Self::world_block_chr).
    ///
    /// Use [Self::block_chrs] to access these safely.
    pub world_block_chr_count: u32,

    /// A pointer to the beginning of [world_block_chr](Self::world_block_chr).
    ///
    /// Use [Self::block_chrs] to access these safely.
    pub world_block_chr_ptr: NonNull<MaybeEmpty<WorldBlockChr>>,

    _unk30: u32,

    /// All human players.
    pub player_chr_set: ChrSet<PlayerIns>,

    /// Bloodstain and replay ghosts.
    pub ghost_chr_set: ChrSet<ReplayGhostIns>,

    /// Debug characters. This doesn't seem to be populated in normal gameplay.
    pub debug_chr_set: ChrSet<ChrIns>,

    /// The local player.
    pub main_player: Option<NonNull<PlayerIns>>,

    /// Another player. Maybe the owner of the host world during multiplayer?
    _unk88: Option<NonNull<PlayerIns>>,

    _unk90: u16,
    _unk92: [u8; 0xd],
    _unka0: u64,
    _unka8: u64,
    _unkb0: u64,
    _unkb8: u64,

    pub loaded_world_block_chr_count: i32,
    pub loaded_world_block_chr_ptr: [Option<NonNull<WorldBlockChr>>; 32],

    _unk1c8: u32,
    _unk1d0: [UnknownStruct<0x18>; 35],
    _unk518: [u8; 0x118],
    _unk630: OwnedPtr<UnknownStruct<0x67c8>>,
    _unk638: OwnedPtr<u8>,
    _unk640: OwnedPtr<u8>,
    _unk648: OwnedPtr<UnknownStruct<0x18>>,
    _chr_thread: usize,
    _unk658: u64,

    /// The pool of [WorldAreaChr]s.
    ///
    /// Use [Self::area_chrs] to access these safely.
    pub world_area_chr: [MaybeUninit<WorldAreaChr>; 20],

    /// The pool of [WorldBlockChr]s.
    ///
    /// Use [Self::block_chrs] to access these safely.
    pub world_block_chr: [MaybeEmpty<WorldBlockChr>; 32],

    _unk2fe0: u64,
    _unk2fe8: u64,
    _unk2ff0: i32,
    _unk2ff8: CxxVec<usize>,
    _debug_chr_creator: usize,
    _debug_chr_perf_checker: usize,
    _unk3028: u64,
    _unk3030: u64,
    _allocator: usize,
    _unk3040: u32,
    _unk3048: u32,
    _unk304c: u16,
    _unk3050: CxxVec<usize>,
    _unk3058: u64,
    _unk3060: u64,
    _unk3068: u64,
    _unk3088: [u64; 35],
    _unk31a0: u64,
    _update_tasks: [UnknownStruct<0x30>; 0xc],
    _unk33e8: u32,
    _void_tasks: [UnknownStruct<0x28>; 0xa],
}

impl WorldChrMan {
    /// A slice of defined [WorldAreaChr]s.
    pub fn area_chrs(&self) -> &[WorldAreaChr] {
        unsafe {
            slice::from_raw_parts(
                self.world_area_chr.as_ptr() as *const WorldAreaChr,
                self.world_area_chr_len as usize,
            )
        }
    }

    /// A slice of defined mutable [WorldAreaChr]s.
    pub fn area_chrs_mut(&mut self) -> &mut [WorldAreaChr] {
        unsafe {
            slice::from_raw_parts_mut(
                self.world_area_chr.as_ptr() as *mut WorldAreaChr,
                self.world_area_chr_len as usize,
            )
        }
    }

    /// An iterator over the non-empty [WorldBlockChr]s.
    pub fn block_chrs(&self) -> impl Iterator<Item = &WorldBlockChr> {
        self.world_block_chr.iter().non_empty()
    }

    /// A mutable iterator over the non-empty [WorldBlockChr]s.
    pub fn block_chrs_mut(&mut self) -> impl Iterator<Item = &mut WorldBlockChr> {
        self.world_block_chr.iter_mut().non_empty()
    }
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldAreaChr {
    _vftable: usize,
    pub world_area_info: NonNull<WorldAreaInfo>,
    _unk10: u32,
    pub world_block_chr: NonNull<WorldBlockChr>,
}

#[repr(C)]
/// Source of name: Copied from ER RTTI
pub struct ChrSet<T>
where
    T: Subclass<ChrIns>,
{
    /// The capacity of [entries](Self::entries). Not every entry within this
    /// capacity will be non-empty.
    pub capacity: u32,

    /// The contents of the set.
    pub entries: OwnedPtr<MaybeEmpty<ChrSetEntry<T>>>,

    _unk10: u32,
}

impl<T> ChrSet<T>
where
    T: Subclass<ChrIns>,
{
    /// Returns a slice over all the entries in this set, whether or not they're
    /// empty.
    pub fn entries(&self) -> &[MaybeEmpty<ChrSetEntry<T>>] {
        unsafe { slice::from_raw_parts(self.entries.as_ptr(), self.capacity as usize) }
    }

    /// Returns a mutable slice over all the entries in this set.
    pub fn entries_mut(&mut self) -> &mut [MaybeEmpty<ChrSetEntry<T>>] {
        unsafe { slice::from_raw_parts_mut(self.entries.as_ptr(), self.capacity as usize) }
    }

    /// Returns an iterator over all the `T`s in this set.
    pub fn iter(&self) -> impl Iterator<Item = &ChrSetEntry<T>> {
        self.entries().iter().non_empty()
    }

    /// Returns a mutable iterator over all the `T`s in this set.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ChrSetEntry<T>> {
        self.entries_mut().iter_mut().non_empty()
    }
}

#[repr(C)]
/// Source of name: Copied from ER RTTI
pub struct ChrSetEntry<T>
where
    T: Subclass<ChrIns>,
{
    /// The character this entry refers to.
    pub chr: OwnedPtr<T>,

    _unk08: u32,
    _unk10: u64,
    _special_effect: usize,
    _unk20: u64,
    _chr_physics_module: usize,
    _unk30: usize,
}

unsafe impl<T> IsEmpty for ChrSetEntry<T>
where
    T: Subclass<ChrIns>,
{
    fn is_empty(value: &MaybeEmpty<ChrSetEntry<T>>) -> bool {
        *unsafe { value.as_non_null().cast::<usize>().as_ref() } == 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x20, size_of::<WorldAreaChr>());
        assert_eq!(0x38, size_of::<ChrSetEntry<ChrIns>>());
        assert_eq!(0x18, size_of::<ChrSet<ChrIns>>());
        assert_eq!(0x3580, size_of::<WorldChrMan>());
    }
}
