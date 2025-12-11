use std::ptr::NonNull;

use bitfield::bitfield;
use pelite::pe64::Pe;
use shared::program::Program;

use super::{CSEzTask, CSEzUpdateTask, ItemId};
use crate::rva;

pub const STATUS_MESSAGE_DEMIGOD_FELLED: i32 = 1;
pub const STATUS_MESSAGE_LEGEND_FELLED: i32 = 2;
pub const STATUS_MESSAGE_GREAT_ENEMY_FELLED: i32 = 3;
pub const STATUS_MESSAGE_ENEMY_FELLED: i32 = 4;
pub const STATUS_MESSAGE_YOU_DIED: i32 = 5;
pub const STATUS_MESSAGE_HOST_VANQUISHED: i32 = 7;
pub const STATUS_MESSAGE_BLOOD_FINGER_VANQUISHED: i32 = 8;
pub const STATUS_MESSAGE_DUTY_FULL_FILLED: i32 = 9;
pub const STATUS_MESSAGE_LOST_GRACE_DISCOVERED: i32 = 11;
pub const STATUS_MESSAGE_COMMENCE: i32 = 13;
pub const STATUS_MESSAGE_VICTORY: i32 = 14;
pub const STATUS_MESSAGE_STALEMATE: i32 = 15;
pub const STATUS_MESSAGE_DEFEAT: i32 = 16;
pub const STATUS_MESSAGE_MAP_FOUND: i32 = 17;
pub const STATUS_MESSAGE_GREAT_RUNE_RESTORED: i32 = 21;
pub const STATUS_MESSAGE_GOD_SLAIN: i32 = 22;
pub const STATUS_MESSAGE_DUELIST_VANQUISHED: i32 = 23;
pub const STATUS_MESSAGE_RECUSANT_VANQUISHED: i32 = 24;
pub const STATUS_MESSAGE_INVADER_VANQUISHED: i32 = 25;
pub const STATUS_MESSAGE_FURLED_FINGER_RANK_ADVANCED: i32 = 30;
pub const STATUS_MESSAGE_FURLED_FINGER_RANK_ADVANCED2: i32 = 31;
pub const STATUS_MESSAGE_DUELIST_RANK_ADVANCED: i32 = 32;
pub const STATUS_MESSAGE_DUELIST_RANK_ADVANCED2: i32 = 33;
pub const STATUS_MESSAGE_BLOODY_FINGER_RANK_ADVANCED: i32 = 34;
pub const STATUS_MESSAGE_BLOODY_FINGER_RANK_ADVANCED2: i32 = 35;
pub const STATUS_MESSAGE_RECUSANT_RANK_ADVANCED: i32 = 36;
pub const STATUS_MESSAGE_RECUSANT_RANK_ADVANCED2: i32 = 37;
pub const STATUS_MESSAGE_HUNTER_RANK_ADVANCED: i32 = 38;
pub const STATUS_MESSAGE_HUNTER_RANK_ADVANCED2: i32 = 39;
pub const STATUS_MESSAGE_HEART_STOLEN: i32 = 40;
pub const STATUS_MESSAGE_MENU_TEXT: i32 = 41;

#[repr(C)]
#[shared::singleton("CSMenuMan")]
pub struct CSMenuManImp {
    vftable: usize,
    menu_data: usize,
    player_status_calculator: usize,
    unk18: [u8; 2],
    pub disable_mouse_cursor: bool,
    unk1b: [u8; 0x65],
    pub popup_menu: Option<NonNull<CSPopupMenu>>,
    window_job: usize,
    unk90: [u8; 0xAC],
    /// disables all save menu callbacks
    /// additionally, can disable auto save
    pub disable_save_menu: u32,
    unk140: [u8; 0x520],
    pub player_menu_ctrl: CSPlayerMenuCtrl,
    null_player_menu_ctrl: usize,
    unk6b0: [u8; 0x60],
    pub back_screen_data: BackScreenData,
    pub loading_screen_data: LoadingScreenData,
    unk748: [u8; 0x118],
    system_announce_view_model: usize,
    pub update_task: CSEzUpdateTask<CSEzTask, Self>,
    unk890: [u8; 0x10],
}

