use crate::allocator::*;

/// Bits per storage word, MSVC uses `unsigned int` (32-bit) as `_Vbase`.
const VBITS: usize = u32::BITS as usize;

/// MSVC [`std::vector<bool>`] specialization.
///
/// [`std::vector<bool>`]: https://en.cppreference.com/w/cpp/container/vector_bool.html
#[repr(C)]
pub struct VectorBool<A: Allocator> {
    #[cfg(any(not(feature = "msvc2012"), feature = "msvc2015"))]
    allocator: A,
    first: *mut u32,
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
    /// Panics if `index >= len()`.
    #[inline]
    pub fn set(&mut self, index: usize, value: bool) {
        assert!(index < self.last, "index out of bounds");
        unsafe { self.set_bit(index, value) }
    }

    /// Flips all bits in all words
    pub fn flip(&mut self) {
        let (words, tail) = self.word_parts_mut();
        let full = words.len();
        words.iter_mut().for_each(|w| *w = !*w);
        if tail > 0 {
            let mask = (1u32 << tail) - 1;
            unsafe { *self.first.add(full) ^= mask }
        }
    }

    pub fn count_ones(&self) -> usize {
        let (words, tail) = self.word_parts();
        let mut n: usize = words.iter().map(|w| w.count_ones() as usize).sum();
        if tail > 0 {
            let mask = (1u32 << tail) - 1;
            n += (unsafe { *self.first.add(words.len()) } & mask).count_ones() as usize;
        }
        n
    }

    #[inline]
    pub fn count_zeros(&self) -> usize {
        self.last - self.count_ones()
    }

    #[inline]
    pub fn any(&self) -> bool {
        let (words, tail) = self.word_parts();
        words.iter().any(|&w| w != 0)
            || tail > 0 && {
                let mask = (1u32 << tail) - 1;
                (unsafe { *self.first.add(words.len()) } & mask) != 0
            }
    }

    #[inline]
    pub fn all(&self) -> bool {
        let (words, tail) = self.word_parts();
        words.iter().all(|&w| w == u32::MAX)
            && (tail == 0 || {
                let mask = (1u32 << tail) - 1;
                (unsafe { *self.first.add(words.len()) } & mask) == mask
            })
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
    /// `index` must be < `self.last`
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
        let mask = 1u32 << (index % VBITS);
        *word = (*word & !mask) | (u32::from(value) << (index % VBITS));
    }

    fn word_parts(&self) -> (&[u32], usize) {
        let full = self.last / VBITS;
        let tail = self.last % VBITS;
        let words = unsafe { std::slice::from_raw_parts(self.first, full) };
        (words, tail)
    }

    fn word_parts_mut(&mut self) -> (&mut [u32], usize) {
        let full = self.last / VBITS;
        let tail = self.last % VBITS;
        let words = unsafe { std::slice::from_raw_parts_mut(self.first, full) };
        (words, tail)
    }

    fn grow(&mut self) {
        let old_words = bits_to_words(self.end);
        // Grow by 1.5x, minimum one word, always round up to word boundary
        let new_bits = (self.end + self.end / 2).max(VBITS).next_multiple_of(VBITS);
        let new_words = bits_to_words(new_bits);

        let new_ptr = self.allocator.allocate_n::<u32>(new_words).as_ptr() as *mut u32;
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
fn bits_to_words(bits: usize) -> usize {
    bits.div_ceil(VBITS)
}
