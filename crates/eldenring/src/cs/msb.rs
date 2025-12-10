use crate::fd4::{FD4FileCap, FD4ResCap, FD4ResRep};
use shared::Subclass;

#[repr(C)]
/// Source of name: RTTI
#[shared::singleton("MsbRepository")]
#[derive(Subclass)]
#[subclass(base = FD4ResRep<MsbFileCap>, base = FD4ResCap)]
pub struct MsbRepository {
    pub res_rep: FD4ResRep<MsbFileCap>,
}

#[repr(C)]
#[derive(Subclass)]
#[subclass(base = FD4FileCap, base = FD4ResCap)]
pub struct MsbFileCap {
    pub file_cap: FD4FileCap,
}
