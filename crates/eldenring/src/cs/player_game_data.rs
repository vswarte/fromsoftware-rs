use std::ops::Index;
use std::ptr::NonNull;

use bitfield::bitfield;
use thiserror::Error;

use crate::{
    BasicVector, Vector,
    cs::{ChrType, MultiplayRole},
};
use shared::{IsEmpty, MaybeEmpty, NonEmptyIteratorExt, NonEmptyIteratorMutExt, OwnedPtr};

use crate::cs::{FieldInsHandle, GaitemHandle, ItemId, OptionalItemId};

#[repr(C)]
/// Source of name: RTTI
pub struct PlayerGameData {
    vftable: usize,
    /// Event id of this game data owner
    pub character_event_id: u32,
    pub player_id: u32,
    pub current_hp: u32,
    pub current_max_hp: u32,
    pub base_max_hp: u32,
    pub current_fp: u32,
    pub current_max_fp: u32,
    pub base_max_fp: u32,
    unk28: f32,
    pub current_stamina: u32,
    pub current_max_stamina: u32,
    pub base_max_stamina: u32,
    unk38: f32,
    pub vigor: u32,
    pub mind: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub arcane: u32,
    pub base_hero_point: f32,
    pub base_hero_point_2: f32,
    pub base_durability: f32,
    pub level: u32,
    pub rune_count: u32,
    pub rune_memory: u32,
    unk74: u32,
    pub poison_resist: u32,
    pub rot_resist: u32,
    pub bleed_resist: u32,
    pub death_resist: u32,
    pub frost_resist: u32,
    pub sleep_resist: u32,
    pub madness_resist: u32,
    pub pending_block_clear_bonus: f32,
    pub chr_type: ChrType,
    character_name: [u16; 17],
    pub gender: u8,
    pub archetype: u8,
    pub vow_type: u8,
    unkc1: u8,
    pub voice_type: u8,
    pub starting_gift: u8,
    unkc4: u8,
    pub unlocked_magic_slots: u8,
    pub unlocked_talisman_slots: u8,
    pub matchmaking_spirit_ashes_level: u8,
    pub total_summon_count: u32,
    pub coop_success_count: u32,
    /// Index into [crate::cs::GameDataMan]'s player game data array
    pub game_data_man_index: u32,
    unkd4: [u8; 0xb],
    pub furlcalling_finger_remedy_active: bool,
    unke0: u8,
    unke1: u8,
    pub matching_weapon_level: u8,
    pub white_ring_active: u8,
    pub blue_ring_active: u8,
    /// [MultiplayRole] of the player this game data belongs to
    pub multiplay_role: MultiplayRole,
    unke6: u8,
    /// True if the player is in their own world.
    pub is_my_world: bool,
    unke8: [u8; 0x3],
    unke9: bool,
    pub character_id: u32,
    pub invasions_success_count: u32,
    pub solo_breakin_point: u32,
    pub invaders_killed: u32,
    pub scadutree_blessing: u8,
    pub reversed_spirit_ash: u8,
    pub resist_curse_item_count: u8,
    pub rune_arc_active: bool,
    unk100: bool,
    pub max_hp_flask: u8,
    pub max_fp_flask: u8,
    unk103: [u8; 0x4],
    pub sell_region: SellRegion,
    unk108: u8,
    pub reached_max_rune_memory: u8,
    unk10a: [u8; 0xE],
    pub password: [u16; 0x8],
    unk128: u16,
    group_password_1: [u16; 0x8],
    unk13a: u16,
    group_password_2: [u16; 0x8],
    unk14c: u16,
    group_password_3: [u16; 0x8],
    unk15e: u16,
    group_password_4: [u16; 0x8],
    unk170: u16,
    group_password_5: [u16; 0x8],
    unk182: [u8; 0x36],
    pub sp_effects: [PlayerGameDataSpEffect; 0xD],
    /// Level after any buffs and corrections
    pub effective_vigor: u32,
    /// Level after any buffs and corrections
    pub effective_mind: u32,
    /// Level after any buffs and corrections
    pub effective_endurance: u32,
    /// Level after any buffs and corrections
    pub effective_vitality: u32,
    /// Level after any buffs and corrections
    pub effective_strength: u32,
    /// Level after any buffs and corrections
    pub effective_dexterity: u32,
    /// Level after any buffs and corrections
    pub effective_intelligence: u32,
    /// Level after any buffs and corrections
    pub effective_faith: u32,
    /// Level after any buffs and corrections
    pub effective_arcane: u32,
    unk2ac: u32,
    pub equipment: EquipGameData,
    pub face_data: FaceData,
    /// Describes the storage box contents.
    pub storage: Option<OwnedPtr<EquipInventoryData>>,
    gesture_game_data: usize,
    ride_game_data: usize,
    unk8e8: usize,
    /// True when this game data belongs to the main (local) player.
    pub is_main_player: bool,
    /// Did this player agreed to voice chat?
    pub is_voice_chat_enabled: bool,
    unk8f2: [u8; 6],
    unk8f8: usize,
    unk900: [u8; 36],
    pub hp_estus_rate: f32,
    pub hp_estus_additional: u8,
    _pad929: [u8; 3],
    pub fp_estus_rate: f32,
    pub fp_estus_additional: u8,
    _pad931: [u8; 3],
    unk934: u32,
    /// Vector of all visited play area IDs
    pub visited_areas: BasicVector<u32>,
    pub mount_handle: FieldInsHandle,
    unk958: [u8; 0x8],
    pub damage_negation_physical: i32,
    pub attack_rating: PlayerDataAttackRating,
    pub damage_negation_magic: i32,
    unk980: f32,
    unk984: f32,
    pub max_equip_load: f32,
    unk98c: u32,
    pub damage_negation_strike: i32,
    pub damage_negation_slash: i32,
    pub damage_negation_pierce: i32,
    pub damage_negation_fire: i32,
    pub damage_negation_lightning: i32,
    pub damage_negation_holy: i32,
    unused_defence_status: [f32; 8],
    pub resistance_gauges: [u32; 7],
    pub resistance_gauge_max: [u32; 7],
    unused_gauge_list: [f32; 7],
    pub proc_status_timers: [f32; 7],
    pub proc_status_timer_max: [f32; 7],
    unka54: u32,
    pub frontend_flags: PlayerGameDataFrontendFlags,
    unka59: [u8; 0xE],
    pub quickmatch_kill_count: u8,
    unka68: [u8; 0x4],
    pub poise: f32,
    pub discovery: u32,
    menu_ref_special_effect_1: usize,
    menu_ref_special_effect_2: usize,
    menu_ref_special_effect_3: usize,
    pub is_using_festering_bloody_finger: bool,
    pub used_invasion_item_type: PlayerDataInvasionItemType,
    unka92: [u8; 2],
    pub packed_time_stamp: u32,
    pub quick_match_team: u8,
    unka99: [u8; 0x3],
    unka9c: i32,
    pub quick_match_duel_points: u16,
    pub quick_match_united_combat_points: u16,
    pub quick_match_spirit_ashes_points: u16,
    pub quickmatch_duel_rank: u8,
    pub quickmatch_united_combat_rank: u8,
    pub quickmatch_spirit_ashes_rank: u8,
    pub unkaa9: bool,
    unkaa: u8,
    pub is_quick_match_host: bool,
    pub quick_match_map_load_ready: bool,
    pub quick_match_desired_team: u8,
    unkaae: u8,
    /// Should sign cooldown be enabled?
    /// Each time your coop player dies and you have someone in your world
    /// you will get a cooldown depending on [crate::param::WHITE_SIGN_COOL_TIME_PARAM_ST] and level from [crate::cs::SosSignMan::white_sign_cool_time_param_id]
    pub sign_cooldown_enabled: bool,
    unkab0: [u8; 0x2],
    pub has_preorder_gesture: bool,
    pub has_preorder_sote_gesture: bool,
    unkab4: [u8; 0x34],
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SellRegion {
    None = 0,
    Japan = 1,
    NorthAmerica = 2,
    Europe = 3,
    Asia = 4,
    Global = 5,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PlayerGameDataFrontendFlags(u8);
    impl Debug;

    bool;
    pub disable_status_effect_bars, set_disable_status_effect_bars: 0;
    pub rune_arc_active, set_rune_arc_active: 1;
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayerDataInvasionItemType {
    BloodyFinger = 0,
    FesteringBloodyFinger = 1,
    RecusantFinger = 2,
}

#[repr(C)]
pub struct PlayerDataAttackRating {
    pub left_armament_primary: i32,
    pub right_armament_primary: i32,
    pub left_armament_secondary: i32,
    pub right_armament_secondary: i32,
    pub left_armament_tertiary: i32,
    pub right_armament_tertiary: i32,
}

#[repr(C)]
pub struct FaceData {
    vftable: usize,
    pub face_data_buffer: FaceDataBuffer,
    unk128: usize,
    unk130: [f32; 7],
    unk14c: [u8; 0x24],
}

#[repr(C)]
pub struct FaceDataBuffer {
    pub magic: [u8; 4],
    pub version: u32,
    pub buffer_size: u32,
    pub buffer: [u8; 276],
}

#[repr(C)]
pub struct PlayerGameDataSpEffect {
    pub sp_effect_id: u32,
    pub duration: f32,
    unk8: u32,
    unkc: u32,
}

#[repr(C)]
pub struct ItemReplenishStateEntry {
    pub item_id: OptionalItemId,
    pub auto_replenish: bool,
}

#[repr(C)]
pub struct ItemReplenishStateEntryUnk {
    pub item_id: OptionalItemId,
    pub auto_replenish: bool,
}

#[repr(C)]
/// Tracks the state of item replenishment from the chest when you sit at a Site of Grace
pub struct ItemReplenishStateTracker {
    entries: [ItemReplenishStateEntry; 2048],
    unk4000: u32,
    unk4004: u32,
    pub count: u64,
    unk4010: [ItemReplenishStateEntryUnk; 256],
}

impl ItemReplenishStateTracker {
    pub fn entries(&self) -> &[ItemReplenishStateEntry] {
        &self.entries[..self.count as usize]
    }

    pub fn entries_mut(&mut self) -> &mut [ItemReplenishStateEntry] {
        &mut self.entries[..self.count as usize]
    }
}

#[repr(C)]
pub struct QMItemBackupVectorItem {
    pub item_id: OptionalItemId,
    pub quantity: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChrAsmEquipEntries {
    pub weapon_primary_left: ItemId,
    pub weapon_primary_right: ItemId,
    pub weapon_secondary_left: ItemId,
    pub weapon_secondary_right: ItemId,
    pub weapon_tertiary_left: ItemId,
    pub weapon_tertiary_right: ItemId,
    pub arrow_primary: OptionalItemId,
    pub bolt_primary: OptionalItemId,
    pub arrow_secondary: OptionalItemId,
    pub bolt_secondary: OptionalItemId,
    pub arrow_tertiary: OptionalItemId,
    pub bolt_tertiary: OptionalItemId,
    pub protector_head: ItemId,
    pub protector_chest: ItemId,
    pub protector_hands: ItemId,
    pub protector_legs: ItemId,
    pub unused40: OptionalItemId,
    pub accessories: [OptionalItemId; 4],
    pub covenant: OptionalItemId,
    pub quick_tems: [OptionalItemId; 10],
    pub pouch: [OptionalItemId; 6],
}

#[repr(C)]
pub struct EquipGameData {
    vftable: usize,
    unk8: [u32; 22],
    unk60: usize,
    unk68: u32,
    pub chr_asm: ChrAsm,
    pub equip_inventory_data: EquipInventoryData,
    pub equip_magic_data: OwnedPtr<EquipMagicData>,
    pub equip_item_data: EquipItemData,
    equip_gesture_data: usize,
    /// Tracker for the item replenishing from the chest
    pub item_replenish_state_tracker: Option<OwnedPtr<ItemReplenishStateTracker>>,
    pub qm_item_backup_vector: OwnedPtr<Vector<QMItemBackupVectorItem>>,
    pub equipment_entries: ChrAsmEquipEntries,
    unk3e0: usize,
    unk3e8: usize,
    pub player_game_data: NonNull<PlayerGameData>,
    /// Whether this equipment data belongs to the main (local) player.
    pub is_main_player: bool,
    /// Result of the last attempt to add an item to the inventory
    pub last_add_item_result: LastAddItemResult,
    /// Bitfield tracking which equipment slots have fully broken equipment
    /// Used to sync visuals of broken equipment in multiplayer
    /// (DS3 leftover)
    pub broken_equipment_slots: BrokenEquipmentSlots,
    unk404: [u8; 0xac],
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LastAddItemResult {
    Success = 0,
    UniqueItemDuplicate = 2,
    InventoryFull = 4,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    /// Flags indicating that certain equipment slots are fully broken
    /// (DS3 leftover)
    pub struct BrokenEquipmentSlots(u32);
    impl Debug;

    bool;
    pub weapon_left1, set_weapon_left1: 0;
    pub weapon_right1, set_weapon_right1: 1;
    pub weapon_left2, set_weapon_left2: 2;
    pub weapon_right2, set_weapon_right2: 3;
    pub weapon_left3, set_weapon_left3: 4;
    pub weapon_right3, set_weapon_right3: 5;
    pub arrow1, set_arrow1: 6;
    pub bolt1, set_bolt1: 7;
    pub arrow2, set_arrow2: 8;
    pub bolt2, set_bolt2: 9;
    pub arrow3, set_arrow3: 10;
    pub bolt3, set_bolt3: 11;
    pub protector_head, set_protector_head: 12;
    pub protector_chest, set_protector_chest: 13;
    pub protector_hands, set_protector_hands: 14;
    pub protector_legs, set_protector_legs: 15;
    pub unused16, set_unused16: 16;
    pub accessory1, set_accessory1: 17;
    pub accessory2, set_accessory2: 18;
    pub accessory3, set_accessory3: 19;
    pub accessory4, set_accessory4: 20;
    pub accessory_covenant, set_accessory_covenant: 21;
}

#[repr(C)]
pub struct InventoryItemListAccessor {
    pub head: NonNull<MaybeEmpty<EquipInventoryDataListEntry>>,
    pub length: NonNull<u32>,
}

#[repr(C)]
pub struct InventoryItemsData {
    /// How many items can one hold in total?
    pub global_capacity: u32,

    /// The maximum capacity of the normal items inventory.
    pub normal_items_capacity: u32,

    /// A pointer to the head of the normal items inventory.
    pub normal_items_head: OwnedPtr<MaybeEmpty<EquipInventoryDataListEntry>>,

    /// The length currently in use of the normal items inventory.
    ///
    /// This isn't necessarily the number of items in the inventory. The
    /// inventory can have gaps (such as when you pick up two items and then
    /// discard the earlier one), and this counts those gaps as part of the
    /// length despite not being actual items.
    pub normal_items_len: u32,

    /// The maximum capacity of the key items inventory.
    pub key_items_capacity: u32,

    /// A pointer to the head of the key items inventory.
    pub key_items_head: OwnedPtr<MaybeEmpty<EquipInventoryDataListEntry>>,

    /// The length currently in use of the key items inventory.
    ///
    /// This isn't necessarily the number of items in the inventory. The
    /// inventory can have gaps (such as when you pick up two items and then
    /// discard the earlier one), and this counts those gaps as part of the
    /// length despite not being actual items.
    pub key_items_len: u32,

    /// The maximum capacity of the multiplayer key items inventory.
    pub multiplay_key_items_capacity: u32,

    /// Holds key items that are available in multiplayer.
    ///
    /// Unless new key items are somehow obtained in multiplayer, this only contains
    /// copies of the items from `key_items` that have `REGENERATIVE_MATERIAL`
    /// and `WONDROUS_PHYSICK_TEAR` types (pots and wondrous physic tears).
    pub multiplay_key_items_head: OwnedPtr<MaybeEmpty<EquipInventoryDataListEntry>>,

    /// The length currently in use of the multiplayer key items inventory.
    ///
    /// This isn't necessarily the number of items in the inventory. The
    /// inventory can have gaps (such as when you pick up two items and then
    /// discard the earlier one), and this counts those gaps as part of the
    /// length despite not being actual items.
    pub multiplay_key_items_len: u32,

    _pad3c: u32,

    /// Pointers to the active normal item list and its length. All inventory
    /// reads and writes in the game go through this.
    ///
    /// Unlike `key_items_accessor`, this is always the same as `normal_items`.
    pub normal_items_accessor: InventoryItemListAccessor,

    /// Pointers to the active key item list and its length. All inventory reads
    /// and writes in the game go through this.
    ///
    /// In single-player, this typically points to `key_items`. In multiplayer,
    /// it switches to `multiplay_key_items`.
    pub key_items_accessor: InventoryItemListAccessor,

    /// Contains the indices into the item ID mapping list.
    item_id_mapping_indices: OwnedPtr<[u16; 2017]>,
    unk68: u64,
    /// Contains table of item IDs and their corresponding location in the equip inventory data
    /// lists.
    item_id_mapping: *mut ItemIdMapping,
    unk78: u64,
}

impl InventoryItemsData {
    /// Returns an iterator over all the non-empty entries in the player's
    /// inventory.
    ///
    /// This iterates over key items first, followed by normal items.
    pub fn items(&self) -> impl Iterator<Item = &EquipInventoryDataListEntry> {
        self.current_key_entries()
            .iter()
            .chain(self.normal_entries().iter())
            .non_empty()
    }

    /// Returns an iterator over all the mutable non-empty entries in the
    /// player's inventory.
    ///
    /// This iterates over key items first, followed by normal items.
    pub fn items_mut(&self) -> impl Iterator<Item = &mut EquipInventoryDataListEntry> {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.key_items_accessor.head.as_ptr(),
                *self.key_items_accessor.length.as_ref() as usize,
            )
        }
        .iter_mut()
        .chain(
            unsafe {
                std::slice::from_raw_parts_mut(
                    self.normal_items_head.as_ptr(),
                    self.normal_items_capacity as usize,
                )
            }
            .iter_mut(),
        )
        .non_empty()
    }

    /// A slice over all the normal item [EquipInventoryDataListEntry] allocated
    /// for this [InventoryItemsData], whether or not they're empty or in range
    /// of [normal_items_len](Self::normal_items_len).
    pub fn normal_entries(&self) -> &[MaybeEmpty<EquipInventoryDataListEntry>] {
        unsafe {
            std::slice::from_raw_parts(
                self.normal_items_head.as_ptr(),
                self.normal_items_capacity as usize,
            )
        }
    }

    /// A mutable slice over all the normal item [EquipInventoryDataListEntry]
    /// allocated for this [InventoryItemsData], whether or not they're empty or
    /// in range of [normal_items_len](Self::normal_items_len).
    pub fn normal_entries_mut(&mut self) -> &mut [MaybeEmpty<EquipInventoryDataListEntry>] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.normal_items_head.as_ptr(),
                self.normal_items_len as usize,
            )
        }
    }

    /// Whether there's no more room left in the normal items inventory and
    /// picking up a new item will fail.
    pub fn is_normal_items_full(&self) -> bool {
        self.normal_items_len >= self.normal_items_capacity
            && self.normal_entries().iter().all(|e| !e.is_empty())
    }

    /// A slice over all the key item [EquipInventoryDataListEntry] allocated
    /// for this [InventoryItemsData], whether or not they're empty or in range
    /// of [key_items_len](Self::key_items_len).
    pub fn key_entries(&self) -> &[MaybeEmpty<EquipInventoryDataListEntry>] {
        unsafe {
            std::slice::from_raw_parts(self.key_items_head.as_ptr(), self.key_items_len as usize)
        }
    }

    /// A mutable slice over all the key item [EquipInventoryDataListEntry]
    /// allocated for this [InventoryItemsData], whether or not they're empty or
    /// in range of [key_items_len](Self::key_items_len).
    pub fn key_entries_mut(&mut self) -> &mut [MaybeEmpty<EquipInventoryDataListEntry>] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.key_items_head.as_ptr(),
                self.key_items_len as usize,
            )
        }
    }

    /// Whether there's no more room left in the key items inventory and picking
    /// up a new item will fail.
    pub fn is_key_items_full(&self) -> bool {
        self.key_items_len >= self.key_items_capacity
            && self.key_entries().iter().all(|e| !e.is_empty())
    }

    /// A slice over all the multiplayer key item [EquipInventoryDataListEntry]
    /// allocated for this [InventoryItemsData], whether or not they're empty or
    /// in range of [multiplay_key_items_len](Self::multiplay_key_items_len).
    pub fn multiplay_key_entries(&self) -> &[MaybeEmpty<EquipInventoryDataListEntry>] {
        unsafe {
            std::slice::from_raw_parts(
                self.multiplay_key_items_head.as_ptr(),
                self.multiplay_key_items_len as usize,
            )
        }
    }

    /// A mutable slice over all the multiplayer key item
    /// [EquipInventoryDataListEntry] allocated for this [InventoryItemsData],
    /// whether or not they're empty or in range of
    /// [multiplay_key_items_len](Self::multiplay_key_items_len).
    pub fn multiplay_key_entries_mut(&mut self) -> &mut [MaybeEmpty<EquipInventoryDataListEntry>] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.multiplay_key_items_head.as_ptr(),
                self.multiplay_key_items_len as usize,
            )
        }
    }

    /// Whether there's no more room left in the multiplayer items inventory and
    /// picking up a new item will fail.
    pub fn is_multiplay_key_items_full(&self) -> bool {
        self.multiplay_key_items_len >= self.multiplay_key_items_capacity
            && self.multiplay_key_entries().iter().all(|e| !e.is_empty())
    }

    /// A slice over all the key item [EquipInventoryDataListEntry] allocated
    /// for this [InventoryItemsData], whether or not they're empty or in range
    /// of the associated length field.
    ///
    /// This is equivalent to either [key_entries](Self::key_entries) and
    /// [multiplay_key_entries](Self::multiplay_key_entries), depending on
    /// whether the player is currently in a multiplayer session.
    pub fn current_key_entries(&self) -> &[MaybeEmpty<EquipInventoryDataListEntry>] {
        unsafe {
            std::slice::from_raw_parts(
                self.key_items_accessor.head.as_ptr(),
                *self.key_items_accessor.length.as_ref() as usize,
            )
        }
    }

    /// A mutable slice over all the key item [EquipInventoryDataListEntry]
    /// allocated for this [InventoryItemsData], whether or not they're empty or
    /// in range of the associated length field.
    ///
    /// This is equivalent to either [key_entries_mut](Self::key_entries_mut)
    /// and [multiplay_key_entries_mut](Self::multiplay_key_entries_mut),
    /// depending on whether the player is currently in a multiplayer session.
    pub fn current_key_entries_mut(&mut self) -> &mut [MaybeEmpty<EquipInventoryDataListEntry>] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.key_items_accessor.head.as_ptr(),
                *self.key_items_accessor.length.as_ref() as usize,
            )
        }
    }
}

