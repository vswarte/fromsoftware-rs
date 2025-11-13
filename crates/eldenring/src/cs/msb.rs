use crate::fd4::{FD4FileCap, FD4ResCap, FD4ResRep};

#[repr(C)]
/// Source of name: RTTI
#[shared::singleton("MsbRepository")]
pub struct MsbRepository {
    pub res_rep: FD4ResRep<MsbFileCap>,
}

pub struct MsbFileCap {
    pub file_cap: FD4FileCap<Self>,
}

impl AsRef<FD4ResCap<MsbFileCap>> for MsbFileCap {
    fn as_ref(&self) -> &FD4ResCap<MsbFileCap> {
        &self.file_cap.res_cap
    }
}
