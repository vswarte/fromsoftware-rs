use std::ptr::NonNull;

use pelite::pe64::{Pe, Rva, Va};
use thiserror::Error;

use crate::Program;

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

/// A trait for C++ types that have multiple different subclasses. Implementing
/// this for a superclass and [Subclass] for its subclasses makes it possible to
/// safely check for the object's actual type based on its vtable.
///
/// ## Safety
///
/// In order to implement this for a struct, you must guarantee:
///
/// * The struct uses C-style layout.
/// * The first element of the struct is a pointer to a C++-style vtable.
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
    ///
    /// **Note:** Because this just checks the address of the virtual method
    /// table, it will return `false` for *subclasses* of `T` even though C++
    /// considers them to be of type `T`. As a special case, if `T` is `Self`,
    /// this will return `true` even for subclasses.
    fn is_subclass<T: Subclass<Self>>(&self) -> bool {
        // If T is Self (which we can check by comparing their VMTs), then we
        // know this is a subclass without checking its VMT. This allows us to
        // always return `true` for the trivial case of the root class.
        let t_vmt = T::vmt_va();
        t_vmt == Self::vmt_va() || t_vmt == self.vmt()
    }

    /// Returns this as a `T` if it is one. Otherwise, returns `None`.
    ///
    /// **Note:** Because this just checks the address of the virtual method
    /// table, it will return `None` for *subclasses* of `T` even though C++
    /// considers them to be of type `T`. As a special case, if `T` is `Self`,
    /// this will return `true` even for subclasses.
    fn as_subclass<T: Subclass<Self>>(&self) -> Option<&T> {
        if self.is_subclass::<T>() {
            // Safety: We require that VMTs indicate object type.
            Some(unsafe { NonNull::from_ref(self).cast::<T>().as_ref() })
        } else {
            None
        }
    }

    /// Returns this as a mutable `T` if it is one. Otherwise, returns `None`.
    ///
    /// **Note:** Because this just checks the address of the virtual method
    /// table, it will return `None` for *subclasses* of `T` even though C++
    /// considers them to be of type `T`. As a special case, if `T` is `Self`,
    /// this will return `true` even for subclasses.
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
/// * An initial subsequence of the struct is a valid isntance of `T`.
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
