use std::ptr::NonNull;

use bitfield::bitfield;
use shared::{F32Vector3, OwnedPtr};

use crate::cs::FieldInsHandle;

use super::{ChrIns, ChrSet};

#[repr(C)]
pub struct NetChrSync {
    world_info_owner: usize,
    pub chr_slot_count: u32,
    _padc: u32,
    pub net_chr_set_sync: [Option<OwnedPtr<NetChrSetSync>>; 196],
}

/// Acts as an update buffer for all the ChrIns sync for a given ChrSet.
/// P2P update tasks will populate the arrays with received values and toggle the readback flag
/// corresponding to the type of sync that was received.
///
/// Source of name: RTTI
#[repr(C)]
pub struct NetChrSetSync {
    vftable: usize,
    /// ChrSet this NetChrSetSync manages the sync for.
    pub chr_set: NonNull<ChrSet<ChrIns>>,
    /// Max amount of ChrIns's this NetChrSetSync will host.
    pub capacity: u32,
    _pad14: u32,

    unk18_readback_values: usize,
    placement_readback_values: *mut ChrSyncPlacementUpdate,
    unk28_readback_values: usize,
    /// Holds incoming health updates.
    health_readback_values: *mut ChrSyncHealthUpdate,
    /// Describes what kinds of updated values are available for a given ChrIns.
    update_flags: *mut ChrSyncUpdateFlags,
    unk40_readback_values: usize,
    unk48_readback_values: usize,
}

impl NetChrSetSync {
    pub fn update_flags(&self) -> &[ChrSyncUpdateFlags] {
        unsafe { std::slice::from_raw_parts(self.update_flags, self.capacity as usize) }
    }

    pub fn health_updates(&self) -> &[ChrSyncHealthUpdate] {
        unsafe { std::slice::from_raw_parts(self.health_readback_values, self.capacity as usize) }
    }

    pub fn placement_updates(&self) -> &[ChrSyncPlacementUpdate] {
        unsafe {
            std::slice::from_raw_parts(self.placement_readback_values, self.capacity as usize)
        }
    }

    pub fn update_flags_mut(&mut self) -> &mut [ChrSyncUpdateFlags] {
        unsafe { std::slice::from_raw_parts_mut(self.update_flags, self.capacity as usize) }
    }

    pub fn health_updates_mut(&mut self) -> &mut [ChrSyncHealthUpdate] {
        unsafe {
            std::slice::from_raw_parts_mut(self.health_readback_values, self.capacity as usize)
        }
    }

    pub fn placement_updates_mut(&mut self) -> &mut [ChrSyncPlacementUpdate] {
        unsafe {
            std::slice::from_raw_parts_mut(self.placement_readback_values, self.capacity as usize)
        }
    }
}

bitfield! {
    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    /// Holds a set of bits where each bit corresponds to a particular type of received sync value.
    pub struct ChrSyncUpdateFlags(u16);
    impl Debug;

    bool;
    /// Whether there is a pending placement update.
    pub has_placement_update, _: 0;
    _, set_has_placement_update: 0;

    /// Whether there is a pending health update.
    pub has_health_update, _: 4;
    _, set_has_health_update: 4;
}

#[repr(C)]
/// Incoming health update, describes how much HP the ChrIns has left as well as how much damage it
/// has taken since the last sync.
pub struct ChrSyncHealthUpdate {
    pub current_hp: i32,
    pub damage_taken: u32,
}

#[repr(C)]
pub struct ChrSyncPlacementUpdate {
    /// Position in MSB space.
    pub position: F32Vector3,
    /// Orientation in radians.
    pub rotation: F32Vector3,
    // these field used for dynamic geometry sync
    unk18: F32Vector3,
    unk24: u32,
    unk28: FieldInsHandle,
    unk30: u16,
    unk34: u32,
}
