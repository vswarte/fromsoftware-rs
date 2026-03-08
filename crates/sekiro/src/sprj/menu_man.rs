use std::borrow::Cow;

use shared::*;

use crate::{dltx::DLString, fd4::FD4Time, rva};

// Source of name: RTTI
#[repr(C)]
pub struct MenuMan {
    _vftable: usize,
    _unk08: [u8; 0x8],
    _unk10: u8,
    _unk11: u8,
    _unk12: u8,
    _unk13: u8,

    /// Seems to be unused.
    pub draw_layout_on_cursor: bool,

    /// Seems to be unused.
    pub draw_layout_only_last: bool,

    /// Seems to be unused.
    pub draw_layout_display_position: bool,

    /// Seems to be unused.
    pub draw_layout_only_register: bool,

    /// Seems to be unused.
    pub show_action_button: bool,

    /// Seems to be unused.
    pub hide_fe: bool,

    /// Set this to true to hide all menus including the HUD.
    pub hide_all_menus: bool,

    /// Various flags each with its own meaning. Known flags have accessor methods.
    pub flags: [i32; 1000],

    _unkfc0: u64,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub move_map_step_number: i32,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub move_map_step_update_time: f32,

    _unkfd0: [u8; 0x4],

    /// Seems to be unused.
    pub talk_message_id: i32,

    /// Seems to be unused.
    pub enable_talk_icon: bool,

    _unkfd9: [u8; 0x3],
    _unkfdc: u8,

    /// Seems to be unused.
    pub talk_shop_message_id: i32,

    _unkfe4: u32,
    _unkfe8: u32,
    _unkfec: u32,
    _unkff0: u8,
    _unkff4: u32,
    _unkff8: u32,
    _unkffc: u8,
    _unk1000: u32,

    /// Seems to be unused.
    pub map_action_data_invisible_time: f32,

    _unk1008: [u8; 0x8],

    pub action_spots: [MaybeEmpty<MenuActionSpot>; 6],

    _unk11f0: u64,
    _unk11f8: u32,
    _unk1200: u64,
    _unk1208: u64,
    _unk1210: u64,
    _unk1218: u64,
    _unk1220: u32,
    _unk1224: u16,
    _unk1228: u32,
    _unk122c: [u8; 0x4],
    _unk1230: u32,
    _unk1234: u32,
    _unk1238: u32,
    _unk123c: u32,
    _unk1240: u64,
    _unk1248: u64,
    _unk1250: u32,
    _unk1254: u16,
    _unk1256: [u8; 0x2],
    _unk1258: u32,
    _unk125c: [u8; 0x4],

    /// Seems to be unused.
    pub drop_equip_type: i32,

    /// Seems to be unused.
    pub drop_equip_id: i32,

    /// Seems to be unused.
    pub drop_durability: i32,

    /// Seems to be unused.
    pub drop_quantity: i32,

    _unk1270: u64,
    _unk1278: u64,
    _unk1280: u8,
    _unk1284: u32,
    _unk1288: u32,
    _unk128c: u32,
    _unk1290: u32,
    _unk1294: u8,
    _unk1295: [u8; 0x3],
    _unk1298: UnknownStruct<0x1f0>,
    _unk1488: u16,
    _unk148a: u16,
    _unk148c: u16,
    _unk148e: u16,
    _unk1490: u16,
    _unk1492: u16,
    _unk1494: u16,
    _unk1496: u16,
    _unk1498: UnknownStruct<0x1f0>,

    /// Seems to be unused.
    pub floating_pc_gauges: [FloatingPcGauge; 0x7],

    /// The position of the targeting reticle.
    pub target_site_position: F32Vector3,

    _unk1eac: u32,
    _unk1eb0: u32,
    _unk1eb4: u32,

    /// Whether the targeting reticle is visible.
    pub target_site_visible: bool,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub target_site_force_visible: bool,

    _unk1eba: [u8; 0x6],
    _unk1ec0: u32,
    _unk1ec4: u32,
    _unk1ec8: u32,
    _unk1ecc: u32,
    _unk1ed0: u32,
    _unk1ed4: u32,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub vfx_data_position: F32Vector2,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub vfx_data_visible: bool,

    _unk1ee1: [u8; 0xf],
    _unk1ef0: UnknownStruct<0xb0>,
    _unk1efa0: u64,
    _unk1efa8: u16,
    _unk1efaa: u8,
    _unk1efb0: u64,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub ugc_restricted: bool,

    pub menu_info_data: MenuInfoData,

    _unk2110: u8,
    _unk2111: [u8; 0x7],
    _unk2118: UnknownStruct<0x10>,

    /// The ID of the last talk event the player loaded.
    pub last_talk_id: i32,

