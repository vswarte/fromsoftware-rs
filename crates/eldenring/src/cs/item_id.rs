use std::fmt::Display;

use thiserror::Error;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ItemId(pub i32);

#[derive(Debug, Error)]
pub enum ItemIdError {
    #[error("Not a valid item category {0}")]
    InvalidCategory(i8),
}

#[repr(i8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemCategory {
    Weapon = 0,
    Protector = 1,
    Accessory = 2,
    Goods = 4,
    Gem = 8,
    None = -1,
}

impl ItemCategory {
    pub const fn from_i8(val: &i8) -> Result<Self, ItemIdError> {
        Ok(match val {
            0 => ItemCategory::Weapon,
            1 => ItemCategory::Protector,
            2 => ItemCategory::Accessory,
            4 => ItemCategory::Goods,
            8 => ItemCategory::Gem,
            15 | -1 => ItemCategory::None,
            _ => return Err(ItemIdError::InvalidCategory(*val)),
        })
    }
}

impl ItemId {
    pub const fn from_parts(item_id: i32, category: ItemCategory) -> Self {
        Self((item_id & 0x0FFFFFFF) | ((category as i32) << 28))
    }

    pub const fn item_id(&self) -> i32 {
        if self.0 == -1 {
            return -1;
        }
        self.0 & 0x0FFFFFFF
    }

    pub const fn category(&self) -> Result<ItemCategory, ItemIdError> {
        ItemCategory::from_i8(&((self.0 as u32 >> 28) as i8))
    }
}

impl From<i32> for ItemId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Display for ItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.category() {
            Ok(category) => {
                write!(f, "ItemId({},{:?})", self.item_id(), category)
            }
            Err(err) => write!(f, "ItemId(0x{:x},{:?})", self.0, err),
        }
    }
}
