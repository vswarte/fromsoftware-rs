use crate::allocator::*;

/// Bits per storage word, MSVC uses `unsigned int` (32-bit) as `_Vbase`.
type VBase = u32;
const VBITS: usize = VBase::BITS as usize;

/// Implementation of MSVC C++ `std::vector<bool>`.
///
/// # References
///
/// - [cppreference - `std::vector<bool>`]
/// - [Raymond Chen's breakdown of `std::vector<bool>`]
///
/// [cppreference - `std::vector<bool>`]: https://en.cppreference.com/w/cpp/container/vector_bool.html
/// [Raymond Chen's breakdown of `std::vector<bool>`]: https://devblogs.microsoft.com/oldnewthing/20200313-00/?p=103559
#[repr(C)]
pub struct VectorBool<A: Allocator> {
    #[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
    allocator: A,
    first: *mut VBase,
    last: usize,
    end: usize,
    #[cfg(all(feature = "msvc2012", not(feature = "msvc2015")))]
    allocator: A,
}

impl<A: Allocator> VectorBool<A> {
    #[inline]
    pub fn len(&self) -> usize {
        self.last
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.last == 0
    }
    #[inline]
    pub fn capacity(&self) -> usize {
        self.end
    }

    /// Returns the bit at `index` or `None` if out of bounds
    #[inline]
    pub fn get(&self, index: usize) -> Option<bool> {
        (index < self.last).then(|| unsafe { self.get_bit(index) })
    }

    /// Sets the bit at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index >= len()`.
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        assert!(index < self.last, "index out of bounds");
        unsafe { self.set_bit(index, value) }
    }

    /// Flips all bits in all words
    pub fn flip(&mut self) {
        let (words, mask) = self.word_parts_mut();
        let full = words.len();
        words.iter_mut().for_each(|w| *w = !*w);
        if mask > 0 {
            unsafe { *self.first.add(full) ^= mask }
        }
    }

    pub fn count_ones(&self) -> usize {
        let (words, tail_word) = self.word_parts();
        words.iter().map(|w| w.count_ones() as usize).sum::<usize>()
            + tail_word.count_ones() as usize
    }

    #[inline]
    pub fn count_zeros(&self) -> usize {
        self.last - self.count_ones()
    }

    #[inline]
    pub fn any(&self) -> bool {
        let (words, tail_word) = self.word_parts();
        words.iter().any(|&w| w != 0) || tail_word != 0
    }

    #[inline]
    pub fn all(&self) -> bool {
        let (words, tail_word) = self.word_parts();
        let mask = ((1 as VBase) << (self.last % VBITS)).wrapping_sub(1);
        words.iter().all(|&w| w == VBase::MAX) && tail_word == mask
    }

    pub fn push_back(&mut self, value: bool) {
        if self.last == self.end {
            self.grow();
        }
        unsafe { self.set_bit(self.last, value) };
        self.last += 1;
    }

    pub fn pop_back(&mut self) -> Option<bool> {
        if self.is_empty() {
            return None;
        }
        self.last -= 1;
        Some(unsafe { self.get_bit(self.last) })
    }

    pub fn iter(&self) -> VectorBoolIter<'_, A> {
        VectorBoolIter {
            vec: self,
            index: 0,
        }
    }

    /// # Safety
    ///
    /// `index` must be < `self.end`
    #[inline]
    unsafe fn get_bit(&self, index: usize) -> bool {
        let word = unsafe { *self.first.add(index / VBITS) };
        (word >> (index % VBITS)) & 1 != 0
    }

    /// # Safety
    ///
    /// `index` must be < `self.end`
    #[inline]
    unsafe fn set_bit(&mut self, index: usize, value: bool) {
        let word = unsafe { &mut *self.first.add(index / VBITS) };
        let mask = (1 as VBase) << (index % VBITS);
        *word = (*word & !mask) | (VBase::from(value) << (index % VBITS));
    }

    fn word_parts(&self) -> (&[VBase], VBase) {
        let full = self.last / VBITS;
        let tail = self.last % VBITS;
        let words = unsafe { std::slice::from_raw_parts(self.first, full) };
        let mask = ((1 as VBase) << tail).wrapping_sub(1);
        let tail_word = unsafe { *self.first.add(full) } & mask;
        (words, tail_word)
    }

    fn word_parts_mut(&mut self) -> (&mut [VBase], VBase) {
        let full = self.last / VBITS;
        let tail = self.last % VBITS;
        let words = unsafe { std::slice::from_raw_parts_mut(self.first, full) };
        let mask = ((1 as VBase) << tail).wrapping_sub(1);
        (words, mask)
    }

    fn grow(&mut self) {
        let old_words = bits_to_words(self.end);
        // Grow by 1.5x, minimum one word, always round up to word boundary
        let new_bits = (self.end + self.end / 2).max(VBITS).next_multiple_of(VBITS);
        let new_words = bits_to_words(new_bits);

        let new_ptr =
            unsafe { self.allocator.allocate_n::<VBase>(new_words).as_ptr() } as *mut VBase;
        unsafe {
            std::ptr::copy_nonoverlapping(self.first, new_ptr, old_words);
            if old_words > 0 {
                self.allocator.deallocate_raw(self.first as _);
            }
        }
        self.first = new_ptr;
        self.end = new_bits;
    }
}

impl<'a, A: Allocator> IntoIterator for &'a VectorBool<A> {
    type Item = bool;
    type IntoIter = VectorBoolIter<'a, A>;

    fn into_iter(self) -> VectorBoolIter<'a, A> {
        self.iter()
    }
}

pub struct VectorBoolIter<'a, A: Allocator> {
    vec: &'a VectorBool<A>,
    index: usize,
}

impl<'a, A: Allocator> Iterator for VectorBoolIter<'a, A> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<bool> {
        let v = self.vec.get(self.index)?;
        self.index += 1;
        Some(v)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.vec.len().saturating_sub(self.index);
        (rem, Some(rem))
    }
}

impl<A: Allocator> ExactSizeIterator for VectorBoolIter<'_, A> {}

#[inline]
const fn bits_to_words(bits: usize) -> usize {
    bits.div_ceil(VBITS)
}
