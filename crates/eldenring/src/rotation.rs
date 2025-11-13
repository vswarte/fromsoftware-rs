use nalgebra_glm as glm;
use std::fmt::Display;

use fromsoftware_shared::F32Vector4;

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion(pub f32, pub f32, pub f32, pub f32);

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EulerAngles(pub f32, pub f32, pub f32);

impl Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Quaternion({}, {}, {}, {})",
            self.0, self.1, self.2, self.3
        )
    }
}

impl Quaternion {
    pub fn to_euler_angles(&self) -> EulerAngles {
        let euler = glm::quat_euler_angles(&glm::quat(self.0, self.1, self.2, self.3));

        EulerAngles(euler.x, euler.y, euler.z)
    }
}

impl From<Quaternion> for glm::Quat {
    fn from(val: Quaternion) -> Self {
        glm::quat(val.0, val.1, val.2, val.3)
    }
}
