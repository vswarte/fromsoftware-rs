use std::cell::{RefCell, UnsafeCell};
use std::marker::PhantomData;

use vtable_rs::VPtr;
use windows::Win32::Foundation::{CloseHandle, HANDLE, WAIT_EVENT, WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows::Win32::System::Threading::{
    CRITICAL_SECTION, CreateEventW, CreateMutexW, DeleteCriticalSection, EnterCriticalSection,
    INFINITE, InitializeCriticalSection, InitializeCriticalSectionAndSpinCount,
    LeaveCriticalSection, ReleaseMutex, SetEvent, TryEnterCriticalSection, WaitForSingleObject,
};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DLSyncError {
    None,
    Resource = -1,
    Busy = -2,
    Timeout = -3,
    Unknown = -4,
}

/// Maps a `WaitForSingleObject`/`WaitForMultipleObjects` result to a
/// [`DLSyncError`].
impl From<WAIT_EVENT> for DLSyncError {
    fn from(event: WAIT_EVENT) -> Self {
        match event {
            WAIT_TIMEOUT => DLSyncError::Timeout,
            _ => DLSyncError::Unknown,
        }
    }
}

trait WaitResult {
    fn into_sync_result(self) -> Result<(), DLSyncError>;
}

impl WaitResult for WAIT_EVENT {
    fn into_sync_result(self) -> Result<(), DLSyncError> {
        match self {
            WAIT_OBJECT_0 => Ok(()),
            other => Err(other.into()),
        }
    }
}

/// A held lock on some [`DLSyncObject`]. Releases the lock when dropped.
#[must_use = "the lock is released immediately if the guard is not held"]
pub struct DLSyncGuard<'a, T: DLSyncObject + ?Sized> {
    object: &'a T,
    _not_send: PhantomData<*const ()>,
}

impl<'a, T: DLSyncObject + ?Sized> DLSyncGuard<'a, T> {
    fn new(object: &'a T) -> Self {
        Self {
            object,
            _not_send: PhantomData,
        }
    }
}

impl<T: DLSyncObject + ?Sized> Drop for DLSyncGuard<'_, T> {
    fn drop(&mut self) {
        let _ = unsafe { self.object.unlock() };
    }
}

pub trait DLSyncObject {
    fn is_valid(&self) -> bool;
    /// # Safety
    ///
    /// Can cause deadlocks when missued. Use [`Self::lock`] or [`Self::try_lock`] instead.
    unsafe fn raw_lock(&self) -> Result<(), DLSyncError>;
    /// # Safety
    ///
    /// Can cause deadlocks when missued. Use [`Self::lock`] or [`Self::try_lock`] instead.
    unsafe fn raw_try_lock(&self) -> Result<(), DLSyncError>;
    /// # Safety
    ///
    /// Each call should be preceded by a successful call to [`Self::raw_lock`] or [`Self::raw_try_lock`].
    unsafe fn unlock(&self) -> Result<(), DLSyncError>;
    /// Blocks until the lock is acquired, returning a guard that releases it on drop.
    fn lock(&self) -> Result<DLSyncGuard<'_, Self>, DLSyncError>
    where
        Self: Sized,
    {
        (unsafe { self.raw_lock() })?;
        Ok(DLSyncGuard::new(self))
    }

    /// Attempts to acquire the lock without blocking.
    fn try_lock(&self) -> Result<DLSyncGuard<'_, Self>, DLSyncError>
    where
        Self: Sized,
    {
        (unsafe { self.raw_try_lock() })?;
        Ok(DLSyncGuard::new(self))
    }
}

pub trait DLTimeoutSyncObject: DLSyncObject {
    /// # Safety
    ///
    /// Can cause deadlocks when missued. Use [`Self::lock_timeout`] instead.
    unsafe fn raw_lock_timeout(&self, timeout: i32) -> Result<(), DLSyncError>;

    fn lock_timeout(&self, timeout: i32) -> Result<DLSyncGuard<'_, Self>, DLSyncError>
    where
        Self: Sized,
    {
        (unsafe { self.raw_lock_timeout(timeout) })?;
        Ok(DLSyncGuard::new(self))
    }
}

