use std::ptr::NonNull;

use crate::cs::ChrIns;

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrMaterialModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub material_param_id: i16,
    unk12: [i16; 4],
    unk1a: u8,
    /// True when material under the character disables fall damage.
    pub disable_fall_damage: bool,
    unk1c: [u8; 0x4],
}

