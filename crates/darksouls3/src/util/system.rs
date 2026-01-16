//! Util to await game startup.
// OG comes from Dasaav
// https://github.com/Dasaav-dsv/libER/blob/main/source/dantelion2/system.cpp
use std::sync::atomic::{AtomicPtr, Ordering};
use std::time::{Duration, Instant};

use pelite::pe64::Pe;
use shared::Program;
use thiserror::Error;

use crate::rva;

static GLOBAL_HINSTANCE: AtomicPtr<usize> = AtomicPtr::new(0x0 as _);

#[derive(Error, Debug)]
pub enum SystemInitError {
    #[error("System initialization timed out")]
    Timeout,
    #[error("Could not translate RVA to VA")]
    InvalidRva,
}

/// Wait for the system to finish initializing by waiting for a global hInstance
/// to be populated for CSWindow. This happens after the CRT init and after
/// duplicate instance checks.
pub fn wait_for_system_init(module: &Program, timeout: Duration) -> Result<(), SystemInitError> {
    if std::ptr::eq(GLOBAL_HINSTANCE.load(Ordering::Relaxed), 0x0 as _) {
        let va = module
            .rva_to_va(rva::get().global_hinstance)
            .map_err(|_| SystemInitError::InvalidRva)?;

        GLOBAL_HINSTANCE.store(va as _, Ordering::Relaxed);
    }

    let start = Instant::now();
    while unsafe { *GLOBAL_HINSTANCE.load(Ordering::Relaxed) } == 0 {
        if start.elapsed() > timeout {
            return Err(SystemInitError::Timeout);
        }
        std::thread::yield_now();
    }

    Ok(())
}
