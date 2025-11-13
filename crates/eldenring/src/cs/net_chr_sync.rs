use std::ptr::NonNull;

use shared::OwnedPtr;

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
    unk20_readback_values: usize,
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

    pub fn update_flags_mut(&mut self) -> &mut [ChrSyncUpdateFlags] {
        unsafe { std::slice::from_raw_parts_mut(self.update_flags, self.capacity as usize) }
    }

    pub fn health_updates_mut(&mut self) -> &mut [ChrSyncHealthUpdate] {
        unsafe {
            std::slice::from_raw_parts_mut(self.health_readback_values, self.capacity as usize)
        }
    }
}

/// Holds a set of bits where most bits correspond to a particular type of received sync value.
#[repr(C)]
pub struct ChrSyncUpdateFlags(pub u16);

const CHR_SYNC_HEALTH_UPDATE: u16 = 0b0000000000010000;

impl ChrSyncUpdateFlags {
    /// Checks if the update flags indicate a pending health update.
    pub fn has_health_update(&self) -> bool {
        self.0 & CHR_SYNC_HEALTH_UPDATE != 0
    }
}

#[repr(C)]
/// Incoming health update, describes how much HP the ChrIns has left as well as how much damage it
/// has taken since the last sync.
pub struct ChrSyncHealthUpdate {
    current_hp: u32,
    damage_taken: u32,
}
