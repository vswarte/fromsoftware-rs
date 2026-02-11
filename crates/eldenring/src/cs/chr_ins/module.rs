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

#[repr(C)]
pub struct ChrInsModuleContainer {
    pub data: OwnedPtr<CSChrDataModule>,
    pub action_flag: OwnedPtr<CSChrActionFlagModule>,
    behavior_script: usize,
    pub time_act: OwnedPtr<CSChrTimeActModule>,
    resist: usize,
    pub behavior: OwnedPtr<CSChrBehaviorModule>,
    behavior_sync: usize,
    ai: usize,
    pub super_armor: OwnedPtr<CSChrSuperArmorModule>,
    pub toughness: OwnedPtr<CSChrToughnessModule>,
    talk: usize,
    pub event: OwnedPtr<CSChrEventModule>,
    magic: usize,
    /// Describes the characters physics-related properties.
    pub physics: OwnedPtr<CSChrPhysicsModule>,
    pub fall: OwnedPtr<CSChrFallModule>,
    pub ladder: OwnedPtr<CSChrLadderModule>,
    pub action_request: OwnedPtr<CSChrActionRequestModule>,
    pub throw: OwnedPtr<CSChrThrowModule>,
    hitstop: usize,
    damage: usize,
    pub material: OwnedPtr<CSChrMaterialModule>,
    knockback: usize,
    sfx: usize,
    vfx: usize,
    pub behavior_data: OwnedPtr<CSChrBehaviorDataModule>,
    unkc8: usize,
    /// Describes a number of render-related inputs, like the color for the phantom effect and
    /// equipment coloring effects.
    pub model_param_modifier: OwnedPtr<CSChrModelParamModifierModule>,
    dripping: usize,
    unke0: usize,
    pub ride: OwnedPtr<CSChrRideModule>,
    bonemove: usize,
    /// Describes if your character is wet for rendering as well as applying speffects.
    pub wet: OwnedPtr<CSChrWetModule>,
    auto_homing: usize,
    above_shadow_test: usize,
    sword_arts: usize,
    pub grass_hit: OwnedPtr<CSChrGrassHitModule>,
    wheel_rot: usize,
    cliff_wind: usize,
    navimesh_cost_effect: usize,
}
