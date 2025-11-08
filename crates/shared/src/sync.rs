/// A trait for From Software types that contain mutexes, and so can (and
/// should) be locked before being modified.
pub trait MutexBearer {
    /// Locks the mutex owned by this object. This must block until the caller
    /// has a global exclusive lock on `self`.
    fn lock(&self);

    /// Unlocks the mutex owned by this object. This must release the caller's
    /// global exclusive lock on `self`. It should never be called unless the
    /// object is already locked.
    fn unlock(&self);
}

// TODO: Add a wrapper type FSMutex<T: MutexBearer> that transparently wraps a
// pointer to a MutexBearer and only provides access while the mutex is locked,
// just like the core library Mutex<T>.
