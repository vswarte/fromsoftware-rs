use std::mem;

use glam::{Mat3A, Mat4, Vec3A, Vec4};

use crate::{F32Matrix4x4, F32Vector4};

/// Row-major 4x4 float model matrix.
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F32ModelMatrix(
    pub F32Vector4,
    pub F32Vector4,
    pub F32Vector4,
    pub F32Vector4,
);

/// Row-major 4x4 float view matrix.
pub type F32ViewMatrix = F32ModelMatrix;

/// Row-major 4x4 float model matrix packed as a column major 4x3.
///
/// The 4x4 is packed by discarding the 4th component of each row.
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F32PackedModelMatrix(pub F32Vector4, pub F32Vector4, pub F32Vector4);

impl F32ModelMatrix {
    /// Construct from an array of row vectors.
    #[inline]
    pub fn new(r0: F32Vector4, r1: F32Vector4, r2: F32Vector4, r3: F32Vector4) -> Self {
        Self(r0, r1, r2, r3)
    }

    /// Extract the rotation matrix.
    #[inline]
    pub fn rotation<T: From<Mat3A>>(&self) -> T {
        let m =
            Mat4::from_cols(self.0.into(), self.1.into(), self.2.into(), self.3.into()).transpose();
        Mat3A::from_mat4(m).into()
    }

    /// Extract the translation vector.
    #[inline]
    pub fn translation<T: From<Vec3A>>(&self) -> T {
        Vec3A::from_vec4(self.3.into()).into()
    }
}

impl F32PackedModelMatrix {
    /// Pack from an array of row vectors.
    #[inline]
    pub fn new(r0: F32Vector4, r1: F32Vector4, r2: F32Vector4, r3: F32Vector4) -> Self {
        let t = Mat4::from_cols(r0.into(), r1.into(), r2.into(), r3.into()).transpose();
        Self(t.x_axis.into(), t.y_axis.into(), t.z_axis.into())
    }

    /// Extract the rotation matrix.
    #[inline]
    pub fn rotation<T: From<glam::Mat3A>>(&self) -> T {
        let m = Mat4::from_cols(self.0.into(), self.1.into(), self.2.into(), Vec4::ZERO);
        Mat3A::from_mat4(m).into()
    }

    /// Extract the translation vector.
    #[inline]
    pub fn translation<T: From<glam::Vec3A>>(&self) -> T {
        Vec3A::from_vec4(self.w_axis().into()).into()
    }

    /// Extract the x axis.
    #[inline]
    pub fn x_axis(&self) -> F32Vector4 {
        F32Vector4(self.0.0, self.1.0, self.2.0, 0.0)
    }

    /// Extract the y axis.
    #[inline]
    pub fn y_axis(&self) -> F32Vector4 {
        F32Vector4(self.0.1, self.1.1, self.2.1, 0.0)
    }

    /// Extract the z axis.
    #[inline]
    pub fn z_axis(&self) -> F32Vector4 {
        F32Vector4(self.0.2, self.1.2, self.2.2, 0.0)
    }

    /// Extract the w axis.
    #[inline]
    pub fn w_axis(&self) -> F32Vector4 {
        F32Vector4(self.0.3, self.1.3, self.2.3, 1.0)
    }
}

impl From<F32ModelMatrix> for F32PackedModelMatrix {
    #[inline]
    fn from(F32ModelMatrix(r0, r1, r2, r3): F32ModelMatrix) -> Self {
        Self::new(r0, r1, r2, r3)
    }
}

impl From<F32ModelMatrix> for F32Matrix4x4 {
    #[inline]
    fn from(F32ModelMatrix(r0, r1, r2, r3): F32ModelMatrix) -> Self {
        Self(r0, r1, r2, r3)
    }
}

impl From<F32ModelMatrix> for Mat4 {
    #[inline]
    fn from(F32ModelMatrix(r0, r1, r2, r3): F32ModelMatrix) -> Self {
        let mut m = Self::from_cols(r0.into(), r1.into(), r2.into(), r3.into());
        let w_axis = mem::replace(&mut m.w_axis, Vec4::W);

        Self {
            w_axis,
            ..m.transpose()
        }
    }
}

impl From<F32PackedModelMatrix> for F32ModelMatrix {
    #[inline]
    fn from(F32PackedModelMatrix(c0, c1, c2): F32PackedModelMatrix) -> Self {
        let t = Mat4::from_cols(c0.into(), c1.into(), c2.into(), Vec4::W).transpose();

        Self(
            t.x_axis.into(),
            t.y_axis.into(),
            t.z_axis.into(),
            t.w_axis.into(),
        )
    }
}

impl From<F32PackedModelMatrix> for F32Matrix4x4 {
    #[inline]
    fn from(m: F32PackedModelMatrix) -> Self {
        F32ModelMatrix::from(m).into()
    }
}

impl From<F32PackedModelMatrix> for Mat4 {
    #[inline]
    fn from(F32PackedModelMatrix(c0, c1, c2): F32PackedModelMatrix) -> Self {
        let x_axis = Vec4::from(c0).with_w(0.0);
        let y_axis = Vec4::from(c1).with_w(0.0);
        let z_axis = Vec4::from(c2).with_w(0.0);
        let w_axis = Vec4::new(c0.3, c1.3, c2.3, 1.0);

        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }
}

impl From<F32Matrix4x4> for F32ModelMatrix {
    #[inline]
    fn from(F32Matrix4x4(r0, r1, r2, r3): F32Matrix4x4) -> Self {
        Self(r0, r1, r2, r3)
    }
}

impl From<F32Matrix4x4> for F32PackedModelMatrix {
    #[inline]
    fn from(m: F32Matrix4x4) -> Self {
        F32ModelMatrix::from(m).into()
    }
}

impl From<Mat4> for F32ModelMatrix {
    #[inline]
    fn from(mut m: Mat4) -> Self {
        let w_axis = mem::replace(&mut m.w_axis, Vec4::W);
        let t = m.transpose();

        Self(
            t.x_axis.into(),
            t.y_axis.into(),
            t.z_axis.into(),
            w_axis.into(),
        )
    }
}

impl From<Mat4> for F32PackedModelMatrix {
    #[inline]
    fn from(m: Mat4) -> Self {
        let x_axis = m.x_axis.with_w(m.w_axis.x).into();
        let y_axis = m.y_axis.with_w(m.w_axis.y).into();
        let z_axis = m.z_axis.with_w(m.w_axis.z).into();

        Self(x_axis, y_axis, z_axis)
    }
}
