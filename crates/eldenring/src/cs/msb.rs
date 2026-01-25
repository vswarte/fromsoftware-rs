use crate::fd4::{FD4FileCap, FD4ResCap, FD4ResCapHolder, FD4ResRep};
use shared::Subclass;

#[repr(C)]
/// Source of name: RTTI
#[shared::singleton("MsbRepository")]
#[derive(Subclass)]
#[subclass(base = FD4ResRep, base = FD4ResCap)]
pub struct MsbRepository {
    pub res_rep: FD4ResRep,
    pub res_cap_holder: FD4ResCapHolder<MsbFileCap>,
}

#[repr(C)]
#[derive(Subclass)]
#[subclass(base = FD4FileCap, base = FD4ResCap)]
pub struct MsbFileCap {
    pub file_cap: FD4FileCap,
}
