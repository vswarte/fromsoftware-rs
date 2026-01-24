use std::ptr::NonNull;

use crate::{
    cs::{AiIns, ChrCtrl, ChrIns},
    dlrf::DLRuntimeClass,
};
use shared::F32Vector4;
use vtable_rs::VPtr;

#[repr(C)]
pub struct ChrManipulator {
    vftable: VPtr<dyn ChrManipulatorVmt, Self>,
    // TODO: fact-check
    unk10: F32Vector4,
    unk28: F32Vector4,
    unk30: u8,
    unk40: F32Vector4,
    unk50: u8,
    // TODO: check if type is proper?
    unk60: F32Vector4,
    unk70: F32Vector4,
    unk80: F32Vector4,
    // TODO: fact-check
    pub motion_multiplier: F32Vector4,
    // TODO: fact-check
    pub network_warp_distance: f32,
    // TODO: fact-check
    pub weight_type: u32,
    owning_chr: NonNull<ChrIns>,
    unkb0: [u8; 16],
}

#[vtable_rs::vtable]
pub trait ChrManipulatorVmt {
    fn runtime_class(&self) -> &DLRuntimeClass;

    fn destructor(&mut self);

    fn manipulator_type(&self) -> ManipulatorType;

    // TODO: rest
}

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum ManipulatorType {
    Default = 0x0,
    Pad = 0x1,
    Network = 0x2,
    Replay = 0x3,
    NetAi = 0x4,
    Com = 0x5,
    Ride = 0x6,
    Follow = 0x7,
}

/// Manipulator that runs on most enemies to bridge the character and the AI.
///
/// Source of name: RTTI
#[repr(C)]
pub struct ComManipulator {
    pub manipulator: ChrManipulator,
    pub ai_ins: Option<NonNull<AiIns>>,
    pub com_think_owner: CSComThinkOwner,
    unke0: [u8; 0x60],
    unk140: F32Vector4,
    unk150: [u8; 0xc],
    pub npc_param_id: i32,
    pub npc_think_param_id: i32,
}

/// Passed around in the AI system a lot to represent the character end of the context.
///
/// Source of name: RTTI
#[repr(C)]
pub struct CSComThinkOwner {
    vftable: isize,
    pub manipulator: NonNull<ComManipulator>,
    pub chr_ctrl: NonNull<ChrCtrl>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<ChrManipulator>(), 0xc0);
        assert_eq!(std::mem::size_of::<ComManipulator>(), 0x170);
        assert_eq!(std::mem::size_of::<CSComThinkOwner>(), 0x18);
    }
}
