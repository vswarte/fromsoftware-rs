use std::ops::{Index, IndexMut};
use std::{num::NonZero, ptr::NonNull};

use bitfield::bitfield;
use shared::{OwnedPtr, UnknownStruct, empty::*};

use super::{ItemId, OptionalItemId};

#[repr(C)]
// Source of name: RTTI
pub struct PlayerGameData {
    _vftable: usize,
    _unk08: u64,
    pub player_info: PlayerInfo,
    _unk190: usize,
    _unk198: u32,
    _unk19c: u32,
    _unk1a0: u64,
    _unk1a8: [u8; 0xa0],
    pub effective_vigor: i32,
    pub effective_attunement: i32,
    pub effective_life_force: i32,
    pub effective_willpower: i32,
    pub effective_endurance: i32,
    pub effective_vitality: i32,
    pub effective_strength: i32,
    pub effective_dexterity: i32,
    pub effective_intelligence: i32,
    pub effective_faith: i32,
    pub effective_luck: i32,
    pub equip_game_data: EquipGameData,
    _face_data: UnknownStruct<0x118>,
    _unk858: u64,
    _unk860: u64,
    _unk868: usize,
    _unk870: u8,
    _unk871: u8,
    _unk878: usize,
    _unk880: u32,
    _unk884: u32,
    _unk888: u32,
    _unk88c: u32,
    _unk890: u64,
    _unk898: u64,
    _unk8a0: u16,
    pub hp_estus_allocate_rate: f32,
    pub hp_estus_allocate_offset: i32,
    pub mp_estus_allocate_rate: f32,
    pub mp_estus_allocate_offset: i32,
    pub nat_type: i32,
    _unk8b8: u64,
    _unk8c0: u64,
    _unk8c8: u64,
    _unk8d0: u64,
    _unk8d8: [u8; 0xe8],
    _unk9c0: u64,
    _unk9c8: u64,
    pub use_consume_invade_type: bool,
    _unk9d4: u32,
    _unk9d8: u32,
    _unk9dc: u32,
    _unk9e0: u32,
    _unk9e4: u8,
    _unk9e8: u64,
    _unk9f0: u32,
    _unk9f4: u16,
    _unk9f6: u16,
    _unk9f8: u32,
    _unk9fc: u16,
    _unka00: u32,
    _unka04: u32,
    _unka08: u64,
}

#[repr(C)]
pub struct PlayerInfo {
    pub player_number: i32,
    pub player_id: i8,
    _unk05: u8,
    _unk06: u8,
    _unk07: u8,

    /// The player's current health.
    pub hp: i32,

    /// The player's maximum health.
    pub max_hp: i32,

    /// The player's maximum health before any modifications.
    pub base_max_hp: i32,

    pub mp: i32,
    _unk18: u32,
    pub max_mp: i32,
    _unk20: u32,

    /// The player's current stamina (which goes down as they block).
    pub sp: i32,

    /// The player's maximum stamina.
    pub max_sp: i32,

    /// The player's maximum stamina before any modifications.
    pub base_max_sp: i32,

    _unk30: u32,
    pub vigor: i32,
    pub attunement: i32,
    pub life_force: i32,
    pub willpower: i32,
    pub endurance: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub faith: i32,
    pub luck: i32,
    _unk5c: u32,
    pub hero_points: i32,
    pub vitality: i32,
    pub soul_level: i32,

    /// The amount of sen the player currently has.
    pub sen: i32,

    /// The total amount of sen the player has ever earned across the course of
    /// their run.
    pub total_sen_earned: u64,

    pub total_add_param: i32,
    pub chr_type: i32,
    _unk80: u16,
    _unk82: [u8; 0x20],
    pub is_male: bool,
    pub voice_type: i8,
    pub shop_level: i16,
    pub archetype: i8,
    pub appearance: i8,
    pub gift: i8,
    _unka9: u8,
    pub net_penalized: bool,
    pub max_weapon_upgrade_level: i8,
    _unkac: u32,
    _unkb0: u32,
    _unkb4: u32,
    _unkb8: u32,
    _unkbc: u32,
    pub rosaria_ranking_points: i32,
    _unkc4: u32,
    _unkc8: u32,
    pub total_kills: i32,
    pub bounty_ranking_points: i32,
    _unkd4: u32,
    pub egg_soul: i32,
    pub poison_resist: i32,
    pub bleed_resist: i32,
    pub toxic_resist: i32,
    pub curse_resist: i32,
    _unkec: u32,
    _unkf0: u32,
    pub face_type: i8,
    pub hair_type: i8,
    pub hair_eyes_color: i8,
    pub curse_level: i8,
    pub invasion_type: i8,
    pub no_invasion: bool,

