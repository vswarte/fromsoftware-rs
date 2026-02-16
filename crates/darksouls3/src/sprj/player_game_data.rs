use std::ops::{Index, IndexMut};
use std::{borrow::Cow, iter, num::NonZero, ptr::NonNull, slice};

use bitfield::bitfield;
use shared::{FromStatic, InstanceResult, OwnedPtr, empty::*};

use crate::CxxVec;
use crate::sprj::{ItemGetMenuMan, ItemId, OptionalItemId, PlayerIns};

mod gesture;

pub use gesture::*;

#[repr(C)]
// Source of name: RTTI
pub struct PlayerGameData {
    _vftable: usize,
    _unk08: u64,
    pub player_info: PlayerInfo,
    _unk150: [u8; 0xD8],
    pub equipment: EquipGameData,
    _unk550: [u8; 0x148],

    pub face_data: FaceData,

    /// The contents of the storage box.
    pub storage: Option<OwnedPtr<EquipInventoryData>>,

    /// Data about the player's gestures.
    pub gesture_data: OwnedPtr<GestureGameData>,

    _unk7c0: [u8; 0x58],
    _unk810: CxxVec<u64>,
    _unk830: [u8; 0xe8],
    _menu_ref_special_effect_1: usize,
    _menu_ref_special_effect_2: usize,
    _unk930: [u8; 0x20],
}

impl PlayerGameData {
    /// Grants the player a gesture, similarly to the `AwardGesture` EMEVD command.
    pub fn grant_gesture(&mut self, gesture_index: u32, item_id: ItemId) {
        self.gesture_data.set_gesture_acquired(gesture_index, true);
        if let Ok(menu_man) = unsafe { ItemGetMenuMan::instance() } {
            menu_man.show_item(item_id, 1, false);
        }
    }
}

impl FromStatic for PlayerGameData {
    fn name() -> Cow<'static, str> {
        "PlayerGameData".into()
    }

    /// Returns the singleton instance of `PlayerGameData` for the main player
    /// character, if it exists.
    ///
    /// This always returns
    /// [InstanceError::NotFound](shared::InstanceError::NotFound) on the main
    /// menu.
    unsafe fn instance() -> InstanceResult<&'static mut Self> {
        // Go through PlayerIns because it doesn't exist on the main menu.
        unsafe { PlayerIns::instance().map(|ins| ins.player_game_data.as_mut()) }
    }
}

#[repr(C)]
/// Source of name: chosen by us
pub struct PlayerInfo {
    pub id: u32,
    _unk04: u32,

    /// The player's current health.
    pub hp: u32,

    /// The player's maximum health.
    pub max_hp: u32,

    /// The player's maximum health before any dynamic adjustments.
    pub base_max_hp: u32,

    /// The player's current MP.
    pub mp: u32,

    /// The player's maximum MP.
    pub max_mp: u32,

    /// The player's maximum MP before any dynamic adjustments.
    pub base_max_mp: u32,

    _unk20: u32,

    /// The player's current stamina.
    pub stamina: u32,

    /// The player's maximum stamina.
    pub max_stamina: u32,

    /// The player's maximum stamina before any dynamic adjustments.
    pub base_max_stamina: u32,

    _unk30: u32,

    /// The player's vigor stat.
    pub vigor: u32,

    /// The player's attunement stat.
    pub attunement: u32,

    /// The player's endurance stat.
    pub endurance: u32,

    /// The player's strength stat.
    pub strength: u32,

    /// The player's dexterity stat.
    pub dexterity: u32,

    /// The player's intelligence stat.
    pub intelligence: u32,

    /// The player's faith stat.
    pub faith: u32,

    /// The player's luck stat.
    pub luck: u32,

    _unk54: u32,
    _unk58: u32,

    /// The player's vitality stat.
    pub vitality: u32,

    _unk60: u64,
    _unk68: u64,
    _unk70: u64,

    /// The character's name, in UTF-16. The final word is always 0, to ensure
    /// the string is null-terminated.
    pub character_name: [u16; 17],

