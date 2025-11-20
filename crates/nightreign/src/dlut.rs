#[repr(C)]
/// Source of name: dantelion2 leak
/// https://archive.org/details/dantelion2
pub struct DLDateTime {
    /// Set to FILETIME on creation.
    pub time64: u64,
    /// Packed datetime value.
    pub date: u64,
}
