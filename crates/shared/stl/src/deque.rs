use crate::allocator::*;
use std::iter::FusedIterator;

const fn block_size<T>() -> usize {
    let sz = std::mem::size_of::<T>();
    if sz <= 1 {
        16
    } else if sz <= 2 {
        8
    } else if sz <= 4 {
        4
    } else if sz <= 8 {
        2
    } else {
        1
    }
}

const INITIAL_CAPACITY: usize = 8;

/// Implementation of MSVC C++ `std::deque`.
///
/// # References
///
/// - [cppreference - `std::deque`]
/// - [MSVC STL source - `deque`]
/// - [Raymond Chen's breakdown of `std::deque`]
///
/// [cppreference - `std::deque`]: https://en.cppreference.com/w/cpp/container/deque.html
/// [MSVC STL source - `deque`]: https://github.com/microsoft/STL/blob/main/stl/inc/deque
/// [Raymond Chen's breakdown of `std::deque`]: https://devblogs.microsoft.com/oldnewthing/20230810-00/?p=108587
#[repr(C)]
pub struct Deque<T, A: StlAllocator> {
    #[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
    pub allocator: A,
    /// `<deque>` uniquely inherits `_Container_base12` unconditionally, so
    /// this `_Myproxy` slot exists even in release builds (unlike every
    /// other container, which drops it via `_Container_base0`).
    debug_proxy: *mut std::ffi::c_void,
    map: *mut *mut T,
    map_capacity: usize,
    map_offset: usize,
    size: usize,
    #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
    pub allocator: A,
}

impl<T, A: StlAllocator> Deque<T, A> {
    /// Creates an empty deque backed by `allocator`.
    ///
    /// Equivalent to `std::deque<T>()` with a custom allocator.
    pub fn new_in(allocator: A) -> Self {
        Self {
            allocator,
            debug_proxy: std::ptr::null_mut(),
            map: std::ptr::null_mut(),
            map_capacity: 0,
            map_offset: 0,
            size: 0,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns a reference to the element at `index`, or `None` if out of bounds.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        (index < self.size).then(|| unsafe { self.get_unchecked(index) })
    }

    /// Returns a mutable reference to the element at `index`, or `None` if out of bounds.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        (index < self.size).then(|| unsafe { self.get_unchecked_mut(index) })
    }

    /// Returns a reference to the first element, or `None` if empty.
    #[inline]
    pub fn front(&self) -> Option<&T> {
        self.get(0)
    }

    /// Returns a mutable reference to the first element, or `None` if empty.
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    /// Returns a reference to the last element, or `None` if empty.
    #[inline]
    pub fn back(&self) -> Option<&T> {
        self.size.checked_sub(1).and_then(|i| self.get(i))
    }

