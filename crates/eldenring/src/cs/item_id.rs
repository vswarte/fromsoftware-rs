use std::fmt::Display;

use bitfield::{bitfield, BitRange};
use thiserror::Error;

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ItemId(i32);
    impl Debug;

    i32;
    /// The raw item ID value, without the category.
    pub item_id_raw, _: 27, 0;
    _, set_item_id_raw: 27, 0;

    i8;
    /// The raw category value.
    pub category_raw, _: 31, 28;
    _, set_category_raw: 31, 28;
}

#[derive(Debug, Error, PartialEq, Eq)]
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
    pub const fn from_i8(val: i8) -> Result<Self, ItemIdError> {
        Ok(match val {
            0 => ItemCategory::Weapon,
            1 => ItemCategory::Protector,
            2 => ItemCategory::Accessory,
            4 => ItemCategory::Goods,
            8 => ItemCategory::Gem,
            15 | -1 => ItemCategory::None,
            _ => return Err(ItemIdError::InvalidCategory(val)),
        })
    }
}

impl ItemId {
    pub fn from_parts(item_id: i32, category: ItemCategory) -> Self {
        let mut id = ItemId(0);
        id.set_item_id_raw(item_id);
        id.set_category_raw(category as i8);
        id
    }

    pub fn item_id(&self) -> i32 {
        if self.0 == -1 {
            return -1;
        }
        self.item_id_raw()
    }

    pub fn category(&self) -> Result<ItemCategory, ItemIdError> {
        ItemCategory::from_i8(self.category_raw())
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

#[cfg(test)]
mod tests {
    use crate::cs::{ItemCategory, ItemId};

    #[test]
    fn test_bitfield() {
        let mut item = ItemId(0);
        item.set_item_id_raw(123);
        item.set_category_raw(ItemCategory::Weapon as i8);

        assert_eq!(item.item_id(), 123);
        assert_eq!(item.category(), Ok(ItemCategory::Weapon));
        assert_eq!(item.0, 123 | (ItemCategory::Weapon as i32) << 28);
    }
}
