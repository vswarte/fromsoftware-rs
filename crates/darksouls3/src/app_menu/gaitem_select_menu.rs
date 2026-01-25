use std::ptr::NonNull;

use shared::{OwnedPtr, Subclass, Superclass, UnknownStruct};

use super::{MenuWindow, MenuWindowCallback, SceneObjProxy};
use crate::{CxxVec, sprj::ItemId};

#[repr(C)]
#[derive(Superclass, Subclass)]
#[superclass(children(GaitemSelectMenu))]
// Source of name: RTTI
pub struct GaitemSelectBaseMenu {
    pub menu_window: MenuWindow,
    _unk9d8: SceneObjProxy,
    pub menu_bg: GaitemSelectBaseMenuBG,
    _unkc18: u64,
    _grid_control_1: GridControl,
    _grid_control_2: GridControl,
    _window_list_item_name: [u8; 0x60],
    _window_list_item_simple_status: [u8; 0x60],
    pub item_select_ctrl: ItemSelectCtrl,
    _unk1968: UnknownStruct<0x28>,
    _unk1990: UnknownStruct<0x80>,
    _unk1a10: CxxVec<u64>,
    pub detail_status_view: GaitemSelectDetailStatusView,
    _unk1ff0: UnknownStruct<0x90>,
}

type GridControl = UnknownStruct<0x638>;

#[repr(C)]
// Source of name: debug text
pub struct GaitemSelectBaseMenuBG {
    pub small: [u8; 0x60],
    pub large: [u8; 0x60],
    pub item_status: [u8; 0x60],
    pub item_detail_status: [u8; 0x60],
    pub player_status: [u8; 0x60],
}

#[repr(C)]
// Source of name: RTTI
pub struct ItemSelectCtrl {
    _vftable: usize,
    _unk08: u32,
    pub parent: NonNull<GaitemSelectBaseMenu>,
}

#[repr(C)]
// Source of name: debug text
pub struct GaitemSelectDetailStatusView {
    _callback1: MenuWindowCallback,
    _callback2: MenuWindowCallback,
    _callback3: MenuWindowCallback,
    _callback4: MenuWindowCallback,
    _unk80: SceneObjProxy,
    _status_uchiku: usize,
    _unke8: [u8; 8],
    _status_item: usize,
    _unkf8: [u8; 8],
    _status_player: u64,
    _unk108: [u8; 0x388],
    _unk490: u32,
    _unk494: [u8; 0x10c],
    _unk5a0: u64,
    _unk5a8: u8,
    _unk5b0: usize,
    _unk5b8: u16,
}

#[repr(C)]
#[derive(Subclass)]
#[subclass(base = GaitemSelectBaseMenu, base = MenuWindow)]
// Source of name: RTTI
pub struct GaitemSelectMenu {
    pub base: GaitemSelectBaseMenu,
    pub item_select_dialog: Option<OwnedPtr<ItemSelectDialog>>,
    _unk2088: Option<OwnedPtr<GaitemSelectMenu_0x2088>>,
}

impl GaitemSelectMenu {
    /// Returns an iterator across all items in this menu.
    pub fn items(&self) -> impl Iterator<Item = &MenuGaitem> {
        self.item_select_dialog
            .as_ref()
            .and_then(|i| i.items.as_ref())
            .map(|i| i.items.iter())
            .unwrap_or_default()
    }
}

#[repr(C)]
// Source of name: RTTI
pub struct ItemSelectDialog {
    _vftable: usize,
    pub ref_count: i32,
    _unk10: u64,
    _unk18: [u8; 0x10],
    _unk28: u64,
    _unk30: SceneObjProxy,
    _grid_control: GridControl,

    /// The items that the dialog allows the player to select between.
    pub items: Option<OwnedPtr<MenuGaitemList>>,
}

impl ItemSelectDialog {
    /// Returns an iterator across all items in this dialog.
    pub fn items(&self) -> impl Iterator<Item = &MenuGaitem> {
        self.items
            .as_ref()
            .map(|i| i.items.iter())
            .unwrap_or_default()
    }
}

#[repr(C)]
// Source of name: RTTI
pub struct MenuGaitemList {
    _vftable: usize,
    pub ref_count: i32,

    /// The items in this list.
    pub items: CxxVec<MenuGaitem>,
}

#[repr(C)]
// Source of name: RTTI
pub struct MenuGaitem {
    /// The number of instances of this item that are available. -1 indicates no
    /// limit.
    pub quantity: i32,

    _unk04: u16,
    _unk06: u8,
    _unk08: u32,

    /// The cost in souls of each instance of this item.
    pub price: u64,

    _unk18: CxxVec<u8>,
    _unk38: u32,

    /// This item's ID.
    pub id: ItemId,

    /// The item's category.
    pub category: MenuGaitemCategory,

    /// The ShopLineupParam ID for this item's shop entry.
    pub shop_lineup_param: u32,

    _unk48: Option<NonNull<usize>>,
    _unk50: u8,
    _unk51: [u8; 0x5f],
}

/// Categories of items in the item select menu. This is probably not yet
/// comprehensive.
#[repr(u32)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum MenuGaitemCategory {
    Tool = 1,
    Material = 2,
    Key = 3,
    Weapon = 4,
    Unk5 = 5,
    StaffOrTalisman = 6,
    ShieldOrTorch = 7,
    HeadArmor = 8,
    TorsoArmor = 9,
    HandArmor = 10,
    LegArmor = 11,
    Unk12 = 12,
    Unk13 = 13,
    Spell = 14,
    Arrow = 15,
    Bolt = 16,
}

#[repr(C)]
pub struct GaitemSelectMenu_0x2088 {
    _unk00: u64,
    _unk08: u64,
    _unk10: [u8; 0x10],
    _unk20: usize,
    _unk28: u8,
    _unk29: u8,
    _unk30: u64,
    _unk38: u64,
    _unk40: u64,
    _unk48: [u8; 0x10],
    _unk58: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0xb0, size_of::<MenuGaitem>());
        assert_eq!(0x30, size_of::<MenuGaitemList>());
        assert_eq!(0x1e0, size_of::<GaitemSelectBaseMenuBG>());
        assert_eq!(0x6d0, size_of::<ItemSelectDialog>());
        assert_eq!(0x18, size_of::<ItemSelectCtrl>());
        assert_eq!(0x5c0, size_of::<GaitemSelectDetailStatusView>());
        assert_eq!(0x2080, size_of::<GaitemSelectBaseMenu>());
        assert_eq!(0x60, size_of::<GaitemSelectMenu_0x2088>());
        assert_eq!(0x2090, size_of::<GaitemSelectMenu>());
    }
}
