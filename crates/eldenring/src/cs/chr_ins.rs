use bitfield::bitfield;
use std::fmt::Display;
use std::mem::transmute;
use std::ptr::NonNull;

use pelite::pe64::Pe;
use vtable_rs::VPtr;

use crate::cs::field_ins::{FieldInsBaseVmt, FieldInsHandle};
use crate::cs::network_session::PlayerNetworkSession;
use crate::cs::player_game_data::{ChrAsm, PlayerGameData};
use crate::cs::session_manager::SessionManagerPlayerEntryBase;
use crate::cs::sp_effect::{NpcSpEffectEquipCtrl, SpecialEffect};
use crate::cs::task::{CSEzRabbitNoUpdateTask, CSEzVoidTask};
use crate::cs::world_chr_man::ChrSetEntry;
use crate::cs::{BlockId, CSPlayerMenuCtrl, EquipmentDurabilityStatus, OptionalItemId};
use crate::dltx::DLString;
use crate::fd4::FD4Time;
use crate::param::{ATK_PARAM_ST, NPC_PARAM_ST};
use crate::position::{BlockPosition, HavokPosition};
use crate::rva;
use shared::program::Program;
use shared::{
    Aabb, F32Matrix4x4, F32ModelMatrix, F32Vector3, F32Vector4, OwnedPtr, Subclass, Superclass,
    for_all_subclasses,
};

mod manipulator;
mod module;

pub use manipulator::*;
pub use module::*;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Used for communicating about characters in the networking layer. This handle is essentially the
/// same as FieldInsHandle but has its BlockId and selector swapped. In packets this might be packed
/// into block_id (4 bytes) + chr_selector (3 bytes). According to Sekiro's debug asserts the packed
/// version is referred to as the "whoid".
pub struct P2PEntityHandle {
    pub block_id: BlockId,
    pub chr_selector: P2PEntitySelector,
}

impl P2PEntityHandle {
    pub fn is_empty(&self) -> bool {
        self.chr_selector.0 == u32::MAX
    }
}

impl Display for P2PEntityHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            write!(f, "P2PEntity(None)")
        } else {
            write!(
                f,
                "P2PEntity({}, {}, {})",
                self.block_id,
                self.chr_selector.container(),
                self.chr_selector.index()
            )
        }
    }
}

bitfield! {
    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    /// Represents a packed ChrSet selector for P2P entity handles.
    pub struct P2PEntitySelector(u32);
    impl Debug;

    /// The index within the container.
    pub index, _: 10, 0;
    _, set_index: 10, 0;

    /// The container for this P2PEntity, used to determine which ChrSet to use.
    pub container, _: 18, 11;
    _, set_container: 18, 11;
}

impl P2PEntitySelector {
    /// Create a new P2PEntitySelector from container and index.
    pub fn from_parts(container: u32, index: u32) -> Self {
        let mut selector = P2PEntitySelector(0);
        selector.set_container(container);
        selector.set_index(index);
        selector
    }
}

#[repr(C)]
pub struct AtkParamLookupResult {
    behavior_param_id: i32,
    unk_param_def_meta: u32,
    is_player_atk_param: bool,
    _pad9: [u8; 7],
    param_row: Option<NonNull<ATK_PARAM_ST>>,
}

#[vtable_rs::vtable]
pub trait ChrInsVmt: FieldInsBaseVmt {
    /// Initializes a batch of combat-related modules for a ChrIns as well as initialize the
    /// initiale SpEffect state and a bunch of other stuff.
    fn initialize_character(&mut self);

    fn initialize_model_resources(&mut self);

