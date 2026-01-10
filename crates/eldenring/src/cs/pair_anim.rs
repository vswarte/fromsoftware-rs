use std::ptr::NonNull;

use super::FieldInsHandle;
use crate::{
    Vector,
    cs::{CSThrowNode, ChrIns},
    dlkr::DLPlainLightMutex,
    position::HavokPosition,
    rotation::EulerAngles,
};
use shared::{F32Vector4, Subclass, Superclass};
use vtable_rs::VPtr;

#[repr(C)]
#[shared::singleton("CSPairAnimManager")]
pub struct CSPairAnimManager {
    unk0: Vector<()>,
    unk20: Vector<CSPairAnimManager20Entry>,
    unk40: Vector<()>,
    unk60: f32,
    unk64: f32,
    // Unknown 1-byte structure. Related to the p2p send but entirely unused?
    unk68: isize,
    // Unknown 1-byte structure. Related to the p2p receive but entirely unused?
    // Referenced when receiving packet 18 (PairAnimStateUpdate) as well as packet 48 (ThrowEscHpUpdate).
    unk70: isize,
    pub mutex: DLPlainLightMutex,
    unka8: Vector<()>,
    unkc0: [u8; 0xB8],
}

#[repr(C)]
pub struct CSPairAnimManager20Entry {
    pub party_a: Option<NonNull<ChrIns>>,
    pub party_b: Option<NonNull<ChrIns>>,
    unk10: isize,
    // Might be clear request? Set to true when clearing a `CSPairAnimNode`.
    pub unk18: bool,
}

#[repr(C)]
/// Does the bookkeeping around paired animations where one party is forwarding animations to a
/// receiver. One such example is riding torrent, another would be backstabs.
/// Both parties will have one of these.
#[derive(Superclass)]
#[superclass(children(CSThrowNode))]
pub struct CSPairAnimNode {
    vftable: VPtr<dyn CSPairAnimNodeVmt, Self>,
    unk8: isize,
    pub owner: NonNull<ChrIns>,
    /// Field ins handle of the `FieldIns` on the other end of this pair anim session.
    pub counter_party: FieldInsHandle,
    /// Set when entering a pair anim "session". Updates with the world shift so it can be
    /// safely used to measure distance when crossing chunks. Becomes identity (0.0, 0.0, 0.0, 1.0)
    /// once the paired anim ends.
    pub start_position: HavokPosition,
    /// Populated when entering a pair anim "session". Becomes all zeroes once the paired anim ends.
    pub start_rotation: EulerAngles,
    // occupied? if true it also potentially removes an entry from CSPairAnimManager->0x20
    // during the clearing procedure.
    unk40: bool,
    unk41: bool,
    unk42: bool,
    unk43: bool,
    unk44: bool,
}

#[vtable_rs::vtable]
pub trait CSPairAnimNodeVmt {
    fn destructor(&mut self);
    /// Seems to be some kind of disengage?
    fn unk8(&mut self);
    /// Sets all properties back to the initial state. If `unk40` is set it will do something with
    /// an entry in `CSPairAnimManager->0x20` as well. Called as part of the CSThrowNode destructor
    /// as well as the torrent dismount procedure.
    fn reset(&mut self);
    /// Updates the start position with an offset. Might be the worldshift being applied?
    fn displace_start_position(&mut self, position: F32Vector4);
    // Returns 0 in base class.
    fn unk20(&mut self);
    // Returns 1 in base class.
    fn unk28(&mut self);
    // Returns 1 in base class.
    fn unk30(&mut self);
    // Returns immediately in base class.
    fn unk38(&mut self);
    // Sets everything to empty.
    fn unk40(&mut self);
    // Returns 0 in base class.
    fn unk48(&mut self);
    // Returns immediately in base class.
    fn unk50(&mut self);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x50, size_of::<CSPairAnimNode>());
    }
}
