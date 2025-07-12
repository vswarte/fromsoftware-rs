use std::{
    ffi::{OsStr, OsString},
    fmt::Display,
    os::windows::ffi::OsStringExt,
    ptr::NonNull,
};

use crate::{dltx::DLString, CSFixedList};
use shared::{FSVector4, OwnedPtr};

use super::{CSMenuManImp, FieldInsHandle};

#[repr(C)]
/// Source of name: RTTI
#[dlrf::singleton("CSFeMan")]
pub struct CSFeManImp {
    vftable: usize,
    unk8: usize,
    game_rend: usize,
    pub menu_man: NonNull<CSMenuManImp>,
    /// Object, containing all scaleform data for FrontEnd (FE) scene
    pub front_end_view: OwnedPtr<FrontEndView>,
    fade_screen: usize,
    fe_system_announce_view: usize,
    fe_summon_message_view: usize,
    clock_view: usize,
    unk48: [u8; 40],
    unk70: usize,
    /// Toggle, to enable/disable the HUD
    pub enable_hud: bool,
    unk79: [u8; 7],
    /// Used for holding intermediate data FrontEndView
    /// read from menu window jobs
    pub frontend_values: FrontEndViewValues,
    auto_hide_ctrl_ptr: usize,
    unk4e68: [u8; 8],
    auto_hide_ctrl: [u8; 0xb04],
    unk5974: [u8; 4],
    menu_resist_gauge: [u8; 0x28],
    unk59a0: [u8; 4],
    /// Ring buffer for proc status message ids
    pub proc_status_messages: [i32; 6],
    /// Index of the last proc status message
    /// Wraps around when it reaches 6
    pub proc_status_messages_read_index: u32,
    /// Index of the next proc status message slot
    /// Wraps around when it reaches 6
    pub proc_status_messages_write_index: u32,
    unk59c4: [u8; 12],
    unk59d0: FSVector4,
    unk59e0: u32,
    unk59e4: [u8; 12],
    /// Data used for the enemy character tags
    /// Will be copied to the enemy character tags in FrontEndView
    /// and then passed to the scaleform
    pub enemy_chr_tag_displays: [ChrEnemyTagEntry; 8],
    /// Data used for the boss health display
    /// Will be copied to the boss health tags in FrontEndView
    pub boss_health_displays: [BossHealthDisplayEntry; 3],
    unk5c50: [u8; 0x10],
    /// Data used for the friendly character tags
    /// Will be copied to the friendly character tags in FrontEndView
    /// and then passed to the scaleform
    pub friendly_chr_tag_displays: [ChrFriendTagEntry; 7],
    /// Time in seconds, after which the damage number will be hidden
    /// Enemy tag will use constant value of 1.5f instead of this
    pub damage_number_decay_time: f32,
    unk6134: [u8; 20],
    fe_menu_chr_state_data: [u8; 0x168],
    pub summon_msg_queue: SummonMsgQueue,
    unk6530: u32,
    /// Id of the fmg text entry for the area welcome message
    /// Can be set by emevd DisplaySubareaWelcomeMessage 2007[13]
    pub subarea_name_popup_message_id: i32,
    unk6538: [u8; 4],
    unk653c: u32,
    unk6540: [u8; 8],
    /// Id of the fmg text entry for the blinking message
    /// Can be set by emevd DisplayBlinkingMessageWithPriority 2007[12]
    pub blinking_message_id: i32,
    /// Priority of the currently displayed blinking message
    pub blinking_message_priority: u8,
    pad654d: [u8; 3],
    unk6550: u32,
    /// Toggle, requesting the area welcome message
    /// to be displayed
    pub area_welcome_message_request: bool,
    unk6555: [u8; 11],
    pub get_item_log_view_model: [u8; 0x1d48],
    unk82a8: [u8; 8],
    pub clock_view_model: usize,
    unk82b0: [u8; 16],
    /// Tag of the debug player
    pub debug_tag: TagHudData,
    unk83f0: [u8; 48],
}

