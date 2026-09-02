use shared::OwnedPtr;

mod action_flag;
mod action_request;
mod behavior;
mod behavior_data;
mod data;
mod event;
mod fall;
mod grass_hit;
mod ladder;
mod material;
mod model_param_modifier;
mod physics;
mod ride;
mod super_armor;
mod throw;
mod time_act;
mod toughness;
mod wet;

pub use action_flag::*;
pub use action_request::*;
pub use behavior::*;
pub use behavior_data::*;
pub use data::*;
pub use event::*;
pub use fall::*;
pub use grass_hit::*;
pub use ladder::*;
pub use material::*;
pub use model_param_modifier::*;
pub use physics::*;
pub use ride::*;
pub use super_armor::*;
pub use throw::*;
pub use time_act::*;
pub use toughness::*;
pub use wet::*;

use crate::dlkr::MainHeapAllocator;

#[repr(C)]
pub struct ChrInsModuleContainer {
    pub data: OwnedPtr<CSChrDataModule, MainHeapAllocator>,
    pub action_flag: OwnedPtr<CSChrActionFlagModule, MainHeapAllocator>,
    behavior_script: OwnedPtr<(), MainHeapAllocator>,
    pub time_act: OwnedPtr<CSChrTimeActModule, MainHeapAllocator>,
    resist: OwnedPtr<(), MainHeapAllocator>,
    pub behavior: OwnedPtr<CSChrBehaviorModule, MainHeapAllocator>,
    behavior_sync: OwnedPtr<(), MainHeapAllocator>,
    ai: OwnedPtr<(), MainHeapAllocator>,
    pub super_armor: OwnedPtr<CSChrSuperArmorModule, MainHeapAllocator>,
    pub toughness: OwnedPtr<CSChrToughnessModule, MainHeapAllocator>,
    talk: OwnedPtr<(), MainHeapAllocator>,
    pub event: OwnedPtr<CSChrEventModule, MainHeapAllocator>,
    magic: OwnedPtr<(), MainHeapAllocator>,
    /// Describes the characters physics-related properties.
    pub physics: OwnedPtr<CSChrPhysicsModule, MainHeapAllocator>,
    pub fall: OwnedPtr<CSChrFallModule, MainHeapAllocator>,
    pub ladder: OwnedPtr<CSChrLadderModule, MainHeapAllocator>,
    pub action_request: OwnedPtr<CSChrActionRequestModule, MainHeapAllocator>,
    pub throw: OwnedPtr<CSChrThrowModule, MainHeapAllocator>,
    hitstop: OwnedPtr<(), MainHeapAllocator>,
    damage: OwnedPtr<(), MainHeapAllocator>,
    pub material: OwnedPtr<CSChrMaterialModule, MainHeapAllocator>,
    knockback: OwnedPtr<(), MainHeapAllocator>,
    sfx: OwnedPtr<(), MainHeapAllocator>,
    vfx: OwnedPtr<(), MainHeapAllocator>,
    pub behavior_data: OwnedPtr<CSChrBehaviorDataModule, MainHeapAllocator>,
    unkc8: OwnedPtr<(), MainHeapAllocator>,
    /// Describes a number of render-related inputs, like the color for the phantom effect and
    /// equipment coloring effects.
    pub model_param_modifier: OwnedPtr<CSChrModelParamModifierModule, MainHeapAllocator>,
    dripping: OwnedPtr<(), MainHeapAllocator>,
    unke0: OwnedPtr<(), MainHeapAllocator>,
    pub ride: OwnedPtr<CSChrRideModule, MainHeapAllocator>,
    bonemove: OwnedPtr<(), MainHeapAllocator>,
    /// Describes if your character is wet for rendering as well as applying speffects.
    pub wet: OwnedPtr<CSChrWetModule, MainHeapAllocator>,
    auto_homing: OwnedPtr<(), MainHeapAllocator>,
    above_shadow_test: OwnedPtr<(), MainHeapAllocator>,
    sword_arts: OwnedPtr<(), MainHeapAllocator>,
    pub grass_hit: OwnedPtr<CSChrGrassHitModule, MainHeapAllocator>,
    wheel_rot: OwnedPtr<(), MainHeapAllocator>,
    cliff_wind: OwnedPtr<(), MainHeapAllocator>,
    navimesh_cost_effect: OwnedPtr<(), MainHeapAllocator>,
}