impl CSMenuManImp {
    // "You died", "Great enemy felled", etc
    pub fn display_status_message(&mut self, message: i32) -> bool {
        let rva = Program::current()
            .rva_to_va(rva::get().cs_menu_man_imp_display_status_message)
            .unwrap();

        let target = unsafe {
            std::mem::transmute::<u64, extern "C" fn(&mut CSMenuManImp, i32) -> bool>(rva)
        };
        target(self, message)
    }
}

#[repr(C)]
pub struct CSMenuData {
    vftable: usize,
    unk8: [u8; 0x54],
    pub show_steam_names: bool,
    unk5d: [u8; 0x13],
    pub menu_gaitem_use_state: CSMenuGaitemUseState,
    unk88: bool,
    unk89: [u8; 0x67],
}

#[repr(C)]
pub struct CSMenuGaitemUseState {
    vftable: usize,
    unk8: u32,
    pub quick_slot_item_id: u32,
    unk10: u32,
    unk14: u32,
}

#[repr(C)]
pub struct CSPopupMenu {
    vftable: usize,
    pub menu_man: NonNull<CSMenuManImp>,
    unk10: usize,
    unk18: usize,
    unk20: [u8; 0x90],
    current_top_menu_job: usize,
    unkb8: [u8; 0xb0],
    input_data: u64,
    unk170: [u8; 0x120],
    pub show_failed_to_save: bool,
    unkb91: [u8; 0x8f],
}

#[repr(C)]
pub struct CSPlayerMenuCtrl {
    vftable: usize,
    pub selected_goods_item: ItemId,
    pub selected_magic_item: ItemId,
    unk10: i32,
    unk14: i32,
    pub chr_menu_flags: CSChrMenuFlags,
    unk28: [u8; 0x20],
}

#[repr(C)]
pub struct CSChrMenuFlags {
    vftable: usize,
    pub flags: ChrMenuFlags,
    // _padc: [u8; 0x4],
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrMenuFlags(u32);
    impl Debug;
    /// Set by TAE Event 0 (action 54 DISABLE_START_INPUTS)
    /// Controls whether the player can open the pause menu
    /// (Equipment, Crafting, Status, Messages, System, Multiplayer, Pouch, Gestures)
    pub pause_menu_state, set_pause_menu_state: 3;
}

#[repr(C)]
pub struct BackScreenData {
    vftable: usize,
    unk8: [u8; 0x8],
}

#[repr(C)]
pub struct LoadingScreenData {
    vftable: usize,
    unk8: [u8; 0x20],
}

#[repr(C)]
pub struct FeSystemAnnounceViewModel {
    menu_view_model: usize,
    view: usize,
    message_queue: FeSystemAnnounceViewModelMessageQueue,
}

#[repr(C)]
pub struct FeSystemAnnounceViewModelMessageQueue {
    unk0: usize,
    unk8: usize,
    elements: usize,
    capacity: usize,
    unk20: usize,
    count: usize,
}

#[cfg(test)]
mod test {
    use crate::cs::{
        BackScreenData, CSMenuData, CSMenuGaitemUseState, CSMenuManImp, CSPlayerMenuCtrl,
        CSPopupMenu, FeSystemAnnounceViewModel, FeSystemAnnounceViewModelMessageQueue,
        LoadingScreenData,
    };

    #[test]
    fn proper_sizes() {
        assert_eq!(0x8a0, size_of::<CSMenuManImp>());
        assert_eq!(0xF0, size_of::<CSMenuData>());
        assert_eq!(0x18, size_of::<CSMenuGaitemUseState>());
        assert_eq!(0x320, size_of::<CSPopupMenu>());
        assert_eq!(0x48, size_of::<CSPlayerMenuCtrl>());
        assert_eq!(0x10, size_of::<BackScreenData>());
        assert_eq!(0x28, size_of::<LoadingScreenData>());
        assert_eq!(0x40, size_of::<FeSystemAnnounceViewModel>());
        assert_eq!(0x30, size_of::<FeSystemAnnounceViewModelMessageQueue>());
    }
}
