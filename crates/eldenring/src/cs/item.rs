use std::alloc::{Layout, LayoutError, alloc_zeroed};
use std::ffi::c_void;
use std::{fmt, ops, ptr, sync::LazyLock};

use pelite::{pattern, pe64::Pe};
use shared::{Program, util::IncompleteArrayField};
use thiserror::Error;

use super::ItemId;

const ITEM_GIVE_PATTERN: &str = "8B 02 83 F8 0A";
const ITEM_GIVE_PATTERN_OFFSET: u32 = 0x52;

/// The maximum number of entries ER's ItemGive path accepts in one call.
pub const MAX_ITEMS_PER_GRANT: usize = 10;

#[shared::singleton("MapItemMan")]
pub struct MapItemMan {
    // TODO: actual data
}

/// The address for the function call that grants an item to the player with a
/// visible popup. Callers can use this to hook the function for their needs.
///
/// The C signature of this function is:
///
/// ```c
/// void ItemGive(MapItemMan* man, ItemBuffer* buffer, ItemGrantCallData* data, uint32_t flags);
/// ```
///
/// This is currently found by a private AOB lookup rather than a generated RVA
/// because a stable RVA for the supported executable has not been validated.
pub static MAP_ITEM_MAN_GRANT_ITEM_VA: LazyLock<u64> = LazyLock::new(|| {
    map_item_man_grant_item_va().expect("Call target for MAP_ITEM_MAN_GRANT_ITEM_VA was not in exe")
});

static MAP_ITEM_MAN_GRANT_ITEM_VA_RESULT: LazyLock<Result<u64, String>> =
    LazyLock::new(|| find_item_give_va().map_err(|err| err.to_string()));

/// An error returned when ER's ItemGive path cannot be called.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ItemGrantError {
    /// The executable scan could not find a unique ItemGive function.
    #[error("{0}")]
    Lookup(String),

    /// More items were passed than ER accepts in one ItemGive call.
    #[error("ER ItemGive accepts at most {max} entries per call, got {actual}")]
    TooManyItems { max: usize, actual: usize },
}

/// Returns the address of ER's ItemGive function.
pub fn map_item_man_grant_item_va() -> Result<u64, ItemGrantError> {
    MAP_ITEM_MAN_GRANT_ITEM_VA_RESULT
        .as_ref()
        .copied()
        .map_err(|err| ItemGrantError::Lookup(err.clone()))
}

fn find_item_give_va() -> Result<u64, String> {
    let scanner_pattern = pattern::parse(ITEM_GIVE_PATTERN)
        .map_err(|_| "failed to parse ER ItemGive AOB pattern".to_string())?;
    let program = Program::current();
    let mut matches = program.scanner().matches_code(&scanner_pattern);
    let mut captures = [0u32; 1];
    if !matches.next(&mut captures) {
        return Err("could not find ER ItemGive function in executable".to_string());
    }

    let mut duplicate = [0u32; 1];
    if matches.next(&mut duplicate) {
        return Err("ER ItemGive AOB pattern matched more than one function".to_string());
    }

    let rva = captures[0]
        .checked_sub(ITEM_GIVE_PATTERN_OFFSET)
        .ok_or("ER ItemGive AOB match appeared before expected function start")?;
    program
        .rva_to_va(rva)
        .map_err(|err| format!("failed to convert ER ItemGive RVA {rva:#x}: {err:?}"))
}

impl MapItemMan {
    /// Grants the player the single `item`, with an on-screen pop-up indicating
    /// that they received it.
    ///
    /// Panics if ER's ItemGive function cannot be found or called. Use
    /// [try_grant_item](Self::try_grant_item) to handle lookup failure.
    pub fn grant_item(&mut self, item: impl Into<ItemBufferEntry>) {
        self.try_grant_item(item)
            .expect("failed to grant item through ER ItemGive")
    }

    /// Grants the player the single `item`, with an on-screen pop-up indicating
    /// that they received it.
    pub fn try_grant_item(
        &mut self,
        item: impl Into<ItemBufferEntry>,
    ) -> Result<(), ItemGrantError> {
        let array = ItemArray::new([item.into()]);
        self.try_grant_items(&array)
    }

    /// Grants the player the given `items`, with an on-screen pop-up indicating
    /// that they received them.
    ///
    /// Panics if ER's ItemGive function cannot be found or called. Use
    /// [try_grant_items](Self::try_grant_items) to handle lookup failure.
    pub fn grant_items(&mut self, items: impl AsRef<ItemBuffer>) {
        self.try_grant_items(items)
            .expect("failed to grant items through ER ItemGive")
    }