#[vtable_rs::vtable]
pub trait DLPlainLightMutexVmt {
    fn destructor(&mut self, flags: u8);
}

#[repr(C)]
/// Source of name: RTTI
pub struct DLPlainLightMutex {
    pub vftable: VPtr<dyn DLPlainLightMutexVmt, Self>,
    pub critical_section: UnsafeCell<CRITICAL_SECTION>,
}

impl Default for DLPlainLightMutex {
    fn default() -> Self {
        let ins = Self {
            vftable: Default::default(),
            critical_section: Default::default(),
        };
        unsafe { InitializeCriticalSection(ins.critical_section.get()) }
        ins
    }
}

impl Drop for DLPlainLightMutex {
    fn drop(&mut self) {
        unsafe { DeleteCriticalSection(self.critical_section.get()) }
    }
}

impl DLPlainLightMutex {
    /// Blocks until the critical section is entered, returning a guard that leaves it on drop.
    pub fn lock(&self) -> DLSyncGuard<'_, Self> {
        unsafe { EnterCriticalSection(self.critical_section.get()) }
        DLSyncGuard::new(self)
    }

    /// Attempts to enter the critical section without blocking.
    pub fn try_lock(&self) -> Option<DLSyncGuard<'_, Self>> {
        unsafe {
            if TryEnterCriticalSection(self.critical_section.get()).as_bool() {
                Some(DLSyncGuard::new(self))
            } else {
                None
            }
        }
    }
}

impl DLSyncObject for DLPlainLightMutex {
    fn is_valid(&self) -> bool {
        true
    }

    unsafe fn raw_lock(&self) -> Result<(), DLSyncError> {
        unsafe { EnterCriticalSection(self.critical_section.get()) }
        Ok(())
    }

    unsafe fn raw_try_lock(&self) -> Result<(), DLSyncError> {
        unsafe {
            if TryEnterCriticalSection(self.critical_section.get()).as_bool() {
                Ok(())
            } else {
                Err(DLSyncError::Busy)
            }
        }
    }

    unsafe fn unlock(&self) -> Result<(), DLSyncError> {
        unsafe { LeaveCriticalSection(self.critical_section.get()) }
        Ok(())
    }
}

impl DLPlainLightMutexVmt for DLPlainLightMutex {
    extern "C" fn destructor(&mut self, flags: u8) {
        unsafe { DeleteCriticalSection(self.critical_section.get()) }
        if flags & 1 != 0 {
            let _ = unsafe { Box::from_raw(self as *mut Self) };
        }
    }
}

#[vtable_rs::vtable]
pub trait PlainAdaptiveMutexImplVmt {
    fn destructor(&mut self, flags: u8);
}

impl PlainAdaptiveMutexImplVmt for PlainAdaptiveMutexImpl {
    extern "C" fn destructor(&mut self, flags: u8) {
        unsafe { DeleteCriticalSection(self.critical_section.get()) }
        if flags & 1 != 0 {
            let _ = unsafe { Box::from_raw(self as *mut Self) };
        }
    }
}

const DEFAULT_SPIN_COUNT: u32 = 4000;

#[repr(C)]
pub struct PlainAdaptiveMutexImpl {
    pub vftable: VPtr<dyn PlainAdaptiveMutexImplVmt, Self>,
    pub critical_section: UnsafeCell<CRITICAL_SECTION>,
    pub spin_count: u32,
}

impl PlainAdaptiveMutexImpl {
    /// Pass `None` to use the default spin count of 4000
    pub fn new(spin_count: Option<u32>) -> Result<Self, DLSyncError> {
        let spin_count = spin_count.unwrap_or(DEFAULT_SPIN_COUNT);
        let ins = Self {
            vftable: Default::default(),
            critical_section: Default::default(),
            spin_count,
        };
        unsafe {
            if InitializeCriticalSectionAndSpinCount(ins.critical_section.get(), spin_count)
                .is_err()
            {
                return Err(DLSyncError::Resource);
            }
        }
        Ok(ins)
    }

