use vtable_rs::VPtr;
use windows::Win32::Foundation::{CloseHandle, HANDLE, WAIT_OBJECT_0, WAIT_TIMEOUT};
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

pub trait DLSyncObject {
    fn is_valid(&self) -> bool;
    fn lock(&mut self, timeout: i32) -> Result<(), DLSyncError>;
    fn try_lock(&mut self) -> Result<(), DLSyncError>;
    fn unlock(&mut self) -> Result<(), DLSyncError>;
}

#[vtable_rs::vtable]
pub trait DLPlainLightMutexVmt {
    fn destructor(&mut self, flags: u8);
}

#[repr(C)]
/// Source of name: RTTI
pub struct DLPlainLightMutex {
    pub vftable: VPtr<dyn DLPlainLightMutexVmt, Self>,
    pub critical_section: CRITICAL_SECTION,
}

impl Default for DLPlainLightMutex {
    fn default() -> Self {
        let mut ins = Self {
            vftable: Default::default(),
            critical_section: Default::default(),
        };
        unsafe { InitializeCriticalSection(&mut ins.critical_section) }
        ins
    }
}

impl Drop for DLPlainLightMutex {
    fn drop(&mut self) {
        unsafe { DeleteCriticalSection(&mut self.critical_section) }
    }
}

impl DLPlainLightMutex {
    pub fn lock(&mut self) {
        unsafe { EnterCriticalSection(&mut self.critical_section) }
    }

    pub fn unlock(&mut self) {
        unsafe { LeaveCriticalSection(&mut self.critical_section) }
    }
}

impl DLSyncObject for DLPlainLightMutex {
    fn is_valid(&self) -> bool {
        true
    }

    fn lock(&mut self, _timeout: i32) -> Result<(), DLSyncError> {
        unsafe { EnterCriticalSection(&mut self.critical_section) }
        Ok(())
    }

    fn try_lock(&mut self) -> Result<(), DLSyncError> {
        unsafe {
            if TryEnterCriticalSection(&mut self.critical_section).as_bool() {
                Ok(())
            } else {
                Err(DLSyncError::Busy)
            }
        }
    }

    fn unlock(&mut self) -> Result<(), DLSyncError> {
        unsafe { LeaveCriticalSection(&mut self.critical_section) }
        Ok(())
    }
}

impl DLPlainLightMutexVmt for DLPlainLightMutex {
    extern "C" fn destructor(&mut self, flags: u8) {
        unsafe { DeleteCriticalSection(&mut self.critical_section) }
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
        unsafe { DeleteCriticalSection(&mut self.critical_section) }
        if flags & 1 != 0 {
            let _ = unsafe { Box::from_raw(self as *mut Self) };
        }
    }
}

const DEFAULT_SPIN_COUNT: u32 = 4000;

#[repr(C)]
pub struct PlainAdaptiveMutexImpl {
    pub vftable: VPtr<dyn PlainAdaptiveMutexImplVmt, Self>,
    pub critical_section: CRITICAL_SECTION,
    pub spin_count: u32,
}

impl PlainAdaptiveMutexImpl {
    /// Pass `None` to use the default spin count of 4000
    pub fn new(spin_count: Option<u32>) -> Result<Self, DLSyncError> {
        let spin_count = spin_count.unwrap_or(DEFAULT_SPIN_COUNT);
        let mut ins = Self {
            vftable: Default::default(),
            critical_section: Default::default(),
            spin_count,
        };
        unsafe {
            if InitializeCriticalSectionAndSpinCount(&mut ins.critical_section, spin_count).is_err()
            {
                return Err(DLSyncError::Resource);
            }
        }
        Ok(ins)
    }
}

impl Default for PlainAdaptiveMutexImpl {
    fn default() -> Self {
        Self::new(None).expect("DLAdaptiveMutex initialization error")
    }
}

impl Drop for PlainAdaptiveMutexImpl {
    fn drop(&mut self) {
        unsafe { DeleteCriticalSection(&mut self.critical_section) }
    }
}

impl DLSyncObject for PlainAdaptiveMutexImpl {
    fn is_valid(&self) -> bool {
        true
    }

    fn lock(&mut self, _timeout: i32) -> Result<(), DLSyncError> {
        unsafe { EnterCriticalSection(&mut self.critical_section) }
        Ok(())
    }

    fn try_lock(&mut self) -> Result<(), DLSyncError> {
        unsafe {
            if windows::Win32::System::Threading::TryEnterCriticalSection(
                &mut self.critical_section,
            )
            .as_bool()
            {
                Ok(())
            } else {
                Err(DLSyncError::Busy)
            }
        }
    }

