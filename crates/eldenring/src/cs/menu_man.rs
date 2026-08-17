use std::ptr::NonNull;

use bitfield::bitfield;
use pelite::pe64::Pe;
use shared::{OwnedPtr, program::Program};

use super::{CSEzTask, CSEzUpdateTask, OptionalItemId};
use crate::{DLDeque, cs::MenuString, rva};

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
    /// States of UI elements, indexed by specific for each element ID.
    pub ui_states: [UIState; 0x46],
    unkd6: [u8; 0x5a],
    unk130: i32,
    unk134: [u8; 0x8],
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
    pub system_announce_view_model: OwnedPtr<FeSystemAnnounceViewModel>,
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

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct UIState(u8);
    impl Debug;
    bool;
    /// Whether the class responsible for this UI element has been created and live.
    pub created, set_created: 0;
    /// Whether this UI element is currently visible on screen
    pub visible, set_visible: 1;
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
    unk170: [u8; 0x10],
    pub current_talk_id: i32,
    unk184: [u8; 0x2c],
    /// Queue of messages to be shown in the popup menu.
    ///
    /// Limited to 4 by the game.
    pub popup_messages: DLDeque<MenuString>,
    unk1e0: [u8; 0x70],
    world_map_view_model: usize,
    unk258: [u8; 0x8],
    multi_play_view_model: usize,
    unk268: [u8; 0x20],
    matching_view_model: usize,
    pub show_failed_to_save: bool,
    unkb91: [u8; 0x8f],
}

#[repr(C)]
pub struct CSPlayerMenuCtrl {
    vftable: usize,
    pub selected_goods_item: OptionalItemId,
    pub selected_magic_item: OptionalItemId,
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
    pub view: NonNull<FeSystemAnnounceView>,
    pub notifications: DLDeque<AnnounceNotification>,
}

#[repr(C)]
pub struct AnnounceNotification {
    pub is_active: bool,
    pub message: MenuString,
}

#[repr(C)]
/// Scaleform-backed HUD widget showing system announcements.
///
/// Queued announcements are read from [`system_announce_view_model`] one at
/// a time and played back through the [`announce_play_state`] state machine.
///
/// [`system_announce_view_model`]: Self::system_announce_view_model
/// [`announce_play_state`]: Self::announce_play_state
pub struct FeSystemAnnounceView {
    unk0: [u8; 0xa38],
    /// Scaleform proxy used to drive the "FadeIn"/"FadeOut" animation labels.
    unka38: [u8; 0x18],
    /// Scaleform proxy for the announcement's text field.
    unka50: [u8; 0x50],
    unkaa0: [u8; 0x60],
    pub system_announce_view_model: NonNull<FeSystemAnnounceViewModel>,
    /// Whether this view is currently allowed to update.
    ///
    /// Set to `false` while a blocking menu is open on `CSMenuMan`, which
    /// pauses the playback state machine until it closes.
    pub is_active: bool,
    /// Whether the announcement banner is currently shown on screen.
    pub is_visible: bool,
    unkb0a: [u8; 0x6],
    /// Announcement currently being played back.
    pub active_announcement: AnnounceNotification,
    /// Current step of the playback state machine.
    pub announce_play_state: SystemAnnounceViewModelState,
    unkb31: [u8; 0x3],
    /// Time remaining before the scroll animation (re)starts.
    ///
    /// Used to pause on the start of the message for a moment before
    /// scrolling begins, rather than scrolling immediately.
    pub system_announce_scroll_buffer_timer: f32,
    /// Number of times left to loop the scroll animation.
    pub system_announce_scroll_count: u32,
    /// Whether the announcement text is wider than the screen and needs to
    /// scroll to be read in full.
    pub needs_scroll: bool,
    unkb2d: [u8; 0x3],
    /// Current horizontal scroll offset of the text field, in pixels.
    ///
    /// Compared against [`scroll_distance`] to determine when the text has
    /// fully scrolled past.
    ///
    /// [`scroll_distance`]: Self::scroll_distance
    pub scroll_offset: i32,
    /// Scroll offset at which the text has fully scrolled past, in pixels.
    ///
    /// Computed as the rendered text width minus the visible text field
    /// width, both converted from Scaleform twips to pixels (`* 0.05`).
    pub scroll_distance: i32,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Step of the [FeSystemAnnounceView] playback state machine.
///
/// Cycles `Load` through `Dequeue` in order, advancing on every update
/// unless a step returns early to wait for a timer or animation.
pub enum SystemAnnounceViewModelState {
    /// Not currently playing back an announcement.
    Idle = 0,
    /// Loads the queued announcement's text and determines whether it needs
    /// to scroll.
    Load = 1,
    /// Waits for the "FadeIn" scaleform animation to finish.
    FadeIn = 2,
    /// Resets the scroll position to the start of the message and sets
    /// [`system_announce_scroll_buffer_timer`], pausing there for a moment
    /// before scrolling begins.
    ///
    /// [`system_announce_scroll_buffer_timer`]: FeSystemAnnounceView::system_announce_scroll_buffer_timer
    ScrollReset = 3,
    /// Waits out [`system_announce_scroll_buffer_timer`], pausing on the
    /// start of the message before scrolling begins.
    ///
    /// [`system_announce_scroll_buffer_timer`]: FeSystemAnnounceView::system_announce_scroll_buffer_timer
    BufferWait = 4,
    /// Waits out a fixed delay for announcements that don't need to scroll.
    NoScrollWait = 5,
    /// Advances the scroll offset until the text has fully scrolled past.
    Scrolling = 6,
    /// Sets [`system_announce_scroll_buffer_timer`], pausing on the end of
    /// the message once it has fully scrolled past.
    ///
    /// [`system_announce_scroll_buffer_timer`]: FeSystemAnnounceView::system_announce_scroll_buffer_timer
    PostScrollBuffer = 7,
    /// Waits out [`system_announce_scroll_buffer_timer`], pausing on the end
    /// of the message once it has fully scrolled past.
    ///
    /// [`system_announce_scroll_buffer_timer`]: FeSystemAnnounceView::system_announce_scroll_buffer_timer
    PostScrollBufferWait = 8,
    /// Decrements [`system_announce_scroll_count`] and loops back to
    /// [`ScrollReset`] while repeats remain.
    ///
    /// [`system_announce_scroll_count`]: FeSystemAnnounceView::system_announce_scroll_count
    /// [`ScrollReset`]: Self::ScrollReset
    RepeatCheck = 9,
    /// Hides the announcement banner.
    HidePlaying = 10,
    /// Waits for the "FadeOut" scaleform animation to finish.
    FadeOut = 11,
    /// Marks the active announcement as no longer active and removes it from
    /// the queue.
    Dequeue = 12,
}

#[cfg(test)]
mod test {
    use crate::cs::{
        AnnounceNotification, BackScreenData, CSMenuData, CSMenuGaitemUseState, CSMenuManImp,
        CSPlayerMenuCtrl, CSPopupMenu, FeSystemAnnounceView, FeSystemAnnounceViewModel,
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
        assert_eq!(0x40, size_of::<AnnounceNotification>());
        assert_eq!(0xb68, size_of::<FeSystemAnnounceView>());
    }
}