#[repr(C)]
pub struct EquipInventoryData {
    vftable: usize,
    pub items_data: InventoryItemsData,
    pub total_item_entry_count: u32,
    /// Next sort ID to assign to newly added items.
    /// Used to sort items by acquisition order.
    pub next_sort_id: u32,
    /// Count of all pot items by their pot group
    pub pot_items_count: [u32; 16],
    /// Capacity of all pot items by their pot group
    pub pot_items_capacity: [u32; 16],
    unk108: [u8; 0x18],
    /// True will allow consumables stack up to 600 like in storage box.
    pub unlimited_consumables: bool,
    /// Should pots be limited to amount of pot capacity by their group?
    pub limited_pots: bool,
    unk122: u8,
    unk123: u8,
    unk124: u32,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    struct ItemIdMappingBits(u32);
    impl Debug;

    u32;
    mapping_index, _: 23, 12;
    item_slot, _: 11, 0;
}

#[repr(C)]
pub struct ItemIdMapping {
    pub item_id: OptionalItemId,
    bits4: ItemIdMappingBits,
}

impl ItemIdMapping {
    /// Returns the offset of the next item ID mapping with the same modulo result.
    pub fn next_mapping_item(&self) -> u32 {
        self.bits4.mapping_index() - 1
    }

    /// Returns the index of the item slot. This index is first checked against the key items
    /// capacity to see if it's contained in that. If not you will need to subtract the key items
    /// capacity to get the index for the normal items list.
    pub fn item_slot(&self) -> u32 {
        self.bits4.item_slot()
    }
}

