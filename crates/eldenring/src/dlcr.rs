use std::ptr::NonNull;

use shared::{FromStatic, OwnedPtr, Program};

use vtable_rs::VPtr;

use crate::{Vector, dlkr::DLAllocatorBase};

#[vtable_rs::vtable]
pub trait DLCipherKeyVmt {
    fn destructor(&mut self, flags: u32);
    fn get_key(&self) -> *const u8;
    fn get_key_length(&self) -> usize;
}

#[repr(C)]
pub struct DLCipherKey {
    vftable: VPtr<dyn DLCipherKeyVmt, Self>,
}

impl DLCipherKey {
    /// Get the key as a byte slice
    /// NOTE: The returned slice may contain trailing null bytes, eg for sd/ keys
    pub fn key(&self) -> &[u8] {
        unsafe {
            let ptr = (self.vftable.get_key)(self);
            let len = (self.vftable.get_key_length)(self);
            std::slice::from_raw_parts(ptr, len)
        }
    }

    /// Get the key as a string (for PEM keys)
    pub fn key_as_str(&self) -> Option<&str> {
        // Strip trailing null if present (present in sd/ keys)
        let bytes = self.key();
        std::str::from_utf8(bytes.strip_suffix(&[0]).unwrap_or(bytes)).ok()
    }
}

#[repr(C)]
pub struct DLSerialCipherKey {
    pub base: DLCipherKey,
    key: OwnedPtr<u8>,
    key_length: usize,
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
    base: DLDecrypter,
    algorithm: OwnedPtr<DLRijndaelAlgorithm>,
}

#[repr(C)]
pub struct DLRijndaelAlgorithm {
    vftable: usize,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CryptoKeyType {
    Null = 0,
    Cerial = 1 << 8,
}

/// CKP - Crypto Key Parameters
/// Used to select a key generator SPI
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CryptoKeyParams {
    /// Magic bytes "CKP\0"
    pub magic: [u8; 4],
    /// Version (typically 0x10000000)
    pub version: u32,
    /// Key type (big-endian)
    pub key_type: CryptoKeyType,
}

impl CryptoKeyParams {
    pub const MAGIC: &'static [u8; 4] = b"CKP\0";