    fn initialize_character_rendering(&mut self);
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OmissionMode {
    NoUpdate = -2,
    Normal = 0,
    OneFps = 1,
    FiveFps = 5,
    TwentyFps = 20,
    ThirtyFps = 30,
}

#[repr(C)]
#[derive(Superclass)]
#[superclass(children(PlayerIns, EnemyIns))]
/// Abstract base class to all characters. NPCs, Enemies, Players, Summons, Ghosts, even gesturing
/// character on bloodmessages inherit from this.
///
/// Source of name: RTTI
pub struct ChrIns {
    pub vftable: VPtr<dyn ChrInsVmt, Self>,
    pub field_ins_handle: FieldInsHandle,
    pub chr_set_entry: NonNull<ChrSetEntry<Self>>,
    unk18: usize,
    pub backread_state: u32,
    unk24: u32,
    chr_res: usize,
    pub block_id: BlockId,
    pub block_id_override: BlockId,
    /// Override Block ID for [ChrIns::block_origin] will be used if not -1
    pub block_origin_override: BlockId,
    /// Block ID used for the overworld map chunk positioning [ChrIns::chunk_position] and
    /// offsets calculations.
    pub block_origin: BlockId,
    pub chr_set_cleanup: u32,
    _pad44: u32,
    unk48: usize,
    pub chr_model_ins: OwnedPtr<CSChrModelIns>,
    pub chr_ctrl: OwnedPtr<ChrCtrl>,
    /// NPC param ID for this character.
    /// See [NPC_PARAM_ST]
    pub npc_param_id: i32,
    /// 4 number identifier for this npc.
    /// eg. 8000 for Torrent
    /// Same as [Self::character_id]
    pub npc_id: i32,
    pub chr_type: ChrType,
    pub team_type: u8,
    pub p2p_entity_handle: P2PEntityHandle,
    unk78: usize,
    /// Position in global map chunk coordinates.
    pub chunk_position: F32Vector4,
    /// Initial position of the character when it was created.
    pub initial_position: HavokPosition,
    /// Initial orientation of the character when it was created (in euler angles).
    pub initial_orientation_euler: F32Vector4,
    /// Time in seconds since last update ran for the ChrIns.
    pub chr_update_delta_time: f32,
    pub omission_mode: OmissionMode,
    /// Amount of frames between updates for this ChrIns.
    /// Uses same values as omission mode.
    pub omission_mode_override: OmissionMode,
    unkbc: OmissionMode,
    pub target_velocity_recorder: usize,
    unkc8: u8,
    pub is_locked_on: bool,
    unkca: [u8; 0x6],
    pub lock_on_target_position: F32Vector4,
    unke0: i32,
    /// Fractional stamina carried between update ticks.
    /// It accumulates the non-integer portion of stamina recovered so that partial recovery across frames is preserved until it sums to a whole stamina point.
    pub stamina_recovery_remainder: f32,
    /// Multiplier applied to stamina recovery rate.
    /// Used together with regen rate percent to compute stamina recovered per tick.
    pub stamina_recovery_modifier: f32,
    unkec: [u8; 0x74],
    /// Used by TAE's UseGoods to figure out what item to actually apply.
    pub tae_queued_use_item: OptionalItemId,
    unk164: u32,
    unk168: u32,
    unk16c: u32,
    unk170: u32,
    unk174: u32,
    /// Container for the speffects applied to this character.
    pub special_effect: OwnedPtr<SpecialEffect>,
    /// Refers to what field ins you were last hit by.
    pub last_hit_by: FieldInsHandle,
    /// 4 number identifier for this character.
    /// eg. 8000 for Torrent
    /// Same as [Self::npc_id]
    pub character_id: u32,
    unk18c: u32,
    pub module_container: OwnedPtr<ChrInsModuleContainer>,
    unk198: usize,
    /// Squared distance at which the character will be deactivated.
    pub squared_deactivation_distance: f32,
    /// Squared distance at which the character will start fading out.
    pub squared_fade_out_start_distance: f32,
    /// Optional squared override for the full deactivation distance. <0 means no override.
    pub squared_deactivation_distance_override: f32,
    /// Optional squared override for the fade-out start distance.  <0 means no override.
    pub squared_fade_out_start_distance_override: f32,
    unk1b0: f32,
    unk1b4: f32,
    unk1b8: f32,
    unk1bc: f32,
    unk1c0: u32,
    pub chr_flags1c4: ChrInsFlags1c4,
    pub chr_flags1c5: ChrInsFlags1c5,
    pub chr_flags1c6: ChrInsFlags1c6,
    pub chr_flags1c7: ChrInsFlags1c7,
    pub chr_flags1c8: ChrInsFlags1c8,
    pub net_chr_sync_flags: NetChrSyncFlags,
    pub chr_flags1ca: ChrInsFlags1ca,
    pub chr_activation_flags: ChrInsActivationFlags,
    unk1d0: F32Vector4,
    unk1e0: u32,
    pub network_authority: u32,
    pub event_entity_id: u32,
    unk1ec: f32,
    unk1f0: usize,
    pub npc_sp_effect_equip_ctrl: OwnedPtr<NpcSpEffectEquipCtrl>,
    unk200: usize,
    /// Amount of coop players currently in the session
    pub coop_players_for_multiplay_correction: u32,
    /// Override for character role param id.
    /// Will be used when the character joins ceremony.
    ///
    /// See [crate::cs::PartyMemberInfo::pseudo_mp_role_param_override]
    pub role_param_id_override: i32,
    unk210: [u8; 0x8],
    /// Steam ID of the player that created this character if it's a summon
    pub character_creator_steam_id: u64,
    /// What asset to use for the mimic veil.
    pub mimicry_asset: i32,
    /// Row ID of the MAP_MIMICRY_ESTABLISHMENT_PARAM, determines stuff like entry and exit
    /// sfx.
    pub mimicry_establishment_param_id: i32,
    unk228: u32,
    unk22c: u32,
    // Possibly contains some id related to current gparam and attached to chr geometry
    unk230: u32,
    // Same as above
    unk234: u32,
    /// Transparency multiplier for the character
    /// Controlled by TAE Event 193 SetOpacityKeyframe
    pub opacity_keyframes_multiplier: f32,
    /// Transparency multiplier, applied to the previous frame.
    pub opacity_keyframes_multiplier_previous: f32,
    /// Transparency multiplier for tint effects
    pub tint_alpha_multiplier: f32,
    /// Modifier for the tint transparency multiplier.
    /// Negative values will fade tint_alpha_multiplier to 0 and
    /// positive values will fade it to 1
    pub tint_alpha_multiplier_modifier: f32,
    /// Camouflage transparency multiplier.
    /// Changed by ChrCamouflageSlot
    pub camouflage_transparency: f32,
    /// Base transparency of the character.
    pub base_transparency: f32,
    /// Modifier for the base transparency of the character.
    /// Negative values will fade base_transparency to 0 and
    /// positive values will fade it to 1
    pub base_transparency_modifier: f32,
    unk254: [u8; 0x8],
    /// Render group mask from MapStudio
    pub render_group_mask: [u8; 0x20],
    render_group_mask_2: [u8; 0x20],
    render_group_mask_3: [u8; 0x20],
    unk2bc: [u8; 0x4c],
    chr_slot_sys: [u8; 0x40],
    unk348: [u8; 0x1c],
    /// Whether this character's position has been synchronized over the network.
    /// Will be set by NetAIManipulator after receiving a position update.
    pub net_position_synchronized: bool,
    unk368: [u8; 0x20],
    last_received_packet60: u32,
    unk38c: [u8; 0xc],
    hka_pose_importer: usize,
    unk3a0: usize,
    anim_skeleton_to_model_modifier: usize,
    unk3b0: usize,
    cloth_state: [u8; 0x30],
    unk3e8: u32,
    unk3ec: u32,
    unk3f0: f32,
    /// Squared 3D distance to the main player character.
    pub distance_to_player_sqr: f32,
    /// Squared horizontal distance to the main player
    pub horizontal_distance_to_player_sqr: f32,
    /// Squared distance-based update priority, lower is higher priority.
    pub distance_based_update_priority_sqr: f32,
    /// Maximum distance at which the character will be rendered.
    pub max_render_range: f32,
    /// Threshold used for comparison with CSOpenChrActivateThresholdRegionMan
    pub chr_activate_threshold: f32,
    /// Final update priority used to sort characters for updates, lower is higher priority.
    pub update_priority: f32,
    unk40c: u32,
    update_data_module_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_chr_ctrl_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_chr_model_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_havok_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_replay_recorder_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_behavior_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    pub debug_flags: ChrDebugFlags,
    /// Amount of stamina points recovered per tick.
    pub stamina_recovery: u32,
    unk538: u32,
    /// Param ID of the current material character standing on
    /// (e.g. water, lava, etc.), -1 if none.
    pub hit_material_override: i32,
    unk540: u32,
    pub debug_role_param_id: i32,
    unk548: [u8; 0x38],
}

#[for_all_subclasses]
pub impl ChrInsExt for Subclass<ChrIns> {
    fn apply_speffect(&mut self, sp_effect: i32, dont_sync: bool) {
        let rva = Program::current()
            .rva_to_va(rva::get().chr_ins_apply_speffect)
            .unwrap();

        let call = unsafe { transmute::<u64, extern "C" fn(&mut Self, i32, bool) -> u64>(rva) };
        call(self, sp_effect, dont_sync);
    }

