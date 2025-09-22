use std::ffi;

use vtable_rs::VPtr;
use windows::Win32::System::Threading::{
    DeleteCriticalSection, EnterCriticalSection, InitializeCriticalSection, LeaveCriticalSection,
    CRITICAL_SECTION,
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
    extern "C" fn destructor(&mut self, param_2: bool) {
        unimplemented!();
    }
}
