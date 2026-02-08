use std::ptr::NonNull;

use shared::OwnedPtr;

use crate::cs::{BlockId, ChrIns};

#[repr(C)]
#[shared::singleton("WorldChrMan")]
pub struct WorldChrMan {
    unk0: [u8; 0x174e8],
    pub main_player: Option<OwnedPtr<ChrIns>>,
    // TODO: rest
}

// #[repr(C)]
// /// Source of name: RTTI
// pub struct WorldAreaChr<T: 'static> {
//     pub base: WorldAreaChrBase,
//     pub world_area_info: usize,
//     unk18: u32,
//     unk1c: u32,
//     pub world_block_chr: NonNull<WorldBlockChr<T>>,
// }
//
// #[repr(C)]
// /// Source of name: RTTI
// pub struct WorldAreaChrBase {
//     vftable: usize,
//     pub world_area_info: usize,
// }
//
// #[repr(C)]
// /// Source of name: RTTI
// pub struct WorldBlockChr<T: 'static> {
//     vftable: usize,
//     pub world_block_info1: usize,
//     unk10: [u8; 0x68],
//     pub chr_set: ChrSet<T>,
//     unkd0: [u8; 0x40],
//     pub world_block_info2: usize,
//     pub chr_set_ptr: NonNull<ChrSet<T>>,
//     allocator: usize,
//     unk128: [u8; 0x30],
//     pub block_id: BlockId,
//     unk15c: u32,
// }
