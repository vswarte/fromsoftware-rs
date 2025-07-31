use std::ops::Index;
use std::ptr::NonNull;

use crate::Vector;
use shared::OwnedPtr;

use crate::cs::{FieldInsHandle, GaitemHandle, ItemId};

#[repr(C)]
/// Source of name: RTTI
pub struct PlayerGameData {
    vftable: usize,
    pub character_type: u32,
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
    unk5c: f32,
    unk60: f32,
    unk64: f32,
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
    unk94: u32,
    unk98: u32,
    character_name: [u16; 16],
    unkbc: u8,
    unkbd: u8,
    pub gender: u8,
    pub archetype: u8,
    pub vow_type: u8,
    unkc1: u8,
    pub voice_type: u8,
    pub starting_gift: u8,
    unkc4: u8,
    pub unlocked_magic_slots: u8,
    pub unlocked_talisman_slots: u8,
    unkc7: [u8; 0x18],
    pub furlcalling_finger_remedy_active: bool,
    unke0: u8,
    unke1: u8,
    pub matching_weapon_level: u8,
    pub white_ring_active: u8,
    pub blue_ring_active: u8,
    pub team_type: u8,
    unke6: u8,
    /// True if the player is in their own world.
    pub is_my_world: bool,
    unke8: [u8; 0x4],
    unkec: u32,
    unkf0: [u8; 0x4],
    pub solo_breakin_point: u32,
    unkf8: u32,
    pub scadutree_blessing: u8,
    pub reversed_spirit_ash: u8,
    pub resist_curse_item_count: u8,
    pub rune_arc_active: bool,
    unk100: u8,
    pub max_hp_flask: u8,
    pub max_fp_flask: u8,
    unk103: [u8; 0x6],
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
    unk182: u16,
    unk184: [u8; 0x34],
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
    pub storage: OwnedPtr<EquipInventoryData>,
    gesture_game_data: usize,
    ride_game_data: usize,
    unk8e8: usize,
    pub is_main_player: bool,
    unk8f1: u8,
    unk8f2: [u8; 6],
    unk8f8: usize,
    unk900: [u8; 36],
    pub hp_estus_rate: f32,
    pub hp_estus_additional: u8,
    _pad929: [u8; 3],
    pub fp_estus_rate: f32,
    pub fp_estus_additional: u8,
    _pad931: [u8; 3],
    unk934: [u8; 0x1c],
    pub mount_handle: FieldInsHandle,
    unk958: [u8; 0x10f],
    pub quickmatch_kill_count: u8,
    unka68: [u8; 11],
    menu_ref_special_effect_1: usize,
    menu_ref_special_effect_2: usize,
    menu_ref_special_effect_3: usize,
    pub is_using_festering_bloody_finger: bool,
    unka91: [u8; 3],
    pub networked_speffect_entry_count: u32,
    pub quick_match_team: u8,
    unka99: [u8; 0x13],
    pub quick_match_map_load_ready: bool,
    unkaad: [u8; 0x2],
    /// Should sign cooldown be enabled?
    /// Each time your coop player dies and you have someone in your world
    /// you will get a cooldown depending on WhiteSignCoolTimeParam and level from SosSignMan
    pub sign_cooldown_enabled: bool,
    unkab0: [u8; 0x2f],
    unkadf: u8,
    unkae0: [u8; 0x8],
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
pub struct AcquiredProjectilesEntry {
    pub item_id: ItemId,
    unk4: u8,
    unk5: u8,
    unk6: u8,
    unk7: u8,
}

#[repr(C)]
pub struct AcquiredProjectiles {
    pub entries: [AcquiredProjectilesEntry; 2048],
    unk4000: u32,
    unk4004: u32,
    pub count: u32,
    unk400c: u32,
    unk4010: [usize; 256],
}

#[repr(C)]
pub struct QMItemBackupVectorItem {
    pub item_id: ItemId,
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
    pub arrow_primary: ItemId,
    pub bolt_primary: ItemId,
    pub arrow_secondary: ItemId,
    pub bolt_secondary: ItemId,
    pub arrow_tertiary: ItemId,
    pub bolt_tertiary: ItemId,
    pub protector_head: ItemId,
    pub protector_chest: ItemId,
    pub protector_hands: ItemId,
    pub protector_legs: ItemId,
    pub unused40: ItemId,
    pub accessories: [ItemId; 4],
    pub covenant: ItemId,
    pub quick_tems: [ItemId; 10],
    pub pouch: [ItemId; 6],
}

#[repr(C)]
pub struct EquipGameData {
    vftable: usize,
    unk8: [u32; 22],
    unk60: usize,
    unk68: u32,
    pub chr_asm: ChrAsm,
    _pad154: u32,
    pub equip_inventory_data: EquipInventoryData,
    pub equip_magic_data: OwnedPtr<EquipMagicData>,
    pub equip_item_data: EquipItemData,
    equip_gesture_data: usize,
    pub acquired_projectiles: OwnedPtr<AcquiredProjectiles>,
    pub qm_item_backup_vector: OwnedPtr<Vector<QMItemBackupVectorItem>>,
    pub equipment_entries: ChrAsmEquipEntries,
    unk3e0: usize,
    unk3e8: usize,
    pub player_game_data: NonNull<PlayerGameData>,
    unk3f8: [u8; 0xb8],
}

#[repr(C)]
pub struct InventoryItemsData {
    /// How many items can one hold in total?
    pub global_capacity: u32,

