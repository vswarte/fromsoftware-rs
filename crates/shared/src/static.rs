use from_singleton::*;
use thiserror::Error;

/// An error type returned by [FromStatic::instance].
#[derive(Error, Debug)]
pub enum InstanceError {
    /// The object's location wasn't found in the executable. This usually means
    /// something is wrong with the logic of how the object is being loaded in
    /// the first place.
    #[error("Static object not found")]
    NotFound,

    /// The static object is defined, but it's currently set to null. For many
    /// objects, this is a normal occurrence, and just means that the caller
    /// should wait until it's defined to start using it.
    #[error("Static object not initialized")]
    Null,
}

/// A [Result] whose error type is [InstanceError].
pub type InstanceResult<T> = Result<T, InstanceError>;

/// A trait for all objects that are instantiated a single time at a fixed point
/// in memory.
///
/// This is automatically implemented for [FromSingleton]s generated using the
/// [from_singleton!] macro, and may be manually implemented for other types
/// that have different ways of looking up their locations in-memory.
pub trait FromStatic {
    /// Looks up the single global instance of this object.
    ///
    /// Implementations may cache information about the object's location to
    /// make this more efficient in future calls.
    ///
    /// ## Safety
    ///
    /// The caller must ensure that access to the static object is exclusive,
    /// both with Rust and the game's code. For single-threaded objects, this
    /// means ensuring that this is only called from the task system or from
    /// hooked functions running in the game's main thread. For multi-threaded
    /// objects, it's sufficient to ensure you have mutex ownership before
    /// accessing any locked fields.
    ///
    /// Individual implementations may add additional safety requirements.
    unsafe fn instance() -> InstanceResult<&'static mut Self>;
}

/// Looks up instances of singleton instances by their name. Some singletons
/// aren't necessarily always instanciated and available. Discovered singletons
/// are cached so invokes after the first will be much faster.
///
/// Note: currently this never returns [GetInstanceError::NotFound], but callers
/// shouldn't rely on that being true into the future.
impl<T: FromSingleton> FromStatic for T {
    /// ## Safety
    ///
    /// In addition to the standard [FromStatic::instance] safety requirements, the
    /// caller must ensure that the main module (the exe) is a From Software title
    /// with DLRF reflection data, and that the DLRF reflection metadata has been
    /// populated (usually by calling the current game's `wait_for_system_init`
    /// function).
    unsafe fn instance() -> InstanceResult<&'static mut T> {
        address_of::<T>()
            .map(|mut ptr| unsafe { ptr.as_mut() })
            .ok_or(InstanceError::NotFound)
    }
}
