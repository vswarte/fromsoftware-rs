use std::ptr::NonNull;

use crate::cs::ChrIns;
use crate::position::HavokPosition;

#[repr(C)]
pub struct CSChrPhysicsModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: [u8; 0x60],
    pub position: HavokPosition,
    // TODO: the rest
}