    fn remove_speffect(&mut self, sp_effect: i32) {
        let rva = Program::current()
            .rva_to_va(rva::get().chr_ins_remove_speffect)
            .unwrap();

        let call = unsafe { transmute::<u64, extern "C" fn(&mut Self, i32) -> u64>(rva) };
        call(self, sp_effect);
    }

    /// Get the effective Block ID for this character.
    fn block_id(&self) -> BlockId {
        if self.superclass().block_id_override.0 != -1 {
            self.superclass().block_id_override
        } else {
            self.superclass().block_id
        }
    }
    /// Get the effective Block ID used for chunk positioning and offsets.
    fn block_id_origin(&self) -> BlockId {
        if self.superclass().block_origin_override.0 != -1 {
            self.superclass().block_origin_override
        } else {
            self.superclass().block_origin
        }
    }

    /// Calculates the role param ID for this character based on its chr_type, vow_type and
    /// whether this character has same group password in case of a player-like summon.
    fn calculate_role_param_id(
        &self,
        character_type: ChrType,
        vow_type: u8,
        from_group_password: bool,
    ) -> i32 {
        if self.superclass().debug_flags.use_debug_role_param() {
            self.superclass().debug_role_param_id
        } else {
            let base = (if from_group_password { 100 } else { 0 }) + vow_type as u32;
            base.saturating_mul(10_000)
                .saturating_add(character_type as u32) as i32
        }
    }
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrInsFlags1c4(u8);
    impl Debug;
    /// Skips omission mode updates
    pub skip_omission_mode_updates, set_skip_omission_mode_updates: 0;
    /// Forces update for this character.
    /// Will be reset every frame.
    pub force_update, set_force_update:                             1;
    /// Set when character is valid for rendering by MapStudio render group mask
    pub is_render_group_enabled, set_is_render_group_enabled:       3;
    /// If set, character is in camera's view frustum.
    /// Doesn't consider walls or other occluders.
    pub is_onscreen, set_is_onscreen:                               4;
    /// Disables gravity for this character.
    pub no_gravity, set_no_gravity:                                 5;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrInsFlags1c5(u8);
    impl Debug;
    /// Enables precision shooting camera mode, eg. when using a bow.
    /// Will be reset every frame.
    pub precision_shooting, set_precision_shooting: 1;
    /// Enables rendering for this character.
    pub enable_render, set_enable_render:           3;
    /// Makes character invincible,
    /// used in lua events while loading in area
    pub is_invincible, set_is_invincible:           4;
    /// Controls whether the character is dead or not.
    pub death_flag, set_death_flag:                 7;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrInsFlags1c6(u8);
    impl Debug;
    /// This flag is used to determine if the character tag (name, hp, etc) should be
    /// rendered on the side of the screen instead of above the character.
    /// Works only on friendly characters tags, not lock on ones.
    pub draw_tag_offscreen, set_draw_tag_offscreen: 4;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrInsFlags1c7(u8);
    /// True when the character is currently using a mimic's veil.
    /// Will mute walk/run sounds.
    pub mimicry_enabled, set_mimicry_enabled:             5;
    /// If set, character's gravity will be disabled when tint_alpha_multiplier reaches 0
    pub tint_alpha_no_gravity, set_tint_alpha_no_gravity: 7;
    impl Debug;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrInsFlags1c8(u8);
    impl Debug;
    /// Request the fall death camera to be enabled.
    pub request_falldeath_camera, set_request_falldeath_camera: 2;
    /// True when update tasks for this character have been registered.
    pub update_tasks_registered, set_update_tasks_registered:   3;
    /// This flag controls whether the character considered active or not
    pub is_active, set_is_active:                               4;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NetChrSyncFlags(u8);
    impl Debug;
    /// Set for main player character.
    pub replay_recorder_enabled, set_replay_recorder_enabled: 3;
    /// When set, the character will use distance-based network update authority.
    pub distance_based_network_update_authority, set_distance_based_network_update_authority: 5;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrInsFlags1ca(u8);
    /// Controls character sounds activation based on distance
    /// When character is close enough to player, this flag is set
    pub sounds_active, set_sounds_active:                             1;
    /// Set when character's chr_activate_threshold is exceeded threshold in CSOpenChrActivateThresholdRegionMan
    pub activate_threshold_exceeded, set_activate_threshold_exceeded: 2;
    impl Debug;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrInsActivationFlags(u32);
    /// Set when the character is determined to be "left behind" (too far from other players).
    pub is_left_behind, set_is_left_behind:         0;
    /// Controlled by npc param NPC_PARAM_ST::disableActivateOpen or NPC_PARAM_ST::disableActivateLegacy depending on
    /// block id in ChrDataModule
    pub activation_enabled, set_activation_enabled: 3;
    impl Debug;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrDebugFlags(u32);
    impl Debug;
    /// Makes character ignore all incoming damage
    pub disabled_hit, set_disabled_hit:                                     3;
    /// Disables attack, jump, crouch and any other actions except movement
    pub disabled_secondary_actions, set_disabled_secondary_actions:         4;
    /// Disables all movement inputs
    pub disabled_movement, set_disabled_movement:                           5;
    /// Enables debug view render of character target and real facing rotation
    pub character_rotation_debug_view, set_character_rotation_debug_view:   6;
    /// Enables debug view render of facing angle correction applied to model
    pub facing_angle_debug_view, set_facing_angle_debug_view:               7;
    /// Disables all updates to the character (physics, ai, etc)
    pub disabled_updates, set_disabled_updates:                             8;
    /// Will set base_transparency_modifier to -1 when set
    /// and change chrSetEntry status to Unloading
    pub force_unloaded, set_force_unloaded:                                 10;
    /// Will set base_transparency_modifier to 1 when set
    /// and change chrSetEntry status to Loading
    pub force_loaded, set_force_loaded:                                     11;
    /// Will disable character entirely (no updates, no rendering, no physics)
    /// Set most of the time on remote character's horses
    pub character_disabled, set_character_disabled:                         12;
    /// Makes character use [ChrIns::debug_role_param_id] instead of combination of chr_type and vow_type
    pub use_debug_role_param, set_use_debug_role_param:                                 13;
    /// Enables debug view render of state info 110 speffect (counter attack frames)
    pub state_info_110_debug_view, set_state_info_110_debug_view:           14;
    /// Enables debug view render of state info 434 speffect
    /// (seems to be used for increasing damage dealt to stance-broken enemies)
    pub state_info_434_debug_view, set_state_info_434_debug_view:           15;
    /// Disables item consumption when using items
    pub infinite_consumables, set_infinite_consumables:                     16;
    /// Enables debug view render of lock-on angle disable cone
    /// (see ChrActionFlags::disableLockOnAng)
    pub lock_on_angle_debug_view, set_lock_on_angle_debug_view:             19;
    /// Enables debug view render of parryable window frames
    pub parryable_window_debug_view, set_parryable_window_debug_view:       23;
    /// Enables debug view render of parry window frames
    pub parry_window_debug_view, set_parry_window_debug_view:               24;
}

#[repr(C)]
/// Source of name: RTTI
pub struct ChrCtrl {
    vftable: usize,
    unk8: u64,
    pub owner: NonNull<ChrIns>,
    pub manipulator: usize,
    animation_ctrl: usize,
    pub ragdoll_ins: usize,
    pub chr_collision: usize,
    unk38: [u8; 0x88],
    hkxpwv_res_cap: usize,
    pub modifier: OwnedPtr<ChrCtrlModifier>,
    hover_warp_ctrl: usize,
    ai_jump_move_ctrl: usize,
    chr_model_pos_easing: usize,
    unke8: [u8; 0x8],
    pub flags: ChrCtrlFlags,
    pub flags_copy: ChrCtrlFlags,
    unkf8: u32,
    pub chr_proxy_flags: ChrCtrlChrProxyFlags,
    unk100: F32Vector4,
    unk110: F32Vector4,
    unk120: [u8; 0x8],
    pub chr_ragdoll_state: u8,
    // _pad129: [u8; 0x3],
    pub ragdoll_revive_time: f32,
    unk130: [u8; 0x48],
    walk_twist: usize,
    joint_modifier: usize,
    unk188: [u8; 0x4],
    pub weight_type: u32,
    unk190: [u8; 0x10],
    /// Offset from the character's dmypoly for the tag position (name, hp, etc).
    /// Will modify position of the resulting tag.
    pub lock_on_chr_tag_dmypoly_offset: F32Vector4,
    /// Stores the model matrix derived from `CSChrPhysicsModule::ConstructModelMatrix`.
    /// Constructed from `CSChrPhysicsModule::position` and `CSChrPhysicsModule::orientation`.
    pub physics_model_matrix: F32ModelMatrix,
    /// Stores the `raw_physics_model_matrix` multiplied by itself.
    pub physics_transform_matrix_squared: F32Matrix4x4,
    /// The primary model matrix for the character.
    /// It's initially constructed by combining:
    /// - Translation from `raw_physics_model_matrix` and `vertical_position_offset`.
    /// - Orientation from `raw_physics_model_matrix` combined with `additional_orientation_quat`.
    /// - Scaling from `scale_size_x`, `scale_size_y`, and `scale_size_z`.
    ///
    /// This matrix is then processed by ChrEasingModule,
    /// and the eased result is stored back into this field. It's the final matrix
    /// propagated to components like `locationMtx44ChrEntity`.
    pub model_matrix: F32ModelMatrix,
    /// Stores the `model_matrix` multiplied by itself after all modifications.chr_ins
    pub model_matrix_squared: F32Matrix4x4,
    unk2b0: F32Vector4,
    /// An additional orientation (quaternion) that is multiplied with the orientation
    /// derived from the `raw_physics_model_matrix` to produce the final orientation
    /// for the `model_matrix`.
    pub additional_orientation_quat: F32Vector4,
    /// An offset applied to the Y-component (vertical) of the character's position
    /// when constructing the translation part of the `model_matrix`.
    pub vertical_position_offset: f32,
    /// Scaling factor applied along the X-axis during `model_matrix` construction.
    pub scale_size_x: f32,
    /// Scaling factor applied along the Y-axis during `model_matrix` construction.
    pub scale_size_y: f32,
    /// Scaling factor applied along the Z-axis during `model_matrix` construction.
    pub scale_size_z: f32,
    pub offset_y: f32,
    unk2e4: [u8; 0x14],
    location_mtx44_chr_entity: usize,
    unk300: u8,
    /// Set by TAE Event 0 ChrActionFlag (action 113 INVOKEHEIGHTCORRECTION)
    pub height_correction_request: bool,
    unk302: u8,
    unk303: u8,
    unk304: f32,
    /// Limit for foot IK error height correction.
    /// Fetched from NpcParam.
    pub foot_ik_error_height_limit: f32,
    /// Limit for foot IK error height correction when gain is on.
    /// Fetched from NpcParam.
    pub foot_ik_error_on_gain: f32,
    /// Limit for foot IK error height correction when gain is off.
    /// Fetched from NpcParam.
    pub foot_ik_error_off_gain: f32,
    unk314: f32,
    unk318: [u8; 0x10],
    /// Should the character match undulation of the map?
    /// Fetched from NpcParam
    pub is_undulation: bool,
    /// Should FootIK be used for undulation correction?
    pub use_ik_normal_by_undulation: bool,
    unk32a: [u8; 0x2],
    /// Forward undulation correction angle.
    /// Fetched from NpcParam
    pub forward_undulation_limit_radians: f32,
    /// Backward undulation correction angle.
    /// Fetched from NpcParam
    pub backward_undulation_limit_radians: f32,
    /// Side undulation correction angle.
    /// Fetched from NpcParam
    pub side_undulation: f32,
    /// Speed of undulation correction.
    /// Fetched from NpcParam
    pub undulation_correction_gain: f32,
    unk33c: [u8; 0x14],
    unk350: F32Vector4,
    unk360: F32Vector4,
    unk370: [u8; 0x10],
    unk380: F32Vector4,
    unk390: [u8; 0x19],
    /// Group, deciding how character will collide with other characters.
    /// Fetched from NpcParam
    pub hit_group_and_navimesh: u8,
    hit_group_and_navimesh_unk: u8,
    unk3ab: [u8; 0x5],
    unk3b0: usize,
    unk3b8: [u8; 0x18],
}

#[repr(C)]
pub struct ChrCtrlModifier {
    pub owner: NonNull<ChrCtrl>,
    pub data: ChrCtrlModifierData,
}

#[repr(C)]
pub struct ChrCtrlModifierData {
    unk0: f32,
    unk4: i32,
    unk8: i32,
    // Set by TAE Event 255 SetSPRegenRatePercent
    pub sp_regen_rate_percent: u8,
    // Set by TAE Event 230 SetFPRegenRatePercent
    pub fp_regen_rate_percent: u8,
    // _pade: [u8; 0x2],
    pub action_flags: ChrCtrlModifierActionFlags,
    pub hks_flags: ChrCtrlModifierHksFlags,
    unk18: u8,
    unk19: u8,
    // _pad1a: [u8; 0x2],
    unk1cflags: u32,
    unk20: [u8; 0x4],
    /// Set by TAE Event 236 RootMotionReduction
    pub root_motion_reduction: f32,
    unk28: [u8; 0x8],
    /// Character movement speed limit
    /// Set by TAE Event 0 ChrActionFlag (actions 90, 91, 89)
    pub movement_limit: ChrMovementLimit,
    unk34: [u8; 0x4],
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChrMovementLimit {
    NoLimit = 0,
    /// Set by TAE Event 0 ChrActionFlag (action 91 LIMIT_MOVE_SPEED_TO_DASH)
    /// Limits movement speed to fast walk.
    LimitToDash = 1,
    /// Set by TAE Event 0 ChrActionFlag (action 90 LIMIT_MOVE_SPEED_TO_WALK)
    /// Limits movement speed to walk.
    LimitToWalking = 2,
    /// Set by TAE Event 0 ChrActionFlag (action 89 DISABLE_ALL_MOVEMENT)
    /// Disables all movement.
    DisableAll = 3,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrCtrlModifierActionFlags(u32);
    impl Debug;
    /// Set by TAE Event 0 ChrActionFlag (action 20 SEND_GHOST_INFO)
    pub send_ghost_info_requested, set_send_ghost_info_requested:   1;
    /// Set by TAE Event 0 ChrActionFlag (action 49 DISABLE_LOCK_ON)
    /// Makes the character unable to be locked on to.
    pub disable_lock_on, set_disable_lock_on:                       2;
    /// Set by TAE Event 0 ChrActionFlag (action 55 DISABLE_ABILITY_TO_LOCK_ON)
    /// Makes the character unable to lock on to other characters.
    pub disable_ability_to_lock_on, set_disable_ability_to_lock_on: 3;
    /// Set by TAE Event 0 ChrActionFlag (action 40 TEMPORARY_DEATH_STATE)
    pub temporary_death_state, set_temporary_death_state:           5;
    /// Makes the character unable to regenerate stamina.
    pub disable_stamina_regeneration, set_disable_stamina_regeneration: 6;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrCtrlModifierHksFlags(u32);
    impl Debug;
    /// Set by TAE Event 236 RootMotionReduction
    pub root_motion_reduction_applied, set_root_motion_reduction_applied:         0;
    /// Set by TAE Event 0 ChrActionFlag (action 61 DISABLE_Y_AXIS_OF_MOVEMENT_TARGET)
    pub disable_y_axis_of_movement_target, set_disable_y_axis_of_movement_target: 1;
    /// Set by TAE Event 0 ChrActionFlag (action 65 EXTEND_SPEFFECT_LIFETIME)
    pub extend_speffect_lifetime, set_extend_speffect_lifetime:                   4;
    /// Set by TAE Event 0 ChrActionFlag (action 66 SPECIAL_TRANSITION_ENV_271)
    pub special_transition_possible, set_special_transition_possible:             5;
    /// Set by TAE Event 0 ChrActionFlag (action 75 CANCEL_ITEM_PICKUP)
    pub cancel_item_pickup, set_cancel_item_pickup:                               7;
    /// Set by TAE Event 0 ChrActionFlag (action 80 INPUT_ITEM_PICKUP)
    pub input_item_pickup, set_input_item_pickup:                                 8;
    /// Set by TAE Event 0 ChrActionFlag (action 81 DISABLE_ACTIONBUTTON_4400)
    pub disable_actionbutton_4400, set_disable_actionbutton_4400:                 9;
    /// Set by TAE Event 0 ChrActionFlag (action 82 LIGHT_LANTERN_WEAPON_STATEINFO_147)
    pub light_effect, set_light_effect:                                           10;
    /// Set by TAE Event 0 ChrActionFlag (action 88 DISABLE_PRECISION_SHOOTING)
    pub disable_precision_shooting, set_disable_precision_shooting:               13;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrCtrlFlags(u32);
    impl Debug;
    /// Disables player to player collision.
    pub disable_player_collision, set_disable_player_collision:                       0;
    /// Disables hits
    pub disable_hit, set_disable_hit:                                                 1;
    /// Disables map collision
    pub disable_map_collision, set_disable_map_collision:                             2;
    /// Disables map collision 2
    disable_map_collision_2, set_disable_map_collision_2:                             3;
    /// Set by TAE Event 0 ChrActionFlag (action 50 DISABLE_CHARACTER_CAPSULE_COLLISION)
    pub disable_character_capsule_collision, set_disable_character_capsule_collision: 17;
    /// Set by TAE Event 0 ChrActionFlag (action 44 DISABLE_OBJECT_COLLISION)
    /// Reset every frame
    pub disable_object_collision, set_disable_object_collision:                       19;
    /// Set by TAE Event 0 ChrActionFlag (action 74 LADDER_COLLISION)
    /// Reset every frame
    pub ladder_collision, set_ladder_collision:                                       20;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrCtrlChrProxyFlags(u32);
    impl Debug;
    /// When 1, underlying havok character position will be updated with the position from the physics module.
    pub position_sync_requested, set_position_sync_requested: 0;
    /// When 1, underlying havok character rotation will be updated with the rotation from the physics module.
    pub rotation_sync_requested, set_rotation_sync_requested: 1;
}

#[repr(C)]
#[derive(Superclass)]
/// Source of name: RTTI
pub struct CSModelIns {
    vftable: usize,
    unk8: usize,
    pub model_item: OwnedPtr<CSFD4ModelItem>,
    pub model_disp_entity: usize,
    pub location_entity: usize,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSFD4ModelItem {
    pub vftable: usize,
    unk8: [u8; 0x18],
    unk20: usize,
    unk28: usize,
    unk30: usize,
    unk38: usize,
    unk40: [u8; 0x8],
    unk48: usize,
    unk50: usize,
    unk58: usize,
    unk60: usize,
    flver_model_data: usize,
    unk70: usize,
    unk78: [u8; 0x5c8],
    mtx43_array_entity: usize,
    unk648: [u8; 0x8],
    default_dmypoly_location_modifier: usize,
    pub location_aabb_exporter: OwnedPtr<CSFD4LocationGxModelMatricesAndAabbExporter>,
    pub owning_model: NonNull<CSModelIns>,
    unk668: [u8; 0x58],
    unk6c0: DLString,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSFD4LocationGxModelMatricesAndAabbExporter {
    csfd4_location_modifier: [u8; 0x48],
    csfd4_location_node0x20: [u8; 0x20],
    csfd4_location_node0x40: [u8; 0x20],
    unk88: bool,
    pub aabb: Aabb,
    // TODO: rest
}

#[repr(C)]
#[derive(Subclass)]
/// Source of name: RTTI
pub struct CSChrModelIns {
    pub model_ins: CSModelIns,
}

#[repr(C)]
#[derive(Subclass)]
/// Source of name: RTTI
pub struct PlayerIns {
    pub chr_ins: ChrIns,
    pub player_game_data: OwnedPtr<PlayerGameData>,
    chr_manipulator: usize,
    unk590: usize,
    pub player_session_holder: PlayerSessionHolder,
    unk5c0: usize,
    pub replay_recorder: Option<OwnedPtr<ReplayRecorder>>,
    unk5d0: u32,
    unk5d4: u32,
    pub snipe_mode_draw_alpha_fade_timer: f32,
    unk5bc: u32,
    unk5e0: usize,
    fg_model: usize,
    pub npc_param: Option<NonNull<NPC_PARAM_ST>>,
    think_param: u32,
    unk5fc: u32,
    rng_sp_effect_equip_ctrl: usize,
    wep_sp_effect_equip_ctrl: usize,
    pro_sp_effect_equip_ctrl: usize,
    npc_sp_effect_equip_ctrl: usize,
    unk620: [u8; 0x18],
    pub chr_asm: OwnedPtr<ChrAsm>,
    chr_asm_model_res: usize,
    chr_asm_model_ins: usize,
    unk650: [u8; 0x28],
    /// Set on player spawn and maybe on arena respawn?
    /// Players cannot be hurt if this is above 0.
    pub invincibility_timer_for_net_player: f32,
    /// Durability statuses for player's equipment (weapons and protectors only).
    /// (DS3 leftover)
    ///
    /// See [crate::cs::ChrAsmSlot] for index mapping.
    pub durability_statuses: [EquipmentDurabilityStatus; 16],
    unk68c: u8,
    /// Hand used for attack calculations, set by HKS.
    pub attack_reference_hand: HandIndex,
    /// Hand used for guard calculations, set by HKS.
    pub guard_reference_hand: HandIndex,
    unk698: u32,
    unk69c: u32,
    pub player_menu_ctrl: NonNull<CSPlayerMenuCtrl>,
    unk6a8: [u8; 0x8],
    pub locked_on_enemy: FieldInsHandle,
    pub session_manager_player_entry: OwnedPtr<SessionManagerPlayerEntryBase>,
    /// Position within the current block.
    pub block_position: BlockPosition,
    /// Current block ID the player is in.
    pub current_block_id: BlockId,
    /// Last safe for saving block position the player was in.
    /// Used for save files and some multiplayer features like signs
    pub safe_block_pos: BlockPosition,
    /// Last safe for saving block ID the player was in.
    /// Used for save files and some multiplayer features like signs
    pub safe_block_id: BlockId,
    /// Current play region id the player is in.
    pub play_region_id: u32,
    unk6f0: usize,
    unk6f8: [u8; 0xb],
    unk703: bool,
    pub quickmatch_is_stalemate: bool,
    unk705: bool,
    unk706: u8,
    unk707: u8,
    pub opacity_keyframes_timer: FD4Time,
    /// When false, chr role is 14 (BattleRoyale) and chr is not an NPC
    /// Will decrease `opacity_keyframes_timer` and set `ChrIns.opacity_keyframes_multiplier` to 0
    pub enable_arena_chr_rendering: bool,
    unk718: [u8; 0x27],
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HandIndex {
    Left = 0,
    Right = 1,
}

#[repr(C)]
/// Source of name: RTTI
pub struct ReplayRecorder {
    pub vftable: usize,
    unk8: usize,
    pub owning_player: NonNull<PlayerIns>,
    unk18: usize,
    unk20: usize,
    unk28: usize,
    unk30: usize,
    unk38: usize,
    pub max_frame_rate: u32,
    pub frame_counter: u32,
    pub frame_duration: u32,
    /// Position of the character in the oldest recorded frame
    pub position: F32Vector3,
    /// Rotation of the character in the oldest recorded frame
    pub rotation: f32,
    /// Block ID of the character in the oldest recorded frame
    pub block_id: BlockId,
    unk60: i32,
    unk64: i32,
    unk68: i32,
    unk6c: i32,
}

#[repr(C)]
#[derive(Subclass)]
/// Source of name: RTTI
pub struct EnemyIns {
    pub chr_ins: ChrIns,
    pub com_manipulator: OwnedPtr<ComManipulator>,
    pub net_ai_manipulator: usize,
    pub ride_manipulator: usize,
    unk598: usize,
    pub npc_think_param: i32,
    unk5a4: u32,
    npc_sp_effect_equip_ctrl: usize,
    map_studio_sp_effect_equip_ctrl: usize,
    unk5b8: [u8; 0x28],
}

#[repr(C)]
/// Source of name: RTTI
pub struct PlayerSessionHolder {
    vftable: usize,
    player_debug_session: usize,
    unk10: usize,
    pub player_network_session: OwnedPtr<PlayerNetworkSession>,
    unk18: usize,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Type of character in PvP/PvE.
/// Changes a lot of things, like appearance, what items you can use, etc.
///
/// Related to [crate::cs::CharacterTypePropertiesTable] and [crate::cs::MultiplayType].
pub enum ChrType {
    None = -1,
    Local = 0,
    WhitePhantom = 1,
    Duelist = 2,
    Ghost = 3,
    Ghost1 = 4,
    Npc = 5,
    Unk6 = 6,
    Unk7 = 7,
    GrayPhantom = 8,
    Unk9 = 9,
    BloodstainGhost = 10,
    BonfireGhost = 11,
    Unk12 = 12,
    Arena = 13,
    MessageGhost = 14,
    BloodyFinger = 15,
    Recusant = 16,
    BluePhantom = 17,
    FesteringBloodyFinger = 18,
    WhiteSummonNpc = 19,
    BloodyFingerNpc = 20,
    RecusantNpc = 21,
    Unk22 = 22,
}
