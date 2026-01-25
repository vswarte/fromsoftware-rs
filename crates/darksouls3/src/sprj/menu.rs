use std::{borrow::Cow, ptr::NonNull};

use pelite::pe64::Pe;
use shared::{FromStatic, InstanceResult, OwnedPtr, Program};

use super::{ItemCategoryHigh, ItemId};
use crate::rva;

// Source of name: RTTI
#[repr(C)]
pub struct ItemGetMenuMan {
    _vftable: usize,
    pub used_display: Option<NonNull<ItemGetMenuManDisplay>>,
    pub next_unused_display: Option<NonNull<ItemGetMenuManDisplay>>,
    pub all_displays: OwnedPtr<[ItemGetMenuManDisplay; 8]>,
    _unk20: u64,
}

impl ItemGetMenuMan {
    /// Displays a pop-up indicating that the player has received `item` without
    /// actually placing it in their inventory.
    ///
    /// If `in_box` is true, the item will be shown as going into the player's
    /// box.
    pub fn show_item(&mut self, item: ItemId, quantity: u32, in_box: bool) {
        let va = Program::current()
            .rva_to_va(rva::get().item_get_menu_man_show_item)
            .unwrap();
        let show_item: extern "C" fn(&mut ItemGetMenuMan, ItemCategoryHigh, u32, u32, bool) =
            unsafe { std::mem::transmute(va) };

        show_item(
            self,
            item.category().into(),
            item.param_id(),
            quantity,
            in_box,
        );
    }
}

impl FromStatic for ItemGetMenuMan {
    fn name() -> Cow<'static, str> {
        "ItemGetMan".into()
    }

    unsafe fn instance() -> InstanceResult<&'static mut Self> {
        unsafe { shared::load_static_indirect(rva::get().item_get_menu_man_ptr) }
    }
}

#[repr(C)]
pub struct ItemGetMenuManDisplay {
    pub next: Option<NonNull<ItemGetMenuManDisplay>>,

    /// The category of item to display.
    pub category: ItemCategoryHigh,

    /// The ID of the item being displayed.
    pub item_id: u32,

    /// The number of items to display.
    pub quantity: u32,

    /// Whether the item is displayed as going to the player's box.
    pub in_box: bool,
}
