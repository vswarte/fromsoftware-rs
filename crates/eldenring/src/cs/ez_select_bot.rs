use std::ffi;

use crate::dltx::DLString;

#[repr(C)]
/// Seems to be used in some control flow around engine settings and debug options.
///
/// Source of name: RTTI
pub struct CSEzSelectBot {
    vftable: *const ffi::c_void,
    pub property: DLString,
}