#[repr(C)]
pub struct EquipInventoryDataListEntry {
    /// Handle to the gaitem instance which describes additional properties to the inventory item,
    /// like durability and gems in the case of weapons.
    pub gaitem_handle: GaitemHandle,
    pub item_id: ItemId,
    /// Quantity of the item we have.
    pub quantity: u32,
    /// Sort ID used to sort items by acquisition order.
    pub sort_id: u32,
    /// Whether the item is newly acquired and should be highlighted in the UI if
    /// "Mark New Items" option is enabled.
    pub is_new: bool,
    /// [pot group] of the item, or -1 if not a pot item.
    ///
    /// [pot group]: crate::param::EQUIP_PARAM_GOODS_ST::pot_group_id
    pub pot_group: i32,
}

unsafe impl IsEmpty for EquipInventoryDataListEntry {
    fn is_empty(value: &MaybeEmpty<EquipInventoryDataListEntry>) -> bool {
        !OptionalItemId::from(unsafe { *value.as_non_null().cast::<u32>().offset(1).as_ref() })
            .is_valid()
    }
}

#[repr(C)]
pub struct EquipMagicData {
    vftable: usize,
    pub equip_game_data: NonNull<EquipGameData>,
    pub entries: [EquipMagicItem; 14],
    pub selected_slot: i32,
    unk84: u32,
}

