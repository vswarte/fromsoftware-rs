use std::ptr::NonNull;

use shared::{F32Matrix4x4, F32Vector4, OwnedPtr};

use crate::position::{HavokPosition, PositionDelta};

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
    pub matrix: F32Matrix4x4,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

impl CSCam {
    pub fn right(&self) -> PositionDelta {
        PositionDelta(
            self.matrix.0 .0,
            self.matrix.0 .1,
            self.matrix.0 .2,
        )
    }

    pub fn up(&self) -> PositionDelta {
        PositionDelta(
            self.matrix.1 .0,
            self.matrix.1 .1,
            self.matrix.1 .2,
        )
    }

    pub fn forward(&self) -> PositionDelta {
        PositionDelta(
            self.matrix.2 .0,
            self.matrix.2 .1,
            self.matrix.2 .2,
        )
    }

    pub fn position(&self) -> HavokPosition {
        HavokPosition(
            self.matrix.3 .0,
            self.matrix.3 .1,
            self.matrix.3 .2,
            self.matrix.3 .3,
        )
    }
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
    unk79: [u8; 0x3],
    pub camera_type: ChrCamType,
    unk80: [u8; 0xc],
    pub pad_accelleration: F32Vector4,
    pub move_accelleration: F32Vector4,
    unkb0: f32,
    pub is_movement_locked: bool,
    unkb5: [u8; 0x11b],
    position_recorder: [u8; 0xf20],
    unk10f0: usize,
    unk10f8: [u8; 0x30],
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
