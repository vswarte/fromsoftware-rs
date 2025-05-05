use std::{fmt::Display, mem::transmute};

use thiserror::Error;

use crate::cs::ItemId;
use shared::OwnedPtr;

#[repr(C)]
#[dlrf::singleton("CSGaitem")]
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

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GaitemHandle(pub u32);

#[derive(Debug, Error)]
pub enum GaitemHandleError {
    #[error("Not a valid Gaitem handle category {0}")]
    InvalidCategory(u8),
}

impl GaitemHandle {
    pub const fn from_parts(selector: u32, category: GaitemCategory) -> Self {
        GaitemHandle(selector & 0x00FFFFFF | (((category as i32) | -8) as u32) << 28)
    }

    /// Indicates if the gaitem handle refers to a GaitemIns available in CSGaitemImp.
    /// Will be true for Protectors, Weapons and Gems.
    pub const fn is_indexed(self) -> bool {
        self.0 >> 23 & 1 == 1
    }

    pub const fn selector(self) -> u32 {
        self.0 & 0x00ffffff
    }

    /// Index of the GaitemIns inside of the CSGaitemImp
    pub const fn index(self) -> u32 {
        self.0 & 0xffff
    }

    pub const fn category(self) -> Result<GaitemCategory, GaitemHandleError> {
        GaitemCategory::from_u8(&((self.0 >> 28 & 7) as u8))
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

impl GaitemCategory {
    pub const fn from_u8(val: &u8) -> Result<Self, GaitemHandleError> {
        Ok(match val {
            0 => GaitemCategory::Weapon,
            1 => GaitemCategory::Protector,
            2 => GaitemCategory::Accessory,
            3 => GaitemCategory::Goods,
            4 => GaitemCategory::Gem,
            _ => return Err(GaitemHandleError::InvalidCategory(*val)),
        })
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
