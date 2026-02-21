use std::{borrow::Cow, mem, ptr::NonNull};

use shared::empty::{IsEmpty, MaybeEmpty};
use shared::{FromStatic, UnknownStruct};

use super::{ItemCategoryHigh, ItemId};
use crate::{fd4::FD4Time, rva};

// Source of name: RTTI
#[repr(C)]
pub struct MenuMan {
    _vftable: usize,
    _unk08: u64,
    _unk10: u64,
    _unk18: u64,
    _unk20: u64,
    _unk28: u64,
    _unk30: u16,

    /// Various flags each with its own meaning. Known flags have accessor methods.
    pub flags: [i32; 500],

    _unk804: u32,
    _unk808: u32,
    _unk80c: u32,
    _unk810: [u8; 0x4],
    _unk814: u8,
    _unk818: u32,
    _unk81c: u32,
    _unk820: u32,
    _unk824: u32,
    _unk828: u8,
    _unk82c: u32,
    _unk830: u32,
    _unk834: u8,
    _unk838: u64,
    pub grant_item_command: MaybeEmpty<GrantItemCommand>,
    _unk850: u64,
    _unk858: u64,
    _unk860: u8,
    _unk864: u32,
    _unk868: u32,
    _unk86c: u32,
    _unk870: u32,
    _unk874: u8,
    _unk878: UnknownStruct<0x240>,
    _unkab8: u16,
    _unkaba: u8,
    _unkabc: u32,
    _unkac0: u32,
    _unkac8: UnknownStruct<0x240>,
    _unkd08: [UnknownStruct<0x118>; 0x7],
    _unk14b0: u32,
    _unk14b4: u32,
    _unk14b8: u32,
    _unk14bc: u32,
    _unk14c0: u16,
    _unk14c2: [u8; 0x6],
    _unk14c8: u64,
    _unk14d0: [u8; 0x8],
    _unk14d8: UnknownStruct<0x158>,
    _unk1630: [u8; 0x8],
    _unk1638: UnknownStruct<0x18>,
    _unk1650: u32,
    _unk1654: u32,
    _unk1658: u16,
    _unk165c: u32,
    _unk1660: u16,
    _unk1668: [UnknownStruct<0x34>; 0xa],
    _unk1870: u32,
    _unk1874: u32,
    _unk1878: [u8; 0x4],
    _unk187c: UnknownStruct<0x140>,
    _unk19bc: u32,
    _unk19c0: u32,
    _unk19c4: u32,
    _unk19c8: u32,
    _unk19cc: u32,
    _unk19d0: [u32; 0x16],
    _unk1a28: u32,
    _unk1a2c: u32,
    _unk1a30: u32,
    _unk1a34: u32,
    _unk1a38: u32,
    _unk1a3c: u32,
    _unk1a40: u32,
    _unk1a44: [u8; 0x4],
    _unk1a48: UnknownStruct<0x28>,
    _unk1a70: UnknownStruct<0x88>,
    _unk1af8: [UnknownStruct<0x98>; 2],
    _unk1c28: [u8; 0x98],
    _unk1cc0: u64,
    _unk1cc8: u32,
    _unk1ccc: u32,
    _unk1cd0: u32,
    _unk1cd4: u32,
    _unk1cd8: u32,
    _unk1cdc: [u32; 0x6],
    _unk1cf4: [u32; 0x6],
    _unk1d0c: u32,
    _unk1d10: u32,
    _unk1d14: u32,
    _unk1d18: u8,
    _unk1d1c: u32,
    _unk1d20: u32,
    _unk1d24: u32,
    _unk1d28: u8,
    _unk1d2c: u32,
    _unk1d30: u8,
    _unk1d38: usize,
    _unk1d40: u64,
    _unk1d48: u32,
    _unk1d4c: u8,
    _unk1d50: u64,
    _unk1d58: u64,
    _unk1d60: [u8; 0x48],
    _unk1da8: u64,
    _unk1db0: u64,
    _unk1db8: u64,
    _unk1dc0: u64,
    _unk1dc8: u64,
    _unk1dd0: u64,
    _unk1dd8: u64,
    _unk1de0: u64,
    _unk1de8: u64,
    _unk1df0: u64,
    _unk1df8: u64,
    _unk1e00: u64,
    _unk1e08: u64,
    _unk1e10: u64,
    _unk1e18: u64,
    _unk1e20: u64,
    _unk1e28: u32,
    _unk1e2c: u32,
    _unk1e30: u32,
    _unk1e34: u8,
    _unk1e38: u64,
    _unk1e40: u16,
    _cs_player_menu_ctrl: UnknownStruct<0x58>,
    _null_player_menu_ctrl: UnknownStruct<0x8>,
    _unk1ea8: FD4Time,
    _ez_task: UnknownStruct<0x18>,
    _self: NonNull<MenuMan>,
    _unk1ed8: usize,
    _unk1ee0: UnknownStruct<0x88>,
    _unk1f68: u8,
    _unk1f70: u64,
}

