use std::ptr::NonNull;

use shared::F32Vector4;

use crate::{cs::ChrIns, fd4::FD4Time};

#[repr(C)]
pub struct CSChrBehaviorModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: usize,
    unk18: usize,
    unk20: usize,
    unk28: usize,
    pub root_motion: F32Vector4,
    unk40: [u8; 0x20],
    unk60: [u8; 0xa48],
    unkaa8: [u8; 0x58],
    unkb00: [u8; 0xa48],
    unk1548: [u8; 0x68],
    unk15b0: FD4Time,
    unk15c0: [u8; 0xc0],
    /// controls IK
    /// Set to -1 by TAE Event 0 ChrActionFlag (action 28 DISABLE_FOOT_IK)
    pub ground_touch_state: u32,
    /// Read from NpcParam, PI by default.
    pub max_ankle_pitch_angle_rad: f32,
    /// Read from NpcParam, PI by default.
    pub max_ankle_roll_angle_rad: f32,
    unk168c: [u8; 0x104],
    unk1790: F32Vector4,
    unk17a0: [u8; 0x10],
    chr_behavior_debug_anim_helper: usize,
    unk17b8: [u8; 0x10],
    pub animation_speed: f32,
    unk17cc: [u8; 0x1f4],
}
