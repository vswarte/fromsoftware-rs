use shared::MutexBearer;

use crate::dlkr::DLPlainLightMutex;

#[repr(C)]
#[shared::singleton("CSActionButtonMan")]
pub struct CSActionButtonManImp {
    vftable: usize,
    unk8: [u8; 0x88],
    pub mutex: DLPlainLightMutex,
}

impl MutexBearer for CSActionButtonManImp {
    fn lock(&self) {
        self.mutex.lock();
    }

    fn unlock(&self) {
        self.mutex.lock();
    }
}