    /// Note: this name comes from the debug info but doesn't seem to correspond
    /// to the player's actual deaths.
    pub death_level: i8,

    _unkfb: u8,
    pub lord_of_cinder: bool,
    pub request_release_lord_of_cinder: bool,
    pub hp_estus_allocation: i8,
    pub mp_estus_allocation: u8,
    pub sin_points: i32,
    _unk104: u32,
    pub is_dead: bool,
    pub is_invader: bool,
    pub seed_of_a_giant_tree_misses: i8,
    pub debt_deceased_level_up_remain: i8,
    pub char_id: i8,
    _unk10d: u8,
    _unk10e: u8,
    _unk10f: u8,
    _unk110: u8,
    pub checking_net_penalty: bool,
    pub net_penalty_points: i16,
    pub net_penalty_time_until_forgive_item: f32,
    _unk118: u64,
    _unk120: u64,
    _unk128: u64,
    _unk130: u8,
    _unk131: u8,
    pub region: u8,
    pub total_soul_over_for_old: bool,
    pub total_soul_over: bool,
    _unk135: u8,
    _unk136: u8,
    _unk137: u8,
    _unk138: u32,
    _unk13c: [u8; 0x8],
    pub skill_level: i32,

    /// The total XP the player has earned across their entire run.
    pub total_experience_points: u64,

    /// The number of skill points the player currently has available to spend.
    pub skill_points: i32,

    _unk154: u32,
    _unk158: u64,
    _unk160: u64,
    _unk168: u8,
    _unk169: u8,
    _unk16a: u8,
    _unk16b: u8,
    _unk16c: u8,
    _unk16d: u8,
    _unk16e: u8,
    _unk16f: u8,
    _unk170: u8,
    _unk171: u8,
    _unk172: u8,
    _unk173: u8,
    _unk174: u8,
    _unk175: u8,
    _unk176: u8,
    _unk177: u8,
    _unk178: u8,
    _unk179: u8,
    _unk17a: u8,
    _unk17b: u8,
    _unk17c: u8,
    _unk17d: u8,
    _unk17e: u8,
    _unk17f: u8,
}

#[repr(C)]
pub struct EquipGameData {
    _vftable: usize,
    _unk08: u64,
    _unk10: u64,
    _unk18: u32,
    _unk1c: u32,
    _unk20: u8,
    _unk24: u32,
    _unk28: [u8; 0x60],
    _unk88: u64,
    _unk90: u32,
    _chr_asm: UnknownStruct<0xa0>,
    _unk138: u64,
    _chr_body_scale: UnknownStruct<0x28>,
    pub equip_inventory_data: EquipInventoryData,
    _equip_magic_data: usize,
    _equip_item_data: UnknownStruct<0xb0>,
    _equip_throw_skill_data: usize,
    _gesture_equip_data: usize,
    _unk350: usize,
    _unk358: u32,
    pub owner: NonNull<PlayerGameData>,
    _unk368: u8,
    _unk36c: u32,
    _unk370: u32,
    _unk374: [u8; 0xa4],
    _unk418: u32,
    _unk41c: u32,
    _unk420: u32,
    _unk424: u32,
    _unk428: u32,
    _unk42c: u32,
    _unk430: u32,
    _unk434: u32,
    _unk438: u32,
    _unk43c: u32,
    _unk440: u32,
    _unk444: u32,
    _unk448: u64,
    _unk450: u64,
    _unk458: u64,
    _unk460: u64,
    _unk468: u64,
    _unk470: u64,
    _unk478: u64,
    _unk480: u64,
    _unk488: u64,
    _unk490: u64,
    _unk498: u64,
    _unk4a0: u64,
    _unk4a8: u64,
    _unk4b0: u64,
    _unk4b8: u64,
    _unk4c0: u64,
}

#[repr(C)]
pub struct EquipInventoryData {
    vftable: usize,
    _unk08: u64,
    pub items_data: InventoryItemsData,
    _unk88: u32,
    _unk8c: u8,
    _unk90: [u32; 0x1e],
    _unk108: u8,
    _unk109: u8,
    _unk10a: u8,
    _unk10b: u8,
    _unk10c: u32,
    _unk110: u32,
    _unk114: u32,
    _unk118: u32,
}

#[repr(C)]
pub struct InventoryItemListAccessor {
    pub head: NonNull<MaybeEmpty<EquipInventoryDataListEntry>>,
    pub len: NonNull<u32>,
}

#[repr(C)]
pub struct InventoryItemsData {
    /// The total number of items the player can hold.
    pub total_capacity: u32,

    /// Capacity of the [normal_items_head](Self::normal_items_head) array.
    pub normal_items_capacity: u32,

