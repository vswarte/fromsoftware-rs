use std::ptr::NonNull;

use crate::cs::ChrIns;

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrSuperArmorModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    /// Current super armor of the character, related to poise.
    pub sa_durability: f32,
    /// Maximum super armor of the character.
    pub sa_durability_max: f32,
    unk18: u32,
    /// Time to lost super armor reset.
    pub recover_time: f32,
    unk20: u8,
    unk21: u8,
    /// Set by TAE Event 0 ChrActionFlag (action 71 POISE_BREAK_UNRECOVERABLE)
    pub poise_broken_state: bool,
    unk23: u8,
    unk24: u32,
}
