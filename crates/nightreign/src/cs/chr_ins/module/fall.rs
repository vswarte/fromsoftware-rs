use std::ptr::NonNull;

use crate::cs::ChrIns;

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrFallModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: i64,
    pub fall_timer: f32,
    hamari_fall_death_checked: bool,
    pub force_max_fall_height: bool,
    pub disable_fall_motion: bool,
}
