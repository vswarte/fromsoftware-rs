use shared::{F32ViewMatrix, OwnedPtr, self};

use crate::position::{HavokPosition, PositionDelta};

#[repr(C)]
#[shared::singleton("CSCamera")]
/// Source of name: RTTI
pub struct CSCamera {
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
    pub matrix: F32ViewMatrix,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

impl CSCam {
    pub fn right(&self) -> PositionDelta {
        PositionDelta(
            self.matrix.0.0,
            self.matrix.0.1,
            self.matrix.0.2,
        )
    }

    pub fn up(&self) -> PositionDelta {
        PositionDelta(
            self.matrix.1.0,
            self.matrix.1.1,
            self.matrix.1.2,
        )
    }

    pub fn forward(&self) -> PositionDelta {
        PositionDelta(
            self.matrix.2.0,
            self.matrix.2.1,
            self.matrix.2.2,
        )
    }

    pub fn position(&self) -> HavokPosition {
        HavokPosition(
            self.matrix.3.0,
            self.matrix.3.1,
            self.matrix.3.2,
            self.matrix.3.3,
        )
    }
}

pub type CSPersCam = CSCam;
