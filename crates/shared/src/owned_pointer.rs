use std::alloc::{Layout, handle_alloc_error};
use std::ops::{Deref, DerefMut};
use std::{fmt, marker::PhantomData, ptr::NonNull};

use crate::{GameAllocator, NoOpAllocator};

/// Pointer to a structure that the containing structure owns. You will generally use this to model
/// structures in foreign memory when extending the game libraries. Do not use this in your own
/// code as you're risking all rusts safety reasoning.
///
/// ## Safety
///
/// ### `OwnedPtr` in FFI
///
/// When declaring definitions of C++ structs and functions that use this type,
/// the author must ensure several invariants hold true:
///
/// * This must be the only way to access the data it refers to without an
///   `unsafe` block (not including the `unsafe` block necessary to call
///   [`FromStatic::instance`]). This means there may not be any other structs
///   or methods that provide an `OwnedPtr` or a reference to the underlying
///   data.
///
///   Although *generally* this means that the struct that h olds an `OwnedPtr`
///   is the same one that "owns" the data it refers to in the sense of being
///   responsible for constructing and destroying it, that's not a hard
///   requirement. In some cases, it may be more ergonomic to expose an
///   `OwnedPtr` (or a reference) through a struct that's easy to obtain and use
///   `NonNull` for the struct that actually has ownership.
///
/// * This must ensure that the backing memory is allocated by an allocator
///   that's *compatible with* `A`. Note that all memory allocated in any way is
///   *compatible with* [`NoOpAllocator`], so this requirement only matters if
///   `A` is set explicitly.
///
/// [`FromStatic::instance`]: crate::FromStatic::instance
///
/// ### `OwnedPtr` and `Drop`
///
/// For any type `T` where Rust code might take ownership over an `OwnedPtr<T>`,
/// the author should be sure that `T`'s [Drop] implementation is correct. (This
/// is true in general for FFI types that Rust code can own.) There are two
/// concerns here:
///
/// * Generally, C++ code will declare a specific destroy function for each
///   type. In addition to calling the destructor method (`~T()`), this destroys
///   any fields as well.
///
/// * Rust drops fields in declaration order but C++ destroys them in reverse
///   declaration order.
///
/// To mitigate these issues, all fields with non-trivial drop/destructor
/// implementations should by wrapped in [std::mem::ManuallyDrop]. If the C++
/// code has a destroy method, it should be called from [Drop::drop]; otherwise,
/// the implementation of [Drop::drop] should drop these fields in reverse
/// declaration order.
#[repr(transparent)]
pub struct OwnedPtr<T, A: GameAllocator = NoOpAllocator> {
    ptr: NonNull<T>,
    _marker: PhantomData<A>,
}

impl<T, A: GameAllocator> OwnedPtr<T, A> {
    /// Allocates memory with `A` and places `value` into it.
    ///
    /// This doesn’t actually allocate if `T` is zero-sized.
    ///
    /// **Important:** any type constructed this way should have an appropriate
    /// [Drop::drop] implementation defined. [See above](#ownedptr-and-drop) for
    /// details.
    pub fn new(value: T) -> Self {
        let layout = Layout::new::<T>();
        if layout.size() == 0 {
            OwnedPtr {
                ptr: NonNull::dangling(),
                _marker: Default::default(),
            }
        } else if let Ok(ptr) = A::allocate(layout) {
            let ptr = ptr.cast::<T>();
            unsafe { ptr.write(value) };
            OwnedPtr {
                ptr,
                _marker: Default::default(),
            }
        } else {
            handle_alloc_error(layout)
        }
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }
}

impl<T: Default, A: GameAllocator> Default for OwnedPtr<T, A> {
    fn default() -> Self {
        OwnedPtr::new(Default::default())
    }
}

impl<T, A: GameAllocator> Deref for OwnedPtr<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T, A: GameAllocator> AsRef<T> for OwnedPtr<T, A> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T, A: GameAllocator> DerefMut for OwnedPtr<T, A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T, A: GameAllocator> AsMut<T> for OwnedPtr<T, A> {
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

impl<T, A: GameAllocator> fmt::Debug for OwnedPtr<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.ptr.fmt(f)
    }
}

impl<T, A: GameAllocator> fmt::Pointer for OwnedPtr<T, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Pointer::fmt(&self.ptr, f)
    }
}

impl<T, A: GameAllocator> Drop for OwnedPtr<T, A> {
    fn drop(&mut self) {
        unsafe {
            self.ptr.drop_in_place();
            if Layout::new::<T>().size() > 0 {
                A::deallocate(self.ptr.cast::<u8>(), Layout::new::<T>());
            }
        }
    }
}

unsafe impl<T: Send, A: GameAllocator + Send> Send for OwnedPtr<T, A> {}
unsafe impl<T: Sync, A: GameAllocator + Sync> Sync for OwnedPtr<T, A> where T: Sync {}