    /// Blocks until the critical section is entered, returning a guard that leaves it on drop.
    pub fn lock(&self) -> DLSyncGuard<'_, Self> {
        unsafe { EnterCriticalSection(self.critical_section.get()) }
        DLSyncGuard::new(self)
    }

    /// Attempts to enter the critical section without blocking.
    pub fn try_lock(&self) -> Option<DLSyncGuard<'_, Self>> {
        unsafe {
            if TryEnterCriticalSection(self.critical_section.get()).as_bool() {
                Some(DLSyncGuard::new(self))
            } else {
                None
            }
        }
    }
}

impl Default for PlainAdaptiveMutexImpl {
    fn default() -> Self {
        Self::new(None).expect("DLAdaptiveMutex initialization error")
    }
}

impl Drop for PlainAdaptiveMutexImpl {
    fn drop(&mut self) {
        unsafe { DeleteCriticalSection(self.critical_section.get()) }
    }
}

impl DLSyncObject for PlainAdaptiveMutexImpl {
    fn is_valid(&self) -> bool {
        true
    }

    unsafe fn raw_lock(&self) -> Result<(), DLSyncError> {
        unsafe { EnterCriticalSection(self.critical_section.get()) }
        Ok(())
    }

    unsafe fn raw_try_lock(&self) -> Result<(), DLSyncError> {
        unsafe {
            if TryEnterCriticalSection(self.critical_section.get()).as_bool() {
                Ok(())
            } else {
                Err(DLSyncError::Busy)
            }
        }
    }

    unsafe fn unlock(&self) -> Result<(), DLSyncError> {
        unsafe { LeaveCriticalSection(self.critical_section.get()) }
        Ok(())
    }
}

#[vtable_rs::vtable]
pub trait DLPlainReadWriteLockVmt {
    fn destructor(&mut self, flags: u8);
}

impl DLPlainReadWriteLockVmt for DLPlainReadWriteLock {
    extern "C" fn destructor(&mut self, flags: u8) {
        unsafe {
            if !self.h_event.is_invalid() {
                let _ = CloseHandle(self.h_event);
            }
            if !self.h_writer_mutex.is_invalid() {
                let _ = CloseHandle(self.h_writer_mutex);
            }
            if !self.h_reader_mutex.is_invalid() {
                let _ = CloseHandle(self.h_reader_mutex);
            }
        }
        if flags & 1 != 0 {
            let _ = unsafe { Box::from_raw(self as *mut Self) };
        }
    }
}

#[repr(C)]
pub struct DLPlainReadWriteLock {
    pub vftable: VPtr<dyn DLPlainReadWriteLockVmt, Self>,
    pub h_event: HANDLE,
    pub h_writer_mutex: HANDLE,
    pub h_reader_mutex: HANDLE,
    pub reader_count: UnsafeCell<i32>,
}
unsafe impl Sync for DLPlainReadWriteLock {}

/// A held read lock. Releases it when dropped.
#[must_use = "the lock is released immediately if the guard is not held"]
pub struct DLReadGuard<'a> {
    lock: &'a DLPlainReadWriteLock,
}

impl Drop for DLReadGuard<'_> {
    fn drop(&mut self) {
        let _ = self.lock.read_unlock();
    }
}

impl DLPlainReadWriteLock {
    pub fn new() -> Result<Self, DLSyncError> {
        unsafe {
            let h_event =
                CreateEventW(None, false, true, None).map_err(|_| DLSyncError::Resource)?;
            let h_reader_mutex =
                CreateMutexW(None, false, None).map_err(|_| DLSyncError::Resource)?;
            let h_writer_mutex =
                CreateMutexW(None, false, None).map_err(|_| DLSyncError::Resource)?;

            Ok(Self {
                vftable: Default::default(),
                h_event,
                h_writer_mutex,
                h_reader_mutex,
                reader_count: UnsafeCell::new(-1),
            })
        }
    }

    fn win_timeout(timeout: i32) -> u32 {
        match timeout {
            -1 => INFINITE,
            t => (t / 1000) as u32,
        }
    }

