use std::ffi;
use std::fmt::Display;
use std::marker::PhantomData;
use std::mem::transmute;
use std::ptr::NonNull;

use vtable_rs::VPtr;

use crate::Tree;
use crate::{cs::ChrIns, Vector};
use shared::{F32Matrix4x4, F32Vector4, OwnedPtr};

use super::{BlockId, ChrCam, FieldInsHandle, NetChrSync, PlayerIns};

#[repr(C)]
/// Source of name: RTTI
#[dlrf::singleton("WorldChrMan")]
pub struct WorldChrMan {
    vftable: usize,
    unk8: usize,
    pub world_area_chr: [WorldAreaChr<ChrIns>; 28],
    pub world_block_chr: [WorldBlockChr<ChrIns>; 192],
    pub world_grid_area_chr: [WorldGridAreaChr; 6],
    pub world_area_info_owner: usize,

    pub world_area_chr_list_count: u32,
    unk10d9c: u32,
    pub world_area_chr_ptr: usize,

    pub world_block_chr_list_count: u32,
    unk10dac: u32,
    pub world_block_chr_ptr: usize,

    pub world_grid_area_chr_list_count: u32,
    unk10dbc: u32,
    pub world_grid_area_chr_ptr: usize,

    pub world_area_list: [OwnedPtr<WorldAreaChrBase>; 34],
    pub world_area_list_count: u32,
    unk10edc: u32,

    /// ChrSet holding the players.
    pub player_chr_set: ChrSet<PlayerIns>,
    /// ChrSet holding bloodmessage and bloodstain ghosts as well as replay ghosts.
    pub ghost_chr_set: ChrSet<ChrIns>,
    /// ChrSet holding spirit ashes as well as Torrent.
    pub summon_buddy_chr_set: ChrSet<ChrIns>,
    /// ChrSet holding debug characters.
    pub debug_chr_set: ChrSet<ChrIns>,
    /// ChrSet holding the map-based characters.
    pub open_field_chr_set: OpenFieldChrSet,
    /// Amount of ChrSets in the chr_set_holder array.
    pub chr_set_holder_count: u32,
    /// Array of ChrSet holders.
    pub chr_set_holders: [ChrSetHolder<ChrIns>; 196],
    pub null_chr_set_holder: ChrSetHolder<ChrIns>,
    pub chr_sets: [Option<OwnedPtr<ChrSet<ChrIns>>>; 196],
    pub null_chr_set: Option<OwnedPtr<ChrSet<ChrIns>>>,
    pub player_grid_area: Option<NonNull<WorldGridAreaChr>>,
    /// Points to the local player.
    pub main_player: Option<OwnedPtr<PlayerIns>>,
    unk_player: Option<OwnedPtr<PlayerIns>>,

    unk_block_id_1: BlockId,
    unk_block_id_2: BlockId,

    unk1e520: [u8; 0x18],
    /// Manages spirit summons (excluding Torrent).
    pub summon_buddy_manager: Option<OwnedPtr<SummonBuddyManager>>,
    unk1e540: usize,
    unk1e548: usize,
    unk1e550: usize,
    unk1e558: u32,
    unk1e55c: f32,
    unk1e560: [u8; 0x80],
    pub net_chr_sync: OwnedPtr<NetChrSync>,
    unk1e5e8: usize,
    unk1e5f0: usize,
    unk1e5f8: usize,
    unk1e600: usize,
    unk1e608: [u8; 0x40],
    pub debug_chr_creator: OwnedPtr<CSDebugChrCreator>,
    unk1e650: usize,
    unk1e658: usize,
    unk1e660: usize,
    unk1e668: usize,
    unk1e670: [u8; 0x18],
    unk1e688: usize,
    unk1e690: usize,
    unk1e698: usize,
    unk1e6a0: usize,
    unk1e6a8: usize,
    unk1e6b0: usize,
    unk1e6b8: [u8; 0x628],
    pub chr_cam: Option<NonNull<ChrCam>>,
    // tasks and other stuff
    // list of players by distance may be useful
    // see ghidra structure for reference
    unk1ece8: [u8; 0x4e8],
    /// A list of ChrIns references sorted by distance to the main player.
    pub chr_inses_by_distance: Vector<ChrInsDistanceEntry>,
    unk1f1f0: [u8; 0x1f0],
}

#[repr(C)]
pub struct ChrInsDistanceEntry {
    pub chr_ins: NonNull<ChrIns>,
    pub distance: f32,
    _unkc: u32,
}

