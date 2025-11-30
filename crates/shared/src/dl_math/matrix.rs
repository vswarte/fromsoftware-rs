use crate::{F32Vector2, F32Vector3, F32Vector4};

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Row-major 4x4 float matrix.
pub struct F32Matrix4x4(
    pub F32Vector4,
    pub F32Vector4,
    pub F32Vector4,
    pub F32Vector4,
);

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Column-major 4x3 float matrix.
pub struct F32Matrix4x3(pub F32Vector4, pub F32Vector4, pub F32Vector4);

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Column-major 4x2 float matrix.
pub struct F32Matrix4x2(
    pub F32Vector2,
    pub F32Vector2,
    pub F32Vector2,
    pub F32Vector2,
);

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Row-major 3x4 float matrix.
pub struct F32Matrix3x4(pub F32Vector4, pub F32Vector4, pub F32Vector4);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Row-major 3x3 float matrix.
pub struct F32Matrix3x3(pub F32Vector3, pub F32Vector3, pub F32Vector3);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Column-major 3x2 float matrix.
pub struct F32Matrix3x2(pub F32Vector2, pub F32Vector2, pub F32Vector2);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Row-major 2x4 float matrix.
pub struct F32Matrix2x4(pub F32Vector4, pub F32Vector4);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Row-major 2x3 float matrix.
pub struct F32Matrix2x3(pub F32Vector3, pub F32Vector3);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Row-major 2x2 float matrix.
pub struct F32Matrix2x2(pub F32Vector2, pub F32Vector2);

macro_rules! impl_matrix_new {
    ($MatrixType:ident, $VectorType:ident, $($param:ident),+) => {
        impl $MatrixType {
            /// Construct from an array of row/column vectors.
            #[inline]
            pub fn new($($param: $VectorType),+) -> Self {
                Self($($param),+)
            }
        }
    };
}

impl_matrix_new!(F32Matrix4x4, F32Vector4, r0, r1, r2, r3);
impl_matrix_new!(F32Matrix4x3, F32Vector4, c0, c1, c2);
impl_matrix_new!(F32Matrix4x2, F32Vector2, c0, c1, c2, c3);
impl_matrix_new!(F32Matrix3x4, F32Vector4, r0, r1, r2);
impl_matrix_new!(F32Matrix3x3, F32Vector3, r0, r1, r2);
impl_matrix_new!(F32Matrix3x2, F32Vector2, c0, c1, c2);
impl_matrix_new!(F32Matrix2x4, F32Vector4, r0, r1);
impl_matrix_new!(F32Matrix2x3, F32Vector3, c0, c1);
impl_matrix_new!(F32Matrix2x2, F32Vector2, r0, r1);

impl From<F32Matrix4x4> for glam::Mat4 {
    #[inline]
    fn from(F32Matrix4x4(r0, r1, r2, r3): F32Matrix4x4) -> Self {
        Self::from_cols(r0.into(), r1.into(), r2.into(), r3.into()).transpose()
    }
}

impl From<F32Matrix3x3> for glam::Mat3 {
    #[inline]
    fn from(F32Matrix3x3(r0, r1, r2): F32Matrix3x3) -> Self {
        Self::from_cols(r0.into(), r1.into(), r2.into()).transpose()
    }
}

impl From<F32Matrix3x3> for glam::Mat3A {
    #[inline]
    fn from(F32Matrix3x3(r0, r1, r2): F32Matrix3x3) -> Self {
        Self::from_cols(r0.into(), r1.into(), r2.into()).transpose()
    }
}

impl From<glam::Mat4> for F32Matrix4x4 {
    #[inline]
    fn from(m: glam::Mat4) -> Self {
        let t = m.transpose();

        Self(
            t.x_axis.into(),
            t.y_axis.into(),
            t.z_axis.into(),
            t.w_axis.into(),
        )
    }
}

impl From<glam::Mat3> for F32Matrix3x3 {
    #[inline]
    fn from(m: glam::Mat3) -> Self {
        let m = m.transpose();
        Self(m.x_axis.into(), m.y_axis.into(), m.z_axis.into())
    }
}

impl From<glam::Mat3A> for F32Matrix3x3 {
    #[inline]
    fn from(m: glam::Mat3A) -> Self {
        let m = m.transpose();
        Self(m.x_axis.into(), m.y_axis.into(), m.z_axis.into())
    }
}
