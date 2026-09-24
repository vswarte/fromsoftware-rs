use vtable_rs::VPtr;

use crate::{
    dlkr::MenuHeapAllocator,
    dlut::{DLFixedVector, DLReferenceCountObjectVmt},
    fd4::FD4Time,
};
use shared::OwnedPtr;

#[repr(C)]
pub struct MenuJobResult {}

#[vtable_rs::vtable]
pub trait MenuJobVmt: DLReferenceCountObjectVmt {
    fn run(&self, result: &mut MenuJobResult, unk: &mut FD4Time);
}

#[repr(C)]
pub struct MenuJobBase {
    pub vftable: VPtr<dyn MenuJobVmt, Self>,
    pub reference_count: u32,
}

#[repr(C)]
pub struct FixOrderJobSequence {
    pub vftable: VPtr<dyn DLReferenceCountObjectVmt, Self>,
    pub reference_count: u32,
    _padc: u32,
    unk10: u32,
    _pad14: u32,
    pub jobs: DLFixedVector<OwnedPtr<MenuJobBase, MenuHeapAllocator>, 8>,
}
