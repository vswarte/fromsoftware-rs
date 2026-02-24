use std::ffi::c_void;

pub trait Allocator {
    fn allocate_raw(&mut self, size: usize, align: usize) -> *mut c_void;

    fn deallocate_raw(&mut self, ptr: *mut c_void) -> *mut c_void;
}

pub trait AllocatorExt {
    fn allocate<T>(&mut self) -> *mut T;
}

impl<A> AllocatorExt for A
where
    A: Allocator,
{
    fn allocate<T: Sized>(&mut self) -> *mut T {
        self.allocate_raw(size_of::<T>(), align_of::<T>()) as _
    }
}