    fn unlock(&mut self) -> Result<(), DLSyncError> {
        unsafe { LeaveCriticalSection(&mut self.critical_section) }
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
            if self.h_event != HANDLE(0) {
                let _ = CloseHandle(self.h_event);
            }
            if self.h_writer_mutex != HANDLE(0) {
                let _ = CloseHandle(self.h_writer_mutex);
            }
            if self.h_reader_mutex != HANDLE(0) {
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
    pub reader_count: i32,
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
                reader_count: -1,
            })
        }
    }

    fn win_timeout(timeout: i32) -> u32 {
        match timeout {
            -1 => INFINITE,
            t => (t / 1000) as u32,
        }
    }

    fn wait_result(result: u32) -> Result<(), DLSyncError> {
        match result {
            r if r == WAIT_OBJECT_0.0 => Ok(()),
            r if r == WAIT_TIMEOUT.0 => Err(DLSyncError::Timeout),
            _ => Err(DLSyncError::Unknown),
        }
    }

    /// Acquire the write lock (exclusive)
    pub fn write_lock(&mut self, timeout: i32) -> Result<(), DLSyncError> {
        assert!(self.is_valid(), "Sync object isn't created");
        assert!(timeout != 0, "Illegal timeout value");

        let ms = Self::win_timeout(timeout);
        unsafe {
            Self::wait_result(WaitForSingleObject(self.h_writer_mutex, ms).0)?;
            match Self::wait_result(WaitForSingleObject(self.h_event, ms).0) {
                Ok(()) => Ok(()),
                Err(e) => {
                    let _ = ReleaseMutex(self.h_writer_mutex);
                    Err(e)
                }
            }
        }
    }

    /// Try to acquire the write lock without blocking
    pub fn try_write_lock(&mut self) -> Result<(), DLSyncError> {
        self.write_lock(1)
    }

    /// Release the write lock
    pub fn write_unlock(&mut self) -> Result<(), DLSyncError> {
        assert!(self.is_valid(), "Sync object isn't created");
        unsafe {
            let _ = SetEvent(self.h_event);
            let _ = ReleaseMutex(self.h_writer_mutex);
        }
        Ok(())
    }

    /// Acquire the read lock
    pub fn read_lock(&mut self, timeout: i32) -> Result<(), DLSyncError> {
        assert!(self.is_valid(), "Sync object isn't created");
        assert!(timeout != 0, "Illegal timeout value");

        let ms = Self::win_timeout(timeout);
        unsafe {
            Self::wait_result(WaitForSingleObject(self.h_reader_mutex, ms).0)?;
            self.reader_count += 1;
            let result = if self.reader_count == 0 {
                match Self::wait_result(WaitForSingleObject(self.h_event, ms).0) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        self.reader_count -= 1;
                        Err(e)
                    }
                }
            } else {
                Ok(())
            };
            let _ = ReleaseMutex(self.h_reader_mutex);
            result
        }
    }

    /// Try to acquire the read lock without blocking
    pub fn try_read_lock(&mut self) -> Result<(), DLSyncError> {
        self.read_lock(1)
    }

    /// Release the read lock
    pub fn read_unlock(&mut self) -> Result<(), DLSyncError> {
        assert!(self.is_valid(), "Sync object isn't created");
        unsafe {
            WaitForSingleObject(self.h_reader_mutex, INFINITE);
            self.reader_count -= 1;
            if self.reader_count < 0 {
                let _ = SetEvent(self.h_event);
            }
            let _ = ReleaseMutex(self.h_reader_mutex);
        }
        Ok(())
    }
}

impl DLSyncObject for DLPlainReadWriteLock {
    fn is_valid(&self) -> bool {
        self.h_event != HANDLE(0)
            && self.h_writer_mutex != HANDLE(0)
            && self.h_reader_mutex != HANDLE(0)
    }

    /// Acquire write lock
    fn lock(&mut self, timeout: i32) -> Result<(), DLSyncError> {
        self.write_lock(timeout)
    }

    /// Try to acquire write lock
    fn try_lock(&mut self) -> Result<(), DLSyncError> {
        self.try_write_lock()
    }

    /// Release write lock
    fn unlock(&mut self) -> Result<(), DLSyncError> {
        self.write_unlock()
    }
}

impl Drop for DLPlainReadWriteLock {
    fn drop(&mut self) {
        unsafe {
            if self.h_event != HANDLE(0) {
                let _ = CloseHandle(self.h_event);
            }
            if self.h_writer_mutex != HANDLE(0) {
                let _ = CloseHandle(self.h_writer_mutex);
            }
            if self.h_reader_mutex != HANDLE(0) {
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
pub struct DLDummySyncObject {
    pub vftable: VPtr<dyn DLDummySyncObjectVmt, Self>,
}

impl DLSyncObject for DLDummySyncObject {
    fn is_valid(&self) -> bool {
        true
    }
    fn lock(&mut self, _timeout: i32) -> Result<(), crate::dlkr::mutex::DLSyncError> {
        Ok(())
    }
    fn try_lock(&mut self) -> Result<(), crate::dlkr::mutex::DLSyncError> {
        Ok(())
    }
    fn unlock(&mut self) -> Result<(), crate::dlkr::mutex::DLSyncError> {
        Ok(())
    }
}

pub trait ThreadingPolicy {
    type LockObject: DLSyncObject;

    const IS_THREAD_SAFE: bool;
}

pub struct DLSingleThreadedPolicy;

impl ThreadingPolicy for DLSingleThreadedPolicy {
    type LockObject = DLDummySyncObject;
    const IS_THREAD_SAFE: bool = false;
}

pub struct DLMultiThreadingPolicy;

impl ThreadingPolicy for DLMultiThreadingPolicy {
    type LockObject = DLPlainLightMutex;
    const IS_THREAD_SAFE: bool = true;
}