    _unk212c: u32,
    _unk2130: u16,
    _unk2132: u8,
    _unk2133: [u8; 0x9],
    _unk213c: u8,
    _unk213d: [u8; 0x3],
    _unk2140: [UnknownStruct<0x34>; 10],

    /// Seems to be unused.
    pub shop_lineup_start_id: i32,

    /// Seems to be unused.
    pub shop_lineup_end_id: i32,

    /// Seems to be unused.
    pub enable_shop_test: bool,

    pub enemy_gauges: [MaybeEmpty<EnemyGauge>; 16],

    _unk2614: u32,
    _unk2618: u32,
    _unk261c: u32,
    _unk2620: u32,
    _unk2624: u32,
    _unk2628: [u8; 8],
    _unk2630: UnknownStruct<0x4c0>,
    _unk2af0: u32,
    _unk2af4: [u8; 0x54],
    _unk2b48: u32,
    _unk2b4c: u32,
    _unk2b50: u32,
    _unk2b54: u32,
    _unk2b58: u32,
    _unk2b5c: u32,
    _unk2b60: u32,
    _unk2b64: [u8; 0x4],
    _unk2b68: [UnknownStruct<0x10>; 2],
    _menu_user_texture_data: [u32; 0x20],

    /// The string ID for the name of the miniboss currently being fought, or -1
    /// if the player isn't fighting a miniboss.
    pub miniboss_gauge_name_id: i32,

    _unk2c0c: u32,

    /// The last damage dealt by the player to the miniboss.
    pub miniboss_gauge_my_damage: i32,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub miniboss_gauge_net_damage: i32,

    _unk2c18: u8,
    _unk2c1c: u32,
    _unk2c20: u64,
    _unk2c28: u64,
    _unk2c30: u64,
    _unk2c38: u64,
    _unk2c40: u64,
    _unk2c48: u64,
    _unk2c50: u64,
    _unk2c58: u64,
    _unk2c60: u64,
    _unk2c68: u64,
    _unk2c70: u64,
    _unk2c78: u64,
    _unk2c80: u64,
    _unk2c88: u64,
    _unk2c90: u64,
    _unk2c98: u64,
    _unk2d00: u64,
    _unk2d08: u64,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub miniboss_gauge_name_id_2: i32,

    _unk2cb4: [u8; 0x15c],

    /// The string ID for the name of the boss currently being fought, or -1 if
    /// the player isn't fighting a boss.
    pub boss_gauge_name_id: i32,

    _unk2e14: u32,

    /// The last damage dealt by the player to the boss.
    pub boss_gauge_my_damage: i32,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub boss_gauge_net_damage: i32,

    _unk2e20: u8,
    _unk2e24: u32,
    _unk2e28: u64,
    _unk2e30: u64,
    _unk2e38: u64,
    _unk2e40: u64,
    _unk2e48: u64,
    _unk2e50: u64,
    _unk2e58: u64,
    _unk2e60: u64,
    _unk2e68: u64,
    _unk2e70: u64,
    _unk2e78: u64,
    _unk2e80: u64,
    _unk2e88: u64,
    _unk2e90: u64,
    _unk2e98: u32,
    _unk2e9c: u32,
    _unk2ea0: u32,
    _unk2ea4: u32,
    _unk2ea8: u32,
    _unk2eac: u32,
    _unk2eb0: u32,
    _unk2eb4: u32,

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub boss_gauge_name_id_2: i32,

    _unk2ebc: [u8; 0x14c],
    _unk3008: u32,
    _unk3010: u32,

    /// Seems to be unused.
    pub select_equip_info_type: i32,

    /// Seems to be unused.
    pub select_equip_info_id: i32,

    /// Seems to be unused.
    pub select_equip_info_durability: i32,

    _unk3020: u32,

    /// Seems to be unused.
    pub condition_message_ids: [i32; 6],

    _unk303c: [u32; 6],

    /// Seems to be unused.
    pub current_condition_message_id: i32,

    /// Seems to be unused.
    pub next_condition_message_id: i32,

    _unk305c: u8,
    _unk3060: i32,
    _unk3064: u8,
    _unk3068: i32,
    _unk306c: i32,
    _unk3070: u32,
    _unk3074: u8,

    /// Seems to be unused.
    pub start_tab_animation_play_speed: f32,

    _unk307c: u8,
    _unk3080: UnknownPtr,
    _unk3088: u64,
    _unk3090: i32,
    _unk3094: u32,
    _unk3098: i32,
    _unk309c: u8,
    _unk30a0: u64,
    _unk30a8: u64,
    _unk30b0: [u8; 0x88],
    _unk3138: u64,
    _unk3140: u64,

    /// Seems to be unused.
    pub last_selected_top_menu_item: i32,

    _unk314c: u32,

