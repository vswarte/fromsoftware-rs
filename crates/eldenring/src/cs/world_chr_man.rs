use std::collections::LinkedList;
use std::ffi;
use std::fmt::Display;
use std::marker::PhantomData;
use std::mem::transmute;
use std::ptr::NonNull;

use vtable_rs::VPtr;

use crate::cs::{CSEzTask, CSEzVoidTask};
use crate::position::HavokPosition;
use crate::{
    cs::{ChrIns, EnemyIns},
    Vector,
};
use crate::{ChainingTree, DoublyLinkedList, Tree};
use shared::{F32Matrix4x4, F32Vector4, OwnedPtr};

use super::{BlockId, ChrCam, FieldInsHandle, NetChrSync, PlayerIns};

#[repr(C)]
/// Source of name: RTTI
#[shared::singleton("WorldChrMan")]
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
    pub summon_buddy_manager: OwnedPtr<SummonBuddyManager>,
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
    // WorldChrMan tasks
    unk1ece8: [u8; 0x4e8],
    /// A list of ChrIns references sorted by distance to the main player.
    pub chr_inses_by_distance: Vector<ChrInsDistanceEntry>,
    unk1f1f0: [u8; 0x10],
    /// A list of ChrIns references sorted by their update priority.
    pub chr_inses_by_update_priority: Vector<NonNull<ChrIns>>,
    /// The remaining budget for characters that can receive high-detail (NORMAL) updates this frame.
    pub omission_update_budget_near: u32,
    /// The remaining budget for characters that can receive medium-detail (LVL2) updates this frame.
    pub omission_update_budget_far: u32,
    unk1f228: [u8; 0x28],
    chr_ins_calc_update_info_perf_begin_task: CSEzVoidTask<CSEzTask, Self>,
    chr_ins_calc_update_info_perf_end_task: CSEzVoidTask<CSEzTask, Self>,
    chr_ins_ailogic_perf_begin_task: CSEzVoidTask<CSEzTask, Self>,
    chr_ins_ailogic_perf_end_task: CSEzVoidTask<CSEzTask, Self>,
    chr_ins_pre_behavior_task: CSEzVoidTask<CSEzTask, Self>,
    chr_ins_pre_behavior_safe_task2: CSEzVoidTask<CSEzTask, Self>,
    chr_ins_pre_cloth_task: CSEzVoidTask<CSEzTask, Self>,
    chr_ins_pre_cloth_safe_task: CSEzVoidTask<CSEzTask, Self>,
    chr_ins_post_physics_task: CSEzVoidTask<CSEzTask, Self>,
    chr_ins_post_physics_safe_task: CSEzVoidTask<CSEzTask, Self>,
}

impl WorldChrMan {
    pub fn chr_ins_by_handle(&mut self, handle: &FieldInsHandle) -> Option<&mut ChrIns> {
        let chr_set_index = handle.selector.container() as usize;
        let chr_set = self.chr_sets.get_mut(chr_set_index)?.as_mut()?;

        chr_set.chr_ins_by_handle(handle)
    }

    pub fn spawn_debug_character(&mut self, request: &ChrDebugSpawnRequest) {
        let mut name_bytes = format!("c{:0>4}", request.chr_id)
            .encode_utf16()
            .collect::<Vec<_>>();

        name_bytes.resize(0x20, 0x0);

        self.debug_chr_creator
            .init_data
            .name
            .clone_from_slice(name_bytes.as_mut());

        self.debug_chr_creator.init_data.chara_init_param_id = request.chara_init_param_id;
        self.debug_chr_creator.init_data.npc_param_id = request.npc_param_id;
        self.debug_chr_creator.init_data.npc_think_param_id = request.npc_think_param_id;
        self.debug_chr_creator.init_data.event_entity_id = request.event_entity_id;
        self.debug_chr_creator.init_data.talk_id = request.talk_id;

        self.debug_chr_creator.init_data.spawn_position =
            F32Vector4(request.pos_x, request.pos_y, request.pos_z, 0.0);

        self.debug_chr_creator.spawn = true;
    }
}

