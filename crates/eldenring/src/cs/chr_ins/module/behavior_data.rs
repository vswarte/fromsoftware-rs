use std::ptr::NonNull;

use crate::{cs::ChrIns, dlut::DLFixedVector};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TwistModifierBehaviorData {
    pub up_limit_angle: f32,
    pub down_limit_angle: f32,
    pub right_limit_angle: f32,
    pub left_limit_angle: f32,
    pub up_minimum_angle: f32,
    pub down_minimum_angle: f32,
    pub right_minimum_angle: f32,
    pub left_minimum_angle: f32,
    pub modifier_id: i32,
    pub target_type: i32,
    pub rank: u8,
    /// Pointer to EnableTwistModifier_Params_t
    pub tae_event: NonNull<EnableTwistModifierArgs>,
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Tae Event Args for EnableTwistModifier (700)
pub struct EnableTwistModifierArgs {
    pub up_limit_angle: f32,
    pub down_limit_angle: f32,
    pub right_limit_angle: f32,
    pub left_limit_angle: f32,
    pub modifier_id: u32,
    pub target_type: u8,
    pub rank: u8,
    pub up_minimum_angle: f32,
    pub down_minimum_angle: f32,
    pub right_minimum_angle: f32,
    pub left_minimum_angle: f32,
}

#[repr(C)]
pub struct CSChrBehaviorDataModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub twist_modifiers: DLFixedVector<TwistModifierBehaviorData, 8>,
    pub min_twist_rank: i16,
    pub has_twist_modifier: bool,
    pub fixed_rotation_direction: bool,
    unk1e4: [u8; 0x5c],
    unk240: f32,
    unk244: f32,
    unk248: u8,
    unk249: [u8; 3],
    pub hks_root_motion_mult: f32,
    pub turn_speed: f32,
    unk254: f32,
    unk258: [u8; 0xb0],
    unk308: u64,
    pub hks_animation_speed_multiplier: f32,
    unk314: [u8; 0xc],
    unk320: i32,
    unk324: [u8; 0x1c],
}
