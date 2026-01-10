use crate::cs::ChrIns;
use bitfield::bitfield;
use std::ptr::NonNull;

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrActionFlagModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub animation_action_flags: ChrActionAnimationFlags,
    unk14: u32,
    unk18_flags: u32,
    /// Damage level of the last received attack.
    /// Determines how much character will stagger when hit.
    pub damage_level: u8,
    // pad1d: [u8; 0x3],
    /// Guard level from the params of the equipped weapon.
    pub guard_level: u32,
    unk24: [u8; 0x10],
    /// Param ID of the last received attack.
    pub received_damage_type: u32,
    unk38: [u8; 0x8],
    pub action_modifiers_flags: ChrActionModifiersFlags,
    unk48: u64,
    unk50: u64,
    unk58: [u8; 0x10],
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model0_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model0_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model1_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model1_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model2_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model2_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model3_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model3_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model0_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model0_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model1_absorp_pos_param_condition: u8,
    /// /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model1_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model2_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model2_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model3_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model3_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub weapon_model_location_overridden: bool,
    unk79: [u8; 0xb],
    /// Set by TAE Event 224 SetTurnSpeed
    pub turn_speed: f32,
    /// Set by TAE Event 706 ChrTurnSpeedForLock
    pub lock_on_turn_speed: f32,
    /// Set by TAE Event 717 SetJointTurnSpeed
    pub joint_turn_speed: f32,
    pub global_turn_speed_priority: i8,
    pub turn_speed_priority: i8,
    pub lock_on_turn_speed_priority: i8,
    pub joint_turn_speed_priority: i8,
    /// Set by TAE Event 704 ChrTurnSpeedEX
    pub speed_default: f32,
    /// Set by TAE Event 704 ChrTurnSpeedEX
    pub speed_extra: f32,
    /// Set by TAE Event 704 ChrTurnSpeedEX
    pub speed_boost: f32,
    unka0: f32,
    unka4: f32,
    /// Set by TAE Event 705 FacingAngleCorrection
    pub facing_angle_correction_rad: f32,
    /// Set by TAE Event 760 BoostRootMotionToReachTarget
    pub root_motion_div: f32,
    /// Set by TAE Event 760 BoostRootMotionToReachTarget
    pub root_motion_mult_min_dist: f32,
    /// Set by TAE Event 760 BoostRootMotionToReachTarget
    pub root_motion_mult_max_dist: f32,
    /// Set by TAE Event 760 BoostRootMotionToReachTarget
    pub root_motion_mult_angle_from_target: f32,
    /// Set by TAE Event 760 BoostRootMotionToReachTarget
    pub root_motion_mult_target_radius: f32,
    unkc0: [u8; 0x110],
    /// Set by TAE Event 0 ChrActionFlag (action 5 SET_PARRYABLE_WINDOW)
    pub unused_parry_window_arg: u8,
    unk1d1: [u8; 0xf],
    /// Angle for a cone to disable lock-on when character is inside it.
    /// Read from npc param [crate::param::NPC_PARAM_ST::disable_lock_on_ang]
    /// Only set for the Fire Giant in the second phase of the fight.
    pub disable_lock_on_angle: f32,
    /// Set by TAE Event 155 SetLockCamParamTarget \
    /// See [crate::param::LOCK_CAM_PARAM_ST]
    pub camera_lock_on_param_id: i32,
    unk1e8: [u8; 0x10],
    /// Set by TAE Event 800 SetMovementMultiplier
    pub mov_dist_multiplier: f32,
    /// Set by TAE Event 800 SetMovementMultiplier
    pub cam_turn_dist_multiplier: f32,
    /// Set by TAE Event 800 SetMovementMultiplier
    pub ladder_dist_multiplier: f32,
    /// Set by TAE Event 0 (3 SET_GUARD_TYPE)
    pub guard_behavior_judge_id: u32,
    /// Set by TAE Event 342 SetSaDurabilityMultiplier
    pub sa_durability_multiplier: f32,
    /// Set by TAE Event 511 SetSpEffectWetConditionDepth
    /// Controls what speffect will be applied by speffect param
    pub sp_effect_wet_condition_depth: SpEffectWetConditionDepth,
    unk20d: [u8; 0x7],
    unk214: u32,
    /// Set by TAE Event 0 ChrActionFlag (action 72 INVOKEKNOCKBACKVALUE)
    pub knockback_value: f32,
    pub action_flags: u32,
    unk220: [u8; 0x18],
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_up_limit: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_down_limit: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_right_limit: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_left_limit: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_up_dead_zone: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_down_dead_zone: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_right_dead_zone: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_left_dead_zone: i16,
    unk248: [u8; 0x10],
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpEffectWetConditionDepth {
    Default = 0,
    LowerBody = 1,
    FullBody = 2,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrActionAnimationFlags(u32);
    impl Debug;

    pub stay_state, set_stay_state: 0;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrActionModifiersFlags(u64);
    impl Debug;

    /// Set by TAE Event 0 ChrActionFlag (action 94 PERFECT_INVINCIBILITY)
    pub perfect_invincibility, set_perfect_invincibility:                                             0;
    /// Set by TAE Event 0 ChrActionFlag (action 8 FLAG_AS_DODGING)
    pub dodging, set_dodging:                                                                         1;
    /// Set by TAE Event 0 ChrActionFlag (action 68 INVINCIBLE_DURING_THROW_ATTACKER)
    pub invincible_during_throw_attacker, set_invincible_during_throw_attacker:                       2;
    /// Set by TAE Event 0 ChrActionFlag (action 67 INVINCIBLE_EXCLUDING_THROW_ATTACKS_DEFENDER)
    pub invincible_excluding_throw_attacks_defender, set_invincible_excluding_throw_attacks_defender: 3;
    /// Set by TAE Event 0 ChrActionFlag (action 132 JUMP_FRAMES_LOWER_BODY_IFRAMES)
    pub jump_frames_lower_body_iframes, set_jump_frames_lower_body_iframes:                           4;
    /// Set by TAE Event 0 ChrActionFlag (action 143 PVE_ONLY_IFRAMES)
    pub pve_only_iframes, set_pve_only_iframes:                                                       5;
    /// Set by TAE Event 0 ChrActionFlag (action 3 SET_GUARD_TYPE)
    pub guard_type_set, set_guard_type_set:                                                           6;
    /// Set by TAE Event 0 ChrActionFlag (action 18 CANCEL_THROW)
    pub cancel_throw, set_cancel_throw:                                                               7;
    /// Set by TAE Event 0 ChrActionFlag (action 24 SUPER_ARMOR)
    pub super_armor, set_super_armor:                                                                 8;
    /// Set by TAE Event 0 ChrActionFlag (action 72 INVOKEKNOCKBACK)
    pub invokeknockbackvalue, set_invokeknockbackvalue:                                               9;
    /// Set by TAE Event 0 ChrActionFlag (action 5 SET_PARRYABLE_WINDOW)
    /// When set, the character can be parried.
    pub parryable, set_parryable:                                                                     10;
    /// Set by TAE Event 0 ChrActionFlag (action 42 SWEETSPOT_DEAL_12_5_MORE_DAMAGE)
    pub take_12_5_percent_more_damage, set_take_12_5_percent_more_damage:                             11;
    /// Set by TAE Event 0 ChrActionFlag (action 59 WEAKSPOT_DEAL_20_LESS_DAMAGE)
    pub weakspot_deal_20_less_damage, set_weakspot_deal_20_less_damage:                               12;
    /// Set by TAE Event 0 ChrActionFlag (action 56 DISABLE_WALL_ATTACK_BOUND)
    pub disable_wall_attack_bound, set_disable_wall_attack_bound:                                     13;
    /// Set by TAE Event 0 ChrActionFlag (action 57 DISABLE_NPC_WALL_ATTACK_BOUND)
    pub disable_npc_wall_attack_bound, set_disable_npc_wall_attack_bound:                             14;
    /// Set by TAE Event 0 ChrActionFlag (action 7 DISABLE_TURNING)
    pub disable_turning, set_disable_turning:                                                         15;
    /// Set by TAE Event 704 ChrTurnSpeedEX
    /// Additionally sets speed_default, speed_boost and speed_extra on CSChrActionFlagModule
    pub turn_speed_modified, set_turn_speed_modified:                                                 16;
    /// Set by TAE Event 0 ChrActionFlag (action 96 SET_IMMORTALITY)
    pub set_immortality, set_set_immortality:                                                         17;
    /// Set by TAE 760 BoostRootMotionToReachTarget
    pub root_motion_multiplier_set, set_root_motion_multiplier_set:                                   19;
    /// Set by TAE 760 BoostRootMotionToReachTarget
    /// Depends on `enable` argument of the event.
    pub root_motion_multiplier_enabled, set_root_motion_multiplier_enabled:                           20;
    /// Set by TAE Event 0 ChrActionFlag (action 102 POISE_FORCED_BREAK)
    pub poise_forced_break, set_poise_forced_break:                                                   21;
    /// Set by TAE Event 705 FacingAngleCorrection
    pub facing_angle_correction_set, set_facing_angle_correction_set:                                 24;
    /// Set by TAE Event 197 DS3FadeOut
    pub fade_out_applied, set_fade_out_applied:                                                       25;
    /// Set by TAE Event 0 ChrActionFlag (action 109 CAN_DOUBLE_CAST_ENV_331)
    pub can_double_cast, set_can_double_cast:                                                         26;
    /// Set by TAE Event 790 DisableDefaultWeaponTrail
    pub disable_default_weapon_trail, set_disable_default_weapon_trail:                               27;
    /// Set by TAE Event 791 PartDamageAdditiveBlendInvalid
    pub part_damage_additive_blend_invalid, set_part_damage_additive_blend_invalid:                   31;
    /// Set by TAE Event 0 ChrActionFlag (action 110 DISABLE_DIRECTION_CHANGE)
    pub disable_direction_change, set_disable_direction_change:                                       32;
    /// Set by TAE Event 0 ChrActionFlag (action 114 ENHANCED_CAMERA_TRACKING)
    pub enhanced_camera_tracking, set_enhanced_camera_tracking:                                       33;
    /// Set by TAE Event 0 ChrActionFlag (action 111 AI_PARRY_POSSIBLE_STATE)
    pub ai_parry_possible_state, set_ai_parry_possible_state:                                         35;
    /// Set by TAE Event 0 ChrActionFlag (action 63 AI_PARRY_SIGNAL)
    pub ai_parry_signal, set_ai_parry_signal:                                                         36;
    /// Set by TAE Event 0 ChrActionFlag (action 119 TRYTOINVOKEFORCEPARRYMODE)
    pub force_parry_mode, set_force_parry_mode:                                                       37;
    /// Set by TAE Event 782 AiReplanningCtrlReset
    pub ai_replanning_ctrl_reset, set_ai_replanning_ctrl_reset:                                       39;
    /// Set by TAE Event 707 ManualAttackAiming
    pub manual_attack_aiming, set_manual_attack_aiming:                                               40;
    /// Set by TAE Event 332 WeaponArtWeaponStyleCheck
    pub weapon_art_weapon_style_check, set_weapon_art_weapon_style_check:                             41;
    /// Set by TAE Event 0 ChrActionFlag (action 53 DISABLE_FLOATING_GAUGE_DISPLAY)
    pub disable_floating_gauge_display, set_disable_floating_gauge_display:                           44;
    /// Set by TAE Event 238 SetBulletAimAngle
    /// Additionally, sets bullet_aim_angle limits on CSChrActionFlagModule
    pub bullet_aim_angle_set, set_bullet_aim_angle_set:                                               45;
    /// Set by TAE Event 781
    pub turn_lower_body, set_turn_lower_body:                                                         46;
}

#[repr(i8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WeaponModelChangeType {
    MoveToDefaultLocation = -1,
    MoveTo1HRightWeaponLocation = 0,
    MoveTo1HLeftWeaponLocation = 1,
    MoveTo2HRightWeaponLocation = 2,
    MoveToSheathedLocation = 3,
    MaintainPreviousChange = 4,
    WeaponIdHardcoded = 5,
    Unknown6 = 6,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<CSChrActionFlagModule>(), 0x258);
    }
}
