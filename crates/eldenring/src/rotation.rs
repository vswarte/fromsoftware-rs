use glam::EulerRot;
use std::fmt::{self, Display};

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Rotation as a Quanternion in format XYZW.
pub struct Quaternion(pub f32, pub f32, pub f32, pub f32);

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Rotation described by axis angles (pitch, yaw, roll).
pub struct EulerAngles(pub f32, pub f32, pub f32);

impl Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Display for EulerAngles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Quaternion {
    #[inline]
    pub fn to_euler_angles(&self) -> EulerAngles {
        let (z, x, y) = glam::Quat::from(*self).to_euler(EulerRot::ZXY);
        EulerAngles(x, y, z)
    }
}

impl From<Quaternion> for glam::Quat {
    #[inline]
    fn from(Quaternion(x, y, z, w): Quaternion) -> Self {
        Self::from_xyzw(x, y, z, w)
    }
}