pub struct ChrDebugSpawnRequest {
    pub chr_id: i32,
    pub chara_init_param_id: i32,
    pub npc_param_id: i32,
    pub npc_think_param_id: i32,
    pub event_entity_id: i32,
    pub talk_id: i32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
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
    pub chr_load_status: ChrLoadStatus,
    pub chr_update_type: ChrUpdateType,
    pub entry_flags: u8,
    _padb: [u8; 5],
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChrLoadStatus {
    Unloaded = 0,
    Initializing = 1,
    Active = 2,
    NetworkInitializing = 3,
    ReadyForActivation = 4,
    Unloading = 5,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChrUpdateType {
    Local = 0,
    Unknown1 = 1,
    Unknown2 = 2,
    Unknown3 = 3,
    Remote = 4,
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
    /// Maps SpEffect IDs to BuddyParam IDs.
    /// Because multiple BuddyParams can share the same SpEffect, this is a chaining tree.
    pub trigger_speffect_to_buddy_map: ChainingTree<i32, i32>,
    /// ID of SpEffect used to request a summon.
    /// Written by TAE goods consume.
    pub request_summon_speffect_id: i32,
    /// Currently active summon SpEffect ID.
    pub active_summon_speffect_id: u32,
    /// Whether or not a disappear has been requested for the active summon.
    pub disappear_requested: bool,
    /// Reference to the SummonBuddy ChrSet on WorldChrMan.
    pub chr_set: NonNull<ChrSet<ChrIns>>,
    /// ID of the entity of the buddy stone the character is currently "talking" to.
    pub buddy_stone_entity_id: u32,
    /// ID of the entity of the buddy stone that manages currently active summon.
    pub active_summmon_buddy_stone_entity_id: u32,
    unk40: usize,
    unk48: usize,
    unk50: usize,
    unk58: usize,
    unk60: usize,
    unk68: usize,
    /// Describes the groups of summons.
    pub groups: Tree<SummonBuddyGroup>,
    unk88: i32,
    /// Delay before the buddy disappears after being requested to disappear.
    pub buddy_disappear_delay_sec: f32,
    /// Cooldown after using a summon item before it can be used again.
    /// Used to prevent quick spawn/despawn spam.
    pub item_use_cooldown_timer: f32,
    unk94: u32,
    unk98: u32,
    unk9c: f32,
    /// Position at which to spawn the next summon
    pub spawn_origin: HavokPosition,
    /// Rotation at which to spawn the next summon
    pub spawn_rotation: f32,
    /// Whether the player currently has an alive summon
    pub player_has_alive_summon: bool,
    /// Whether the player is within buddy stone activation range
    pub is_within_activation_range: bool,
    /// Whether the previous update found the player within buddy stone activation range
    pub prev_is_within_activation_range: bool,
    /// Whether the player is within buddy stone warn range (15 units less than activation range)
    pub is_within_warn_range: bool,
    /// Whether the previous update found the player within buddy stone warn range
    pub prev_is_within_warn_range: bool,
    pub last_buddy_slot: u32,
    unkc0: f32,
    unkc4: f32,
    pub eliminate_target_entries: Tree<SummonBuddyStoneEliminateTargetEntry>,
    /// ID of the summon goods that was requested to be used by TAE.
    pub requested_summon_goods_id: i32,
    /// ID of the currently active summon goods.
    pub active_summon_goods_id: i32,
    pub warp_manager: OwnedPtr<SummonBuddyWarpManager>,
    unkf0: usize,
    /// BuddyStoneParam ID used for debug and memory profiling.
    pub debug_buddy_stone_param_id: u32,
    unk100: usize,
    unk108: usize,
}

#[repr(C)]
pub struct SummonBuddyWarpManager {
    pub entries: Tree<SummonBuddyWarpEntry>,
    /// See: [crate::param::GAME_SYSTEM_COMMON_PARAM_ST::buddy_warp_trigger_time_ray_blocked]
    pub trigger_time_ray_block: f32,
    /// See: [crate::param::GAME_SYSTEM_COMMON_PARAM_ST::buddy_warp_trigger_dist_to_player]
    pub trigger_dist_to_player: f32,
    /// See: [crate::param::GAME_SYSTEM_COMMON_PARAM_ST::buddy_warp_threshold_time_path_stacked]
    pub trigger_threshold_time_path_stacked: f32,
    /// See: [crate::param::GAME_SYSTEM_COMMON_PARAM_ST::buddy_warp_threshold_range_path_stacked]
    pub trigger_threshold_range_path_stacked: f32,
    unk28: u32,
    unk2c: i32,
    unk30: bool,
}

#[repr(C)]
pub struct SummonBuddyWarpEntry {
    pub handle: FieldInsHandle,
    unk8: usize,
    pub warp_stage: SummonBuddyWarpStage,
    unk18: usize,
    pub target_position: HavokPosition,
    pub q_target_rotation: F32Vector4,
    pub flags: u32,
    pub time_ray_blocked: f32,
    unk48: f32,
    unk50: F32Vector4,
    unk60: f32,
    pub time_path_stacked: f32,
    unk68: usize,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SummonBuddyWarpStage {
    None = 0,
    RequestWarp = 1,
    Warping = 2,
    FadeIn = 3,
}

#[repr(C)]
pub struct SummonBuddyGroup {
    /// Event ID of the owner.
    pub owner_event_id: i32,
    /// List of group entries
    pub entries: DoublyLinkedList<SummonBuddyGroupEntry>,
}

#[repr(C)]
pub struct SummonBuddyGroupEntry {
    /// ChrIns this group entry is for.
    pub chr_ins: NonNull<ChrIns>,
    unk8: bool,
    /// Whether this SummonBuddy has mount or not.
    pub has_mount: bool,
    /// This SummonBuddy's BuddyParam ID.
    pub buddy_param_id: i32,
    /// Buddy stone param ID this SummonBuddy was spawned from.
    pub buddy_stone_param_id: i32,
    /// See [crate::param::BUDDY_STONE_PARAM_ST::doping_sp_effect_id]
    pub doping_sp_effect_id: i32,
    /// SpEffect applied by leveling this summon buddy up.
    pub dopping_level_sp_effect_id: u32,
    /// Animation ID to play on spawn.
    /// See [crate::param::BUDDY_PARAM_ST::generate_anim_id]
    pub spawn_animation: u32,
    /// Whether or not warp has been requested for this SummonBuddy.
    pub warp_requested: bool,
    /// Set to true when the SummonBuddy is going to be despawned.
    pub disappear_requested: bool,
    /// Delay before the SummonBuddy disappears.
    pub disappear_delay_sec: f32,
    /// Whether the valid spawn point was found for this SummonBuddy.
    pub has_spawn_point: bool,
    /// See [crate::param::BUDDY_PARAM_ST::disable_pc_target_share]
    pub disable_pc_target_share: bool,
    /// See [crate::param::BUDDY_PARAM_ST::pc_follow_type]
    pub follow_type: u8,
    /// Whether this SummonBuddy was created by remote request.
    pub is_remote: bool,
    /// Whether creator of this SummonBuddy has the Mogh's Great Rune buff.
    pub has_mogh_great_rune_buff: bool,
    unk2d: bool,
}

#[repr(C)]
/// Source of name: RTTI
pub struct SummonBuddyStoneEliminateTargetEntry {
    /// Refers to the SummonBuddy this entry represents.
    pub buddy_field_ins_handle: FieldInsHandle,
    /// Keeps track of if the buddy stones eliminate target is in range.
    pub target_calc: CSBuddyStoneEliminateTargetCalc,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSBuddyStoneEliminateTargetCalc {
    vftable: u64,
    /// Refers to the SummonBuddy this target calc belongs to.
    pub owner_field_ins_handle: FieldInsHandle,
    /// Refers to the BuddyStoneParam that the SummonBuddy was spawned from.
    pub buddy_stone_param_id: i32,
    /// Refers to the elimination target using an event entity ID for this target calc.
    /// This can be a group.
    pub target_event_entity_id: i32,
    /// Is the targeted entity in range of the SummonBuddy.
    pub target_in_range: bool,
    /// Framecount since last update, used to update target_in_range every 33 frames.
    pub range_check_counter: u32,
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use crate::cs::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x20, size_of::<CSBuddyStoneEliminateTargetCalc>());
        assert_eq!(0x28, size_of::<SummonBuddyStoneEliminateTargetEntry>());
        assert_eq!(0x30, size_of::<SummonBuddyGroupEntry>());
        assert_eq!(0x20, size_of::<SummonBuddyGroup>());
        assert_eq!(0x70, size_of::<SummonBuddyWarpEntry>());
        assert_eq!(0x38, size_of::<SummonBuddyWarpManager>());
        assert_eq!(0x110, size_of::<SummonBuddyManager>());
    }
}
