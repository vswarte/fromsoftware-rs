use std::{fmt::Display, mem::transmute};

use bitfield::bitfield;
use thiserror::Error;

use crate::cs::ItemId;
use shared::OwnedPtr;

#[repr(C)]
#[shared::singleton("CSGaitem")]
pub struct CSGaitemImp {
    vftable: usize,
    pub gaitems: [Option<OwnedPtr<CSGaitemIns>>; 5120],
    // TODO: fact-check this
    gaitem_descriptors: [CSGaitemImpEntry; 5120],
    indexes: [u32; 5120],
    write_index: u32,
    read_index: u32,
    rand_xorshift: [u8; 0x18],
    unk23028: [u8; 8],
    /// Becomes true if the CSGaitemImp is being serialized for saving to the save file.
    pub is_being_serialized: bool,
    unk23038: [u8; 7],
}

#[repr(C)]
pub struct CSGaitemIns {
    vftable: usize,
    pub gaitem_handle: GaitemHandle,
    pub item_id: ItemId,
}

impl CSGaitemIns {
    /// Downcast the CSGaitemIns to the derivant class. Will return None if the requested type
    /// does not match the gaitem ins's type.
    pub fn as_wep(&self) -> Option<&CSWepGaitemIns> {
        Some(match self.gaitem_handle.category() {
            // Safety: consumers are not allowed to make their own CSGaitemIns and other instances
            // come from the game. The category can reliably be used to do this downcast.
            Ok(GaitemCategory::Weapon) => unsafe {
                transmute::<&CSGaitemIns, &CSWepGaitemIns>(self)
            },
            _ => return None,
        })
    }

    /// Downcast the CSGaitemIns to the derivant class. Will return None if the requested type
    /// does not match the gaitem ins's type.
    pub fn as_wep_mut(&mut self) -> Option<&mut CSWepGaitemIns> {
        Some(match self.gaitem_handle.category() {
            // Safety: consumers are not allowed to make their own CSGaitemIns and other instances
            // come from the game. The category can reliably be used to do this downcast.
            Ok(GaitemCategory::Weapon) => unsafe {
                transmute::<&mut CSGaitemIns, &mut CSWepGaitemIns>(self)
            },
            _ => return None,
        })
    }

    /// Downcast the CSGaitemIns to the derivant class. Will return None if the requested type
    /// does not match the gaitem ins's type.
    pub fn as_gem(&self) -> Option<&CSGemGaitemIns> {
        Some(match self.gaitem_handle.category() {
            // Safety: consumers are not allowed to make their own CSGaitemIns and other instances
            // come from the game. The category can reliably be used to do this downcast.
            Ok(GaitemCategory::Gem) => unsafe { transmute::<&CSGaitemIns, &CSGemGaitemIns>(self) },
            _ => return None,
        })
    }

    /// Downcast the CSGaitemIns to the derivant class. Will return None if the requested type
    /// does not match the gaitem ins's type.
    pub fn as_gem_mut(&mut self) -> Option<&mut CSGemGaitemIns> {
        Some(match self.gaitem_handle.category() {
            // Safety: consumers are not allowed to make their own CSGaitemIns and other instances
            // come from the game. The category can reliably be used to do this downcast.
            Ok(GaitemCategory::Gem) => unsafe {
                transmute::<&mut CSGaitemIns, &mut CSGemGaitemIns>(self)
            },
            _ => return None,
        })
    }
}

#[repr(C)]
pub struct CSGaitemImpEntry {
    unindexed_gaitem_handle: u32,
    ref_count: u32,
}

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct GaitemHandle(u32);
    impl Debug;

    /// The index of the GaitemIns inside the CSGaitemImp.
    pub index, _: 15, 0;
    _, set_index: 15, 0;

    pub selector, _: 23, 0;
    _, set_selector: 23, 0;

    /// Indicates if the gaitem handle refers to a GaitemIns available in CSGaitemImp.
    /// Will be true for Protectors, Weapons and Gems.
    pub is_indexed, _: 23;
    _, set_is_indexed: 23;

    u8;
    /// The category of the GaitemHandle.
    pub category_raw, _: 30, 28;
    _, set_category_raw: 30, 28;

    /// A flag that is always set along with the category.
    /// Separated into it's own bitfield to avoid bitshifts on the category.
    category_flag, set_category_flag: 31;
}

#[derive(Debug, Error)]
pub enum GaitemHandleError {
    #[error("Not a valid Gaitem handle category {0}")]
    InvalidCategory(u8),
}

impl GaitemHandle {
    pub fn from_parts(selector: u32, category: GaitemCategory) -> Self {
        let mut handle = GaitemHandle(0);
        handle.set_selector(selector);
        handle.set_category_raw(category as u8);
        handle.set_category_flag(true);
        handle
    }

    pub fn category(self) -> Result<GaitemCategory, GaitemHandleError> {
        GaitemCategory::try_from(self.category_raw())
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GaitemCategory {
    Weapon = 0,
    Protector = 1,
    Accessory = 2,
    Goods = 3,
    Gem = 4,
}

impl TryFrom<u8> for GaitemCategory {
    type Error = GaitemHandleError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GaitemCategory::Weapon),
            1 => Ok(GaitemCategory::Protector),
            2 => Ok(GaitemCategory::Accessory),
            3 => Ok(GaitemCategory::Goods),
            4 => Ok(GaitemCategory::Gem),
            _ => Err(GaitemHandleError::InvalidCategory(value)),
        }
    }
}

impl Display for GaitemHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.category() {
            Ok(category) => match self.is_indexed() {
                true => write!(
                    f,
                    "GaitemHandle({},0x{:x},{:?})",
                    self.index(),
                    self.selector(),
                    category
                ),
                false => write!(f, "GaitemHandle(-1,{},{:?})", self.selector(), category),
            },
            Err(err) => write!(f, "GaitemHandle(0x{:x},{:?})", self.0, err),
        }
    }
}

#[repr(C)]
pub struct CSWepGaitemIns {
    pub gaitem_ins: CSGaitemIns,
    /// Item durability mechanic. Unused in ER.
    pub durability: u32,
    // _pad14: [u8; 0x4],
    /// Gem slots, used for ashes of war in ER.
    pub gem_slot_table: CSGemSlotTable,
}

#[repr(C)]
pub struct CSGemSlotTable {
    vtable: usize,
    pub gem_slots: [CSGemSlot; 1],
}

#[repr(C)]
pub struct CSGemSlot {
    vtable: usize,
    /// Refers to the actual gem entry in the CSGaitemImp.
    pub gaitem_handle: GaitemHandle,
    // _padc: [u8; 0x4],
}

#[repr(C)]
pub struct CSGemGaitemIns {
    pub gaitem_ins: CSGaitemIns,
    /// Handle of the weapon this gem is attached to
    pub weapon_handle: GaitemHandle,
    // _pad14: [u8; 0x4],
}

#[cfg(test)]
mod test {
    use crate::cs::{
        CSGaitemImp, CSGaitemIns, CSGemGaitemIns, CSGemSlot, CSGemSlotTable, CSWepGaitemIns,
    };

    #[test]
    fn proper_sizes() {
        assert_eq!(0x19038, size_of::<CSGaitemImp>());
        assert_eq!(0x10, size_of::<CSGaitemIns>());
        assert_eq!(0x30, size_of::<CSWepGaitemIns>());
        assert_eq!(0x18, size_of::<CSGemSlotTable>());
        assert_eq!(0x10, size_of::<CSGemSlot>());
        assert_eq!(0x18, size_of::<CSGemGaitemIns>());
    }
}
