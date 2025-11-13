#[repr(C)]
#[fromsoftware_shared::singleton("CSDlc")]
pub struct CSDlc {
    vftable: usize,
    unk8: [u8; 0x09],
    pub dlc1_installed: bool,
    pub dlc2_installed: bool,
}
