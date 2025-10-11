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
            fn add(self, rhs: $t) -> Self::Output {
                $t($(self.$i + rhs.$i),+)
            }
        }

        impl Sub<$t> for $t {
            type Output = $t;
            fn sub(self, rhs: $t) -> Self::Output {
                $t($(self.$i - rhs.$i),+)
            }
        }
    };
}

impl_add_sub!(F32Vector4, 0, 1, 2, 3);
impl_add_sub!(F32Vector3, 0, 1, 2);
impl_add_sub!(F32Vector2, 0, 1);