    /// Grants the player the given `items`, with an on-screen pop-up indicating
    /// that they received them.
    pub fn try_grant_items(&mut self, items: impl AsRef<ItemBuffer>) -> Result<(), ItemGrantError> {
        let items = items.as_ref();
        let len = items.len();
        if len > MAX_ITEMS_PER_GRANT {
            return Err(ItemGrantError::TooManyItems {
                max: MAX_ITEMS_PER_GRANT,
                actual: len,
            });
        }

        let mut data = ItemGrantCallData::new(items.as_slice());
        let grant_items: unsafe extern "C" fn(
            *mut c_void,
            *const ItemBuffer,
            *mut ItemGrantCallData<MAX_ITEMS_PER_GRANT>,
            u32,
        ) = unsafe { std::mem::transmute(map_item_man_grant_item_va()?) };

        unsafe {
            grant_items(
                self as *mut Self as *mut c_void,
                data.buffer.as_ref(),
                &mut data,
                0,
            );
        }
        Ok(())
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
    /// Creates a new ItemBuffer with the given length.
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

    /// Returns the number of entries in the buffer.
    pub fn len(&self) -> usize {
        self.length.try_into().unwrap()
    }

    /// Returns whether the buffer has no entries.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the list of items in the buffer as a slice.
    pub fn as_slice(&self) -> &[ItemBufferEntry] {
        // Safety: We trust the game to report lengths accurately.
        unsafe { self.items.as_slice(self.len()) }
    }

    /// Returns the list of items in the buffer as a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [ItemBufferEntry] {
        // Safety: We trust the game to report lengths accurately.
        unsafe { self.items.as_mut_slice(self.len()) }
    }

    /// Removes the entry at `index` from the buffer, shifting all elements
    /// after it to the left.
    ///
    /// ## Panics
    ///
    /// Panics if `index` is out-of-bounds.
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemBufferEntry {
    /// The ID of the item that was granted.
    pub id: ItemId,

    /// The number of this item that the player received.
    pub quantity: u32,

    /// Legacy durability field. `u32::MAX` means full durability.
    pub durability: u32,

    /// Gem or Ash of War field. `u32::MAX` means no gem.
    pub gem: u32,
}

impl ItemBufferEntry {
    /// Creates an [ItemBufferEntry] containing `quantity` full-durability items
    /// with this ID.
    pub fn new(id: ItemId, quantity: u32) -> Self {
        Self {
            id,
            quantity,
            durability: u32::MAX,
            gem: u32::MAX,
        }
    }

    fn empty() -> Self {
        Self {
            id: ItemId::try_from(0).expect("weapon 0 should be a valid item ID"),
            quantity: 0,
            durability: u32::MAX,
            gem: u32::MAX,
        }
    }
}

impl From<ItemId> for ItemBufferEntry {
    /// Creates an [ItemBufferEntry] containing a single full-durability item
    /// with this ID.
    fn from(id: ItemId) -> ItemBufferEntry {
        ItemBufferEntry::new(id, 1)
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

    fn with_length(items: [ItemBufferEntry; N], length: usize) -> Self {
        assert!(length <= N);
        ItemArray {
            length: length.try_into().unwrap(),
            items,
        }
    }
}

#[repr(C)]
struct ItemGrantCallData<const N: usize> {
    scratch: [u8; 0x20],
    buffer: ItemArray<N>,
}

impl ItemGrantCallData<MAX_ITEMS_PER_GRANT> {
    fn new(items: &[ItemBufferEntry]) -> Self {
        assert!(items.len() <= MAX_ITEMS_PER_GRANT);

        let mut buffer_items = [ItemBufferEntry::empty(); MAX_ITEMS_PER_GRANT];
        buffer_items[..items.len()].copy_from_slice(items);
        Self {
            scratch: [0; 0x20],
            buffer: ItemArray::with_length(buffer_items, items.len()),
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

        // Safety: We know the pointer is valid because we just created it, and
        // ItemArray is defined to have a valid memory layout for ItemBuffer.
        // The unbounded lifetime we create here is cast to the narrower self
        // lifetime upon return.
        unsafe { &*(pointer as *const ItemBuffer) }
    }
}

impl<const N: usize> AsMut<ItemBuffer> for ItemArray<N> {
    fn as_mut(&mut self) -> &mut ItemBuffer {
        let pointer: *mut ItemArray<N> = ptr::from_mut(self);

        // Safety: See the AsRef implementation above.
        unsafe { &mut *(pointer as *mut ItemBuffer) }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::{offset_of, size_of};

    use super::*;
    use crate::cs::ItemCategory;

    #[test]
    fn item_give_layout_matches_er_buffer() {
        assert_eq!(size_of::<ItemBufferEntry>(), 0x10);
        assert_eq!(offset_of!(ItemBuffer, items), 0x4);
        assert_eq!(offset_of!(ItemArray<1>, items), 0x4);
    }

    #[test]
    fn item_entry_defaults_to_full_legacy_durability() {
        let id = ItemId::new(ItemCategory::Goods, 100).unwrap();
        let entry = ItemBufferEntry::from(id);

        assert_eq!(entry.id, id);
        assert_eq!(entry.quantity, 1);
        assert_eq!(entry.durability, u32::MAX);
        assert_eq!(entry.gem, u32::MAX);
    }

    #[test]
    fn item_array_references_as_item_buffer() {
        let id = ItemId::new(ItemCategory::Goods, 100).unwrap();
        let array = ItemArray::new([ItemBufferEntry::new(id, 3)]);
        let buffer = array.as_ref();

        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.as_slice()[0].id, id);
        assert_eq!(buffer.as_slice()[0].quantity, 3);
    }

    #[test]
    fn item_grant_call_data_matches_er_call_frame() {
        let id = ItemId::new(ItemCategory::Goods, 100).unwrap();
        let data = ItemGrantCallData::new(&[ItemBufferEntry::new(id, 3)]);
        let buffer = data.buffer.as_ref();

        assert_eq!(offset_of!(ItemGrantCallData<1>, buffer), 0x20);
        assert_eq!(buffer.len(), 1);
        assert_eq!(buffer.as_slice()[0].id, id);
        assert_eq!(buffer.as_slice()[0].quantity, 3);
    }
}