#[repr(C)]
pub struct BossHealthDisplayEntry {
    /// Id of the fmg text entry for the boss name
    pub fmg_id: i32,
    unk4: [u8; 4],
    pub field_ins_handle: FieldInsHandle,
    pub damage_taken: i32,
    unk14: u32,
    unk18: [u8; 0x8],
}

#[repr(C)]
pub struct SummonMsgQueue {
    vftable: usize,
    pub current: SummonMsgData,
    pub list: CSFixedList<SummonMsgData, 4>,
    unk278: u32,
    unk27c: u32,
}

#[repr(C)]
pub struct SummonMsgData {
    vftable: usize,
    pub priority: i16,
    pub force_play: bool,
    unkb: [u8; 5],
    pub text: MenuString,
    unk48: bool,
    unk49: [u8; 7],
}

#[repr(C)]
pub struct FrontEndView {
    menu_window: [u8; 0xa31],
    unka31: [u8; 0x7],
    root_scene: [u8; 0x251c8],
    pub front_end_view_values: NonNull<FrontEndViewValues>,
}

#[repr(C)]
/// Values that will be read in FrontEndView update procedure
/// from the menu window jobs
pub struct FrontEndViewValues {
    unk0: [u8; 4],
    /// Current player hp
    pub player_hp: u32,
    /// Max recoverable hp
    /// Only works if rally mechanic is active
    /// by using the malenia rune arc
    pub max_recoverable_hp: u32,
    /// Difference between the max hp and uncapped max hp
    pub hp_max_uncapped_difference: u32,
    /// Uncapped max player hp
    pub hp_max_uncapped: u32,
    /// Is hp rally mechanic enabled?
    /// Makes the player able to recover hp by attacking enemies
    pub enable_hp_rally: bool,
    unk1a: [u8; 3],
    /// Current player fp
    pub fp: u32,
    unk20: [u8; 8],
    /// Max player fp
    pub fp_max: u32,
    /// Toggle that enables the equipment hud
    /// eg equipped weapons, consumables and magic
    pub enable_equip_hud: bool,
    unk2a: [u8; 3],
    /// Current player stamina
    pub stamina: u32,
    unk30: [u8; 8],
    /// Max player stamina
    pub stamina_max: u32,
    unk38: [u8; 0xf74],
    /// String, containing the name of the current usable Ash of War
    pub sword_arts_name_string: MenuString,
    unkfe8: [u8; 0x19c],
    /// Number of eliminations in the arena
    pub quickmatch_elimination_count: i32,
    unk1184: [u8; 0x8],
    quickmatch_data: [u8; 0x40],
    pub enemy_chr_tag_data: [TagHudData; 8],
    pub boss_list_tag_data: [TagHudData; 3],
    pub friendly_chr_tag_data: [TagHudData; 7],
    unk26a0: [u8; 0xfb8],
    unk3658: MenuString,
    unk3690: [u8; 0x10],
    /// String, containing name from the latest proc status message
    pub proc_status_message: MenuString,
    /// When the timer exceeds 3.0f, the message will be removed
    /// and read counter will be increased
    pub proc_status_message_timer: f32,
    unk36dc: [u8; 4],
    /// Id of the full screen message to be displayed
    /// Eg. "You Died", "Victory", "Defeat"
    pub full_screen_message_request_id: FullScreenMessage,
    unk36e4: [u8; 4],
    unk36e8: MenuString,
    unk3720: MenuString,
    unk3758: MenuString,
    unk3790: MenuString,
    unk37c8: MenuString,
    unk3800: [u8; 0x1558],
    pub summoned_spirit_ash_count: u32,
    unk4d5c: [u8; 4],
    pub spirit_ash_display: [CSFeSpiritAshDisplay; 5],
    unk4dd8: [u8; 8],
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FullScreenMessage {
    None = -1,
    DemigodFelled = 1,
    LegendFelled = 2,
    GreatEnemyFelled = 3,
    EnemyFelled = 4,
    YouDied = 5,
    HostVanquished = 7,
    BloodFingerVanquished = 8,
    DutyFullFilled = 9,
    LostGraceDiscovered = 11,
    Commence = 13,
    Victory = 14,
    Stalemate = 15,
    Defeat = 16,
    MapFound = 17,
    GreatRuneRestored = 21,
    GodSlain = 22,
    DuelistVanquished = 23,
    RecusantVanquished = 24,
    InvaderVanquished = 25,
    FurledFingerRankAdvanced = 26,
    FurledFingerRankAdvanced2 = 31,
    DuelistRankAdvanced = 32,
    DuelistRankAdvanced2 = 33,
    BloodyFingerRankAdvanced = 34,
    BloodyFingerRankAdvanced2 = 35,
    RecusantRankAdvanced = 36,
    RecusantRankAdvanced2 = 37,
    HunterRankAdvanced = 38,
    HunterRankAdvanced2 = 39,
    HeartStolen = 40,
    MenuText = 41,
    YouDiedWithFade = 42,
}

#[repr(C)]
pub struct CSFeSpiritAshDisplay {
    pub field_ins_handle: FieldInsHandle,
    pub hp: u32,
    unkc: u32,
    pub hp_max_uncapped_difference: u32,
    pub hp_max_uncapped: u32,
}

#[repr(C)]
/// Custom string type used to interact with the scaleform
/// If string value is static, it will be stored in the static_string pointer
/// otherwise it will be stored in the DLString and the static_string pointer will be null
/// Source of name: RTTI of std::function
pub struct MenuString {
    pub static_string: *const u16,
    pub allocated_string: DLString,
}

impl Display for MenuString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.allocated_string.is_empty() {
            return write!(f, "{}", self.allocated_string);
        }

