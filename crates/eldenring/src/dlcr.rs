use std::slice::from_raw_parts;

use shared::OwnedPtr;

#[repr(C)]
pub struct DLSerialCipherKey {
    vftable: usize,
    key: *mut u8,
    key_length: usize,
}

impl DLSerialCipherKey {
    pub fn key(&self) -> &[u8] {
        unsafe { from_raw_parts(self.key, self.key_length) }
    }
}
#[repr(C)]
pub struct AESEncrypter {
    vftable: usize,
    algorithm: OwnedPtr<DLRijndaelAlgorithm>,
    /// Set to 0x60, might be size of the IV or nonce?
    unk10: u64,
}

#[repr(C)]
pub struct AESDecrypter {
    vftable: usize,
    algorithm: OwnedPtr<DLRijndaelAlgorithm>,
}

#[repr(C)]
pub struct DLRijndaelAlgorithm {
    vftable: usize,
}
