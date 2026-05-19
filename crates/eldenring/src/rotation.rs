use glam::EulerRot;

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Rotation as a Quanternion in format XYZW.
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Rotation described by axis angles (pitch, yaw, roll).
pub struct EulerAngles {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl Quaternion {
    #[inline]
    pub fn to_euler_angles(&self) -> EulerAngles {
        let (roll, pitch, yaw) = glam::Quat::from(*self).to_euler(EulerRot::ZXY);
        EulerAngles { pitch, yaw, roll }
    }
}

impl From<Quaternion> for glam::Quat {
    #[inline]
    fn from(Quaternion { x, y, z, w }: Quaternion) -> Self {
        Self::from_xyzw(x, y, z, w)
    }
}