        if self.static_string.is_null() {
            return write!(f, "");
        }

        unsafe {
            let string_size = (0..)
                .map(|i| *self.static_string.add(i))
                .take_while(|&c| c != 0)
                .count();

            if string_size == 0 {
                return write!(f, "");
            }

            let slice = std::slice::from_raw_parts(self.static_string, string_size);
            write!(f, "{}", String::from_utf16_lossy(slice))
        }
    }
}

#[repr(C)]
/// Used to display the tag on the screen
/// Read by menu window jobs and written by CSMenuManImp update task
/// by copying the data from the ChrFriendTagEntry and ChrEnemyTagEntry on CSFeManImp
pub struct TagHudData {
    /// Is this tag shown on screen?
    pub is_visible: bool,
    /// Should this tag screen position be updated?
    pub update_position: bool,
    /// Is the tag owner character currently not on screen?
    /// If this is true, the tag will be rendered on the left side of the screen
    pub is_not_on_screen: bool,
    unk3: [u8; 5],
    /// Handle to the tag owner character
    pub field_ins_handle: FieldInsHandle,
    /// Position of the tag on the screen - X
    pub screen_pos_x: f32,
    /// Position of the tag on the screen - Y
    pub screen_pos_y: f32,
    /// Current hp of the character
    pub hp: u32,
    unk1c: [u8; 0x4],
    /// Difference between the max hp and uncapped max hp
    pub hp_max_uncapped_difference: u32,
    /// Uncapped max hp of the character
    pub hp_max_uncapped: u32,
    /// Name of the character
    pub chr_name: MenuString,
    /// Role of the character
    /// eg. "Duelist"
    pub role_string: MenuString,
    unk98: [u8; 0x40],
    /// Is this character downscaled?
    /// True when character has the sp effect 590
    pub is_down_scaled: bool,
    unkd9: [u8; 0x3],
    /// Number of the damage taken to show on the tag
    pub last_damage_taken: i32,
    /// Last hp value before the damage taken
    /// Used to render the hp bar depletion
    pub last_hp_value: u32,
    /// Enum of the tag text color
    /// 1 for friend summon (white)
    /// 2 for enemy summon (red)
    pub role_name_color: u8,
    /// Does this character have an active rune arc?
    /// True when PlayerGameData->0xa58 bit 1 is set
    /// Will render circle icon on the tag
    pub has_rune_arc: bool,
    unke6: [u8; 0x42],
}