    _unk3150: u64,
    _unk3158: u32,

    /// Seems to be unused.
    pub last_selected_equipment: i32,

    _unk3160: u64,

    /// Seems to be unused.
    pub last_selected_inventory_tab: i32,

    /// Seems to be unused.
    pub last_selected_tab_scroll_position: i32,

    /// Seems to be unused.
    pub last_selected_inventory_item: i32,

    /// Seems to be unused.
    pub last_selected_inventory_item_scroll_position: i32,

    _unk3178: u32,

    /// Seems to be unused.
    pub last_selected_message_tab: i32,

    _unk3180: u32,

    /// Seems to be unused.
    pub last_selected_message_item: i32,

    /// Seems to be unused.
    pub last_selected_message_item_scroll_position: i32,

    /// Seems to be unused.
    pub last_selected_message_edit_mode: i32,

    /// Seems to be unused.
    pub last_selected_option_tab: i32,

    _unk3194: u32,

    /// Seems to be unused.
    pub last_selected_option_item: i32,

    _unk319c: u32,

    _unk31a0: u64,
    _unk31a8: u64,
    _unk31b0: u64,
    _unk31b8: u32,
    _unk31bc: u8,

    /// Seems to be unused.
    pub current_player_pad_index: i32,

    /// Seems to be unused.
    pub desired_player_pad_index: i32,

    _unk31c8: u8,
    _unk31d0: u64,
    _unk31d8: u32,
    _unk31dc: [u8; 0x4],
    _player_menu_ctrl: UnknownStruct<0x58>,
    _unk3238: UnknownPtr,
    _unk3240: FD4Time,
    _unk3250: UnknownPtr,
    _unk3258: [u8; 0x10],
    _unk3268: UnknownPtr,
    _unk3270: UnknownPtr,
    _unk3278: UnknownStruct<0x88>,
    _unk3300: u8,
    _unk3308: u64,
}

impl MenuMan {
    /// Whether menu mode is currently enabled.
    ///
    /// In menu mode, the cursor is visible and neither the mouse nor face
    /// buttons on the controller control any in-game actions. The main menu is
    /// not considered to be menu mode.
    pub fn is_menu_mode(&self) -> bool {
        // As far as we know this can only be 0 or 2
        self.flags[9] > 0
    }

    /// Enables or disables menu mode. If this is enabled outside of a menu, you
    /// must manually disable it or the player will be stuck unable to interact
    /// with most of the game.
    ///
    /// In menu mode, the cursor is visible and neither the mouse nor face
    /// buttons on the controller control any in-game actions. The main menu is
    /// not considered to be menu mode.
    pub fn set_menu_mode(&mut self, enabled: bool) {
        self.flags[0] = if enabled { 2 } else { 0 };
    }

    /// Seems to be unused.
    pub fn caption_flag(&self) -> i32 {
        self.flags[0x37]
    }

    /// Seems to be unused.
    pub fn selected_inventory_id(&self) -> i32 {
        self.flags[0x67]
    }

    /// Seems to be unused.
    pub fn drop_ret_inventory_id(&self) -> i32 {
        self.flags[0x68]
    }

    /// Seems to be unused.
    pub fn selected_inventory_slot(&self) -> i32 {
        self.flags[0x69]
    }

    /// Seems to be unused.
    pub fn inventory_page_state(&self) -> i32 {
        self.flags[0x6a]
    }

    /// Seems to be unused.
    pub fn selected_sort_inventory_id(&self) -> i32 {
        self.flags[0x6b]
    }

    /// Seems to be unused.
    pub fn disable_popup_menu(&self) -> i32 {
        self.flags[0xb2]
    }

    /// Seems to be unused.
    pub fn set_disable_popup_menu(&mut self, value: i32) {
        self.flags[0xb2] = value
    }

    /// Seems to be unused.
    pub fn menu_tab_type(&self) -> i32 {
        self.flags[0x7d]
    }

    /// Returns the active action spots.
    pub fn action_spots(&self) -> impl Iterator<Item = &MenuActionSpot> {
        self.action_spots.iter().non_empty()
    }

    /// Returns the mutable active action spots.
    pub fn action_spots_mut(&mut self) -> impl Iterator<Item = &mut MenuActionSpot> {
        self.action_spots.iter_mut().non_empty()
    }

    pub fn enemy_gauges(&self) -> impl Iterator<Item = &EnemyGauge> {
        self.enemy_gauges.iter().non_empty()
    }