    _unk9a: [u8; 0xa6],
}

impl PlayerInfo {
    /// Returns the player's name.
    pub fn name(&self) -> String {
        let length = self
            .character_name
            .iter()
            .position(|c| *c == 0)
            .unwrap_or(self.character_name.len());
        String::from_utf16(&self.character_name[..length]).unwrap()
    }
}

#[repr(C)]
pub struct EquipGameData {
    _vftable: usize,
    _unk08: [u8; 0x1c],

    /// A mapping from equipment slots to the [EquipInventoryData] indices of
    /// items currently in those slots.
    pub equipment_indexes: [i32; 22],

    _unk7c: [u8; 0x12C],
    pub equip_inventory_data: EquipInventoryData,
    _unk248: [u8; 0xe0],
}

impl EquipGameData {
    /// For whatever reason, DS3 has an EquipGameData active on the main menu as
    /// well as in the context of an individual game. This function returns
    /// whether a given instance is for the synthetic loading screen character
    /// rather than a real loaded world.
    pub fn is_main_menu(&self) -> bool {
        // For some even stranger reason, the loading screen save actually does
        // have a handful of items. However, even a totally fresh save has more
        // items than that, so we check for 12 items which is exactly how many
        // the loading screen has. This could be tricked if a player discarded
        // all their starting equipment, so... don't do that.
        self.equip_inventory_data.items_data.normal_items_count == 12
    }
}

#[repr(C)]
pub struct EquipInventoryData {
    _vftable: usize,
    _unk08: u64,
    pub items_data: InventoryItemsData,

    /// The largest [EquipInventoryData] index that *might* not be empty.
    /// There's no guarantee that this isn't actually empty.
    pub max_item_index: i32,

    pub is_inventory_full: bool,
    _unk8d: [u8; 0x13],
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

    _unk24: [u8; 0x14],

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

    _unk60: u64,

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

    _unk72: [u8; 0x6],
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
    pub fn items(&self) -> ItemsIterator<'_> {
        self.key_entries()
            .iter()
            .chain(self.normal_entries().iter())
            .non_empty()
    }

    /// Returns an iterator over all the mutable non-empty entries in the
    /// player's inventory.
    ///
    /// This iterates over key items first, followed by normal items.
    pub fn items_mut(&self) -> ItemsIteratorMut<'_> {
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

/// An iterator over both normal and key items in [InventoryItemsData] that
/// exposes only non-empty entries.
///
/// Returned by [InventoryItemsData.items].
pub type ItemsIterator<'a> = NonEmptyIter<
    'a,
    EquipInventoryDataListEntry,
    iter::Chain<
        slice::Iter<'a, MaybeEmpty<EquipInventoryDataListEntry>>,
        slice::Iter<'a, MaybeEmpty<EquipInventoryDataListEntry>>,
    >,
>;

/// A mutable iterator over both normal and key items in [InventoryItemsData]
/// that exposes only non-empty entries.
///
/// Returned by [InventoryItemsData.items_mut].
pub type ItemsIteratorMut<'a> = NonEmptyIterMut<
    'a,
    EquipInventoryDataListEntry,
    iter::Chain<
        slice::IterMut<'a, MaybeEmpty<EquipInventoryDataListEntry>>,
        slice::IterMut<'a, MaybeEmpty<EquipInventoryDataListEntry>>,
    >,
>;

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

#[repr(C)]
pub struct FaceData {
    _vftable: usize,
    _unk08: [u8; 0x108],
    pub player_game_data: *mut PlayerGameData,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x10, size_of::<EquipInventoryDataListEntry>());
        assert_eq!(0x118, size_of::<FaceData>());
        assert_eq!(0x78, size_of::<InventoryItemsData>());
        assert_eq!(0xa0, size_of::<EquipInventoryData>());
        assert_eq!(0x328, size_of::<EquipGameData>());
        assert_eq!(0x140, size_of::<PlayerInfo>());
        assert_eq!(0x950, size_of::<PlayerGameData>());
    }
}
