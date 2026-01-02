use bitfield::bitfield;
use std::ops::{Deref, DerefMut};
use std::{fmt, mem::MaybeUninit};
use thiserror::Error;

#[repr(C)]
pub struct DLFixedStdAllocator<T: Sized + Copy, const N: usize> {
    buffer: [T; N],
    flags: DLFixedStdAllocatorFlags<T>,
}

struct DLFixedStdAllocatorFlags<T: Sized + Copy>(MaybeUninit<T>);

impl<T: Sized + Copy> DLFixedStdAllocatorFlags<T> {
    fn new(flags: AllocatorStateFlags) -> Self {
        let mut uninit = Self(MaybeUninit::uninit());
        uninit.as_mut().write(flags);
        uninit
    }

    fn as_ref(&self) -> &MaybeUninit<AllocatorStateFlags> {
        unsafe {
            &*(&raw const *self)
                .wrapping_add(1)
                .wrapping_byte_sub(1)
                .cast()
        }
    }

    fn as_mut(&mut self) -> &mut MaybeUninit<AllocatorStateFlags> {
        unsafe { &mut *(&raw mut *self).wrapping_add(1).wrapping_byte_sub(1).cast() }
    }
}

impl<T: Sized + Copy> Clone for DLFixedStdAllocatorFlags<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Sized + Copy> Copy for DLFixedStdAllocatorFlags<T> {}

impl<T: Sized + Copy> Default for DLFixedStdAllocatorFlags<T> {
    fn default() -> Self {
        Self::new(AllocatorStateFlags(0))
    }
}

impl<T: Sized + Copy> Deref for DLFixedStdAllocatorFlags<T> {
    type Target = AllocatorStateFlags;

    fn deref(&self) -> &Self::Target {
        unsafe { self.as_ref().assume_init_ref() }
    }
}

impl<T: Sized + Copy> DerefMut for DLFixedStdAllocatorFlags<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.as_mut().assume_init_mut() }
    }
}

impl<T: Sized + Copy> fmt::Debug for DLFixedStdAllocatorFlags<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

bitfield! {
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct AllocatorStateFlags(u8);
    impl Debug;
    /// Bit 0: "copied allocator" restriction
    pub copied_allocator, set_copied_allocator: 0;
    /// Bit 1: "memory allocated" - buffer is in use
    pub is_occupied, set_occupied: 1;
}

impl<T: Sized + Copy, const N: usize> Default for DLFixedStdAllocator<T, N> {
    fn default() -> Self {
        Self {
            buffer: [unsafe { std::mem::zeroed() }; N],
            flags: DLFixedStdAllocatorFlags::default(),
        }
    }
}

#[derive(Error, Debug)]
pub enum DLFixedStdAllocatorError {
    #[error("Tried to allocate too large memory block from copied DLFixedStdAllocator.")]
    CopiedAllocatorTooLarge,
    #[error("Expected buffer size too large.")]
    BufferSizeTooLarge,
    #[error("Memory already allocated.")]
    MemoryAlreadyAllocated,
}

impl<T: Sized + Copy, const N: usize> DLFixedStdAllocator<T, N> {
    /// Allocate from the fixed buffer
    /// Returns an Error if:
    /// - size > N (buffer overflow)
    /// - copied_allocator flag set and size != 1
    /// - buffer already occupied
    pub fn allocate(&mut self, size: usize) -> Result<*mut T, DLFixedStdAllocatorError> {
        if size > N {
            return Err(DLFixedStdAllocatorError::BufferSizeTooLarge);
        }

        if self.flags.copied_allocator() && size != 1 {
            return Err(DLFixedStdAllocatorError::CopiedAllocatorTooLarge);
        }

        if self.flags.is_occupied() {
            return Err(DLFixedStdAllocatorError::MemoryAlreadyAllocated);
        }

        self.flags.set_occupied(true);

        // Return aligned pointer to buffer
        let buffer_addr = self.buffer.as_ptr() as usize;
        let alignment_offset = (-(buffer_addr as isize) & 1) as usize;

        unsafe {
            Ok(self
                .buffer
                .as_mut_ptr()
                .byte_add(alignment_offset * std::mem::size_of::<T>()))
        }
    }

    /// Deallocate - just clears the occupied flag for fixed buffers
    pub fn deallocate(&mut self) {
        if self.flags.is_occupied() {
            self.flags.set_occupied(false);
        }
    }

    /// Get pointer to the buffer (may be unaligned)
    pub fn buffer_ptr(&self) -> *const T {
        self.buffer.as_ptr()
    }

    /// Get mutable pointer to the buffer
    pub fn buffer_ptr_mut(&mut self) -> *mut T {
        self.buffer.as_mut_ptr()
    }
}
