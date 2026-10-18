use std::ffi::c_void;

use vtable_rs::VPtr;

bitflags::bitflags! {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct DLHeapCapability: u32 {
        const CAN_ALLOCATE           = 0b00000001;
        const CAN_FREE               = 0b00000010;
        const CAN_USE_HANDLE         = 0b00000100;
        const CAN_RELOCATE           = 0b00001000;
        const IS_THREAD_SAFE         = 0b00010000;
        const CAN_USE_FOR_CONTAINERS = 0b00100000;
        const HEAP_INTERCHANGEABLE   = 0b01000000;
    }
}

/// Identifier assigned by [`crate::dlkr::DLSystemHeapImpl`] to each registered heap
#[repr(transparent)]
pub struct DLHeapIdentifier(pub i32);

#[vtable_rs::vtable]
pub trait DLAllocatorVmt {
    fn destructor(&mut self, flags: u8);
    /// Get [`DLHeapIdentifier`] assigned by [`crate::dlkr::DLSystemHeapImpl`] of the heap this allocator manages.
    fn get_heap_id(&self) -> DLHeapIdentifier;
    /// Deprecated method to get the allocator's ID. Returns -1 for all of them.
    fn get_allocator_id(&self) -> i32;

    fn heap_capability<'a>(&self, capability: &'a mut DLHeapCapability)
    -> &'a mut DLHeapCapability;

    fn get_total_size(&self) -> usize;

    fn get_free_size(&self) -> usize;

    fn get_max_size(&self) -> usize;

    fn get_block_number(&self) -> usize;

    /// Retrieves allocation size for a specific allocation.
    fn get_block_size(&self, block: *const u8) -> usize;

    fn allocate(&self, size: usize) -> *const u8;

    fn allocate_aligned(&self, size: usize, alignment: usize) -> *const u8;

    fn reallocate(&self, allocation: *const u8, size: usize) -> *const u8;

    fn reallocate_aligned(&self, allocation: *const u8, size: usize, alignment: usize)
    -> *const u8;

    fn deallocate(&self, allocation: *const u8);

    fn deallocate_all(&mut self);

    fn back_allocate(&mut self, size: usize) -> *const u8;

    fn back_allocate_aligned(&mut self, size: usize, alignment: usize) -> *const u8;

    fn back_reallocate(&mut self, allocation: *const u8, size: usize) -> *const u8;

    fn back_reallocate_aligned(
        &mut self,
        allocation: *const u8,
        size: usize,
        alignment: usize,
    ) -> *const u8;

    fn back_deallocate(&mut self, allocation: *const u8);

    fn self_diagnose(&self) -> bool;

    fn is_valid_block(&mut self, allocation: *const u8) -> bool;

    fn get_next_block(&mut self, cursor: *const c_void) -> bool;

    fn lock(&self);

    fn unlock(&self);

    fn find_corrupted_field(&mut self, allocation: *const u8) -> *const u8;
}

pub struct DLAllocator {
    pub vftable: VPtr<dyn DLAllocatorVmt, Self>,
}
