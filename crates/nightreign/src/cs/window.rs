#[repr(C)]
/// Source of name: RTTI
#[shared::singleton("CSWindow")]
pub struct CSWindowImp {
    vftable: usize,
    pub window_handle: isize,
    // TODO: rest
}
