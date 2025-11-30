use std::ops::{Add, Sub};

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F32Vector4(pub f32, pub f32, pub f32, pub f32);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F32Vector3(pub f32, pub f32, pub f32);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F32Vector2(pub f32, pub f32);

macro_rules! impl_add_sub {
    ($t:ident, $($i:tt),+ $(,)?) => {
        impl Add<$t> for $t {
            type Output = $t;
            #[inline]
            fn add(self, rhs: $t) -> Self::Output {
                $t($(self.$i + rhs.$i),+)
            }
        }

        impl Sub<$t> for $t {
            type Output = $t;
            #[inline]
            fn sub(self, rhs: $t) -> Self::Output {
                $t($(self.$i - rhs.$i),+)
            }
        }
    };
}

impl_add_sub!(F32Vector4, 0, 1, 2, 3);
impl_add_sub!(F32Vector3, 0, 1, 2);
impl_add_sub!(F32Vector2, 0, 1);

impl From<F32Vector4> for glam::Vec4 {
    #[inline]
    fn from(F32Vector4(x, y, z, w): F32Vector4) -> Self {
        Self::new(x, y, z, w)
    }
}

impl From<F32Vector3> for glam::Vec3 {
    #[inline]
    fn from(F32Vector3(x, y, z): F32Vector3) -> Self {
        Self::new(x, y, z)
    }
}

impl From<F32Vector3> for glam::Vec3A {
    #[inline]
    fn from(F32Vector3(x, y, z): F32Vector3) -> Self {
        Self::new(x, y, z)
    }
}

impl From<F32Vector2> for glam::Vec2 {
    #[inline]
    fn from(F32Vector2(x, y): F32Vector2) -> Self {
        Self::new(x, y)
    }
}

impl From<glam::Vec4> for F32Vector4 {
    #[inline]
    fn from(v: glam::Vec4) -> Self {
        Self(v.x, v.y, v.z, v.w)
    }
}

impl From<glam::Vec3> for F32Vector3 {
    #[inline]
    fn from(v: glam::Vec3) -> Self {
        Self(v.x, v.y, v.z)
    }
}

impl From<glam::Vec3A> for F32Vector3 {
    #[inline]
    fn from(v: glam::Vec3A) -> Self {
        Self(v.x, v.y, v.z)
    }
}

impl From<glam::Vec2> for F32Vector2 {
    #[inline]
    fn from(v: glam::Vec2) -> Self {
        Self(v.x, v.y)
    }
}