#[repr(C)]
pub struct EquipMagicItem {
    pub param_id: i32,
    pub charges: i32,
}

#[repr(C)]
pub struct EquipItemData {
    vftable: usize,
    pub quick_slots: [EquipDataItem; 10],
    pub pouch_slots: [EquipDataItem; 6],
    pub great_rune: EquipDataItem,
    pub equip_entries: OwnedPtr<ChrAsmEquipEntries>,
    pub inventory: OwnedPtr<EquipInventoryData>,
    pub selected_quick_slot: i32,
    unka4: u32,
}

#[repr(C)]
pub struct EquipDataItem {
    pub gaitem_handle: GaitemHandle,
    pub index: i32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChrAsmSlot {
    WeaponLeft1 = 0,
    WeaponRight1 = 1,
    WeaponLeft2 = 2,
    WeaponRight2 = 3,
    WeaponLeft3 = 4,
    WeaponRight3 = 5,
    Arrow1 = 6,
    Bolt1 = 7,
    Arrow2 = 8,
    Bolt2 = 9,
    Arrow3 = 10,
    Bolt3 = 11,
    ProtectorHead = 12,
    ProtectorChest = 13,
    ProtectorHands = 14,
    ProtectorLegs = 15,
    Unused16 = 16,
    Accessory1 = 17,
    Accessory2 = 18,
    Accessory3 = 19,
    Accessory4 = 20,
    AccessoryCovenant = 21,
}

impl<T> Index<ChrAsmSlot> for [T] {
    type Output = T;

