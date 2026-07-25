use bitfield::bitfield;
use std::cell::Cell;
use std::ffi::c_void;
use thiserror::Error;

#[repr(C)]
struct DLFixedStdAllocatorFlags<T: Sized + Copy>(Cell<T>);

impl<T: Sized + Copy> DLFixedStdAllocatorFlags<T> {
    fn new() -> Self {
        Self(Cell::new(unsafe { std::mem::zeroed() }))
    }

    /// # Safety
    ///
    /// The returned reference is valid for the lifetime of `&self` and
    /// aliases the last byte of the underlying `Cell<T>`.
    fn flags_cell(&self) -> &Cell<AllocatorStateFlags> {
        let last_byte = self.0.as_ptr().wrapping_add(1).wrapping_byte_sub(1)
            as *const Cell<AllocatorStateFlags>;
        unsafe { &*last_byte }
    }

    fn get(&self) -> AllocatorStateFlags {
        self.flags_cell().get()
    }

    fn set(&self, flags: AllocatorStateFlags) {
        self.flags_cell().set(flags);
    }
}

#[repr(C)]
pub struct DLFixedStdAllocator<T: Sized + Copy, const N: usize> {
    buffer: Cell<[T; N]>,
    flags: DLFixedStdAllocatorFlags<T>,
}

bitfield! {
    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct AllocatorStateFlags(u8);
    impl Debug;
    /// Bit 0: "copied allocator" restriction
    pub copied_allocator, set_copied_allocator: 0;
    /// Bit 1: reentrancy guard, held only for the duration of a single
    /// `allocate()` call
    pub is_allocating, set_allocating: 1;
}

impl<T: Sized + Copy, const N: usize> Default for DLFixedStdAllocator<T, N> {
    fn default() -> Self {
        Self {
            buffer: Cell::new([unsafe { std::mem::zeroed() }; N]),
            flags: DLFixedStdAllocatorFlags::new(),
        }
    }
}

#[derive(Error, Debug)]
pub enum DLFixedStdAllocatorError {
    #[error("Tried to allocate too large memory block from copied DLFixedStdAllocator.")]
    CopiedAllocatorTooLarge,
    #[error("Expected buffer size too large.")]
    BufferSizeTooLarge,
    #[error("Reentrant call to allocate() on the same DLFixedStdAllocator.")]
    MemoryAlreadyAllocated,
}

impl<T: Sized + Copy, const N: usize> DLFixedStdAllocator<T, N> {
    /// Allocate from the fixed buffer
    /// Returns an Error if:
    /// - size > N (buffer overflow)
    /// - copied_allocator flag set and size != 1
    /// - buffer already occupied (reentrant call)
    pub fn allocate(&self, size: usize) -> Result<*mut T, DLFixedStdAllocatorError> {
        if size > N {
            return Err(DLFixedStdAllocatorError::BufferSizeTooLarge);
        }

        let mut flags = self.flags.get();

        if flags.copied_allocator() && size != 1 {
            return Err(DLFixedStdAllocatorError::CopiedAllocatorTooLarge);
        }

        if flags.is_allocating() {
            return Err(DLFixedStdAllocatorError::MemoryAlreadyAllocated);
        }

        flags.set_allocating(true);
        self.flags.set(flags);

        // Return aligned pointer to buffer
        let buffer_ptr = self.buffer.as_ptr().cast::<T>();
        let alignment_offset = (-(buffer_ptr as isize) & 1) as usize;
        let ptr = unsafe { buffer_ptr.byte_add(alignment_offset * std::mem::size_of::<T>()) };

        flags.set_allocating(false);
        self.flags.set(flags);

        Ok(ptr)
    }

    pub fn deallocate(&self) {}
}

impl<T: Sized + Copy, const N: usize> Clone for DLFixedStdAllocator<T, N> {
    fn clone(&self) -> Self {
        let mut flags = self.flags.get();
        flags.set_copied_allocator(true);
        let cloned_flags = DLFixedStdAllocatorFlags::new();
        cloned_flags.set(flags);
        Self {
            buffer: Cell::new(self.buffer.get()),
            flags: cloned_flags,
        }
    }
}

impl<T: Sized + Copy, const N: usize> fromsoftware_shared_stl::StlAllocator
    for DLFixedStdAllocator<T, N>
{
    unsafe fn allocate_raw(&self, size: usize, align: usize) -> *mut c_void {
        debug_assert!(align <= std::mem::align_of::<T>());

        let count = size.div_ceil(std::mem::size_of::<T>()).max(1);

        match self.allocate(count) {
            Ok(ptr) => ptr as *mut c_void,
            Err(err) => panic!("DLFixedStdAllocator failed to allocate: {err}"),
        }
    }

    unsafe fn deallocate_raw(&self, _ptr: *mut c_void) {
        self.deallocate();
    }
}
