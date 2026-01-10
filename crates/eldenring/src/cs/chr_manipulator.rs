use std::ptr::NonNull;

use crate::{cs::ChrIns, dlrf::DLRuntimeClass};
use shared::F32Vector4;
use vtable_rs::VPtr;

#[repr(C)]
pub struct ChrManipulator {
    vftable: VPtr<dyn ChrManipulatorVmt, Self>,
    // TODO: could be pos?
    unk10: F32Vector4,
    // TODO: fact-check
    rotation: F32Vector4,
    unk30: u8,
    unk40: F32Vector4,
    unk50: u8,
    // TODO: check if type is proper?
    look_position: F32Vector4,
    unk70: F32Vector4,
    unk80: F32Vector4,
    // TODO: fact-check
    motion_multiplier: F32Vector4,
    // TODO: fact-check
    network_warp_distance: f32,
    // TODO: fact-check
    weight_type: u32,
    owning_chr: NonNull<ChrIns>,
    unkb0: [u8; 16],
}

#[vtable_rs::vtable]
pub trait ChrManipulatorVmt {
    fn runtime_class(&self) -> &DLRuntimeClass;

    fn destructor(&mut self);

    fn manipulator_type(&self) -> &ManipulatorType;
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