    fn index(&self, index: ChrAsmSlot) -> &Self::Output {
        &self[index as usize]
    }
}

#[derive(Debug, Error)]
pub enum ChrAsmSlotError {
    #[error("Invalid ChrAsmSlot index: {0}")]
    InvalidIndex(u32),
}

impl ChrAsmSlot {
    pub fn from_index(index: u32) -> Result<Self, ChrAsmSlotError> {
        match index {
            0 => Ok(ChrAsmSlot::WeaponLeft1),
            1 => Ok(ChrAsmSlot::WeaponRight1),
            2 => Ok(ChrAsmSlot::WeaponLeft2),
            3 => Ok(ChrAsmSlot::WeaponRight2),
            4 => Ok(ChrAsmSlot::WeaponLeft3),
            5 => Ok(ChrAsmSlot::WeaponRight3),
            6 => Ok(ChrAsmSlot::Arrow1),
            7 => Ok(ChrAsmSlot::Bolt1),
            8 => Ok(ChrAsmSlot::Arrow2),
            9 => Ok(ChrAsmSlot::Bolt2),
            10 => Ok(ChrAsmSlot::Arrow3),
            11 => Ok(ChrAsmSlot::Bolt3),
            12 => Ok(ChrAsmSlot::ProtectorHead),
            13 => Ok(ChrAsmSlot::ProtectorChest),
            14 => Ok(ChrAsmSlot::ProtectorHands),
            15 => Ok(ChrAsmSlot::ProtectorLegs),
            16 => Ok(ChrAsmSlot::Unused16),
            17 => Ok(ChrAsmSlot::Accessory1),
            18 => Ok(ChrAsmSlot::Accessory2),
            19 => Ok(ChrAsmSlot::Accessory3),
            20 => Ok(ChrAsmSlot::Accessory4),
            21 => Ok(ChrAsmSlot::AccessoryCovenant),
            _ => Err(ChrAsmSlotError::InvalidIndex(index)),
        }
    }
}

#[repr(C)]
pub struct ChrAsmEquipmentSlots {
    /// Points to the slot in the equipment list used for rendering the left-hand weapon.
    /// 0 for primary, 1 for secondary, 2 for tertiary.
    pub left_weapon_slot: u32,
    /// Points to the slot in the equipment list used for rendering the right-hand weapon.
    /// 0 for primary, 1 for secondary, 2 for tertiary.
    pub right_weapon_slot: u32,
    /// Points to the slot in the equipment list used for rendering the left-hand arrow.
    /// 0 for primary, 1 for secondary.
    pub left_arrow_slot: u32,
    /// Points to the slot in the equipment list used for rendering the right-hand arrow.
    /// 0 for primary, 1 for secondary.
    pub right_arrow_slot: u32,
    /// Points to the slot in the equipment list used for rendering the left-hand bolt.
    /// 0 for primary, 1 for secondary.
    pub left_bolt_slot: u32,
    /// Points to the slot in the equipment list used for rendering the right-hand bolt.
    /// 0 for primary, 1 for secondary.
    pub right_bolt_slot: u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChrAsmArmStyle {
    EmptyHanded = 0,
    OneHanded = 1,
    LeftBothHands = 2,
    RightBothHands = 3,
}

#[repr(C)]
pub struct ChrAsmEquipment {
    /// Determines how you're holding your weapon.
    pub arm_style: ChrAsmArmStyle,
    pub selected_slots: ChrAsmEquipmentSlots,
}

#[repr(C)]
/// Describes how the character should be rendered in terms of selecting the
/// appropriate parts to be rendered.
///
/// Source of name: RTTI in earlier games (vmt has been removed from ER after some patch?)
pub struct ChrAsm {
    unk0: i32,
    unk4: i32,
    pub equipment: ChrAsmEquipment,
    /// Holds references to the inventory slots for each equipment piece.
    pub gaitem_handles: [GaitemHandle; 22],
    /// Holds the param IDs for each equipment piece.
    pub equipment_param_ids: [i32; 22],
    unkd4: u32,
    unkd8: u32,
    _paddc: [u8; 12],
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EquipmentDurabilityStatus {
    Ok = 0,
    AtRisk = 1,
    Broken = 2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_id_mapping() {
        let mapping = ItemIdMapping {
            item_id: OptionalItemId::from(0x40002760),
            bits4: ItemIdMappingBits(0x003B8000),
        };
        assert_eq!(mapping.item_id, OptionalItemId::from(0x40002760));
        assert_eq!(
            mapping.next_mapping_item(),
            ((mapping.bits4.0 >> 12) & 0xFFF) - 1
        );
        assert_eq!(mapping.item_slot(), mapping.bits4.0 & 0xFFF);
    }
}
