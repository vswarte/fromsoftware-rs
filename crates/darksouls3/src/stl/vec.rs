use std::alloc::System as SysAlloc;
use std::{marker::PhantomData, ptr};

use cxx_stl::alloc::{CxxProxy, WithCxxProxy};

use shared::OwnedPtr;

pub use cxx_stl::vec::{CxxVecLayout, RawVec};

/// The MSVC 2015 vector layout, which places the allocator after the data.
#[repr(C)]
pub struct Layout<A: CxxProxy> {
    val: RawVec,
    alloc: A,
}

/// An MSVC-2015-compatible (and thus DS3-compatible) vector type.
pub type CxxVec<T, A = SysAlloc> = CxxVecLayout<T, A, Layout<A>>;

impl<A: CxxProxy> Layout<A> {
    pub const fn new_in(alloc: A) -> Self {
        Self {
            val: RawVec {
                first: ptr::null_mut(),
                last: ptr::null_mut(),
                end: ptr::null_mut(),
            },
            alloc,
        }
    }
}

pub fn new<T>() -> CxxVec<T, SysAlloc> {
    CxxVec::from_rust_vec_in(vec![], SysAlloc)
}

pub fn new_in<T, A: CxxProxy>(alloc: A) -> CxxVec<T, A> {
    CxxVec::from_rust_vec_in(vec![], alloc)
}

impl<A: CxxProxy> WithCxxProxy for Layout<A> {
    type Value = RawVec;
    type Alloc = A;

    fn value_as_ref(&self) -> &Self::Value {
        &self.val
    }

    fn value_as_mut(&mut self) -> &mut Self::Value {
        &mut self.val
    }

    fn alloc_as_ref(&self) -> &Self::Alloc {
        &self.alloc
    }

    fn new_in(alloc: Self::Alloc) -> Self {
        Self {
            val: RawVec {
                first: ptr::null_mut(),
                last: ptr::null_mut(),
                end: ptr::null_mut(),
            },
            alloc,
        }
    }
}