    /// Returns a mutable reference to the last element, or `None` if empty.
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.size.checked_sub(1).and_then(|i| self.get_mut(i))
    }

    /// Appends an element to the back.
    pub fn push_back(&mut self, value: T) {
        let (block_idx, elem_idx) = Self::split_abs(self.back_abs());

        if elem_idx == 0 {
            if block_idx >= self.map_capacity {
                self.grow_map();
            }

            let current_block_idx = Self::split_abs(self.back_abs()).0;
            let slot = self.map_slot(current_block_idx);

            unsafe {
                if (*self.map.add(slot)).is_null() {
                    self.install_block_at(current_block_idx);
                }
            }
        }

        unsafe { self.elem_ptr(self.back_abs()).write(value) };
        self.size += 1;
    }

    /// Appends an element to the front.
    pub fn push_front(&mut self, value: T) {
        if self.map_offset == 0 {
            self.grow_map();
        }

        self.map_offset -= 1;
        let (block_idx, elem_idx) = Self::split_abs(self.map_offset);

        // Crossed into a new block moving left.
        if elem_idx == block_size::<T>() - 1 {
            let slot = self.map_slot(block_idx);
            unsafe {
                if (*self.map.add(slot)).is_null() {
                    self.install_block_at(block_idx);
                }
            }
        }

        unsafe { self.elem_ptr(self.map_offset).write(value) };
        self.size += 1;
    }

    /// Removes and returns the last element, or `None` if empty.
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.size -= 1;

        let abs = self.back_abs();
        let (block_idx, elem_idx) = Self::split_abs(abs);
        let value = unsafe { self.elem_ptr(abs).read() };

        // Guard against freeing the front block when the deque just became empty.
        if elem_idx == 0 && block_idx != Self::split_abs(self.map_offset).0 {
            unsafe { self.free_block_at(block_idx) };
        }

        Some(value)
    }

    /// Removes and returns the first element, or `None` if empty.
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let abs = self.map_offset;
        let (block_idx, elem_idx) = Self::split_abs(abs);
        let value = unsafe { self.elem_ptr(abs).read() };

        self.map_offset += 1;
        self.size -= 1;

        if elem_idx == block_size::<T>() - 1 {
            unsafe { self.free_block_at(block_idx) };
        }

        Some(value)
    }

    /// Clears the deque, removing all values and freeing their underlying blocks.
    pub fn clear(&mut self) {
        let old_offset = self.map_offset;
        let old_size = self.size;
        self.size = 0;
        let block_sz = block_size::<T>();
        let center_block = self.map_capacity / 2;
        self.map_offset = center_block * block_sz;

        for i in 0..old_size {
            unsafe { std::ptr::drop_in_place(self.elem_ptr(old_offset + i)) };
        }

        if !self.map.is_null() {
            for slot in 0..self.map_capacity {
                unsafe {
                    let block = *self.map.add(slot);
                    if !block.is_null() {
                        self.allocator.deallocate_raw(block as _);
                        *self.map.add(slot) = std::ptr::null_mut();
                    }
                }
            }
        }
    }

    /// Returns an iterator over references to elements in front-to-back order.
    pub fn iter(&self) -> DequeIter<'_, T, A> {
        DequeIter {
            deque: self,
            front: 0,
            back: self.size,
        }
    }

    /// Returns an iterator over mutable references to elements in front-to-back order.
    pub fn iter_mut(&mut self) -> DequeIterMut<'_, T, A> {
        let back = self.size;
        DequeIterMut {
            deque: self,
            front: 0,
            back,
        }
    }

    /// Splits an absolute element position into a block
    /// number and an index within that block.
    #[inline]
    const fn split_abs(abs: usize) -> (usize, usize) {
        let block_sz = block_size::<T>();
        (abs / block_sz, abs % block_sz)
    }

    /// Maps a block index onto its slot in `map`.
    #[inline]
    fn map_slot(&self, block_idx: usize) -> usize {
        block_idx & (self.map_capacity - 1)
    }

    /// Position one past the last live element.
    #[inline]
    const fn back_abs(&self) -> usize {
        self.map_offset + self.size
    }

    /// # Safety
    ///
    /// A valid, non-null block must exist at `map[abs / BLOCK_SIZE]`.
    #[inline]
    unsafe fn elem_ptr(&self, abs: usize) -> *mut T {
        let (block_idx, elem_idx) = Self::split_abs(abs);
        let slot = self.map_slot(block_idx);
        unsafe {
            let block = *self.map.add(slot);
            block.add(elem_idx)
        }
    }

    fn install_block_at(&mut self, block_idx: usize) {
        let block = Self::alloc_block(&mut self.allocator);
        let slot = self.map_slot(block_idx);
        unsafe { *self.map.add(slot) = block };
    }

    /// # Safety
    ///
    /// `map[block_idx & (map_capacity - 1)]` must be a valid, non-null
    /// pointer returned by `install_block_at`.
    unsafe fn free_block_at(&mut self, block_idx: usize) {
        let slot = self.map_slot(block_idx);
        let block = unsafe { *self.map.add(slot) };
        unsafe {
            self.allocator.deallocate_raw(block as _);
            *self.map.add(slot) = std::ptr::null_mut();
        }
    }

    /// # Safety
    ///
    /// `index` must be < `self.size`.
    #[inline]
    unsafe fn get_unchecked(&self, index: usize) -> &T {
        unsafe { &*self.elem_ptr(self.map_offset + index) }
    }

    /// # Safety
    ///
    /// `index` must be < `self.size`.
    #[inline]
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        unsafe { &mut *self.elem_ptr(self.map_offset + index) }
    }

    fn alloc_map(allocator: &mut A, cap: usize) -> *mut *mut T {
        if cap == 0 {
            return std::ptr::null_mut();
        }
        let ptr = allocator.allocate_n::<*mut T>(cap).as_ptr() as *mut *mut T;
        unsafe { std::ptr::write_bytes(ptr, 0, cap) };
        ptr
    }

    fn alloc_block(allocator: &mut A) -> *mut T {
        let block_sz = block_size::<T>();
        allocator.allocate_n::<T>(block_sz).as_ptr() as *mut T
    }

    /// Doubles the map's capacity, re-centering the used blocks.
    fn grow_map(&mut self) {
        let block_sz = block_size::<T>();
        let old_cap = self.map_capacity;
        let new_cap = (old_cap * 2).max(INITIAL_CAPACITY);
        let new_map = Self::alloc_map(&mut self.allocator, new_cap);

        let first_block = Self::split_abs(self.map_offset).0;
        let used = if self.map.is_null() {
            0
        } else {
            self.used_blocks()
        };

        let new_first = (new_cap - used) / 2;
        unsafe {
            for i in 0..used {
                let old_slot = (first_block + i) & (old_cap - 1);
                *new_map.add(new_first + i) = *self.map.add(old_slot);
            }
            if !self.map.is_null() {
                self.allocator.deallocate_raw(self.map as _);
            }
        }

        self.map_offset = new_first * block_sz + (self.map_offset % block_sz);
        self.map = new_map;
        self.map_capacity = new_cap;
    }

    /// Number of blocks currently holding live data.
    fn used_blocks(&self) -> usize {
        if self.size == 0 {
            return 1;
        }
        let first = Self::split_abs(self.map_offset).0;
        let last = Self::split_abs(self.map_offset + self.size - 1).0;
        last - first + 1
    }
}

