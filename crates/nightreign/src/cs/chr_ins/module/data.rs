use std::ptr::NonNull;

use crate::cs::ChrIns;

#[repr(C)]
pub struct CSChrDataModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: [u8; 0x130],
    pub hp: u32,
    pub hp_max: u32,
    unk148: [u8; 0x40],
    unk188: bool,  // 0x189
    pub no_dead: bool,  // 0x189
    // TODO: rest
}