    /// # Safety
    ///
    /// Caller must hold `h_reader_mutex`.
    unsafe fn reader_count(&self) -> i32 {
        unsafe { *self.reader_count.get() }
    }

    /// # Safety
    ///
    /// Caller must hold `h_reader_mutex`.
    unsafe fn set_reader_count(&self, value: i32) {
        unsafe { *self.reader_count.get() = value }
    }

    /// Acquire the write lock (exclusive), returning a guard that releases it on drop.
    pub fn write_lock(&self, timeout: i32) -> Result<DLSyncGuard<'_, Self>, DLSyncError> {
        self.raw_write_lock(timeout)?;
        Ok(DLSyncGuard::new(self))
    }

    /// Try to acquire the write lock without blocking
    pub fn try_write_lock(&self) -> Result<DLSyncGuard<'_, Self>, DLSyncError> {
        self.raw_try_write_lock()?;
        Ok(DLSyncGuard::new(self))
    }

    fn raw_write_lock(&self, timeout: i32) -> Result<(), DLSyncError> {
        assert!(self.is_valid(), "Sync object isn't created");
        assert!(timeout != 0, "Illegal timeout value");

        let ms = Self::win_timeout(timeout);
        unsafe { WaitForSingleObject(self.h_writer_mutex, ms) }.into_sync_result()?;

        if let Err(e) = unsafe { WaitForSingleObject(self.h_event, ms) }.into_sync_result() {
            let _ = unsafe { ReleaseMutex(self.h_writer_mutex) };
            return Err(e);
        }
        Ok(())
    }

    /// Try to acquire the write lock without blocking.
    fn raw_try_write_lock(&self) -> Result<(), DLSyncError> {
        assert!(self.is_valid(), "Sync object isn't created");

        unsafe { WaitForSingleObject(self.h_writer_mutex, 0) }
            .into_sync_result()
            .map_err(|_| DLSyncError::Busy)?;

        if unsafe { WaitForSingleObject(self.h_event, 0) }
            .into_sync_result()
            .is_err()
        {
            let _ = unsafe { ReleaseMutex(self.h_writer_mutex) };
            return Err(DLSyncError::Busy);
        }
        Ok(())
    }

    /// Release the write lock
    pub fn write_unlock(&self) -> Result<(), DLSyncError> {
        assert!(self.is_valid(), "Sync object isn't created");
        unsafe {
            let _ = SetEvent(self.h_event);
            let _ = ReleaseMutex(self.h_writer_mutex);
        }
        Ok(())
    }

    /// Acquire the read lock, returning a guard that releases it on drop.
    pub fn read_lock(&self, timeout: i32) -> Result<DLReadGuard<'_>, DLSyncError> {
        assert!(self.is_valid(), "Sync object isn't created");
        assert!(timeout != 0, "Illegal timeout value");

        let ms = Self::win_timeout(timeout);
        unsafe { WaitForSingleObject(self.h_reader_mutex, ms) }.into_sync_result()?;

        let reader_count = unsafe { self.reader_count().saturating_add(1) };
        unsafe { self.set_reader_count(reader_count) };

        if reader_count == 0
            && let Err(e) = unsafe { WaitForSingleObject(self.h_event, ms) }.into_sync_result()
        {
            unsafe { self.set_reader_count(-1) };
            let _ = unsafe { ReleaseMutex(self.h_reader_mutex) };
            return Err(e);
        }

        let _ = unsafe { ReleaseMutex(self.h_reader_mutex) };
        Ok(DLReadGuard { lock: self })
    }

    /// Try to acquire the read lock without blocking
    pub fn try_read_lock(&self) -> Result<DLReadGuard<'_>, DLSyncError> {
        assert!(self.is_valid(), "Sync object isn't created");

        if unsafe { WaitForSingleObject(self.h_reader_mutex, 0) }
            .into_sync_result()
            .is_err()
        {
            return Err(DLSyncError::Busy);
        }

        let reader_count = unsafe { self.reader_count().saturating_add(1) };
        unsafe { self.set_reader_count(reader_count) };

        let mut result = Ok(());
        if reader_count == 0
            && unsafe { WaitForSingleObject(self.h_event, 0) }
                .into_sync_result()
                .is_err()
        {
            unsafe { self.set_reader_count(-1) };
            result = Err(DLSyncError::Busy);
        }

        let _ = unsafe { ReleaseMutex(self.h_reader_mutex) };
        result?;
        Ok(DLReadGuard { lock: self })
    }

    /// Release the read lock
    fn read_unlock(&self) -> Result<(), DLSyncError> {
        assert!(self.is_valid(), "Sync object isn't created");

        let _ = unsafe { WaitForSingleObject(self.h_reader_mutex, INFINITE) };
        let reader_count = unsafe { self.reader_count().saturating_sub(1) };
        unsafe { self.set_reader_count(reader_count) };
        if reader_count < 0 {
            let _ = unsafe { SetEvent(self.h_event) };
        }
        let _ = unsafe { ReleaseMutex(self.h_reader_mutex) };
        Ok(())
    }
}

