use std::ptr::NonNull;

use shared::UnknownStruct;

use super::ParamResCap;
use crate::{CxxVec, fd4::FD4BasicHashString};

#[repr(C)]
#[shared::singleton("SoloParamRepository")]
/// A repository that holds references to some but not all parameter structs.
///
/// Source of name: RTTI
pub struct SoloParamRepository {
    _vftable: usize,
    name: FD4BasicHashString,
    _unk48: u64,
    _unk50: u64,
    _unk58: u64,
    _unk60: u32,
    _unk64: u32,

    pub params: [SoloParamCell; 0x61],
    _wep_reinforce_tree: UnknownStruct<0x18>,
    _unk1bc8: CxxVec<u8>,
    _unk1be8: u64,
}

impl SoloParamRepository {
    /// An iterator over all aprameters in this repository.
    pub fn iter(&self) -> impl Iterator<Item = &ParamResCap> {
        self.params.iter().filter_map(|c| c.param())
    }
}

#[repr(C)]
/// A cell in a [SoloParamRepository] containing pointers to up to 8 parameters.
pub struct SoloParamCell {
    /// The number of parameters this cell contains. In practice, this is always
    /// 0 or 1.
    pub len: usize,

    /// The parameters contained by this cell.
    pub params: [Option<NonNull<ParamResCap>>; 0x8],
}

impl SoloParamCell {
    /// Returns the parameter associated with this cell, if it exists.
    pub fn param(&self) -> Option<&ParamResCap> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { self.params[0].unwrap().as_ref() })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x48, size_of::<SoloParamCell>());
        assert_eq!(0x1bf0, size_of::<SoloParamRepository>());
    }
}
