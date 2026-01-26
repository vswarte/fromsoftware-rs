/// Source of name: RTTI
#[repr(C)]
#[shared::singleton("CSWindow")]
pub struct CSWindowImp {
    vftable: usize,
    pub window_handle: isize,
    // TODO: rest
}