    /// Pointer to the head of the normal items inventory.
    ///
    /// **Note:** This array is not dense. If an entry in the middle is emptied
    /// due to an item being removed from the player's inventory, other items
    /// are *not* rearranged to fill the hole.
    pub normal_items_head: OwnedPtr<MaybeEmpty<EquipInventoryDataListEntry>>,

    /// The number of normal items in the inventory.
    pub normal_items_count: u32,

    /// Capacity of the [key_items_head](Self::key_items_head) array.
    pub key_items_capacity: u32,

    /// Pointer to the head of the key items inventory.
    ///
    /// **Note:** This array is not dense. If an entry in the middle is emptied
    /// due to an item being removed from the player's inventory, other items
    /// are *not* rearranged to fill the hole.
    pub key_items_head: OwnedPtr<MaybeEmpty<EquipInventoryDataListEntry>>,

    /// The number of key items in the inventory.
    pub key_items_count: u32,

    /// Pointer to the head of the multiplayer key items inventory. Likely
    /// unused in Sekiro.
    ///
    /// **Note:** This array is not dense. If an entry in the middle is emptied
    /// due to an item being removed from the player's inventory, other items
    /// are *not* rearranged to fill the hole.
    pub multiplayer_key_items_head: OwnedPtr<MaybeEmpty<EquipInventoryDataListEntry>>,

    /// The number of key items in the multiplayer inventory. Likely unused in
    /// Sekiro.
    pub multiplayer_key_items_count: u32,

    /// Pointers to the active normal item list and its count. All inventory
    /// reads and writes in the game will go through this.
    pub normal_items_accessor: InventoryItemListAccessor,

    /// Pointers to the active key item list and its count. All inventory reads
    /// and writes in the game will go through this.
    pub key_items_accessor: InventoryItemListAccessor,

    /// A map from item IDs (mod 2017) to the index of their mapping linked list
    /// in [item_id_mappings](Self::item_id_mappings).
    ///
    /// This is populated as items are added to the inventory. All entries begin
    /// as -1.
    pub item_id_mapping_indices: OwnedPtr<[i16; 2017]>,

    _unk60: u32,

    /// A [total_capacity](Self::total_capacity)-length array of mappings from
    /// item IDs to indices in [normal_items_head](Self::normal_items_head) or
    /// [key_items_head](Self::key_items_head).
    ///
    /// This is iteslf indexed by
    /// [item_id_mapping_indices](Self::item_id_mapping_indices).
    pub item_id_mappings: OwnedPtr<ItemIdMapping>,

    /// The index into [item_id_mappings](Self::item_id_mappings) that should be
    /// used next time an item is added to the inventory whose index (mod 2017)
    /// hasn't yet been allocated to
    /// [item_id_mapping_indices](Self::item_id_mapping_indices).
    pub next_index: u16,
}

impl InventoryItemsData {
    /// The total number of items in the inventory.
    pub fn items_len(&self) -> u32 {
        self.normal_items_count + self.key_items_count
    }

    /// Returns an iterator over all the non-empty entries in the player's
    /// inventory.
    ///
    /// This iterates over key items first, followed by normal items.
    pub fn items(&self) -> impl Iterator<Item = &EquipInventoryDataListEntry> {
        self.key_entries()
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
                self.key_items_head.as_ptr(),
                self.key_items_capacity as usize,
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

    /// Returns a slice over all the [EquipInventoryDataListEntry] allocated for
    /// this [InventoryItemsData], whether or not they're empty or in range of
    /// [key_items_count](Self::key_items_count).
    pub fn key_entries(&self) -> &[MaybeEmpty<EquipInventoryDataListEntry>] {
        unsafe {
            std::slice::from_raw_parts(
                self.key_items_head.as_ptr(),
                self.key_items_capacity as usize,
            )
        }
    }

    /// Returns a mutable slice over all the [EquipInventoryDataListEntry]
    /// allocated for this [InventoryItemsData], whether or not they're empty or
    /// in range of [key_items_count](Self::key_items_count).
    pub fn key_entries_mut(&mut self) -> &mut [MaybeEmpty<EquipInventoryDataListEntry>] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.key_items_head.as_ptr(),
                self.key_items_capacity as usize,
            )
        }
    }

    /// Returns a slice over all the [EquipInventoryDataListEntry] allocated for
    /// this [InventoryItemsData], whether or not they're empty or in range of
    /// [normal_items_count](Self::normal_items_count).
    pub fn normal_entries(&self) -> &[MaybeEmpty<EquipInventoryDataListEntry>] {
        unsafe {
            std::slice::from_raw_parts(
                self.normal_items_head.as_ptr(),
                self.normal_items_capacity as usize,
            )
        }
    }

    /// Returns a mutable slice over all the [EquipInventoryDataListEntry]
    /// allocated for this [InventoryItemsData], whether or not they're empty or
    /// in range of [normal_items_count](Self::normal_items_count).
    pub fn normal_entries_mut(&mut self) -> &mut [MaybeEmpty<EquipInventoryDataListEntry>] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.normal_items_head.as_ptr(),
                self.normal_items_capacity as usize,
            )
        }
    }
}

