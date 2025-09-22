/// Util to await game startup.

// OG comes from Dasaav
// https://github.com/Dasaav-dsv/libER/blob/main/source/dantelion2/system.cpp
use std::sync::atomic::{AtomicPtr, Ordering};
use std::time::{Duration, Instant};

use pelite::pattern;
use pelite::pattern::Atom;
use pelite::pe64::{Pe, Rva};
use thiserror::Error;

use shared::program::Program;

// WinMain -> SethInstance
// used to set global hInstance later used by CSWindow
// and can be used to determine if the game has finished initializing
const GLOBAL_SET_HINSTANCE_PATTERN: &[Atom] = pattern!(
    "
    48 8b ce
    48 8b f8
    e8 $ {
        48 89 0d $ { ' }
        c3
    }
    "
);

static GLOBAL_HINSTANCE: AtomicPtr<usize> = AtomicPtr::new(0x0 as _);

#[derive(Error, Debug)]
pub enum SystemInitError {
    #[error("System initialization timed out")]
    Timeout,
    #[error("Could not translate RVA to VA")]
    InvalidRva,
}

/// Wait for the system to finish initializing by waiting a global hInstance to be populated for CSWindow.
/// This happens after the CRT init and after duplicate instance checks.
pub fn wait_for_system_init(module: &Program, timeout: Duration) -> Result<(), SystemInitError> {
    if std::ptr::eq(GLOBAL_HINSTANCE.load(Ordering::Relaxed), 0x0 as _) {
        let mut captures = [Rva::default(); 2];
        module
            .scanner()
            .finds_code(GLOBAL_SET_HINSTANCE_PATTERN, &mut captures);

        let global_hinstance = module
            .rva_to_va(captures[1])
            .map_err(|_| SystemInitError::InvalidRva)?;

        GLOBAL_HINSTANCE.store(global_hinstance as _, Ordering::Relaxed);
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
