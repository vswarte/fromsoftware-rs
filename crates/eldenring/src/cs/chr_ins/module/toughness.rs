use std::ptr::NonNull;

use crate::cs::ChrIns;

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrToughnessModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    /// Current toughness of the character, related to stance break
    pub toughness: f32,
    toughness_unk: f32,
    /// Maximum toughness of the character
    pub toughness_max: f32,
    /// Time to lost toughness reset.
    pub recover_time: f32,
    unk20: [u8; 0xd],
    /// Set to "true" to update the character's maximum toughness. (Normally happens when gear changes)
    pub trigger_max_toughness_update: bool,
    unk2e: [u8; 0xfa],
}
