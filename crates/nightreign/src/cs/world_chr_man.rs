use shared::OwnedPtr;

#[repr(C)]
#[shared::singleton("WorldChrMan")]
pub struct WorldChrMan {
    unk0: [u8; 0x174e8],
    pub main_player: Option<OwnedPtr<ChrIns>>,
}

#[repr(C)]
pub struct ChrIns {
    unk0: [u8; 0x38],
    pub current_map_id: i32,
    pub previous_map_id: i32,
    unk40: [u8; 0x20],
    pub chr_ctrl: OwnedPtr<ChrCtrl>,
    unk68: [u8; 0x150],
    pub modules: OwnedPtr<ChrModules>,
}

#[repr(C)]
pub struct ChrCtrl {
    unk0: [u8; 0xf0],
    pub flags: u8,
}

#[repr(C)]
pub struct ChrModules {
    pub data: OwnedPtr<ChrDataModule>,
    // TODO: rest
}

#[repr(C)]
pub struct ChrDataModule {
    unk0: [u8; 0x189],
    pub no_dead: bool,
    // TODO: rest
}
