use from_singleton::*;
use thiserror::Error;

/// An error type returned by [get_instance] and other functions that load
/// singleton instances by other means.
#[derive(Error, Debug)]
pub enum GetInstanceError {
    /// The singleton's location wasn't found in the executable. This usually
    /// means something is wrong with the logic of how the singleton is being
    /// loaded in the first place.
    #[error("Singleton not found")]
    NotFound,

    /// The singleton is defined, but it's currently set to null. For many
    /// singletons, this is a normal occurrence, and just means that the caller
    /// should wait until it's defined to start using it.
    #[error("Singleton not initialized")]
    Null,
}

/// A [Result] whose error type is [GetInstanceError].
pub type GetInstanceResult<T> = Result<T, GetInstanceError>;

/// Looks up instances of singleton instances by their name. Some singletons
/// aren't necessarily always instanciated and available. Discovered singletons
/// are cached so invokes after the first will be much faster.
///
/// Note: currently this never returns [GetInstanceError::NotFound], but callers
/// shouldn't rely on that being true into the future.
///
/// # Safety
/// User must ensure that:
///  - The main module (the exe) is a From Software title with DLRF reflection data.
///  - The DLRF reflection metadata has been populated (wait_for_system_init).
///  - Access to the singleton is exclusive (either by hooking or utilizing the task system).
///  - get_instance is not called multiple times such that it spawns multiple mutable references to the same singleton.
pub unsafe fn get_instance<T: FromSingleton + Sized>() -> GetInstanceResult<&'static mut T> {
    address_of::<T>()
        .map(|mut ptr| ptr.as_mut())
        .ok_or(GetInstanceError::NotFound)
}
