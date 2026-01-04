use std::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};

use vtable_rs::VPtr;

#[vtable_rs::vtable]
pub trait DLAllocatorVmt {
    fn destructor(&mut self, param_2: bool);

    fn unk08(&self);
    fn unk10(&self);
    fn unk18(&self);
    fn unk20(&self);
    fn unk28(&self);
    fn unk30(&self);
    fn unk38(&self);
    fn unk40(&self);

    fn allocate(&self, size: usize) -> *mut u8;

    fn allocate_aligned(&self, size: usize, alignment: usize) -> *mut u8;

    fn unk58(&self);
    fn unk60(&self);

    fn deallocate(&self, allocation: *mut u8);
}

#[repr(transparent)]
pub struct DLAllocatorBase {
    pub vftable: VPtr<dyn DLAllocatorVmt, Self>,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct DLAllocatorRef(NonNull<DLAllocatorBase>);

impl From<NonNull<DLAllocatorBase>> for DLAllocatorRef {
    fn from(ptr: NonNull<DLAllocatorBase>) -> Self {
        Self(ptr)
    }
}

unsafe impl GlobalAlloc for DLAllocatorRef {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let allocator = unsafe { self.0.as_ref() };
        (allocator.vftable.allocate_aligned)(allocator, layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let allocator = unsafe { self.0.as_ref() };
        (allocator.vftable.deallocate)(allocator, ptr);
    }
}
