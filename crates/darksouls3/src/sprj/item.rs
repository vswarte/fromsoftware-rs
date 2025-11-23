use std::alloc::{alloc_zeroed, Layout, LayoutError};
use std::{convert::TryFrom, ffi, fmt, iter::zip, marker::PhantomData, ops, ptr, sync::LazyLock};

use pelite::{pattern, pattern::Atom, pe64::Pe};
use thiserror::Error;

use shared::{
    util::IncompleteArrayField, FromStatic, InstanceError, InstanceResult, OwnedPtr, Program,
    RecurringTask, SharedTaskImp,
};

use crate::rva;

#[derive(Error, Debug, Copy, Clone)]
#[error("not a valid category ID: {0}")]
pub struct TryFromItemCategoryError(u8);

/// All categories of items that are present in DS3.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ItemCategory {
    Weapon = 0,
    Protector = 1,
    Accessory = 2,
    GemAsGoods = 3,
    Goods = 4,
    Gem = 8,
}

impl TryFrom<u8> for ItemCategory {
    type Error = TryFromItemCategoryError;

    fn try_from(value: u8) -> Result<ItemCategory, Self::Error> {
        match value {
            0 => Ok(ItemCategory::Weapon),
            1 => Ok(ItemCategory::Protector),
            2 => Ok(ItemCategory::Accessory),
            3 => Ok(ItemCategory::GemAsGoods),
            4 => Ok(ItemCategory::Goods),
            8 => Ok(ItemCategory::Gem),
            _ => Err(TryFromItemCategoryError(value)),
        }
    }
}

#[derive(Error, Debug, Copy, Clone)]
#[error("not a valid item ID: {0}")]
pub struct TryFromItemIDError(u32);

/// An item ID that includes category information in the higher bits.
///
/// A [CategorizedItemID] is considered valid if its upper 4 bits are used
/// exclusively to represent a valid [ItemCategory]. DS3 does sometimes use
/// other sentinel values (such as -1), but these should be represented as enums
/// instead.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CategorizedItemID(u32);

impl CategorizedItemID {
    /// Creates a new [CategorizedItemID] from a [u32], or returns [None] if
    /// it has any of the four high bits is set.
    pub fn try_new(
        category: ItemCategory,
        uncategorized: u32,
    ) -> Result<CategorizedItemID, TryFromItemIDError> {
        let base: UncategorizedItemID = uncategorized.try_into()?;
        Ok(base.categorize(category))
    }

    /// Creates a new [CategorizedItemID] from a raw ID. This panics if [id] is
    /// not a valid [CategorizedItemID].
    ///
    /// This is intended for creating new constant IDs where you know ahead of
    /// time that the ID is valid. [try_new] or `try_from` should be used when
    /// constructing an ID at runtime, since that could fail.
    pub const fn new_const(id: u32) -> Self {
        let result = CategorizedItemID(id);
        result.category(); // Panic if the category isn't valid
        result
    }

    /// Returns the numeric value of this ID, including its category
    /// information.
    pub const fn value(&self) -> u32 {
        self.0
    }

    /// Returns this ID's category.
    pub const fn category(&self) -> ItemCategory {
        use ItemCategory::*;
        match self.0 & 0xF0000000 {
            0x00000000 => Weapon,
            0x10000000 => Protector,
            0x20000000 => Accessory,
            0x30000000 => GemAsGoods,
            0x40000000 => Goods,
            0x80000000 => Gem,
            _ => panic!("Unknown category ID"),
        }
    }

    /// Returns the uncategorized portion of this item's ID.
    pub const fn uncategorized(&self) -> UncategorizedItemID {
        UncategorizedItemID(self.0 & 0x0FFFFFFF)
    }
}

impl TryFrom<u32> for CategorizedItemID {
    type Error = TryFromItemIDError;

    fn try_from(value: u32) -> Result<CategorizedItemID, Self::Error> {
        // Verify that the top 4 bits are a valid category before constructing
        // the ID.
        let Ok(byte): Result<u8, _> = (value >> 28).try_into() else {
            return Err(TryFromItemIDError(value));
        };
        let Ok(_): Result<ItemCategory, _> = byte.try_into() else {
            return Err(TryFromItemIDError(value));
        };

        Ok(CategorizedItemID(value))
    }
}