impl Index<u32> for InventoryItemsData {
    type Output = MaybeEmpty<EquipInventoryDataListEntry>;

    /// Indexes both the key and normal item entries of [InventoryItemsData]
    /// using the same logic as the game.
    ///
    /// If `index` is less than [key_items_capacity](Self::key_items_capacity),
    /// this returns a key items entry. If it's greater than or equal to
    /// [key_items_capacity](Self::key_items_capacity) but less than that plus
    /// [normal_items_capacity](Self::normal_items_capacity), this returns a
    /// normal item entry. Otherwise, it panics.
    fn index(&self, index: u32) -> &Self::Output {
        if index < self.key_items_capacity {
            return &self.key_entries()[index as usize];
        }

        let index = index - self.key_items_capacity;
        if index < self.normal_items_capacity {
            return &self.normal_entries()[index as usize];
        }

        panic!("index {} out of range", index)
    }
}

impl IndexMut<u32> for InventoryItemsData {
    /// Mutably indexes both the key and normal item entries of
    /// [InventoryItemsData] using the same logic as the game.
    ///
    /// If `index` is less than [key_items_capacity](Self::key_items_capacity),
    /// this returns a key items entry. If it's greater than or equal to
    /// [key_items_capacity](Self::key_items_capacity) but less than that plus
    /// [normal_items_capacity](Self::normal_items_capacity), this returns a
    /// normal item entry. Otherwise, it panics.
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        if index < self.key_items_capacity {
            return &mut self.key_entries_mut()[index as usize];
        }

        let index = index - self.key_items_capacity;
        if index < self.normal_items_capacity {
            return &mut self.normal_entries_mut()[index as usize];
        }

        panic!("index {} out of range", index)
    }
}

/// An entry in [InventoryItemsData].
#[repr(C)]
pub struct EquipInventoryDataListEntry {
    /// Handle to the gaitem instance which describes additional properties of
    /// the inventory item, like durability.
    pub gaitem_handle: NonZero<u32>,

    /// The raw ID of the item in this inventory slot. This is invalid if the
    /// inventory item has since been removed.
    pub item_id: ItemId,

    /// Quantity of the item we have.
    pub quantity: u32,

    _unk0c: [u8; 4],
}

unsafe impl IsEmpty for EquipInventoryDataListEntry {
    fn is_empty(value: &MaybeEmpty<EquipInventoryDataListEntry>) -> bool {
        !OptionalItemId::from(unsafe { *value.as_non_null().cast::<u32>().offset(1).as_ref() })
            .is_valid()
    }
}

#[repr(C)]
pub struct ItemIdMapping {
    /// The ID of the item whose mapping this represents. This is invalid if
    /// there aren't currently any items in this bucket.
    item_id: OptionalItemId,

    /// Indices into [InventoryItemsData]'s lists related to this mapping.
    indices: ItemIdMappingIndices,
}

bitfield! {
    pub struct ItemIdMappingIndices(u32);
    impl Debug;

    /// The index in [InventoryItemsData] at which [ItemIdMapping.item_id]
    /// appears.
    ///
    /// If this is less than [InventoryItemsData.key_items_capacity], it's a
    /// direct index into [InventoryItemsData.key_items_head]. Otherwise, this
    /// minus [InventoryItemsData.key_items_capacity] is an index into
    /// [InventoryItemsData.normal_items_head].
    pub u16, inventory_index, set_inventory_index: 12, 0;

    /// If [ItemIdMapping.item_id_raw] is invalid, this is one plus the index
    /// into [InventoryItemsData.item_id_mappings] that should be used for the
    /// next new mapping after this one has been allocated.
    ///
    /// If [ItemIdMapping.item_id_raw] is valid and there are additional items
    /// in the same bucket as this one, this is one plus the index into
    /// [InventoryItemsData.item_id_mappings] for the next item in the bucket.
    ///
    /// Otherwise, this is 0.
    pub u16, next_index, set_next_index: 24, 12;

    unk04_25, _: 25;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x180, size_of::<PlayerInfo>());
        assert_eq!(0x120, size_of::<EquipInventoryData>());
        assert_eq!(0x4c8, size_of::<EquipGameData>());
        assert_eq!(0xa10, size_of::<PlayerGameData>());
    }
}