impl MenuMan {
    /// Whether the game is currently in a load screen.
    pub fn is_load_screen(&self) -> bool {
        self.flags[0] == 0
    }

    /// Whether menu mode is currently enabled.
    ///
    /// In menu mode, the cursor is visible and neither the mouse nor face
    /// buttons on the controller control any in-game actions. The main menu is
    /// not considered to be menu mode.
    pub fn is_menu_mode(&self) -> bool {
        // As far as we know this can only be 0 or 2
        self.flags[8] > 0
    }

    /// Enables or disables menu mode. If this is enabled outside of a menu, you
    /// must manually disable it or the player will be stuck unable to interact
    /// with most of the game.
    ///
    /// In menu mode, the cursor is visible and neither the mouse nor face
    /// buttons on the controller control any in-game actions. The main menu is
    /// not considered to be menu mode.
    pub fn set_menu_mode(&mut self, enabled: bool) {
        self.flags[8] = if enabled { 2 } else { 0 };
    }
}

impl FromStatic for MenuMan {
    fn name() -> Cow<'static, str> {
        "MenuMan".into()
    }

    unsafe fn instance() -> fromsoftware_shared::InstanceResult<&'static mut Self> {
        unsafe { shared::load_static_indirect(rva::get().sprj_menu_man_ptr) }
    }
}

#[repr(C)]
pub struct GrantItemCommand {
    // We require that these collectively produce a valid ItemId.
    category: ItemCategoryHigh,
    item_id: u32,

    pub durability: i32,
    pub quantity: u32,
}

impl GrantItemCommand {
    /// Creates a new [GrantItemCommand] with the given fields.
    pub fn new(item_id: ItemId, durability: i32, quantity: u32) -> Self {
        Self {
            category: item_id.category().into(),
            item_id: item_id.param_id(),
            durability,
            quantity,
        }
    }

    /// Creates the canonical form of an empty [GrantItemCommand].
    pub fn empty() -> MaybeEmpty<Self> {
        unsafe { mem::transmute::<[i32; 4], MaybeEmpty<Self>>([-1, -1, -1, 0]) }
    }

    /// Returns item ID granted by [GrantItemCommand].
    pub fn item_id(&self) -> ItemId {
        unsafe { mem::transmute::<u32, ItemId>(self.category as u32 | self.item_id) }
    }
}

impl From<ItemId> for GrantItemCommand {
    /// Creates an [GrantItemCommand] containing a single full-durability item
    /// with this ID.
    fn from(id: ItemId) -> Self {
        Self::new(id, -1, 1)
    }
}

unsafe impl IsEmpty for GrantItemCommand {
    fn is_empty(value: &MaybeEmpty<Self>) -> bool {
        *unsafe { value.as_non_null().cast::<i32>().as_ref() } == -1
    }
}

#[cfg(test)]
mod test {
    use crate::sprj::MenuMan;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x1f78, size_of::<MenuMan>());
    }
}
