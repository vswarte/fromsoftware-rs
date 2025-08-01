use std::ptr::NonNull;

use shared::{FSMatrix4x4, FSVector4, OwnedPtr};

use super::ChrIns;

#[repr(C)]
/// Source of name: RTTI
#[dlrf::singleton("CSCamera")]
pub struct CSCamera {
    vftable: usize,
    pub pers_cam_1: OwnedPtr<CSPersCam>,
    pub pers_cam_2: OwnedPtr<CSPersCam>,
    pub pers_cam_3: OwnedPtr<CSPersCam>,
    pub pers_cam_4: OwnedPtr<CSPersCam>,

    // 0b00100000 // Copy from pers_cam_4 into pers_cam_1
    // 0b00010000 // Copy from pers_cam_3 into pers_cam_1
    // 0b00001000 // Copy from pers_cam_2 into pers_cam_1
    // 0b00000100 // Copy from pers_cam_4 into pers_cam_1
    // 0b00000010 // Copy from pers_cam_4 into pers_cam_1
    // 0b00000001 // Copy from pers_cam_2 into pers_cam_1
    pub camera_mask: u32,

    unk2c: u32,
    unk30: usize,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSCam {
    vftable: usize,
    unk8: u32,
    unkc: u32,
    pub matrix: FSMatrix4x4,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

pub type CSPersCam = CSCam;

#[repr(C)]
pub struct ChrCam {
    pub pers_cam: CSPersCam,
    ex_follow_cam: OwnedPtr<CSPersCam>,
    aim_cam: OwnedPtr<CSPersCam>,
    dist_view_cam: OwnedPtr<CSPersCam>,
    /// Setting this to True will reset the camera to the default position.
    /// (behind player's back)
    pub request_camera_reset: bool,
    _unk79: [u8; 0x3],
    pub camera_type: ChrCamType,
    _unk80: [u8; 0xc],
    pub pad_accelleration: FSVector4,
    pub move_accelleration: FSVector4,
    _unkb0: f32,
    pub is_movement_locked: bool,
    _unkb5: [u8; 0x11b],
    _position_recorder: [u8; 0xf20],
    _unk10f0: usize,
    _unk10f8: [u8; 0x30],
    pub death_cam_target: Option<NonNull<ChrIns>>,
}

#[repr(u32)]
pub enum ChrCamType {
    Unk0 = 0,
    Unk1 = 1,
    Unk2 = 2,
    Unk3 = 3,
    Unk4 = 4,
    Unk5 = 5,
    Unk6 = 6,
    DeathCam = 7,
}
