/// Defines some helper methods around dealing with math
use std::ops::{Add, Sub};

pub trait MatrixLayout {}

pub enum RowMajor {}
impl MatrixLayout for RowMajor {}

pub enum ColMajor {}
impl MatrixLayout for ColMajor {}

#[repr(C)]
#[derive(Debug)]
pub struct F32Matrix4x4<L: MatrixLayout = RowMajor>(
    pub F32Vector4,
    pub F32Vector4,
    pub F32Vector4,
    pub F32Vector4,
    std::marker::PhantomData<L>,
);

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct F32Vector4(pub f32, pub f32, pub f32, pub f32);

impl Sub<F32Vector4> for F32Vector4 {
    type Output = F32Vector4;

    fn sub(self, rhs: F32Vector4) -> Self::Output {
        F32Vector4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Add<F32Vector4> for F32Vector4 {
    type Output = F32Vector4;

    fn add(self, rhs: F32Vector4) -> Self::Output {
        F32Vector4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct F32Vector3(pub f32, pub f32, pub f32);
