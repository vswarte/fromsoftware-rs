pub use from_singleton::*;

/// Looks up instances of singleton instances by their name.
/// Some singletons aren't necessarily always instanciated and available.
/// Discovered singletons are cached so invokes after the first will be much faster.
///
/// # Safety
/// User must ensure that:
///  - The main module (the exe) is a From Software title with DLRF reflection data.
///  - The DLRF reflection metadata has been populated (wait_for_system_init).
///  - Access to the singleton is exclusive (either by hooking or utilizing the task system).
///  - get_instance is not called multiple times such that it spawns multiple mutable references to the same singleton.
pub unsafe fn get_instance<T: FromSingleton + Sized>() -> Option<&'static mut T> {
    address_of::<T>().map(|mut ptr| ptr.as_mut())
}
