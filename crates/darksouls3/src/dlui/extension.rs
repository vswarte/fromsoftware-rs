use crate::dlui::DynamicBitset;

#[repr(C)]
/// Source of name: RTTI
pub struct DLUserInputSuppressor {
    _vftable: usize,
    pub bitset1: DynamicBitset,
    pub bitset2: DynamicBitset,
}
