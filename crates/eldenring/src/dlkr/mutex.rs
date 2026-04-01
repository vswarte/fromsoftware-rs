use vtable_rs::VPtr;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Threading::{
    CRITICAL_SECTION, DeleteCriticalSection, EnterCriticalSection, InitializeCriticalSection,
    LeaveCriticalSection,
};

#[vtable_rs::vtable]
pub trait DLPlainLightMutexVmt {
    fn destructor(&mut self, param_2: bool);
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

impl DLPlainLightMutexVmt for DLPlainLightMutex {
    extern "C" fn destructor(&mut self, _param_2: bool) {
        unimplemented!();
    }
}

#[repr(C)]
pub struct PlainAdaptiveMutexImpl {
    vftable: usize,
    pub critical_section: CRITICAL_SECTION,
    pub spin_count: u32,
}

#[repr(C)]
pub struct DLPlainReadWriteLock {
    vftable: usize,
    pub h_event: HANDLE,
    pub h_writer_mutex: HANDLE,
    pub h_reader_mutex: HANDLE,
    pub reader_count: i32,
}
