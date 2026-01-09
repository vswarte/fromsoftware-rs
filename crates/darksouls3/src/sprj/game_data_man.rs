use std::borrow::Cow;

use pelite::pe64::Pe;
use shared::{FromStatic, InstanceResult, OwnedPtr, Program};

use super::{ItemId, PlayerGameData};
use crate::rva;

#[repr(C)]
/// Source of name: RTTI
pub struct GameDataMan {
    _vftable: usize,
    _trophy_equip_data: usize,
    pub main_player_game_data: OwnedPtr<PlayerGameData>,
    pub network_players: OwnedPtr<[PlayerGameData; 5]>,
    _unk20: [u8; 0x38],
    _game_settings: usize,
    _menu_system_save_load: usize,
    _profile_summary: usize,
    _pc_option_data: usize,
    _unk78: [u8; 0xB8],
}

impl GameDataMan {
    /// Gives the player `quantity` instances of `item`.
    ///
    /// Note that this won't give more than one copy of certain key items.
    pub fn give_item_directly(&mut self, item: ItemId, quantity: u32) {
        let va = Program::current()
            .rva_to_va(rva::get().lua_event_man_give_item_directly)
            .expect("Call target for lua_event_man_give_item_directly was not in exe");

        // Because this function comes from the event manager, it takes the
        // LuaEventMan as its first argument rather than GameDataMan. It instead
        // accesses GameDataMan through the global variable. To avoid needing to
        // mark this function unsafe, though, we make it a method on
        // `GameDataMan` anyway. Since there's only one instance of this
        // globally, if we have a mutable reference to it we know it's safe to
        // run code that modifies it through the global variable.
        let give_item_directly: extern "C" fn(usize, u32, u32, u32) =
            unsafe { std::mem::transmute(va) };

        // The LuaEventMan isn't actually used.
        give_item_directly(0, (item.category() as u32) << 28, item.param_id(), quantity);
    }

    /// Removes `quantity` instances of `item` from the player's inventory.
    pub fn remove_item(&mut self, item: ItemId, quantity: u32) {
        // As above, this takes LuaEventMan but doesn't use it.
        let va = Program::current()
            .rva_to_va(rva::get().lua_event_man_remove_item)
            .unwrap();
        let remove_item: extern "C" fn(usize, u32, u32, u32) = unsafe { std::mem::transmute(va) };

        remove_item(0, (item.category() as u32) << 28, item.param_id(), quantity);
    }
}

impl FromStatic for GameDataMan {
    fn name() -> Cow<'static, str> {
        "GameDataMan".into()
    }

    /// Returns the singleton instance of `GameDataMan`.
    unsafe fn instance() -> InstanceResult<&'static mut Self> {
        unsafe { shared::load_static_indirect(rva::get().game_data_man_ptr) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x130, size_of::<GameDataMan>());
    }
}
