use std::alloc::{Layout, LayoutError, alloc_zeroed};
use std::{borrow::Cow, fmt, ops, ptr, sync::LazyLock};

use pelite::pe64::Pe;

use shared::{FromStatic, InstanceError, InstanceResult, Program, util::IncompleteArrayField};

use super::ItemId;
use crate::rva;

#[repr(C)]
pub struct MapItemMan {
    // TODO: actual data
}

impl FromStatic for MapItemMan {
    fn name() -> Cow<'static, str> {
        "MapItemMan".into()
    }

    /// Returns the singleton instance of `MapItemMan`.
    unsafe fn instance() -> InstanceResult<&'static mut Self> {
        let target = Program::current()
            .rva_to_va(rva::get().map_item_man_ptr)
            .map_err(|_| InstanceError::NotFound)? as *const *mut Self;

        unsafe { (*target).as_mut() }.ok_or(InstanceError::Null)
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
    /// Grants the player the single `item`, with an on-screen pop-up indicating
    /// that they received it.
    pub fn grant_item(&self, item: impl Into<ItemBufferEntry>) {
        let array = ItemArray::new([item.into()]);
        self.grant_items(&array);
    }

    /// Grants the player the given `items`, with an on-screen pop-up indicating
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
#[derive(Debug, Clone)]
pub struct ItemBufferEntry {
    /// The ID of the item that was granted.
    pub id: ItemId,

    /// The number of this item that the player received.
    pub quantity: u32,

    /// The durability of the item being granted. -1 means full durability.
    pub durability: i32,
}

impl From<ItemId> for ItemBufferEntry {
    /// Creates an [ItemBufferEntry] containing a single full-durability item
    /// with this ID.
    fn from(id: ItemId) -> ItemBufferEntry {
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
