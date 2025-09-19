use crate::dlkr::DLPlainLightMutex;

#[repr(C)]
#[dlrf::singleton("CSActionButtonMan")]
pub struct CSActionButtonManImp {
    vftable: usize,
    unk8: [u8; 0x88],
    pub mutex: DLPlainLightMutex,
 }
