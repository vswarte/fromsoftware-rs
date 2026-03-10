use crate::allocator::*;
use std::ops::{Deref, DerefMut};

#[repr(C)]
/// Implementation of MSVC C++ `std::vector`.
///
/// # References
///
/// - [cppreference - `std::vector`]
/// - [MSVC STL source - `vector`]
/// - [Raymond Chen's breakdown of `std::vector`]
///
/// [cppreference - `std::vector`]: https://en.cppreference.com/w/cpp/container/vector.html
/// [MSVC STL source - `vector`]: https://github.com/microsoft/STL/blob/main/stl/inc/vector
/// [Raymond Chen's breakdown of `std::vector`]: https://devblogs.microsoft.com/oldnewthing/20230802-00/?p=108524
pub struct Vector<T, A: Allocator> {
    #[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
    allocator: A,
    first: *mut T,
    last: *mut T,
    end: *mut T,
    #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
    allocator: A,
}

impl<T, A: Allocator> Vector<T, A> {
    #[inline]
    pub fn capacity(&self) -> usize {
        unsafe { self.end.offset_from(self.first) as usize }
    }

    /// Creates an empty vector backed by `allocator`.
    ///
    /// Equivalent to `std::vector<T>()` with a custom allocator
    pub fn new_in(allocator: A) -> Self {
        Self {
            allocator,
            first: std::ptr::null_mut(),
            last: std::ptr::null_mut(),
            end: std::ptr::null_mut(),
        }
    }

    /// Creates a vector from a slice, copying all elements into it
    pub fn from_slice_in(items: &[T], mut allocator: A) -> Self
    where
        T: Copy,
    {
        let len = items.len();

        if len == 0 {
            return Self::new_in(allocator);
        }

        let ptr = unsafe { allocator.allocate_n::<T>(len).as_ptr() } as *mut T;
        unsafe {
            std::ptr::copy_nonoverlapping(items.as_ptr(), ptr, len);
        }

        Self {
            allocator,
            first: ptr,
            last: unsafe { ptr.add(len) },
            end: unsafe { ptr.add(len) },
        }
    }

    pub fn push_back(&mut self, value: T) {
        if self.last == self.end {
            self.grow();
        }

        unsafe {
            self.last.write(value);
            self.last = self.last.add(1);
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        unsafe {
            self.last = self.last.sub(1);
            Some(self.last.read())
        }
    }

    /// MSVC growth policy: 1.5x capacity
    fn grow(&mut self) {
        let old_len = self.len();
        let old_cap = self.capacity();
        let new_cap = (old_cap + old_cap / 2).max(old_cap + 1).max(4);

        let new_ptr = unsafe { self.allocator.allocate_n::<T>(new_cap).as_ptr() } as _;

        unsafe {
            std::ptr::copy_nonoverlapping(self.first, new_ptr, old_len);
            if old_cap > 0 {
                self.allocator.deallocate_raw(self.first as _);
            }
        }

        self.first = new_ptr;
        self.last = unsafe { new_ptr.add(old_len) };
        self.end = unsafe { new_ptr.add(new_cap) };
    }
}

impl<T, A: Allocator> Deref for Vector<T, A> {
    type Target = [T];
    #[inline]
    fn deref(&self) -> &[T] {
        if self.first.is_null() {
            return &[];
        }
        // Safety: both pointers belong to the same allocation
        let len = unsafe { self.last.offset_from(self.first) as usize };
        // Safety: [first, last) is always a valid, initialized slice
        unsafe { std::slice::from_raw_parts(self.first, len) }
    }
}

impl<T, A: Allocator> DerefMut for Vector<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        if self.first.is_null() {
            return &mut [];
        }
        // Safety: both pointers belong to the same allocation
        let len = unsafe { self.last.offset_from(self.first) as usize };
        // Safety: [first, last) is always a valid, initialized slice
        unsafe { std::slice::from_raw_parts_mut(self.first, len) }
    }
}

impl<T, A: Allocator> Drop for Vector<T, A> {
    fn drop(&mut self) {
        // Drop every live element in [first, last) before releasing the buffer
        unsafe {
            std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(self.first, self.len()));
        }

        // guard against empty vectors
        if self.capacity() > 0 {
            unsafe { self.allocator.deallocate_raw(self.first as _) };
        }
    }
}
