use std::fmt::Display;

use bitfield::bitfield;
use thiserror::Error;

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ItemId(i32);
    impl Debug;

    i32;
    /// The raw item ID value, without the category.
    pub param_id_raw, _: 27, 0;
    _, set_param_id_raw: 27, 0;

    u8;
    /// The raw category value.
    pub category_raw, _: 31, 28;
    _, set_category_raw: 31, 28;
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ItemIdError {
    #[error("Not a valid item category {0}")]
    InvalidCategory(u8),
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemCategory {
    Weapon = 0,
    Protector = 1,
    Accessory = 2,
    Goods = 4,
    Gem = 8,
    None = u8::MAX,
}

impl ItemCategory {
    pub const fn from_u8(val: u8) -> Result<Self, ItemIdError> {
        Ok(match val {
            0 => ItemCategory::Weapon,
            1 => ItemCategory::Protector,
            2 => ItemCategory::Accessory,
            4 => ItemCategory::Goods,
            8 => ItemCategory::Gem,
            15 | u8::MAX => ItemCategory::None,
            _ => return Err(ItemIdError::InvalidCategory(val)),
        })
    }
}

impl ItemId {
    pub fn from_parts(param_id: i32, category: ItemCategory) -> Self {
        let mut id = ItemId(0);
        id.set_param_id_raw(param_id);
        id.set_category_raw(category as u8);
        id
    }

    pub fn param_id(&self) -> i32 {
        if self.0 == -1 {
            return -1;
        }
        self.param_id_raw()
    }

    pub fn category(&self) -> Result<ItemCategory, ItemIdError> {
        ItemCategory::from_u8(self.category_raw())
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
                write!(f, "ItemId({},{:?})", self.param_id(), category)
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
        item.set_param_id_raw(123);
        item.set_category_raw(ItemCategory::Gem as u8);

        assert_eq!(item.param_id(), 123);
        assert_eq!(item.category(), Ok(ItemCategory::Gem));
        assert_eq!(item.0, 123 | (ItemCategory::Gem as i32) << 28);

        item = ItemId(-1);
        assert_eq!(item.param_id(), -1);
        assert_eq!(item.category(), Ok(ItemCategory::None));
    }
}