impl<T, A: StlAllocator> Drop for Deque<T, A> {
    fn drop(&mut self) {
        self.clear();
        if !self.map.is_null() {
            unsafe { self.allocator.deallocate_raw(self.map as _) };
        }
    }
}

impl<'a, T, A: StlAllocator> IntoIterator for &'a Deque<T, A> {
    type Item = &'a T;
    type IntoIter = DequeIter<'a, T, A>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, A: StlAllocator> IntoIterator for &'a mut Deque<T, A> {
    type Item = &'a mut T;
    type IntoIter = DequeIterMut<'a, T, A>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct DequeIter<'a, T, A: StlAllocator> {
    deque: &'a Deque<T, A>,
    front: usize,
    back: usize,
}

impl<'a, T, A: StlAllocator> Iterator for DequeIter<'a, T, A> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        if self.front >= self.back {
            return None;
        }
        let v = unsafe { self.deque.get_unchecked(self.front) };
        self.front += 1;
        Some(v)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.back - self.front;
        (rem, Some(rem))
    }
}

impl<T, A: StlAllocator> DoubleEndedIterator for DequeIter<'_, T, A> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        Some(unsafe { self.deque.get_unchecked(self.back) })
    }
}

impl<T, A: StlAllocator> ExactSizeIterator for DequeIter<'_, T, A> {}
impl<T, A: StlAllocator> FusedIterator for DequeIter<'_, T, A> {}

pub struct DequeIterMut<'a, T, A: StlAllocator> {
    deque: &'a mut Deque<T, A>,
    front: usize,
    back: usize,
}

impl<'a, T, A: StlAllocator> Iterator for DequeIterMut<'a, T, A> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<&'a mut T> {
        if self.front >= self.back {
            return None;
        }
        let v = unsafe { &mut *(self.deque.get_unchecked_mut(self.front) as *mut T) };
        self.front += 1;
        Some(v)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.back - self.front;
        (rem, Some(rem))
    }
}

impl<'a, T, A: StlAllocator> DoubleEndedIterator for DequeIterMut<'a, T, A> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut T> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        Some(unsafe { &mut *(self.deque.get_unchecked_mut(self.back) as *mut T) })
    }
}

impl<T, A: StlAllocator> ExactSizeIterator for DequeIterMut<'_, T, A> {}
impl<T, A: StlAllocator> FusedIterator for DequeIterMut<'_, T, A> {}
