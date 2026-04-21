use std::{borrow::Cow, ptr::NonNull};

use from_singleton::*;
use pelite::pe64::{Pe, Rva};
use thiserror::Error;

use crate::Program;

/// An error type returned by [FromStatic::instance].
#[derive(Error, Debug)]
pub enum InstanceError {
    /// The object's location wasn't found in the executable. This usually means
    /// something is wrong with the logic of how the object is being loaded in
    /// the first place.
    #[error("Static object not found: {0}")]
    NotFound(Cow<'static, str>),

    /// The static object is defined, but it's currently set to null. For many
    /// objects, this is a normal occurrence, and just means that the caller
    /// should wait until it's defined to start using it.
    #[error("Static object not initialized: {0}")]
    Null(Cow<'static, str>),
}

/// A [Result] whose error type is [InstanceError].
pub type InstanceResult<T> = Result<T, InstanceError>;

/// A trait for all objects that are instantiated a single time at a fixed point
/// in memory.
///
/// This is automatically implemented for [FromSingleton]s generated using the
/// [singleton](crate::singleton) attribute macro, and may be manually
/// implemented for other types that have different ways of looking up their
/// locations in-memory.
pub trait FromStatic {
    /// The name of this object. Used for debugging purposes.
    fn name() -> Cow<'static, str>;

    /// Looks up the single global instance of this object as a reference.
    ///
    /// This function is safe because it's always already unsafe to dereference
    /// a pointer. Most callers should use [FromStatic::instance] or
    /// [FromStatic::instance_mut] instead, which have more explicit safety
    /// requirements but provide access to Rust references in exchange.
    fn instance_ptr() -> InstanceResult<*mut Self>;

    /// Looks up the single global instance of this object as a mutable
    /// reference.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that access to the static object is exclusive,
    /// both with Rust and the game's code. This means that this should only
    /// ever be called from the game's main thread (typically either from the
    /// task system or from hooked functions that run in the main thread).
    ///
    /// Individual implementations may add additional safety requirements.
    unsafe fn instance_mut() -> InstanceResult<&'static mut Self> {
        Self::instance_ptr()
            .and_then(|p| unsafe { p.as_mut() }.ok_or(InstanceError::Null(Self::name())))
    }

    /// Looks up the single global instance of this object as a reference.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that no mutable references exist to the static
    /// object and that no fields outside of [UnsafeCell]s are mutated by the
    /// game while this reference exists. This is generally safe to use on the
    /// main thread. It's safe to use on other threads as long as the object is
    /// thread-safe, which should be noted in its documentation.
    ///
    /// [UnsafeCell]: std::cell::UnsafeCell
    ///
    /// Individual implementations may add additional safety requirements.
    unsafe fn instance() -> InstanceResult<&'static Self> {
        Self::instance_ptr()
            .and_then(|p| unsafe { p.as_ref() }.ok_or(InstanceError::Null(Self::name())))
    }
}

/// Looks up instances of singleton instances by their name. Some singletons
/// aren't necessarily always instanciated and available. Discovered singletons
/// are cached so invokes after the first will be much faster.
///
/// Note: currently this never returns [InstanceError::NotFound], but callers
/// shouldn't rely on that being true into the future.
impl<T: FromSingleton> FromStatic for T {
    fn name() -> Cow<'static, str> {
        <Self as FromSingleton>::name()
    }

    /// ## Safety
    ///
    /// In addition to the standard [FromStatic::instance] safety requirements, the
    /// caller must ensure that the main module (the exe) is a From Software title
    /// with DLRF reflection data, and that the DLRF reflection metadata has been
    /// populated (usually by calling the current game's `wait_for_system_init`
    /// function).
    fn instance_ptr() -> InstanceResult<*mut T> {
        address_of::<T>()
            .map(|nn| nn.as_ptr())
            .ok_or(InstanceError::NotFound(Self::name()))
    }
}

/// Loads a static reference to `T` from an [Rva] that points directly to its
/// memory. Because this always assumes that the underlying object is
/// initialized, it can only return [InstanceError::Null] if `rva` itself is 0.
pub fn load_static_direct<T: FromStatic>(rva: Rva) -> InstanceResult<*mut T> {
    Program::current()
        .rva_to_va(rva)
        .map_err(|_| InstanceError::NotFound(T::name()))
        .map(|a| a as *mut T)
}

/// Loads a static reference to `T` from an [Rva] that points to a pointer to
/// its memory.
///
/// ## Safety
///
/// The caller must ensure that `rva` points to a pointer.
pub unsafe fn load_static_indirect<T: FromStatic>(rva: Rva) -> InstanceResult<*mut T> {
    let target = Program::current()
        .rva_to_va(rva)
        .map_err(|_| InstanceError::NotFound(T::name()))?
        as *mut Option<NonNull<T>>;

    unsafe {
        target
            .as_mut()
            .and_then(|opt| opt.as_mut())
            .map(|nn| nn.as_ptr())
            .ok_or(InstanceError::Null(T::name()))
    }
}
