use std::ptr::NonNull;

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
    fn allocate_raw(&mut self, size: usize, allign: usize) -> NonNull<std::ffi::c_void> {
        let allocator = self.0.as_ptr();
        let allocation =
            unsafe { ((*allocator).vftable.allocate_aligned)(&mut *allocator, size, allign) };
        if allocation.is_null() {
            panic!("DLAllocator returned null pointer")
        }
        unsafe { NonNull::new_unchecked(allocation as _) }
    }

    fn deallocate_raw(&mut self, ptr: *mut std::ffi::c_void) {
        let allocator = self.0.as_ptr();
        unsafe {
            ((*allocator).vftable.deallocate)(&mut *allocator, ptr as _);
        }
    }
}

/// An MSVC 2012-compatible vector that contains a custom DS3 allocator. This is
/// the type of vector generally used in DS3.
pub type DLVector<T> = fromsoftware_shared_stl::Vector<T, DLAllocatorForStl>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x20, size_of::<DLVector<usize>>());
    }
}
