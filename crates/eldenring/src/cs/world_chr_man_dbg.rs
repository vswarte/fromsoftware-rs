use crate::cs::ChrIns;
use std::ptr::NonNull;

use super::PlayerSessionHolder;

#[repr(C)]
#[fromsoftware_shared::singleton("WorldChrManDbg")]
pub struct WorldChrManDbg {
    vftable: usize,
    unk8: u8,
    pub lod_level_debug_view: bool,
    unka: [u8; 0x12],
    /// Will be changed depending on current performance load.
    pub omission_update_num_type: OmissionUpdateNumType,
    /// Overrides `omission_update_num_type` if not -1 (None).
    pub omission_update_num_type_override: OmissionUpdateNumType,
    /// The budgets for characters that can receive high-detail (NORMAL) updates this frame.
    pub omission_update_num_near: OmissionUpdateNum,
    /// The budgets for characters that can receive medium-detail (LVL2) updates this frame.
    pub omission_update_num_far: OmissionUpdateNum,
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
    pub chr_update_state_reason_debug_view: bool,
    unk61: [u8; 4],
    /// Graphics.DbgDraw.ChrDbgDrawEnable
    pub chr_dbg_draw_enable: bool,
    unk66: u8,
    unk67: u8,
    pub toughness_debug_view: bool,
    pub poise_debug_view: bool,
    unk6a: u8,
    unk6b: u8,
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
    unkc0: [u8; 0x14],
    /// Game.Debug.IsEnableDefaultBonfireMenu
    pub is_enable_default_bonfire_menu: bool,
    unkd5: [u8; 0x1b],
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
/// Name source: debug properties OmissionUpdateNum.Normal.Near/Overload/Emergency
pub struct OmissionUpdateNum {
    pub normal: i32,
    pub overload: i32,
    pub emergency: i32,
}

#[repr(i32)]
/// Name source: debug properties OmissionUpdateNum.Normal.Near/Overload/Emergency
pub enum OmissionUpdateNumType {
    None = -1,
    Normal = 0,
    Overload = 1,
    Emergency = 2,
}

#[repr(C)]
#[derive(Debug)]
/// Usually located immediately after the `WorldChrManDbg` singleton.
/// Game also checks if WorldChrManDbg exists before accessing this struct.
pub struct WorldChrManDbgFlags {
    /// prevents death by setting HP to 1 when they are less than 0
    /// Read from debug property GameData.PlayerNoDead
    pub player_no_dead: bool,
    /// prevents death of the player's horse
    /// Read from debug property GameData.PlayerHorseNoDead
    pub player_horse_no_dead: bool,
    /// deals 9999999 damage on every hit
    /// Read from debug property GameData.PlayerExterminate
    pub player_exterminate: bool,
    /// prevents consumption of usable items
    /// Read from debug property GameData.PlayerNoGoodsConsume
    pub player_no_goods_consume: bool,
    /// prevents stamina consumption
    /// Read from debug property GameData.AllNoStaminaConsume
    pub all_no_stamina_consume: bool,
    /// prevents MP consumption
    /// Read from debug property GameData.AllNoMpConsume
    pub all_no_mp_consume: bool,
    /// prevents arrow consumption
    /// Read from debug property GameData.AllNoArrowConsume
    pub all_no_arrow_consume: bool,
    /// prevents spell consumption (leftover from Dark Souls)
    /// Read from debug property GameData.AllNoMagicQtyConsume
    pub all_no_magic_qty_consume: bool,
    unk8: bool,
    unk9: bool,
    /// prevents death of enemies, same as `no_dead`
    /// Read from debug property GameData.AllNoDead
    pub all_no_dead: bool,
    /// prevents enemies from being hit
    /// Read from debug property GameData.AllNoDamage
    pub all_no_damage: bool,
    /// prevents enemies from hitting the player
    /// Read from debug property GameData.AllNoHit
    pub all_no_hit: bool,
    /// prevents enemies from attacking the player
    /// Read from debug property GameData.AllNoAttack
    pub all_no_attack: bool,
    /// prevents enemies from moving
    /// Read from debug property GameData.AllNoMove
    pub all_no_move: bool,
    unkf: u8,
    /// prevents durability loss on weapons and protectors
    /// Read from debug property GameRule.IsDbgNotDurabilityLossWeaponProtector
    pub is_dbg_not_durability_loss_weapon_protector: bool,
    /// prevents fp consumption by ashes of war
    /// Read from debug property GameRule.IsNoArtsPointConsume
    pub is_no_arts_point_consume: bool,
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
    unk1c: i32,
    unk20: i32,
    unk24: u8,
    unk25: u8,
    unk26: u8,
    unk27: u8,
    unk28: bool,
    unk29: bool,
    unk2a: bool,
    /// Read from debug property Game.Debug.IsIgnoreChrDisableBackread
    pub is_ignore_chr_disable_backread: bool,
    unk2c: u8,
    /// Read from debug property GameData.TaeDebugEnableBehaviorFlag
    pub tae_debug_enable_behavior_flag: bool,
    /// Read from debug property GameData.TaeDebugEnableAnimePlaySpped
    pub tae_debug_enable_anime_play_spped: bool,
    /// Read from debug property GameData.TaeDebugEnableTestParam
    pub tae_debug_enable_test_param: bool,
    /// Read from debug property GameData.TaeDebugEnableMovementAdjustment
    pub tae_debug_enable_movement_adjustment: bool,
    /// Read from debug property GameData.TaeDebugPlayerEnableBehaviorFlag
    pub tae_debug_player_enable_behavior_flag: bool,
    /// Read from debug property GameData.TaeDebugPlayerEnableAnimePlaySpped
    pub tae_debug_player_enable_anime_play_spped: bool,
    /// Read from debug property GameData.TaeDebugPlayerEnableTestParam
    pub tae_debug_player_enable_test_param: bool,
    /// Read from debug property GameData.TaeDebugPlayerEnableMovementAdjustment
    pub tae_debug_player_enable_movement_adjustment: bool,
    unk35: u8,
    unk36: bool,
    unk37: u8,
    unk38: u8,
    unk39: u8,
    unk3c: i32,
    unk40: i32,
}