#[repr(C)]
pub struct CSDebugChrCreator {
    vftable: usize,
    stepper_fns: usize,
    unk10: usize,
    unk18_tree: Tree<()>,
    unk30: [u8; 0x14],
    pub spawn: bool,
    unk45: [u8; 0x3],
    unk48: [u8; 0x68],
    pub init_data: CSDebugChrCreatorInitData,
    pub last_created_chr: Option<NonNull<ChrIns>>,
    unk1b8: usize,
}

#[repr(C)]
pub struct CSDebugChrCreatorInitData {
    pub spawn_position: F32Vector4,
    spawn_rotation: F32Vector4,
    unk20: F32Vector4,
    spawn_scale: F32Vector4,
    pub npc_param_id: i32,
    pub npc_think_param_id: i32,
    pub event_entity_id: i32,
    pub talk_id: i32,
    pub name: [u16; 0x20],
    unk90: usize,
    name_pointer: usize,
    unka0: usize,
    unka8: usize,
    name_capacity: usize,
    unkb8: usize,
    unkc0: usize,
    enemy_type: u8,
    hamari_simulate: bool,
    unkca: [u8; 0x2],
    pub chara_init_param_id: i32,
    spawn_manipulator_type: u32,
    unkd4: [u8; 0x18],
    spawn_count: u32,
    unkf0: [u8; 0x10],
}

#[repr(C)]
pub struct ChrSetHolder<T: 'static> {
    pub chr_set: NonNull<ChrSet<T>>,
    pub chr_set_index: u32,
    _padc: u32,
    pub world_block_chr: NonNull<WorldBlockChr<T>>,
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldAreaChr<T: 'static> {
    pub base: WorldAreaChrBase,
    pub world_area_info: usize,
    unk18: u32,
    unk1c: u32,
    pub world_block_chr: NonNull<WorldBlockChr<T>>,
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldAreaChrBase {
    vftable: usize,
    pub world_area_info: usize,
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldBlockChr<T: 'static> {
    vftable: usize,
    pub world_block_info1: usize,
    unk10: [u8; 0x68],
    pub chr_set: ChrSet<T>,
    unkd0: [u8; 0x40],
    pub world_block_info2: usize,
    pub chr_set_ptr: NonNull<ChrSet<T>>,
    allocator: usize,
    unk128: [u8; 0x30],
    pub block_id: BlockId,
    unk15c: u32,
}

#[vtable_rs::vtable]
trait ChrSetVmt {
    /// Gets the max amount of ChrInses this ChrSet can hold.
    fn get_capacity(&self) -> u32;

    /// Wrapped version of get_chr_ins_by_index which also validates the
    /// index against the ChrSet capacity.
    fn safe_get_chr_ins_by_index(&mut self, index: u32) -> Option<&mut ChrIns>;

    /// Retrieves a ChrIns from the ChrSet by its index. Avoid using this.
    /// Prefer using safe_get_chr_ins_by_index.
    fn get_chr_ins_by_index(&mut self, index: u32) -> Option<&mut ChrIns>;

    /// Retrieves a ChrIns from the ChrSet by its FieldIns handle.
    fn get_chr_ins_by_handle(&mut self, handle: FieldInsHandle) -> Option<&mut ChrIns>;

    /// Wrapped version of get_chr_ins_by_index which also validates the
    /// index against the ChrSet capacity.
    fn safe_get_chr_set_entry_by_index(&mut self, index: u32) -> Option<&mut ChrSetEntry<ChrIns>>;

    /// Retrieves a ChrSetEntry from the ChrSet by its index. Avoid using this.
    /// Prefer using safe_get_chr_ins_by_index.
    fn get_chr_set_entry_by_index(&mut self, index: u32) -> Option<&mut ChrSetEntry<ChrIns>>;

    /// Retrieves a ChrSetEntry from the ChrSet by its index. Avoid using this.
    /// Prefer using safe_get_chr_ins_by_index.
    fn get_chr_set_entry_by_handle(
        &mut self,
        handle: FieldInsHandle,
    ) -> Option<&mut ChrSetEntry<ChrIns>>;

    /// Retrieves a ChrSetEntry from the ChrSet by its index. Avoid using this.
    /// Prefer using safe_get_chr_ins_by_index.
    fn get_index_by_handle(&self, handle: FieldInsHandle) -> u32;

    /// Deallocates all ChrInses hosted by the ChrSet.
    fn free_chr_list(&mut self);

    fn unk48(&mut self);

    fn unk50(&mut self);

    fn unk58(&mut self, param_2: usize);

    fn unk60(&mut self, param_2: usize);

    fn unk68(&mut self, param_2: usize, param_3: usize, param_4: u8, param_5: u8);
}