    /// Validate the magic bytes
    pub fn is_valid(&self) -> bool {
        &self.magic == Self::MAGIC
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CipherMode {
    Null = 0,
    Cbc = 1 << 8,
    Ecb = 2 << 8,
    Cfb = 3 << 8,
    Ofb = 4 << 8,
    CtrNw = 5 << 8,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CipherType {
    /// Pass-through (no encryption)
    Null = 0,
    /// FromSoftware's own AES block cipher implementation
    Aes = 1 << 8,
    /// OpenSSL AES block cipher
    OpenSslAES = 2 << 8,
    /// OpenSSL RSA cipher
    OpenSslRsa = 3 << 8,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CipherPaddingMode {
    None = 0,
    Pkcs5 = 1 << 8,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyUsageType {
    Null = 0,
    Default = 1 << 8,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyType {
    Common = 0,
    Public = 1 << 8,
    Private = 2 << 8,
}

/// CIP - Cipher Init Parameters
/// Used to select a cipher SPI and create encrypter/decrypter
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CipherInitParams {
    /// Magic bytes "CIP\0"
    pub magic: [u8; 4],
    /// Version (typically 0x20000000)
    pub version: u32,
    /// Mode (typically 0x10100)
    pub mode: u32,
    /// Cipher type (big-endian)
    pub cipher_type: CipherType,
    /// Block cipher mode (big-endian)
    pub cipher_mode: CipherMode,
    /// Padding mode (big-endian)
    pub padding_mode: CipherPaddingMode,
    /// Key usage type (big-endian)
    pub key_usage: KeyUsageType,
    /// Key type (big-endian)
    pub key_type: KeyType,
    /// Reserved
    pub reserved: u16,
    /// Reserved
    pub reserved2: u64,
}

impl CipherInitParams {
    pub const MAGIC: &'static [u8; 4] = b"CIP\0";

    /// Validate the magic bytes
    pub fn is_valid(&self) -> bool {
        &self.magic == Self::MAGIC
    }
}

#[vtable_rs::vtable]
pub trait DLDecrypterVmt {
    fn destructor(&mut self, flags: u32);
    /// Decrypts data from input to output buffers
    /// Returns 0 on success, decryptor-specific error code otherwise
    fn decrypt(
        &self,
        input: *mut u8,
        input_size: usize,
        output: *mut u8,
        output_size: usize,
    ) -> i32;
}

#[repr(C)]
pub struct DLDecrypter {
    vftable: VPtr<dyn DLDecrypterVmt, Self>,
}

#[repr(C)]
pub struct OpenSslAesCipher {
    pub cipher_key: NonNull<DLCipherKey>,
    /// Block cipher mode: CBC=1, ECB=2, CFB=3, OFB=4, CTR=5
    /// See [CipherMode] enum
    pub cipher_mode: u32,
    /// Padding mode: None=0, PKCS5=1
    /// See [CipherPaddingMode] enum
    pub padding_mode: u32,
    /// Key usage type: Default=1
    /// See [KeyUsageType] enum
    pub key_usage: u32,
    pub allocator: NonNull<DLAllocatorBase>,
    evp_cipher: usize,
    evp_cipher_ctx: usize,
    pub requires_iv: bool,
    pub padding_enabled: bool,
}

#[repr(C)]
pub struct OpenSslAesDecrypter {
    pub base: DLDecrypter,
    pub cipher_data: OwnedPtr<OpenSslAesCipher>,
}

#[repr(C)]
pub struct OpenSslRsaCipher {
    pub cipher_key: NonNull<DLCipherKey>,
    /// Padding mode: None=0, PKCS5=1
    /// See [CipherPaddingMode] enum
    pub padding_mode: u32,
    /// Block size in bytes
    pub block_size: u32,
    pub allocator: NonNull<DLAllocatorBase>,
    /// Whether the key is public or private
    pub use_public_key: bool,
    bio: usize,
    evp_pkey_ctx: usize,
    /// RSA key size in bytes
    pub rsa_size: u32,
    /// Input buffer for block operations
    pub block_input_buffer: OwnedPtr<u8>,
    /// Output buffer for block operations
    pub block_output_buffer: OwnedPtr<u8>,
}

#[repr(C)]
pub struct OpenSslRsaDecrypter {
    pub base: DLDecrypter,
    pub cipher_data: OwnedPtr<OpenSslRsaCipher>,
}

#[vtable_rs::vtable]
pub trait DLCipherSPIVmt {
    fn destructor(&mut self, flags: u32);
    fn get_decrypter(
        &self,
        params: &CipherInitParams,
        key: &DLCipherKey,
        allocator: &DLAllocatorBase,
    ) -> Option<NonNull<DLDecrypter>>;
    fn get_encrypter(
        &self,
        params: &CipherInitParams,
        key: &DLCipherKey,
        allocator: &DLAllocatorBase,
    ) -> usize;
}

#[repr(C)]
pub struct DLCipherSPI {
    vftable: VPtr<dyn DLCipherSPIVmt, Self>,
}

#[vtable_rs::vtable]
pub trait DLKeyGeneratorSPIVmt {
    fn destructor(&mut self, flags: u32);
    fn get_cipher_key(
        &self,
        params: &CryptoKeyParams,
        key: &str,
        key_len: u32,
        allocator: &DLAllocatorBase,
    ) -> Option<NonNull<DLCipherKey>>;
}

#[repr(C)]
pub struct DLKeyGeneratorSPI {
    vftable: VPtr<dyn DLKeyGeneratorSPIVmt, Self>,
}

#[repr(C)]
pub struct CryptoSPIRegistry {
    pub key_generators: Vector<NonNull<DLKeyGeneratorSPI>>,
    pub cipher_spis: Vector<NonNull<DLCipherSPI>>,
}

impl CryptoSPIRegistry {
    pub fn get_decrypter(
        &self,
        params: &CipherInitParams,
        key: &DLCipherKey,
        allocator: &DLAllocatorBase,
    ) -> Option<NonNull<DLDecrypter>> {
        if !params.is_valid() {
            return None;
        }
        for spi_ptr in self.cipher_spis.items().iter() {
            let cipher_spi = unsafe { spi_ptr.as_ref() };
            if let Some(decrypter) =
                (cipher_spi.vftable.get_decrypter)(cipher_spi, params, key, allocator)
            {
                return Some(decrypter);
            }
        }
        None
    }

    pub fn get_cipher_key(
        &self,
        params: &CryptoKeyParams,
        rsa_key: &str,
        allocator: &DLAllocatorBase,
    ) -> Option<NonNull<DLCipherKey>> {
        if !params.is_valid() {
            return None;
        }

        let rsa_key_len: u32 = rsa_key.len() as u32;

        for spi_ptr in self.key_generators.items().iter() {
            let keygen_spi = unsafe { spi_ptr.as_ref() };
            if let Some(cipher_key) = (keygen_spi.vftable.get_cipher_key)(
                keygen_spi,
                params,
                rsa_key,
                rsa_key_len,
                allocator,
            ) {
                return Some(cipher_key);
            }
        }

        None
    }
}

impl FromStatic for CryptoSPIRegistry {
    unsafe fn instance() -> fromsoftware_shared::InstanceResult<&'static mut Self> {
        use crate::rva;
        use pelite::pe64::Pe;

        let target = Program::current()
            .rva_to_va(rva::get().crypto_spi_registry)
            .map_err(|_| fromsoftware_shared::InstanceError::NotFound)?
            as *mut Option<NonNull<CryptoSPIRegistry>>;

        unsafe {
            target
                .as_mut()
                .and_then(|opt| opt.as_mut())
                .map(|nn| nn.as_mut())
                .ok_or(fromsoftware_shared::InstanceError::Null)
        }
    }
}