impl fmt::Debug for CategorizedItemID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}:{}", self.category(), self.uncategorized().value())
    }
}

/// An item ID that doesn't include category information in the higher bits.
///
/// An [UncategorizedItemID] is considered valid if its upper 4 bits are all 0.
/// DS3 does sometimes use other sentinel values (such as -1), but these should
/// be represented as enums instead.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct UncategorizedItemID(u32);

impl UncategorizedItemID {
    /// Returns the numeric value of this ID. This can be used as an index into
    /// the various item params.
    pub const fn value(&self) -> u32 {
        self.0
    }

    /// Embeds [category] into this ID and returns the result.
    pub const fn categorize(&self, category: ItemCategory) -> CategorizedItemID {
        CategorizedItemID(self.0 | ((category as u32) << 28))
    }
}

impl TryFrom<u32> for UncategorizedItemID {
    type Error = TryFromItemIDError;

    fn try_from(uncategorized: u32) -> Result<UncategorizedItemID, Self::Error> {
        if uncategorized & 0xF0000000 == 0 {
            Ok(UncategorizedItemID(uncategorized))
        } else {
            Err(TryFromItemIDError(uncategorized))
        }
    }
}

#[repr(C)]
pub struct MapItemMan {
    // TODO: actual data
}

static MAP_ITEM_MAN_PTR_VA: LazyLock<Option<u64>> = LazyLock::new(|| {
    Program::current()
        .rva_to_va(rva::get().map_item_man_ptr)
        .ok()
});

impl FromStatic for MapItemMan {
    /// Returns the singleton instance of `MapItemMan`.
    unsafe fn instance() -> InstanceResult<&'static mut Self> {
        let Some(va) = *MAP_ITEM_MAN_PTR_VA else {
            return Err(InstanceError::NotFound);
        };
        let pointer = *(va as *const *mut Self);
        unsafe { pointer.as_mut() }.ok_or(InstanceError::Null)
    }
}

/// The address for the function call that grants an item to the player with a
/// visible popup. Callers can use this to hook the function for their needs.
///
/// The C signature of this function is:
///
/// ```c
/// void HookedItemGib(MapItemMan* man, ItemBuffer* buffer, int32_t* unknown);
/// ```
pub static MAP_ITEM_MAN_GRANT_ITEM_VA: LazyLock<u64> = LazyLock::new(|| {
    Program::current()
        .rva_to_va(rva::get().map_item_man_grant_item)
        .expect("Call target for MAP_ITEM_MAN_GRANT_ITEM_VA was not in exe")
});

impl MapItemMan {
    /// Grants the player the single [item], with an on-screen pop-up indicating
    /// that they received it.
    pub fn grant_item(&self, item: impl Into<ItemBufferEntry>) {
        let array = ItemArray::new([item.into()]);
        self.grant_items(&array);
    }

    /// Grants the player the given [items], with an on-screen pop-up indicating
    /// that they received them.
    pub fn grant_items(&self, items: impl AsRef<ItemBuffer>) {
        let grant_items: extern "C" fn(&MapItemMan, &ItemBuffer, &i32) =
            unsafe { std::mem::transmute(*MAP_ITEM_MAN_GRANT_ITEM_VA) };

        // We don't know what this does exactly, but we do know that setting it
        // to 1 causes it to be ignored.
        let unknown = 1;
        grant_items(self, items.as_ref(), &unknown);
    }
}

/// A set of items granted to the player.
#[repr(C)]
pub struct ItemBuffer {
    /// The number of items granted.
    length: u32,

    /// Information about the items themselves.
    items: IncompleteArrayField<ItemBufferEntry>,
}

impl ItemBuffer {
    /// Creates a new ItemBuffer with the given contents.
    ///
    /// The returned buffer's entries begin zeroed out.
    pub fn new(length: u32) -> Box<Self> {
        let layout = Self::layout(length.try_into().unwrap()).unwrap();
        // Safety: We're allocating directly from the standard allocator into a
        // Box.
        let mut buffer = unsafe { Box::from_raw(alloc_zeroed(layout) as *mut ItemBuffer) };
        buffer.length = length;
        buffer
    }

