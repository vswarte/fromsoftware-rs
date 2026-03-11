use std::{alloc::Layout, ptr::NonNull};

use thiserror::Error;

/// An error indicating that an allocation failed for any reason, including
/// memory exhaustion or because the requested layout didn't meet the
/// allocator's alignment requirements.
#[derive(Error, Debug)]
#[error("Allocation failed")]
pub struct AllocError;

/// A trait for global allocators within a game that are able to deallocate any
/// memory on the game's main heap. Generally corresponds to the drop logic for
/// `DLUT::DLAutoDeletePtr` in games from _Bloodborne_ forward.
///
/// The API for this generally matches [`std::alloc::Allocator`] where possible.
/// However, it diverges in certain places that are necessary for compatibility
/// with the games' memory management.
///
/// ### Allocator compatibility
///
/// An allocator is *compatible with* this allocator if memory it allocates can
/// be passed to [`deallocate`] without causing undefined behavior. A given
/// [`GameAllocator]` must be compatible with itself, but other allocators may
/// also be compatible. For example, the `GameAllocator` may track global
/// allocation arenas and determine which global allocator to use to deallocate
/// a given pointer based on where it appears in those arenas.
///
/// ### Currently allocated memory
///
/// Some of the methods require that a memory block is *currently allocated* by
/// an allocator. This means that:
///
/// * the starting address for that memory block was previously returned by
///   [`allocate`] *or* by another allocator that's known to be [*compatible
///   with*] this one, and
/// * the memory block has not subsequently been deallocated.
///
/// A memory block is deallocated by a call to [`deallocate`].
///
/// [`allocate`]: Self::allocate
/// [*compatible with*]: #allocator-compatibility
/// [`deallocate`]: Self::deallocate
///
/// ### Memory fitting
///
/// Some of the methods require that a `layout` *fit* a memory block or vice
/// versa. This means that the following conditions must hold:
///
/// * the memory block must be *currently allocated* with alignment of
///   [`layout.align()`], and
///
/// * [`layout.size()`] must fall in the range `min ..= max`, where:
///   - `min` is the size of the layout used to allocate the block, and
///   - `max` is the actual size returned from [`allocate`].
///
/// [`layout.align()`]: Layout::align
/// [`layout.size()`]: Layout::size
pub trait GameAllocator {
    /// Attempts to allocate a block of memory.
    ///
    /// On success, returns a `NonNull<[u8]>` meeting the size and alignment
    /// guarantees of `layout`. The returned block may have a larger size than
    /// specified by `layout.size()`, and may or may not have its contents
    /// initialized.
    ///
    /// The returned block of memory remains valid until it's passed to
    /// [`deallocate`].
    ///
    /// [`deallocate`]: Self::deallocate
    ///
    /// **Note:** Unlike [std::alloc::Allocator], this does not take a reference
    /// to `&self`. This implies that any state involved in allocation must be
    /// global, thread-safe, and must be valid for the duration of execution.
    ///
    /// ## Errors
    ///
    /// Unlike [std::alloc::Allocator], this may return an error for reasons
    /// other than the memory being exhausted or `layout` not meeting the
    /// allocator's size or alignment constraints.
    ///
    /// Clients wishing to abort computation in response to an allocation error
    /// are encouraged to call the [`handle_alloc_error`] function, rather than
    /// directly invoking [`panic!`] or similar.
    ///
    /// [`handle_alloc_error`]: std::alloc::handle_alloc_error
    fn allocate(layout: Layout) -> Result<NonNull<[u8]>, AllocError>;

    /// Deallocates the memory referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// * `ptr` must denote a block of memory [*currently allocated*] via this
    ///   allocator, and
    /// * `layout` must [*fit*] that block of memory.
    ///
    /// [*currently allocated*]: #currently-allocated-memory
    /// [*fit*]: #memory-fitting
    unsafe fn deallocate(ptr: NonNull<u8>, layout: Layout);
}

/// A [`GameAllocator`] that never actually allocates or drops memory.
///
/// [`allocate`] always returns [`AllocError`]. [`deallocate`] panics if
/// `cfg!(debug_assertions)` is set and leaks memory otherwise.
///
/// [`allocate`]: Self::allocate
/// [`deallocate`]: Self::deallocate
///
/// This is intended to by used as the allocator for types such as [`OwnedPtr`]
/// that represent memory allocated by the game that *don't* use the game's main
/// heap and so whose deallocation behavior is unknown. It's always incorrect to
/// try to create or destroy such types in Rust code.
///
/// [`OwnedPtr`]: crate::OwnedPtr
///
/// Because [`NoOpAllocator::deallocate`] never has undefined behavior for any
/// memory blocks, all allocators are technically *compatible with* it.
pub struct NoOpAllocator;

impl GameAllocator for NoOpAllocator {
    /// Always returns [`AllocError`].
    fn allocate(_layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        Err(AllocError)
    }

    /// Panics if `cfg!(debug_assertions)` is set and leaks memory otherwise.
    unsafe fn deallocate(_ptr: NonNull<u8>, _layout: Layout) {
        if cfg!(debug_assertions) {
            panic!("Can't drop data with NoOpAllocator");
        }
    }
}