    /// Holds ordinary items.
    pub normal_item_capacity: u32,
    normal_item_head: OwnedPtr<EquipInventoryDataListEntry>,
    pub normal_item_count: u32,

    /// Holds key items.
    pub key_item_capacity: u32,
    key_item_head: OwnedPtr<EquipInventoryDataListEntry>,
    pub key_item_count: u32,

    /// Holds key items as well?
    pub secondary_key_item_capacity: u32,
    secondary_key_item_head: OwnedPtr<EquipInventoryDataListEntry>,
    pub secondary_key_item_count: u32,

    _pad3c: u32,

    normal_item_head_ptr: NonNull<EquipInventoryDataListEntry>,
    normal_item_count_ptr: NonNull<u32>,
    key_item_head_ptr: NonNull<EquipInventoryDataListEntry>,
    key_item_count_ptr: NonNull<u32>,

    /// Contains the indices into the item ID mapping list.
    item_id_mapping_indices: OwnedPtr<[u16; 2017]>,
    unk68: u64,
    /// Contains table of item IDs and their corresponding location in the equip inventory data
    /// lists.
    item_id_mapping: *mut ItemIdMapping,
    unk78: u64,
}

#[repr(C)]
pub struct EquipInventoryData {
    vftable: usize,
    pub items_data: InventoryItemsData,
    pub total_item_entry_count: u32,
    unk84: u32,
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

#[repr(C)]
pub struct ItemIdMapping {
    pub item_id: u32,
    bits4: u32,
}

impl ItemIdMapping {
    /// Returns the offset of the next item ID mapping with the same modulo result.
    pub fn next_mapping_item(&self) -> u32 {
        ((self.bits4 >> 12) & 0xFFF) - 1
    }

    /// Returns the index of the item slot. This index is first checked against the key items
    /// capacity to see if it's contained in that. If not you will need to subtract the key items
    /// capacity to get the index for the normal items list.
    pub fn item_slot(&self) -> u32 {
        self.bits4 & 0xFFF
    }
}

impl InventoryItemsData {
    pub fn normal_items(&self) -> &[EquipInventoryDataListEntry] {
        unsafe {
            std::slice::from_raw_parts(
                self.normal_item_head.as_ptr(),
                self.normal_item_count as usize,
            )
        }
    }

    pub fn normal_items_mut(&mut self) -> &mut [EquipInventoryDataListEntry] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.normal_item_head.as_ptr(),
                self.normal_item_count as usize,
            )
        }
    }

    pub fn key_items(&self) -> &[EquipInventoryDataListEntry] {
        unsafe {
            std::slice::from_raw_parts(self.key_item_head.as_ptr(), self.key_item_count as usize)
        }
    }

    pub fn key_items_mut(&mut self) -> &mut [EquipInventoryDataListEntry] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.key_item_head.as_ptr(),
                self.key_item_count as usize,
            )
        }
    }

    pub fn secondary_key_items(&self) -> &[EquipInventoryDataListEntry] {
        unsafe {
            std::slice::from_raw_parts(
                self.secondary_key_item_head.as_ptr(),
                self.secondary_key_item_count as usize,
            )
        }
    }

    pub fn secondary_key_items_mut(&mut self) -> &mut [EquipInventoryDataListEntry] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.secondary_key_item_head.as_ptr(),
                self.secondary_key_item_count as usize,
            )
        }
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
    pub display_id: u32,
    unk10: u8,
    _pad11: [u8; 3],
    pub pot_group: i32,
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
    Accessory1 = 17,
    Accessory2 = 18,
    Accessory3 = 19,
    Accessory4 = 20,
    AccessoryCovenant = 21,
    // ----- Slots below are not used in the param id lists and handles -----
    QuickItem1 = 22,
    QuickItem2 = 23,
    QuickItem3 = 24,
    QuickItem4 = 25,
    QuickItem5 = 26,
    QuickItem6 = 27,
    QuickItem7 = 28,
    QuickItem8 = 29,
    QuickItem9 = 30,
    QuickItem10 = 31,
    Pouch1 = 32,
    Pouch2 = 33,
    Pouch3 = 34,
    Pouch4 = 35,
    Pouch5 = 36,
    Pouch6 = 37,
    GreatRune = 38,
}

impl<T> Index<ChrAsmSlot> for [T] {
    type Output = T;

    fn index(&self, index: ChrAsmSlot) -> &Self::Output {
        &self[index as usize]
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
#[derive(Debug)]
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
