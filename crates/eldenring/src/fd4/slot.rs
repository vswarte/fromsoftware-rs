use std::ptr::NonNull;

use vtable_rs::VPtr;

use crate::{dlkr::DLAllocatorBase, Vector};

pub struct FD4SlotInsBase {
    pub vftable: VPtr<dyn FD4SlotInsBaseVmt, Self>,
}

#[vtable_rs::vtable]
pub trait FD4SlotInsBaseVmt {
    fn destructor(&mut self, param_2: bool);

    fn unk8(&mut self) -> bool;

    fn unk10(&mut self);

    fn unk18(&mut self);

    fn unk20(&mut self) -> u32;
}

pub struct FD4SlotSysBase {
    pub vftable: VPtr<dyn FD4SlotInsBaseVmt, Self>,
    pub allocator: NonNull<DLAllocatorBase>,
    unk10: usize,
    unk18_vector: Vector<()>,
}

#[vtable_rs::vtable]
pub trait FD4SlotSysBaseVmt {
    fn destructor(&mut self, param_2: bool);

    fn unk8(&mut self, param_2: usize) -> bool;

    fn unk10(&mut self);

    fn unk18(&mut self);

    fn unk20(&mut self) -> u32;
}
