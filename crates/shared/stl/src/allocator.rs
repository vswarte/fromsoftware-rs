use std::{
    ffi::c_void,
    mem::{MaybeUninit, align_of, size_of},
    ptr::NonNull,
};

pub trait Allocator: Clone {
    /// # Safety
    ///
    /// `size` must be non-zero. `align` must be a power of two.
    /// The returned pointer is valid for `size` bytes and aligned to `align`
    unsafe fn allocate_raw(&mut self, size: usize, align: usize) -> NonNull<c_void>;
    /// # Safety
    ///
    /// `ptr` must have been obtained from a previous call to `allocate_raw`
    /// on the same allocator instance. It must not be used after this call
    unsafe fn deallocate_raw(&mut self, ptr: *mut c_void);

    /// # Safety
    ///
    /// Caller must initialize the returned `MaybeUninit<T>` before reading it.
    unsafe fn allocate<T>(&mut self) -> NonNull<MaybeUninit<T>> {
        unsafe {
            self.allocate_raw(size_of::<T>(), align_of::<T>())
                .cast::<MaybeUninit<T>>()
        }
    }
    /// # Safety
    ///
    /// `count` must be non-zero. Caller must initialize all `count` elements
    /// of the returned slice before reading any of them
    unsafe fn allocate_n<T>(&mut self, count: usize) -> NonNull<[MaybeUninit<T>]> {
        let size = size_of::<T>()
            .checked_mul(count)
            .expect("allocation size overflow");

        let ptr = unsafe {
            self.allocate_raw(size, align_of::<T>())
                .cast::<MaybeUninit<T>>()
                .as_ptr()
        };

        // Safety: ptr is non-null and we own `count` elements
        unsafe { NonNull::new_unchecked(std::ptr::slice_from_raw_parts_mut(ptr, count)) }
    }
}