#[repr(C)]
/// Source of name: RTTI
pub struct ChrSet<T: 'static> {
    vftable: VPtr<dyn ChrSetVmt, Self>,
    pub index: i32,
    unkc: i32,
    /// Max amount of ChrInses that can fit inside of the ChrSet.
    pub capacity: u32,
    _pad14: u32,
    /// Entries managed by this ChrSet.
    pub entries: NonNull<ChrSetEntry<T>>,
    unk20: i32,
    _pad24: u32,
    /// Maps ChrSetEntry's to their event entity IDs.
    pub entity_id_mapping: Tree<ChrSetEntityIdMapping<T>>,
    /// Maps ChrSetEntry's to a group.
    pub group_id_mapping: Tree<ChrSetGroupMapping<T>>,
}

#[repr(C)]
pub struct ChrSetEntityIdMapping<T> {
    pub entity_id: u32,
    _pad4: u32,
    pub chr_set_entry: NonNull<ChrSetEntry<T>>,
}

#[repr(C)]
pub struct ChrSetGroupMapping<T> {
    pub group_id: u32,
    _pad4: u32,
    pub chr_set_entry: NonNull<ChrSetEntry<T>>,
}

impl<T> ChrSet<T> {
    pub fn get_capacity(&self) -> u32 {
        (self.vftable.get_capacity)(self)
    }

    pub fn chr_ins_by_handle(&mut self, field_ins_handle: &FieldInsHandle) -> Option<&mut ChrIns> {
        (self.vftable.get_chr_ins_by_handle)(self, field_ins_handle.to_owned())
    }
}

impl<T> ChrSet<T> {
    pub fn characters(&self) -> impl Iterator<Item = &mut T> {
        let mut current = self.entries;
        let end = unsafe { current.add(self.capacity as usize) };

        std::iter::from_fn(move || {
            while current != end {
                let mut chr_ins = unsafe { current.as_mut().chr_ins };
                current = unsafe { current.add(1) };
                let Some(mut chr_ins) = chr_ins else {
                    continue;
                };

                return Some(unsafe { chr_ins.as_mut() });
            }

            None
        })
    }
}

#[repr(C)]
pub struct ChrSetEntry<T> {
    pub chr_ins: Option<NonNull<T>>,
    unk8: u16,
    pub entry_flags: u8,
    _padb: [u8; 5],
}

#[repr(C)]
/// Source of name: RTTI
pub struct OpenFieldChrSet {
    pub base: ChrSet<ChrIns>,
    // TODO: type needs fact-checking
    unk58: Tree<()>,
    unk70: f32,
    pad74: u32,
    list1: [OpenFieldChrSetList1Entry; 1500],
    unk5e38: u32,
    unk5e3c: u32,
    unk5e40: u32,
    unk5e44: u32,
    list2: [OpenFieldChrSetList2Entry; 1500],
    unkbc08: u64,
    unkbc10: u64,
}

#[repr(C)]
pub struct OpenFieldChrSetList1Entry {
    unk0: u64,
    pub chr_ins: NonNull<ChrIns>,
}

#[repr(C)]
pub struct OpenFieldChrSetList2Entry {
    unk0: u64,
    unk8: u32,
    unkc: u32,
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldGridAreaChr {
    pub base: WorldAreaChrBase,
    pub world_grid_area_info: usize,
    unk_tree: Tree<()>,
}

#[repr(C)]
/// Source of name: "SummonBuddy" mentioned in DLRF metadata for the update fn.
pub struct SummonBuddyManager {
    allocator: usize,
    unk8: usize,
    unk10: usize,
    unk18: usize,
    pub to_spawn_buddy_param: i32,
    pub spawned_buddy_param: i32,
    unk28: usize,
    pub chr_set: NonNull<ChrSet<ChrIns>>,
    unk38: u32,
    unk3c: u32,
    unk40: usize,
    unk48: usize,
    unk50: usize,
    unk58: usize,
    unk60: usize,
    unk68: usize,
    pub summon_groups: Tree<()>,
    unk88: i32,
    pub buddy_disappear_delay_sec: f32,
    unk90: f32,
    unk94: u32,
    unk98: u32,
    unk9c: u32,
    unka0: F32Vector4,
    unkb0: f32,
    unkb4: u32,
    unkb8: u32,
    pub next_buddy_slot: u32,
    // unk38: [u8; 0xb0],
    // pub warp: OwnedPtr<SummonBuddyWarpManager>,
    // TODO: some debug structures after this
}

#[repr(C)]
pub struct SummonBuddyWarpManager {
    allocator: usize,
    root_node: usize,
    unk10: usize,
    pub trigger_time_ray_block: f32,
    pub trigger_dist_to_player: f32,
    pub trigger_threshold_time_path_stacked: f32,
    pub trigger_threshold_range_path_stacked: f32,
    unk28: [u8; 0x10],
}