#[repr(C)]
/// Used to store and update the tag data for friendly characters
/// Data from here will be copied to the TagHudData in FrontEndViewValues
/// and then passed to the scaleform
pub struct ChrFriendTagEntry {
    /// Screen position in format:
    /// X, Y - screen position
    /// Z - depth, seems to be increased when character is further away, not used elsewhere
    /// W - unused, always 0
    pub screen_pos: FSVector4,
    /// Should tag be visible on screen?
    pub is_visible: bool,
    /// Is line of sight to this character blocked?
    /// Casts a ray to the world position where tag will be rendered
    /// and checks if it hits anything
    /// If this is true, the tag will be rendered on the left side of the screen
    /// is_not_on_screen will be true and is_visible will be false
    pub is_line_of_sight_blocked: bool,
    /// Is this character currently not on screen?
    /// If this is true, is_visible will be false and tag will be rendered
    /// on the left side of the screen
    pub is_not_on_screen: bool,
    /// Enum of the tag text color
    /// 1 for friend summon (white)
    /// 2 for enemy summon (red)
    pub role_name_color: u8,
    _pad14: [u8; 4],
    /// String, containing the role of the character
    pub role_string: DLString,
    /// String, containing the name of the character
    pub name_string: DLString,
    /// The max hp of the character, uncapped
    pub hp_max_uncapped: u32,
    /// The current hp of the character
    pub hp: u32,
    /// The max recoverable hp of the character
    /// Only works if character has malenia rune arc active
    pub max_recoverable_hp: u32,
    /// The max hp of the character
    pub max_hp: u32,
    unk88: [u8; 4],
    /// Time since the last damage taken
    pub last_damage_time_delta: f32,
    pub voice_chat_state: u32,
    _pad94: [u8; 4],
    /// Handle to the character
    pub field_ins_handle: FieldInsHandle,
    _pada0: [u8; 2],
    /// The team type of the character
    pub team_type: u8,
    /// Is this character a downscaled summon?
    /// True when character has the sp effect 590
    pub is_down_scaled: bool,
    /// Enables the rune arc icon on the tag
    /// True when PlayerGameData->0xa58 bit 1 is set
    pub has_rune_arc: bool,
    /// Is this summon a debug summon
    /// Will replace the name to "Debug" in japanese
    /// and role to white summon
    pub is_debug_summon: bool,
    _pada6: [u8; 10],
}

#[repr(C)]
/// Used to store and update the tag data for enemy
/// (everyone you can lock on) characters
/// Data from here will be copied to the TagHudData in FrontEndViewValues
/// and then passed to the scaleform
pub struct ChrEnemyTagEntry {
    /// Handle of the character
    pub field_ins_handle: FieldInsHandle,
    unk8: [u8; 0x8],
    /// Screen position in format:
    /// X, Y - screen position
    /// Z - depth, seems to be increased when character is further away, not used elsewhere
    /// W - unused, always 0
    pub screen_pos: FSVector4,
    unk20: [u8; 0x4],
    /// Amount of hp lost
    /// Used to render the damage number on the tag
    pub damage_taken: i32,
    /// Last hp value before the damage taken
    /// Used to render the hp bar depletion
    pub pre_damage_hp: u32,
    /// Delta time from last update of the tag
    /// If it exceeds 1.5f, the tag will be removed
    pub last_update_time_delta: f32,
    /// Delta time from last damage taken
    /// If it exceeds 1.5f, the damage number will be removed
    pub last_damage_time_delta: f32,
    /// Is this tag shown on screen?
    pub is_visible: bool,
    unk35: [u8; 0xb],
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x8420, size_of::<CSFeManImp>());
        assert_eq!(0x280, size_of::<SummonMsgQueue>());
        assert_eq!(0x25c08, size_of::<FrontEndView>());
        assert_eq!(0x38, size_of::<MenuString>());
    }
}
