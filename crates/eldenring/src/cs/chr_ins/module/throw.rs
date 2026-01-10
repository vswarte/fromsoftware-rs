use std::ptr::NonNull;
use bitfield::bitfield;
use shared::{OwnedPtr, Subclass};

use crate::cs::{CSPairAnimNode, ChrIns, P2PEntityHandle};

#[repr(C)]
#[derive(Subclass)]
#[subclass(base = CSPairAnimNode)]
/// Source of name: RTTI
pub struct CSThrowNode {
    pub pair_anim_node: CSPairAnimNode,
    unk58: [u8; 0x18],
    pub throw_state: ThrowNodeState,
    unk6c: u32,
    unk70: f32,
    unk74: f32,
    unk78: f32,
    unk7c: [u8; 0x34],
    /// available only for main player
    throw_self_esc: usize,
    unkb8: [u8; 0xb8],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrThrowModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub throw_node: OwnedPtr<CSThrowNode>,
    pub flags: ThrowModuleFlags,
    unk1c: u32,
    unk20: u32,
    // p2p handle of the target?, need verification
    p2p_entity_handle: P2PEntityHandle,
    // field ins handle of the target?, need verification
    throw_target: usize,
    unk28: [u8; 0x8],
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ThrowNodeState {
    Unk1 = 1,
    Unk2 = 2,
    InThrowAttacker = 3,
    InThrowTarget = 4,
    DeathAttacker = 5,
    DeathTarget = 6,
    Unk7 = 7,
    Unk8 = 8,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ThrowModuleFlags(u32);
    impl Debug;
    /// Set by TAE Event 0 ChrActionFlag (action 70 THROW_ESCAPE_TRANSITION_ATTACKER)
    pub escape_transition, set_escape_transition: 0;
    /// Set by TAE Event 0 ChrActionFlag (action 69 THROW_DEATH_TRANSITION_DEFENDER)
    pub death_transition, set_death_transition:   1;
}
