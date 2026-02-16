use std::ptr::NonNull;
use crate::cs::{CSChrModelParamModifierModuleEntry, ChrIns};
use crate::param::SWORD_ARTS_PARAM_ST;
use crate::Vector;

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrSwordArtsModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub modifiers: Vector<CSChrModelParamModifierModuleEntry>,
}

#[repr(C)]
pub struct SwordArtsParamLookupResult {
    pub param_id: i32,
    _pad: [u8; 4],
    pub param_row: Option<NonNull<SWORD_ARTS_PARAM_ST>>,
}
