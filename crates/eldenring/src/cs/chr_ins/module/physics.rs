use bitfield::bitfield;
use shared::{F32ModelMatrix, F32Vector4};
use std::ptr::NonNull;

use crate::{
    cs::{CSChrDataModule, ChrIns, PlayerGameData},
    fd4::FD4Time,
    position::HavokPosition,
    rotation::Quaternion,
};

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrPhysicsModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: NonNull<ChrIns>,
    unk18: [u8; 0x8],
    pub data_module: NonNull<CSChrDataModule>,
    unk28: [u8; 0x28],
    pub orientation: Quaternion,
    /// Rotation, controlled by specifics of the character's movement,
    /// can be changed by tae and interpolated towards the target rotation
    pub interpolated_orientation: Quaternion,
    pub position: HavokPosition,
    pub last_update_position: HavokPosition,
    unk90: bool,
    pub chr_proxy_pos_update_requested: bool,
    pub standing_on_solid_ground: bool,
    pub touching_solid_ground: bool,
    unk94: [u8; 0x4],
    chr_proxy: usize,
    chr_proxy2: usize,
    unka8: [u8; 0x8],
    hk_collision_shape: usize,
    unkb8: [u8; 0x10],
    unkc8: f32,
    pub adjust_to_hi_collision: bool,
    unkcd: [u8; 0x3],
    root_motion: F32Vector4,
    root_motion_unk: F32Vector4,
    unkf0: F32Vector4,
    unk100: [u8; 0x4],
    pub chr_push_up_factor: f32,
    ground_offset: f32,
    ground_offset_unk: f32,
    unk110: [u8; 0x10],
    gravity: F32Vector4,
    gravity_unk: F32Vector4,
    unk140: [u8; 0x10],
    unk150: F32Vector4,
    unk160: F32Vector4,
    unk170: F32Vector4,
    unk180: F32Vector4,
    pub additional_rotation: F32Vector4,
    unk1a0: [u8; 0x8],
    unk1a8: FD4Time,
    unk1b8: f32,
    unk1bc: f32,
    /// Set by TAE Event 0 ChrActionFlag
    /// (action 124 EnableRotationInterpolationMultiplier or 125 SnapToTargetRotation)
    /// Controls how much the character's rotation is interpolated towards the target rotation.
    pub rotation_multiplier: f32,
    pub motion_multiplier: f32,
    unk1c8: [u8; 0x4],
    pub gravity_multiplier: f32,
    pub is_falling: bool,
    pub is_touching_ground: bool,
    unk1d2: u8,
    /// Fade out disable gravity
    /// Set when PlayerIns.tint_alpha_multiplier is at 0 and ChrInsFlags1c7 bit 7 is set
    pub fade_out_gravity_disabled: bool,
    unk1d4: u8,
    /// Set by TAE Event 0 ChrActionFlag (action 27 DISABLE_GRAVITY)
    pub gravity_disabled: bool,
    unk1d6: u8,
    unk1d7: u8,
    unk1d8: u8,
    /// Set by TAE Event 0 ChrActionFlag (action 38 FLYING_CHARACTER_FALL)
    pub flying_character_fall_requested: bool,
    unk1da: u8,
    /// Should the character's rotation use world Y alignment logic.
    pub use_world_y_alignment_logic: bool,
    pub is_surface_constrained: bool,
    unk1dd: [u8; 0x4],
    /// Only true for Watcher Stones character (stone sphere catapillars).
    pub is_watcher_stones: bool,
    unk1e2: [u8; 0xe],
    /// Information about the material the character is currently on
    pub material_info: ChrPhysicsMaterialInfo,
    /// Information about character's sliding state
    pub slide_info: ChrPhysicsSlideInfo,
    unk290: [u8; 0x30],
    unkposition: F32Vector4,
    pub orientation_euler: F32Vector4,
    pub chr_hit_height: f32,
    pub chr_hit_radius: f32,
    unk2e8: [u8; 0x8],
    pub hit_height: f32,
    pub hit_radius: f32,
    unk2f8: [u8; 0x8],
    pub weight: f32,
    unk304: f32,
    unk308: f32,
    unk30c: [u8; 0x4],
    chr_push_up_factor2: f32,
    pub default_max_turn_rate: f32,
    unk318: f32,
    unk31c: [u8; 0x4],
    pub move_type_flags: MoveTypeFlags,
    unk324: [u8; 0x4],
    pub player_game_data: Option<NonNull<PlayerGameData>>,
    unk330: f32,
    unk334: f32,
    unk338: [u8; 0x8],
    unk340: f32,
    unk344: f32,
    unk348: [u8; 0x48],
    hk_frame_data: usize,
    unk398: f32,
    unk39c: [u8; 0x4],
    unk3a0: F32Vector4,
    unk3b0: [u8; 0x10],
    unk3c0: F32Vector4,
    unk3d0: F32Vector4,
    unk3e0: [u8; 0x6],
    /// Loaded from NpcParam
    pub is_enable_step_disp_interpolate: bool,
    unk3e7: u8,
    /// Loaded from NpcParam
    pub step_disp_interpolate_time: f32,
    /// Loaded from NpcParam
    pub step_disp_interpolate_trigger_value: f32,
    unk3f0: u8,
    no_gravity_unk2: bool,
    unk3f2: u8,
    pub debug_draw_orientation: bool,
    unk3f4: u8,
    unk3f5: u8,
    pub debug_draw_character_slope_capsule: bool,
    unk3f7: [u8; 0x29],
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MoveTypeFlags(u8);
    impl Debug;
    pub use_world_y_alignment, set_use_world_y_alignment:   0;
    pub is_surface_constrained, set_is_surface_constrained: 1;
    pub is_pad_manipulated, set_is_pad_manipulated:         4;
}

#[repr(C)]
pub struct ChrPhysicsMaterialInfo {
    /// Local orientation matrix (right, up, forward, 1)
    pub orientation_matrix: F32ModelMatrix,
    /// Normal vector of the hit surface
    pub normal_vector: F32Vector4,
    unk50: [u8; 8],
    unk58: i32,
    unk5c: i32,
    /// Material ID of the surface the character is standing on
    pub hit_material: i32,
    /// True when hit_material / 100 = 1
    /// completely disables sliding
    pub is_non_slippery_surface: bool,
    /// True when hit_material / 100 = 4 or 5
    /// makes so sliding calculations ignore ChrPhysicsSlideInfo.max_slide_angle
    /// and makes character always slide
    pub is_slippery_surface: bool,
    unk66: bool,
}

#[repr(C)]
pub struct ChrPhysicsSlideInfo {
    /// Slide direction vector
    pub slide_vector: F32Vector4,
    /// Information about the material the character is standing on
    pub material_info: NonNull<ChrPhysicsMaterialInfo>,
    /// Angle (radians) of the slope the character is currently on
    pub normal_angle: f32,
    /// Is character currently sliding?
    pub is_sliding: bool,
    /// Enable angle checks for sliding
    pub enable_angle_check: bool,
    /// Angle (degrees) of the slope the character is currently on, derived from normal_angle
    pub normal_angle_deg: f32,
    /// Enable/disable sliding
    /// false when CSChrPhysicsModule.no_gravity_unk2 is true
    pub enabled: bool,
    /// When true use smoothed/interpolated slide updates; when false use immediate additive updates
    pub enable_slide_interpolation: bool,
}
