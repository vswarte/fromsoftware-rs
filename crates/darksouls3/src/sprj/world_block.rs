use std::{ptr::NonNull, slice};

use shared::{OwnedPtr, empty::*};

use crate::sprj::ChrSet;

use super::{ChrIns, FieldInsSelector, WorldBlockInfo};

#[repr(C)]
/// Source of name: RTTI
pub struct WorldBlockChr {
    _vftable: usize,
    pub world_block_info: NonNull<WorldBlockInfo>,
    _unk10: [u64; 0xd],
    _unk78: u32,

    /// The set of character entities associated with this block.
    pub chr_set: ChrSet<ChrIns>,

    _unk98: u32,
    _unka0: u64,

    /// The length of [mappings](#structfield.mappings).
    ///
    /// Use [Self::mappings] to access this safely.
    pub mappings_length: i32,

    /// Mappings from entity IDs to [FieldInsSelector]s.
    ///
    /// Use [Self::mappings] to access this safely.
    pub mappings: OwnedPtr<WorldBlockMapping>,

    _unkb8: u32,
    _unkc0: [u64; 5],
    _unke8: [u8; 0x48],
    _unk134: u32,
}

unsafe impl IsEmpty for WorldBlockChr {
    fn is_empty(value: &MaybeEmpty<WorldBlockChr>) -> bool {
        *unsafe {
            value
                .as_non_null()
                .cast::<usize>()
                // Offset for mappings. We can't check the vtable because it can
                // be set even for empty values.
                .offset(0x16)
                .as_ref()
        } == 0
    }
}

impl WorldBlockChr {
    /// Returns a slice over all the mappings in this block.
    pub fn mappings(&self) -> &[WorldBlockMapping] {
        unsafe { slice::from_raw_parts(self.mappings.as_ptr(), self.mappings_length as usize) }
    }

    /// Returns a mutable slice over all the mappings in this block.
    pub fn mappings_mut(&mut self) -> &mut [WorldBlockMapping] {
        unsafe { slice::from_raw_parts_mut(self.mappings.as_ptr(), self.mappings_length as usize) }
    }
}

/// A mapping from an entity ID to a [FieldInsSelector].
#[repr(C)]
pub struct WorldBlockMapping {
    /// The entity this mapping refers to.
    pub entity_id: i32,

    /// The selector corresponding to this entity ID.
    pub selector: FieldInsSelector,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x138, size_of::<WorldBlockChr>());
    }
}
