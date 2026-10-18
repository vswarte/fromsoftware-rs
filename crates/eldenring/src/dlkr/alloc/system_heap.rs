use std::{alloc::Layout, borrow::Cow, ffi::c_void, ptr::NonNull};

use fromsoftware_shared_stl::{StlAllocator, Vector};
use shared::{AllocError, FromStatic, GameAllocator, OwnedPtr, load_static_indirect};

use crate::dlkr::{DLAllocator, DLPlainLightMutex, DLPlainReadWriteLock, MainHeapAllocator};

#[repr(C)]
/// Heap representing all other heaps.
pub struct DLSystemHeapImpl {
    base: [u8; 0x440],
    sync: DLPlainLightMutex,
    pub allocator: &'static DLAllocator,
    pub heap_registry: OwnedPtr<Vector<HeapAllocatorEntry, &'static Self>, Self>,
    pub registry_lock: DLPlainReadWriteLock,
    pub last_heap_id: u32,
}

impl StlAllocator for &'static DLSystemHeapImpl {
    unsafe fn allocate_raw(&self, size: usize, align: usize) -> *mut c_void {
        unsafe { self.allocator.allocate_raw(size, align) }
    }
    unsafe fn deallocate_raw(&self, ptr: *mut c_void) {
        unsafe {
            self.allocator.deallocate_raw(ptr);
        }
    }
}

#[repr(C)]
pub struct HeapAllocatorEntry {
    pub allocator: &'static DLAllocator,
    pub heap_start: *mut c_void,
    pub heap_end: *mut c_void,
}

impl DLSystemHeapImpl {
    /// Finds the allocator responsible for the given pointer
    pub fn get_allocator_of(&self, ptr: *const c_void) -> Option<&'static DLAllocator> {
        let _guard = self.registry_lock.read_lock(-1).ok()?;

        self.heap_registry
            .iter()
            .rev()
            .find(|e| {
                let p = ptr as usize;
                p >= e.heap_start as usize && p < e.heap_end as usize
            })
            .map(|e| e.allocator)
    }
}

impl FromStatic for DLSystemHeapImpl {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DLSystemHeapImpl")
    }

    fn instance_ptr() -> shared::InstanceResult<*mut Self> {
        unsafe { load_static_indirect(crate::rva::get().system_heap) }
    }
}

impl GameAllocator for DLSystemHeapImpl {
    fn allocate(layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let heap = unsafe { DLSystemHeapImpl::instance() }.map_err(|_| AllocError)?;
        let ptr = unsafe { heap.allocate_raw(layout.size(), layout.align()) };
        let ptr = NonNull::new(ptr.cast::<u8>()).ok_or(AllocError)?;
        Ok(NonNull::slice_from_raw_parts(ptr, layout.size()))
    }

    unsafe fn deallocate(ptr: NonNull<u8>) {
        let Ok(heap) = (unsafe { DLSystemHeapImpl::instance() }) else {
            return;
        };
        unsafe { heap.deallocate_raw(ptr.as_ptr().cast::<c_void>()) }
    }
}

/// Dynamic allocator type for use in [`OwnedPtr`] when concrete
/// allocator type is not known or can be different every time and game uses allocator regestry to keep track
/// of each allocation. Uses [`MainHeapAllocator`] when new allocation is requested.
pub struct DynamicMainHeapAllocator;

impl GameAllocator for DynamicMainHeapAllocator {
    fn allocate(layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        MainHeapAllocator::allocate(layout)
    }

    unsafe fn deallocate(ptr: NonNull<u8>) {
        let Ok(heap) = (unsafe { DLSystemHeapImpl::instance() }) else {
            return;
        };
        let ptr = ptr.as_ptr().cast::<c_void>();
        if let Some(allocator) = heap.get_allocator_of(ptr) {
            unsafe { allocator.deallocate_raw(ptr) }
        }
    }
}
