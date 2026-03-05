use std::{
    ffi::c_void,
    mem::{MaybeUninit, align_of, size_of},
    ptr::NonNull,
};

pub trait Allocator {
    fn allocate_raw(&mut self, size: usize, align: usize) -> NonNull<c_void>;
    fn deallocate_raw(&mut self, ptr: *mut c_void);
}

pub trait AllocatorExt {
    fn allocate<T>(&mut self) -> NonNull<MaybeUninit<T>>;
    fn allocate_n<T>(&mut self, count: usize) -> NonNull<[MaybeUninit<T>]>;
}

impl<A: Allocator> AllocatorExt for A {
    fn allocate<T>(&mut self) -> NonNull<MaybeUninit<T>> {
        self.allocate_raw(size_of::<T>(), align_of::<T>())
            .cast::<MaybeUninit<T>>()
    }

    fn allocate_n<T>(&mut self, count: usize) -> NonNull<[MaybeUninit<T>]> {
        let size = size_of::<T>()
            .checked_mul(count)
            .expect("allocation size overflow");

        let ptr = self
            .allocate_raw(size, align_of::<T>())
            .cast::<MaybeUninit<T>>()
            .as_ptr();

        // Safety: ptr is non-null and we own `count` elements
        unsafe { NonNull::new_unchecked(std::ptr::slice_from_raw_parts_mut(ptr, count)) }
    }
}
