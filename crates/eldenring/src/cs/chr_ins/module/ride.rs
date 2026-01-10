use crate::cs::ChrIns;
use std::ptr::NonNull;

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrRideModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    /// `ChrIns` used as a mount of this `ChrIns`.
    pub mount_chr_ins: NonNull<ChrIns>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<CSChrRideModule>(), 0x1e0);
    }
}
