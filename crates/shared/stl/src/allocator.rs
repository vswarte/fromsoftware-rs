use std::{
    ffi::c_void,
    mem::{align_of, size_of},
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

    /// `T` must not be a zero-sized type.
    fn allocate<T>(&mut self) -> NonNull<T> {
        unsafe {
            self.allocate_raw(size_of::<T>(), align_of::<T>())
                .cast::<T>()
        }
    }

    /// Allocates `count` elemets. Panics if `count` is zero
    fn allocate_n<T>(&mut self, count: usize) -> NonNull<[T]> {
        if count == 0 {
            panic!("allocate_n called with 0 elements")
        }
        let size = size_of::<T>()
            .checked_mul(count)
            .expect("allocation size overflow");

        let ptr = unsafe {
            self.allocate_raw(size, align_of::<T>())
                .cast::<T>()
                .as_ptr()
        };

        // Safety: ptr is non-null and we own `count` elements
        unsafe { NonNull::new_unchecked(std::ptr::slice_from_raw_parts_mut(ptr, count)) }
    }
}