    /// Returns the memory layout for an [ItemBuffer] with the given number of
    /// elements.
    fn layout(length: usize) -> Result<Layout, LayoutError> {
        let layout = Layout::new::<ItemBuffer>();
        let (layout, _) = layout.extend(Layout::array::<ItemBufferEntry>(length)?)?;
        Ok(layout.pad_to_align())
    }

    /// Returns the list of items in the buffer as a slice.
    pub fn as_slice(&self) -> &[ItemBufferEntry] {
        // Safety: We trust the game to report lengths accurately.
        unsafe { self.items.as_slice(self.length.try_into().unwrap()) }
    }

    /// Returns the list of items in the buffer as a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [ItemBufferEntry] {
        // Safety: We trust the game to report lengths accurately.
        unsafe { self.items.as_mut_slice(self.length.try_into().unwrap()) }
    }

    /// Removes the entry at [index] from the buffer, shifting all elements
    /// after it to the left.
    ///
    /// ## Panics
    ///
    /// Panics if [index] is out-of-bounds.
    pub fn remove(&mut self, index: usize) -> ItemBufferEntry {
        let len = self.length as usize;
        if index >= len {
            panic!("removal index (is {index}) should be < len (is {len})");
        }

        // Implementation copied from Rust's Vec.
        unsafe {
            // infallible
            let ret;
            {
                // the place we are taking from.
                let ptr = self.as_mut_ptr().add(index);
                // copy it out, unsafely having a copy of the value on
                // the stack and in the vector at the same time.
                ret = ptr::read(ptr);

                // Shift everything down to fill in that spot.
                ptr::copy(ptr.add(1), ptr, len - index - 1);
            }
            self.length = (len - 1) as u32;
            ret
        }
    }
}

impl From<&[ItemBufferEntry]> for Box<ItemBuffer> {
    fn from(items: &[ItemBufferEntry]) -> Box<ItemBuffer> {
        let mut buffer = ItemBuffer::new(items.len().try_into().unwrap());
        buffer.as_mut_slice().clone_from_slice(items);
        buffer
    }
}

impl ops::Deref for ItemBuffer {
    type Target = [ItemBufferEntry];

    fn deref(&self) -> &[ItemBufferEntry] {
        self.as_slice()
    }
}

impl ops::DerefMut for ItemBuffer {
    fn deref_mut(&mut self) -> &mut [ItemBufferEntry] {
        self.as_mut_slice()
    }
}

impl fmt::Debug for ItemBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.as_slice().fmt(f)
    }
}

/// A single item granted to the player.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ItemBufferEntry {
    /// The ID of the item that was granted.
    pub id: CategorizedItemID,

    /// The number of this item that the player received.
    pub quantity: u32,

    /// The durability of the item being granted. -1 means full durability.
    pub durability: i32,
}

impl From<CategorizedItemID> for ItemBufferEntry {
    /// Creates an [ItemBufferEntry] containing a single full-durability item
    /// with this ID.
    fn from(id: CategorizedItemID) -> ItemBufferEntry {
        ItemBufferEntry {
            id,
            quantity: 1,
            durability: -1,
        }
    }
}

/// A fixed-size type whose references are convertible to [ItemBuffer], so it
/// can be stack-allocated.
#[repr(C)]
pub struct ItemArray<const N: usize> {
    /// The number of items granted.
    length: u32,

    /// Information about the items themselves.
    pub items: [ItemBufferEntry; N],
}

impl<const N: usize> ItemArray<N> {
    /// Creates a new fixed-size item buffer with the given contents.
    #[inline]
    pub fn new(items: [ItemBufferEntry; N]) -> Self {
        ItemArray {
            length: N.try_into().unwrap(),
            items,
        }
    }
}

impl<const N: usize> fmt::Debug for ItemArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.items.fmt(f)
    }
}

impl<const N: usize> AsRef<ItemBuffer> for ItemArray<N> {
    fn as_ref(&self) -> &ItemBuffer {
        let pointer: *const ItemArray<N> = ptr::from_ref(self);

        // Safety: We know the pointer is valid because we just crated it, and
        // ItemArray is defined to have a valid memory layout for ItemBuffer.
        // The unbounded lifetime we create here is cast to the narrower self
        // lifetime upon return.
        unsafe { &*(pointer as *const ItemBuffer) }
    }
}
