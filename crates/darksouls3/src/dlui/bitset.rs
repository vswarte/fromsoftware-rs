use std::{fmt, slice};

use crate::dlkr::DLAllocatorRef;

use shared::OwnedPtr;

#[repr(C)]
/// Source of name: RTTI
pub struct DynamicBitset {
    _vftable: usize,
    pub size: usize,
    pub data: OwnedPtr<u8>,
    pub allocator: DLAllocatorRef,
}

impl DynamicBitset {
    /// Sets all bits in this set to [value].
    pub fn fill(&mut self, value: bool) {
        let value = if value { 0xFF } else { 0x00 };
        for byte in self.as_mut_slice() {
            *byte = value;
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        // Safety: We trust the game to report lengths accurately.
        unsafe { slice::from_raw_parts(self.data.as_ptr(), self.size) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        // Safety: We trust the game to report lengths accurately.
        unsafe { slice::from_raw_parts_mut(self.data.as_ptr(), self.size) }
    }
}

impl fmt::Debug for DynamicBitset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for byte in self.as_slice() {
            write!(f, "{byte:b}")?;
        }
        Ok(())
    }
}
