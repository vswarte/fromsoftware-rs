use fromsoftware_shared::OwnedPtr;

use crate::cs::BlockId;

mod module;
pub use module::*;


#[repr(C)]
pub struct ChrIns {
    unk0: [u8; 0x38],
    // Both of these seemingly switch between some variation map ID and overworld ID?
    pub current_map_id: BlockId,
    pub previous_map_id: BlockId,
    unk40: [u8; 0x20],
    pub chr_ctrl: OwnedPtr<ChrCtrl>,
    unk68: [u8; 0x150],
    pub module_container: OwnedPtr<ChrInsModuleContainer>,
    // TODO: rest
}

#[repr(C)]
pub struct ChrCtrl {
    unk0: [u8; 0xf0],
    pub flags: u8,
    // TODO: rest
}
