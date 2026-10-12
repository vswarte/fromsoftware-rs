use shared::OwnedPtr;

use crate::cs::ChrIns;

#[repr(C)]
#[shared::singleton("WorldChrMan")]
pub struct WorldChrMan {
    unk0: [u8; 0x174e8],
    pub main_player: Option<OwnedPtr<ChrIns>>,
    // TODO: rest
}
