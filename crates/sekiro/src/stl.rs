use std::{ffi::c_void, ptr::NonNull};

use crate::dlkr::DLAllocatorBase;

#[derive(Clone)]
#[repr(transparent)]
/// Special type to use in std types.
pub struct DLAllocatorForStl(NonNull<DLAllocatorBase>);

impl From<NonNull<DLAllocatorBase>> for DLAllocatorForStl {
    fn from(ptr: NonNull<DLAllocatorBase>) -> Self {
        Self(ptr)
    }
}

impl fromsoftware_shared_stl::Allocator for DLAllocatorForStl {
    unsafe fn allocate_raw(&mut self, size: usize, allign: usize) -> *mut c_void {
        let allocator = self.0.as_ptr();
        let allocation =
            unsafe { ((*allocator).vftable.allocate_aligned)(&mut *allocator, size, allign) };
        if allocation.is_null() {
            panic!("DLAllocator returned null pointer")
        }
        allocation as _
    }

    unsafe fn deallocate_raw(&mut self, ptr: *mut c_void) {
        let allocator = self.0.as_ptr();
        unsafe {
            ((*allocator).vftable.deallocate)(&mut *allocator, ptr as _);
        }
    }
}

pub type DLVector<T> = fromsoftware_shared_stl::Vector<T, DLAllocatorForStl>;
