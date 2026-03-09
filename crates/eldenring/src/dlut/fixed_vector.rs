use std::{
    hint::assert_unchecked,
    mem::MaybeUninit,
    ops::{Index, IndexMut},
    slice,
};

#[repr(C)]
// A container with a fixed number of elements stored inline without an additional heap allocation
pub struct DLFixedVector<T, const C: usize> {
    elements: [MaybeUninit<T>; C],
    unk1: usize,
    checked_len: usize,
}

impl<T, const C: usize> Default for DLFixedVector<T, C> {
    fn default() -> Self {
        Self {
            elements: [const { MaybeUninit::uninit() }; C],
            unk1: 0,
            checked_len: 0,
        }
    }
}

impl<T, const C: usize> DLFixedVector<T, C> {
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    pub const fn capacity(&self) -> usize {
        C
    }

    pub fn as_slice(&self) -> &'_ [T] {
        unsafe {
            // Safety: enforced by `push()` and `truncate()`
            assert_unchecked(self.checked_len <= self.capacity());

            // Safety: elements up to `self.checked_len` are initialized
            slice::from_raw_parts(self.elements[0].as_ptr(), self.checked_len)
        }
    }

    pub fn as_mut_slice(&mut self) -> &'_ mut [T] {
        unsafe {
            // Safety: enforced by `push()` and `truncate()`
            assert_unchecked(self.checked_len <= self.capacity());

            // Safety: elements up to `self.checked_len` are initialized
            slice::from_raw_parts_mut(self.elements[0].as_mut_ptr(), self.checked_len)
        }
    }

    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    // Appends an element if there is sufficient spare capacity, otherwise an error is returned
    // with the element.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        let prev_len = self.len();
        if prev_len + 1 > self.capacity() {
            return Err(value);
        }

        self.elements[prev_len] = MaybeUninit::new(value);
        self.checked_len = prev_len + 1;
        Ok(())
    }

    // Truncates the vector to the given length, dropping elements that should no longer be
    // initialized.
    pub fn truncate(&mut self, new_len: usize) {
        let prev_len = self.len();
        if new_len < prev_len {
            for i in new_len..prev_len {
                // Safety: elements up to `self.checked_len` are initialized
                unsafe { self.elements[i].assume_init_drop() };
            }
            self.checked_len = new_len;
        }
    }
}

impl<T: Clone, const C: usize> DLFixedVector<T, C> {
    // Grows or shrinks the vector to the given length, initializing new elements with `value`,
    // or return an error with the value if there is insufficient capacity.
    pub fn resize(&mut self, new_len: usize, value: T) -> Result<(), T> {
        if new_len > self.capacity() {
            return Err(value);
        }

        if new_len < self.len() {
            self.truncate(new_len);
        } else {
            for i in self.len()..new_len {
                self.elements[i] = MaybeUninit::new(value.clone());
            }
            self.checked_len = new_len;
        }

        Ok(())
    }
}

impl<T, const C: usize> Index<usize> for DLFixedVector<T, C> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        self.as_slice().index(index)
    }
}

impl<T, const C: usize> IndexMut<usize> for DLFixedVector<T, C> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.as_mut_slice().index_mut(index)
    }
}

impl<T, const C: usize> Drop for DLFixedVector<T, C> {
    fn drop(&mut self) {
        self.truncate(0);
    }
}
