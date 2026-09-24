use fromsoftware_shared_stl::{FnTarget, Function};
use num_enum::TryFromPrimitive;
use vtable_rs::VPtr;

use crate::{
    dlut::{DLFixedVector, DLReferenceCountObjectVmt},
    fd4::FD4Time,
};
use shared::OwnedPtr;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum MenuJobState {
    Continue = 1,
    Success = 2,
    Failed = 3,
}

#[repr(C)]
pub struct MenuJobResult {
    pub state: MenuJobState,
    unk4: i32,
}

#[vtable_rs::vtable]
pub trait MenuJobVmt: DLReferenceCountObjectVmt {
    fn run(&self, result: &mut MenuJobResult, unk: &mut FD4Time);
}

#[repr(C)]
pub struct MenuJobBase {
    pub vftable: VPtr<dyn MenuJobVmt, Self>,
    pub reference_count: u32,
    _padc: u32,
}

#[repr(C)]
pub struct MenuFunctorJob {
    pub base: MenuJobBase,
    pub callback: Function<fn(*mut MenuJobResult, *mut FD4Time), MenuFunctorJobCallableWrapper>,
}

#[repr(C)]
pub struct MenuFunctorJobCallableWrapper {
    pub func: Function<fn(*mut MenuJobResult, *mut FD4Time)>,
    pub result: MenuJobResult,
}

unsafe impl FnTarget for MenuFunctorJobCallableWrapper {}

#[repr(C)]
pub struct FixOrderJobSequenceBase {
    pub vftable: VPtr<dyn DLReferenceCountObjectVmt, Self>,
    pub reference_count: u32,
    _padc: u32,
    unk10: u32,
    _pad14: u32,
    pub jobs: DLFixedVector<OwnedPtr<MenuJobBase>, 8>,
}
