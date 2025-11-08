use std::{cell::UnsafeCell, ffi};

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
    pub critical_section: UnsafeCell<CRITICAL_SECTION>,
    _unk30: [u8; 0x8],
}

impl Drop for DLPlainLightMutex {
    fn drop(&mut self) {
        unsafe { DeleteCriticalSection(self.critical_section.get()) }
    }
}

impl DLPlainLightMutex {
    pub fn lock(&self) {
        unsafe { EnterCriticalSection(self.critical_section.get()) }
    }

    pub fn unlock(&self) {
        unsafe { LeaveCriticalSection(self.critical_section.get()) }
    }
}
