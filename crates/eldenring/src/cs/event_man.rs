use std::ptr::NonNull;

use shared::{get_instance, OwnedPtr};

use crate::cs::WorldAreaTime;

use super::CSSosSignMan;

#[shared::singleton("CSEventMan")]
#[repr(C)]
pub struct CSEventManImp {
    vftable: usize,
    simple_info: usize,
    dead_reset: usize,
    obj_sfx: usize,
    parts_damage: usize,
    drop_item: usize,
    sound: usize,
    damage: usize,
    dam_obj_hit: usize,
    unk48: usize,
    unk50: usize,
    unk58: usize,
    pub sos_sign: OwnedPtr<CSEventSosSignCtrl>,
    unk68: usize,
    obj_act_exec: usize,
    unk78: usize,
    bloodstain: usize,
    script: usize,
    corpse: usize,
    unk98: usize,
    generator: usize,
    unka8: usize,
    system_flag: usize,
    turn: usize,
    pub world_area_time: OwnedPtr<CSEventWorldAreaTimeCtrl>,
    fade_warp: usize,
    unkd0: usize,
    unkd8: usize,
    retry_points: usize,
    network_error_return_title_step: usize,
    cutscene_warp: usize,
}

#[repr(C)]
pub struct CSEventSosSignCtrl {
    vftable: usize,
    unk8: [u8; 0x40],
    pub sos_sign_man: Option<NonNull<CSSosSignMan>>,
    unk50: u32,
    unk54: u32,
}

#[repr(C)]
pub struct CSEventWorldAreaTimeCtrl {
    /// Base step machine interface (vftable + state management)
    base: [u8; 0x40],
    unk40: [u8; 0x68],
    unka8: bool,
    /// Hours component of target time (0-23)
    /// Represents absolute hour to set
    pub target_hours: u32,
    /// Minutes component of target time (0-59)
    /// Represents absolute minutes to set
    pub target_minutes: u32,
    /// Seconds component of target time (0-59)
    /// Represents absolute seconds to set
    pub target_seconds: u32,
    /// Fade transition flag
    /// Indicates whether a fade transition is currently active
    pub fade_transition: bool,
    /// Minimum duration to keep screen black during transition (seconds)
    pub black_screen_time: f32,
    /// Entity ID of the bonfire
    /// Used to determine estus flask restoration level
    /// 0 if not a bonfire rest
    pub bonfire_entity_id: u32,
    /// Whether to reset the world (respawn enemies, items, etc.)
    pub reset_world: bool,
    /// Whether to reset the main character (HP, FP, status effects)
    pub reset_main_character: bool,
    /// Whether to reset magic spell charges
    pub reset_magic_charges: bool,
    /// Whether to restore estus flasks based on bonfire level
    pub restore_estus: bool,
    /// Whether to show the clock UI during time transition
    pub show_clock: bool,
    /// Delay before clock UI starts animating (seconds)
    pub clock_startup_delay_s: f32,
    /// Duration of clock hand movement animation (seconds)
    pub clock_move_time_s: f32,
    /// Delay after clock finishes animating before continuing (seconds)
    pub clock_finish_delay_s: f32,
    /// Duration of fade out transition (seconds)
    pub fade_out_time: f32,
    /// Duration of fade in transition (seconds)
    pub fade_in_time: f32,
    /// Whether a fade out has been requested
    /// Set to true to trigger time transition sequence
    pub fade_out_requested: bool,
    /// Accumulated time during time application phase (seconds)
    /// Used to track how long the world has been processing time change
    pub update_elapsed_time: f32,
    /// Accumulated time during black screen phase (seconds)
    /// Compared against `black_screen_time` to ensure minimum duration
    pub black_screen_elapsed_time: f32,
    /// Flag indicating Lua event respawn is pending
    /// Cleared when respawn processing completes or times out
    pub respawn_wait_flag: bool,
    unked: bool,
    unkee: bool,
    unkef: bool,
    unkf0: bool,
    /// Total elapsed time since transition started (seconds)
    /// Accumulates through all phases of the time change
    pub total_elapsed_time: f32,
    /// Timeout threshold for respawn wait phase (seconds)
    /// If exceeded, transition continues even if respawn incomplete
    pub black_screen_timeout: f32,
}

impl CSEventWorldAreaTimeCtrl {
    pub fn fade_out_and_pass_time(&mut self, params: TimeTransitionParams) {
        if self.fade_out_requested {
            return;
        }

        self.fade_out_requested = true;

        if let Some(wat) = (unsafe { get_instance::<WorldAreaTime>() }) {
            let current_total_seconds = (wat.clock.hours() as i64 * 3600)
                + (wat.clock.minutes() as i64 * 60)
                + wat.clock.seconds() as i64;

            let delta_total_seconds = (params.add_hours as i64 * 3600)
                + (params.add_minutes as i64 * 60)
                + params.add_seconds as i64;

            let normalized_seconds =
                ((current_total_seconds + delta_total_seconds) % 86400 + 86400) % 86400;

            self.target_hours = (normalized_seconds / 3600) as u32;
            self.target_minutes = ((normalized_seconds % 3600) / 60) as u32;
            self.target_seconds = (normalized_seconds % 60) as u32;
        }

        self.clock_startup_delay_s = params.clock_startup_delay_s;
        self.clock_move_time_s = params.clock_move_time_s;
        self.clock_finish_delay_s = params.clock_finish_delay_s;
        self.black_screen_time = params.black_screen_time;
        self.fade_transition = true;
        self.show_clock = false;

        self.bonfire_entity_id = params.bonfire_entity_id;

        self.reset_world = params.reset_world;
        self.reset_main_character = params.reset_main_character;
        self.reset_magic_charges = params.reset_magic_charges;
        self.restore_estus = params.restore_estus;
        self.fade_out_time = params.fade_out_time;
        self.fade_in_time = params.fade_in_time;
    }
}

#[derive(Debug, Clone)]
pub struct TimeTransitionParams {
    pub add_hours: i32,
    pub add_minutes: i32,
    pub add_seconds: i32,
    pub black_screen_time: f32,
    pub bonfire_entity_id: u32,
    pub reset_world: bool,
    pub reset_main_character: bool,
    pub reset_magic_charges: bool,
    pub restore_estus: bool,
    pub clock_startup_delay_s: f32,
    pub clock_move_time_s: f32,
    pub clock_finish_delay_s: f32,
    pub fade_out_time: f32,
    pub fade_in_time: f32,
}

impl Default for TimeTransitionParams {
    fn default() -> Self {
        Self {
            add_hours: 0,
            add_minutes: 0,
            add_seconds: 0,
            black_screen_time: 1.5,
            bonfire_entity_id: 0,
            reset_world: false,
            reset_main_character: false,
            reset_magic_charges: false,
            restore_estus: false,
            clock_startup_delay_s: 0.0,
            clock_move_time_s: 0.0,
            clock_finish_delay_s: 0.0,
            fade_out_time: 0.75,
            fade_in_time: 0.5,
        }
    }
}

impl TimeTransitionParams {
    pub fn bonfire_rest() -> Self {
        Self {
            reset_world: true,
            reset_main_character: true,
            reset_magic_charges: true,
            restore_estus: true,
            ..Default::default()
        }
    }

    pub fn time_skip(hours: i32, minutes: i32, seconds: i32) -> Self {
        Self {
            add_hours: hours,
            add_minutes: minutes,
            add_seconds: seconds,
            ..Default::default()
        }
    }
}
