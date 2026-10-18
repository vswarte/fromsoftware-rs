use std::borrow::Cow;
use std::ptr::NonNull;

use shared::{AllocError, FromStatic, GameAllocator, InstanceResult, load_static_indirect};

use crate::dlkr::{DLAllocator, DLPlainLightMutex, HeapAllocator};

use super::heap_allocators::{heap_allocate, heap_deallocate};

#[repr(C)]
pub struct Win32RuntimeHeapImpl {
    pub heap_allocator: HeapAllocator<Self>,
    pub sync: DLPlainLightMutex,
}

impl std::ops::Deref for Win32RuntimeHeapImpl {
    type Target = DLAllocator;

    fn deref(&self) -> &Self::Target {
        &self.heap_allocator.base
    }
}

impl FromStatic for Win32RuntimeHeapImpl {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("Win32RuntimeHeapImpl")
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().runtime_heap_allocator) }
    }
}

impl GameAllocator for Win32RuntimeHeapImpl {
    fn allocate(layout: std::alloc::Layout) -> Result<NonNull<[u8]>, AllocError> {
        heap_allocate::<Self>(layout)
    }

    unsafe fn deallocate(ptr: NonNull<u8>) {
        unsafe { heap_deallocate::<Self>(ptr) }
    }
}
