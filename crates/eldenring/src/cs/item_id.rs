use std::{fmt, mem};

use bitfield::bitfield;
use thiserror::Error;

/// An error indicating that a valid [ItemId] or [ItemCategory] couldn't be
/// constructed.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ItemIdError {
    /// The category, or the bits in the ID representing the category, is
    /// invalid. This is always the error used when interpreting a [u32]
    /// directly as an [ItemId].
    #[error("Invalid item category {0}")]
    InvalidCategory(u8),

    /// The parameter ID isn't valid. Because the only valid parameter IDs are
    /// those whose 1 bits overlap with the category, this error is only used
    /// when supplying a parameter ID separately from a category.
    #[error("Invalid param ID {0}")]
    InvalidParamId(u32),
}

/// All item categories that correspond to an `EQUIP_PARAM_*` parameter table.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemCategory {
    Weapon = 0,
    Protector = 1,
    Accessory = 2,
    Goods = 4,
    Gem = 8,
}

impl TryFrom<u8> for ItemCategory {
    type Error = ItemIdError;

    fn try_from(value: u8) -> Result<ItemCategory, Self::Error> {
        Ok(match value {
            0 => ItemCategory::Weapon,
            1 => ItemCategory::Protector,
            2 => ItemCategory::Accessory,
            4 => ItemCategory::Goods,
            8 => ItemCategory::Gem,
            _ => return Err(ItemIdError::InvalidCategory(value)),
        })
    }
}

bitfield! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct OptionalItemId(u32);

    u32;
    /// The raw item ID value, without the category.
    param_id_raw, set_param_id_raw: 27, 0;

    u8;
    /// The raw category value.
    category_raw, set_category_raw: 31, 28;
}

/// An item ID that includes category information in the higher bits, but may
/// also be the special invalid ID [NONE](Self::NONE) that represents no item.
///
/// This is often used by the game to represent items in situations where
/// they're optional or uninitialized. It can be safely converted to a valid
/// [ItemId] using [as_valid](Self::as_valid).
impl OptionalItemId {
    /// The ID used by the game to indicate the absence of an item.
    ///
    /// It's generally expected that all invalid item IDs are this value, since
    /// the game uses it consistently and the Rust library normalizes invalid
    /// IDs. Don't assume that this is the case, though, because there could be
    /// unexpected situations where the game uses different invalid IDs.
    pub const NONE: OptionalItemId = OptionalItemId(u32::MAX);

    /// If this is a valid ID, returns it as an [ItemId]. Otherwise, returns
    /// `None`.
    pub fn as_valid(&self) -> Option<ItemId> {
        if self.is_valid() {
            Some(ItemId(*self))
        } else {
            None
        }
    }

    /// Whether this represents a valid [ItemId]. In most cases, it's better to
    /// use [as_valid](Self::as_valid) to check this and get the valid value at
    /// once.
    pub fn is_valid(&self) -> bool {
        self.category().is_some()
    }

    /// If this is valid, returns its parameter ID. Otherwise, returns `None`.
    pub fn param_id(&self) -> Option<u32> {
        if self.is_valid() {
            Some(self.param_id_raw())
        } else {
            None
        }
    }

    /// If this is valid, returns its category. Otherwise, returns `None`.
    pub fn category(&self) -> Option<ItemCategory> {
        ItemCategory::try_from(self.category_raw()).ok()
    }

    /// Returns the underlying numeric value of the item ID.
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

impl From<u32> for OptionalItemId {
    /// Converts a [u32] into a [OptionalItemId].
    ///
    /// This normalizes all invalid item IDs into the single
    /// [OptionalItemId::NONE] value.
    fn from(value: u32) -> Self {
        let id = Self(value);
        if id.is_valid() { id } else { Self::NONE }
    }
}

impl From<ItemId> for OptionalItemId {
    fn from(value: ItemId) -> OptionalItemId {
        value.0
    }
}

impl fmt::Debug for OptionalItemId {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        match ItemCategory::try_from(self.category_raw()) {
            Ok(category) => {
                write!(f, "ItemId({}, {:?})", self.param_id_raw(), category)
            }
            Err(err) => write!(f, "ItemId(0x{:x}, {:?})", self.0, err),
        }
    }
}

/// An item ID that includes category information in the higher bits and is
/// guaranteed to be valid.
///
/// "Valid" in this case means that the category is known to be a meaningful
/// item category that corresponds to a parameter table. It doesn't guarantee
/// that a parameter exists at the given parameter ID.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ItemId(OptionalItemId);

impl ItemId {
    /// Creates a new [ItemId] from the given category and param ID. Returns an
    /// [ItemIdError] if [param_id](Self::param_id) is greater than 0xFFFFFFF.
    pub const fn new(category: ItemCategory, param_id: u32) -> Result<Self, ItemIdError> {
        if param_id > 0xFFFFFFF {
            Err(ItemIdError::InvalidParamId(param_id))
        } else {
            Ok(Self(OptionalItemId(((category as u32) << 28) | param_id)))
        }
    }

    /// Returns this ID's category.
    pub fn category(&self) -> ItemCategory {
        // Safety: We check that the category is valid at all API boundaries.
        unsafe { mem::transmute(self.0.category_raw()) }
    }

    /// Returns the parameter ID of the item this ID represents. This is the ID
    /// of the row in the parameter struct that corresponds to
    /// [category](Self::category).
    pub fn param_id(&self) -> u32 {
        self.0.param_id_raw()
    }

    /// Returns the underlying numeric value of the item ID.
    pub fn into_inner(self) -> u32 {
        self.0.into_inner()
    }
}

impl TryFrom<u32> for ItemId {
    type Error = ItemIdError;

    fn try_from(value: u32) -> Result<ItemId, Self::Error> {
        ItemId::try_from(OptionalItemId(value))
    }
}

impl TryFrom<OptionalItemId> for ItemId {
    type Error = ItemIdError;

    fn try_from(value: OptionalItemId) -> Result<ItemId, Self::Error> {
        if value.is_valid() {
            // Safety: We just checked that the value is valid.
            Ok(Self(value))
        } else {
            Err(ItemIdError::InvalidCategory(value.category_raw()))
        }
    }
}

impl fmt::Debug for ItemId {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        // It shouldn't ever be possible to have an invalid category for an
        // [ItemId], but it could happen if a game API is assumed to only ever
        // have valid IDs but in fact does not, or if it's pointing to the wrong
        // memory entirely. We don't want to panic while creating a debug
        // string, so we call out to the inner formatter which handles that case
        // gracefully.
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::cs::{ItemCategory, OptionalItemId};

    #[test]
    fn test_bitfield() {
        let mut item = OptionalItemId(0);
        item.set_param_id_raw(123);
        item.set_category_raw(ItemCategory::Gem as u8);

        assert_eq!(item.param_id(), Some(123));
        assert_eq!(item.category(), Some(ItemCategory::Gem));
        assert_eq!(item.0, 123 | (ItemCategory::Gem as u32) << 28);

        item = OptionalItemId(u32::MAX);
        assert_eq!(item.param_id(), None);
        assert_eq!(item.category(), None);
    }
}