impl DLSyncObject for DLPlainReadWriteLock {
    fn is_valid(&self) -> bool {
        !self.h_event.is_invalid()
            && !self.h_writer_mutex.is_invalid()
            && !self.h_reader_mutex.is_invalid()
    }

    /// Acquire write lock
    unsafe fn raw_lock(&self) -> Result<(), DLSyncError> {
        self.raw_write_lock(-1)
    }

    /// Try to acquire write lock
    unsafe fn raw_try_lock(&self) -> Result<(), DLSyncError> {
        self.raw_try_write_lock()
    }

    /// Release write lock
    unsafe fn unlock(&self) -> Result<(), DLSyncError> {
        self.write_unlock()
    }
}

impl DLTimeoutSyncObject for DLPlainReadWriteLock {
    unsafe fn raw_lock_timeout(&self, timeout: i32) -> Result<(), DLSyncError> {
        self.raw_write_lock(timeout)
    }
}

impl Drop for DLPlainReadWriteLock {
    fn drop(&mut self) {
        unsafe {
            if !self.h_event.is_invalid() {
                let _ = CloseHandle(self.h_event);
            }
            if !self.h_writer_mutex.is_invalid() {
                let _ = CloseHandle(self.h_writer_mutex);
            }
            if !self.h_reader_mutex.is_invalid() {
                let _ = CloseHandle(self.h_reader_mutex);
            }
        }
    }
}

#[vtable_rs::vtable]
pub trait DLDummySyncObjectVmt {
    fn destructor(&mut self, flags: u8);
}

impl DLDummySyncObjectVmt for DLDummySyncObject {
    extern "C" fn destructor(&mut self, flags: u8) {
        if flags & 1 != 0 {
            let _ = unsafe { Box::from_raw(self as *mut Self) };
        }
    }
}

#[repr(C)]
#[derive(Default)]
pub struct DLDummySyncObject {
    pub vftable: VPtr<dyn DLDummySyncObjectVmt, Self>,
    _not_sync: PhantomData<RefCell<()>>,
}

impl DLDummySyncObject {
    pub fn new() -> Self {
        Default::default()
    }
}

impl DLSyncObject for DLDummySyncObject {
    fn is_valid(&self) -> bool {
        true
    }
    unsafe fn raw_lock(&self) -> Result<(), DLSyncError> {
        Ok(())
    }
    unsafe fn raw_try_lock(&self) -> Result<(), DLSyncError> {
        Ok(())
    }
    unsafe fn unlock(&self) -> Result<(), DLSyncError> {
        Ok(())
    }
}

pub trait ThreadingPolicy {
    type LockObject: DLSyncObject;
    const IS_THREAD_SAFE: bool;
}

pub struct DLSingleThreadingPolicy;

impl ThreadingPolicy for DLSingleThreadingPolicy {
    type LockObject = DLDummySyncObject;
    const IS_THREAD_SAFE: bool = false;
}

pub struct DLMultiThreadingPolicy;

impl ThreadingPolicy for DLMultiThreadingPolicy {
    type LockObject = DLPlainLightMutex;
    const IS_THREAD_SAFE: bool = true;
}
