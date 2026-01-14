use std::ptr::NonNull;

use crate::{cs::ChrIns, fd4::FD4Time};

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrGrassHitModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    /// Param ID of the grass this character is currently colliding with.
    /// Can be only set to 0 or 1 by the game.
    pub grass_hit_param_id: u8,
    /// Param ID of the grass this character collided with on the last update.
    pub last_update_grass_hit_param_id: u8,
    /// Timer that counts when grass_hit_param_id should be reset to 0.
    pub state_decay_timer: FD4Time,
    /// Time in seconds after which grass_hit_param_id is reset to 0.
    /// Set to 0.1 by default.
    pub default_decay_time: f32,
    unk2c: [u8; 0x14],
}
