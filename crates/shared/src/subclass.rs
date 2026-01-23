use std::ptr::NonNull;

use pelite::pe64::{Pe, Rva, Va, msvc::RTTICompleteObjectLocator};
use thiserror::Error;

use crate::{Program, is_base_class};

/// The error type returend when a superclass isn't an instance of a subclass.
#[derive(Error, Debug)]
#[error("superclass is not an instance of {0}")]
pub struct TryFromSuperclassError(String);

impl TryFromSuperclassError {
    /// Returns a [TryFromSuperclassError] for the given subclass name.
    pub fn new(subclass: String) -> Self {
        TryFromSuperclassError(subclass)
    }
}

/// Gets the MSVC RTTI complete object locator for the given vftable
///
/// # Safety
///
/// The vftable must point to a valid MSVC virtual method table with RTTI in the current program
unsafe fn complete_object_locator(vmt: Va) -> &'static RTTICompleteObjectLocator {
    // A pointer to the complete object locator is located in the address immediately before the vmt
    // https://www.lukaszlipski.dev/post/rtti-msvc/
    let va = vmt - size_of::<Va>() as Va;
    unsafe { &**(va as *const *const _) }
}

/// A trait for C++ types that have multiple different subclasses. Implementing
/// this for a superclass and [Subclass] for its subclasses makes it possible to
/// safely check for the object's actual type based on its vtable.
///
/// ## Safety
///
/// In order to implement this for a struct, you must guarantee:
///
/// * The struct uses C-style layout.
/// * The first element of the struct is a pointer to a vtable for a class with MSVC RTTI.
/// * There's a 1-to-1 correspondence between vtable address and C++ class.
pub unsafe trait Superclass: Sized {
    /// The RVA of this class's virtual method table.
    fn vmt_rva() -> Rva;

    /// The VA of this class's virtual method table.
    fn vmt_va() -> Va {
        Program::current()
            .rva_to_va(Self::vmt_rva())
            .expect("VMT address not in executable!")
    }

    /// Returns the [Va] for the runtime virtual method table for this.
    fn vmt(&self) -> Va {
        *unsafe { NonNull::from_ref(self).cast::<Va>().as_ref() }
    }

    /// Returns whether this is an instance of `T`.
    fn is_subclass<T: Subclass<Self>>(&self) -> bool {
        let instance_vmt = self.vmt();
        let subclass_vmt = T::vmt_va();

        // Short circuit to handle the common case where self is direct instance of Subclass
        if subclass_vmt == instance_vmt {
            return true;
        }

        // Otherwise, dynamically check using RTTI data
        let instance_col = unsafe { complete_object_locator(instance_vmt) };
        let subclass_col = unsafe { complete_object_locator(subclass_vmt) };
        is_base_class(&Program::current(), subclass_col, instance_col).unwrap_or(false)
    }

    /// Returns this as a `T` if it is one. Otherwise, returns `None`.
    fn as_subclass<T: Subclass<Self>>(&self) -> Option<&T> {
        if self.is_subclass::<T>() {
            // Safety: We require that VMTs indicate object type.
            Some(unsafe { NonNull::from_ref(self).cast::<T>().as_ref() })
        } else {
            None
        }
    }

    /// Returns this as a mutable `T` if it is one. Otherwise, returns `None`.
    fn as_subclass_mut<T: Subclass<Self>>(&mut self) -> Option<&mut T> {
        if self.is_subclass::<T>() {
            // Safety: We require that VMTs indicate object type.
            Some(unsafe { NonNull::from_ref(self).cast::<T>().as_mut() })
        } else {
            None
        }
    }
}

/// A trait for C++ subclasses of the superclass `T`. Implementing this trait
/// makes it possible for Rust code to be generic over all subclasses of a given
/// C++ supertype.
///
/// ## Safety
///
/// In order to implement this for a struct, you must guarantee:
///
/// * The struct uses C-style layout.
/// * An initial subsequence of the struct is a valid instance of `T`.
pub unsafe trait Subclass<T: Superclass> {
    /// The RVA of this class's virtual method table.
    fn vmt_rva() -> Rva;

    /// The VA of this class's virtual method table.
    fn vmt_va() -> Va {
        Program::current()
            .rva_to_va(Self::vmt_rva())
            .expect("VMT address not in executable!")
    }

    /// Returns this as a `T`.
    fn superclass(&self) -> &T {
        // The implementer has vouched that this type's struct layout begins
        // with its superclass.
        unsafe { NonNull::from_ref(self).cast::<T>().as_ref() }
    }

    /// Returns this as a mutable `T`.
    fn superclass_mut(&mut self) -> &mut T {
        // The implementer has vouched that this type's struct layout begins
        // with its superclass.
        unsafe { NonNull::from_ref(self).cast::<T>().as_mut() }
    }
}

// Safety: Superclass has the same safety requirements as subclass.
unsafe impl<T> Subclass<T> for T
where
    T: Superclass,
{
    fn vmt_rva() -> Rva {
        <T as Superclass>::vmt_rva()
    }
}
