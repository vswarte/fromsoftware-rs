use crate::cs::ChrIns;
use std::ptr::NonNull;

use super::PlayerSessionHolder;

#[repr(C)]
#[shared::singleton("WorldChrManDbg")]
pub struct WorldChrManDbg {
    vftable: usize,
    unk8: u8,
    pub lod_level_debug_view: bool,
    unka: [u8; 0x32],
    /// Distance threshold determining whether characters receive LVL5 (every 5 FPS) updates
    /// rather than LVL30 (every 30 FPS) updates.
    pub close_omission_threshold: f32,
    /// Maximum distance at which offscreen characters still receive priority updates.
    /// Characters beyond this distance will only receive minimal updates.
    pub offscreen_omission_distance: f32,
    /// Distance threshold used in WorldChrMan_CalcOmissionLevel_End.
    /// Controls transition between update levels based on distance.
    pub omission_level_transition_distance: f32,
    /// Modifier for character update priority when the character was recently on screen.
    pub update_priority_modifier_onscreen_recent: f32,
    /// Modifier for character update priority when the character is on screen.
    pub update_priority_modifier_onscreen: f32,
    /// Modifier for character update priority when the character is an NPC.
    pub update_priority_modifier_npc: f32,
    /// Modifier for character update priority when character is turning its lower body.
    pub update_priority_modifier_lower_body_turn_requested: f32,
    /// Default modifier for character update priority.
    pub update_priority_default_modifier: f32,
    /// Modifier for character update priority when the character is moving.
    pub update_priority_modifier_is_moving: f32,
    unk60: [u8; 8],
    pub toughness_debug_view: bool,
    pub poise_debug_view: bool,
    unk6a: [u8; 2],
    unk6c: f32,
    unk70: f32,
    unk74: f32,
    unk78: f32,
    unk7c: f32,
    unk80: f32,
    unk84: f32,
    unk88: f32,
    unk8c: f32,
    unk90: u32,
    unk94: f32,
    unk98: f32,
    unk9c: f32,
    unka0: f32,
    unka4: u32,
    pub debug_manipulator: usize,
    pub player_session_holder: Option<NonNull<PlayerSessionHolder>>,
    pub cam_override_chr_ins: Option<NonNull<ChrIns>>,
    unkc0: [u8; 0x30],
    unkf0: u32,
    unkf4: [u8; 0x8],
    unkfc: u32,
    pub chr_load_state_debug_view: bool,
    /// Modifier for character activation threshold when the character is out of render distance.
    pub chr_activate_threshold_modifier_out_of_range: f32,
    /// Modifier for character activation threshold when the character's tag is visible on screen.
    pub chr_activate_threshold_modifier_visible_tag: f32,
    unk10c: [u8; 0x1c],
}

#[repr(C)]
#[derive(Debug)]
/// Usually located immediately after the `WorldChrManDbg` singleton.
/// Game also checks if WorldChrManDbg exists before accessing this struct.
pub struct WorldChrManDbgFlags {
    /// prevents death by setting HP to 1 when they are less than 0
    pub no_dead: bool,
    unk1: bool,
    /// deals 9999999 damage on every hit
    pub exterminate: bool,
    /// prevents consumption of usable items
    pub no_goods_consume: bool,
    /// prevents stamina consumption
    pub no_stamina_consume: bool,
    /// prevents FP consumption
    pub no_fp_consume: bool,
    /// prevents durability loss (leftover from Dark Souls)
    pub no_item_damage: bool,
    /// prevents spell consumption (leftover from Dark Souls)
    pub no_spells_consume: bool,
    unk8: bool,
    unk9: bool,
    /// prevents death of enemies, same as `no_dead`
    pub enemy_no_dead: bool,
    /// does the same as `no_fp_consume`
    pub no_fp_consume2: bool,
    /// prevents enemies from being hit
    pub enemy_no_hit: bool,
    /// prevents enemies from attacking
    pub enemy_no_attack: bool,
    /// prevents enemies from pursuing the player
    pub enemy_no_pursuit: bool,
    /// prevents enemies from moving
    pub enemy_no_move: bool,
    unk10: bool,
    /// prevents fp consumption by ashes of war
    pub no_aow_fp_consume: bool,
    /// same as `no_goods_consume` but for enemies (gives infinite heal flasks for npc invaders)
    pub enemy_no_goods_consume: bool,
    /// auto-parries all attacks for both player and enemies
    pub auto_parry: bool,
    /// disables enemy rendering
    pub enemy_no_draw: bool,
    /// replaces AOW attack animations with no FP versions
    pub no_fp_aow: bool,
    unk16: bool,
    unk17: bool,
    unk18: bool,
    unk19: bool,
    unk1a: u8,
    unk1b: u8,
    unk1c: i32,
    unk20: i32,
    unk24: u8,
    unk25: u8,
    unk26: u8,
    unk27: u8,
    unk28: bool,
    unk29: bool,
    unk2a: bool,
    unk2b: bool,
    unk2c: u8,
    unk2d: u8,
    pub hks_enemy_anim_speed_multiplier_enabled: bool,
    unk2f: bool,
    unk30: u8,
    unk31: u8,
    pub hks_player_anim_speed_multiplier_enabled: bool,
    unk33: bool,
    unk34: u8,
    unk35: u8,
    unk36: bool,
    unk37: u8,
    unk38: u8,
    unk39: u8,
    unk3a: u8,
    unk3b: u8,
    unk3c: i32,
    unk40: i32,
}