    pub fn enemy_gauge_mut(&mut self) -> impl Iterator<Item = &mut EnemyGauge> {
        self.enemy_gauges.iter_mut().non_empty()
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

/// A spot on screen that shows a display indicating that the player can
/// interact with it with the action button.
#[repr(C)]
// Source of name: debug menus
pub struct MenuActionSpot {
    pub unique_id: u32,
    _unk04: [u8; 0xb],
    _unk10: u64,
    _unk18: u64,

    /// The distance from the player to the action spot's locus. Distance 0 is
    /// the point at which the action button can be activated, but this can be
    /// below 0 once the player is inside the activation radius.
    pub distance: f32,

    /// The maximum distance from which the action spot will be displayed.
    pub max_distance: f32,

    _unk28: u64,

    /// The ranking of how appropriate this spot is relative to others. Higher
    /// is more appropriate. This always seems to be negative.
    pub score: i32,

    /// Seems to be unused.
    pub position: F32Vector2,

    /// The FMG ID of the message to display for the action button prompt.
    pub text_id: i32,

    /// The time in seconds since the action spot was on screen.
    pub time_since_vanish: f32,

    _unk44: u16,
    _unk46: [u8; 0xa],
}

unsafe impl IsEmpty for MenuActionSpot {
    fn is_empty(value: &MaybeEmpty<Self>) -> bool {
        *unsafe { value.as_non_null().cast::<u32>().as_ref() } == 0
    }
}

/// This seems to be unused.
#[repr(C)]
// Source of name: debug menus
pub struct FloatingPcGauge {
    pub position: F32Vector3,
    pub is_visible: bool,
    pub is_shielded: bool,
    pub display_permanently: bool,

    _unkf: u8,
    _unk10: u32,
    _unk14: [u8; 0x4],
    _string1: DLString,
    _string2: DLString,
    _unk78: u64,

    pub damage_value: i32,
    pub damage_value_display_timer: f32,

    _unk88: u32,
    _unk8c: u32,
    _unk90: u8,
    _unk91: [u8; 0x2],

    pub force_visible: bool,

    _unk94: u8,
    _unk95: [u8; 0x3],
    _unk98: u8,
    _unka0: u64,
    _unka8: u64,
    _unkb0: u32,
    _unkb8: u64,
    _unkc0: u64,
    _unkc8: u64,
    _unkd0: u32,
    _unkd4: u8,
    _unkd8: u32,
    _unke0: u64,
    _unke8: u64,
    _unkf0: u32,
    _unkf8: u64,
    _unk100: u64,
    _unk108: u64,
    _unk110: u32,
    _unk114: u8,
    _unk118: u32,
    _unk11c: [u8; 0x4],
    _unk120: u8,
}

/// Seems to be unused.
#[repr(C)]
// Source of name: debug menus
pub struct MenuInfoData {
    pub current_stack_index: i32,
    pub entries: [MenuInfoDataEntry; 5],

    _unk148: u32,
}

/// Seems to be unused.
#[repr(C)]
pub struct MenuInfoDataEntry {
    _unk0: u32,
    _unk4: u32,
    _unk8: u32,
    _unkc: u32,
    _unk10: u32,
    _unk14: u32,

    pub system_message_id: i32,
    _use_list_data: UnknownPtr,
    _use_buffer_data: UnknownPtr,
    pub is_in_use: bool,
    pub use_tos_menu: bool,
    pub use_any_pad_menu: bool,
    pub button_type: i32,
    pub user_id: i32,

    _unk3c: u32,
}

/// An enemy's status bar which indicates their health and stamina.
#[repr(C, packed(4))]
// Source of name: debug menus
pub struct EnemyGauge {
    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub handle: u32,

    /// The location of the gauge in screen coordinates.
    pub draw_position: F32Vector2,

    _unkc: [u8; 0x4],

    /// This name comes from debug data, but the behavior isn't yet well-understood.
    pub is_lock: bool,

    _unk11: u8,

    /// Whether this gauge is visible.
    pub is_visible: bool,

    _unk13: u8,

    /// The amount of damage that was most recently taken by this enemy. This
    /// amy be 0 or -1 when the enemy has not taken damage since the gauge appeared.
    pub damage_value: i32,

    _unk18: u32,
    _unk1c: u32,

    /// The time in seconds since the gauge was last on the scren.
    pub time_since_vanish: f32,

    /// The time in seconds since the gauge appeared.
    pub time_since_appear: f32,

    /// The time in seconds since the enemy last took a hit.
    pub time_since_hit: f32,
}

unsafe impl IsEmpty for EnemyGauge {
    fn is_empty(value: &MaybeEmpty<Self>) -> bool {
        *unsafe { value.as_non_null().cast::<i32>().as_ref() } == -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x50, size_of::<MenuActionSpot>());
        assert_eq!(0x128, size_of::<FloatingPcGauge>());
        assert_eq!(0x40, size_of::<MenuInfoDataEntry>());
        assert_eq!(0x150, size_of::<MenuInfoData>());
        assert_eq!(0x3310, size_of::<MenuMan>());
    }
}
