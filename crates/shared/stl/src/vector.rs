use crate::allocator::*;
use std::ops::{Deref, DerefMut};

#[repr(C)]
pub struct Vector<T, A: Sized> {
    #[cfg(not(feature = "msvc2012"))]
    allocator: A,
    first: *mut T,
    last: *mut T,
    end: *mut T,
    #[cfg(feature = "msvc2012")]
    allocator: A,
}

impl<T, A: crate::Allocator> Vector<T, A> {
    #[inline]
    pub fn capacity(&self) -> usize {
        unsafe { self.end.offset_from(self.first) as usize }
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
        let new_cap = (old_cap + old_cap / 2).max(1);

        let new_ptr = self.allocator.allocate_n::<T>(new_cap).as_ptr() as _;

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

impl<T, A: Sized> Deref for Vector<T, A> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        // Safety: both pointers belong to the same allocation
        let len = unsafe { self.last.offset_from(self.first) as usize };
        // Safety: [first, last) is always a valid, initialized slice
        unsafe { std::slice::from_raw_parts(self.first, len) }
    }
}

impl<T, A: Sized> DerefMut for Vector<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        // Safety: both pointers belong to the same allocation
        let len = unsafe { self.last.offset_from(self.first) as usize };
        // Safety: [first, last) is always a valid, initialized slice
        unsafe { std::slice::from_raw_parts_mut(self.first, len) }
    }
}
