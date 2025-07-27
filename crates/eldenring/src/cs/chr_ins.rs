use bitfield::bitfield;
use std::ffi;
use std::ops::Index;
use std::ptr::NonNull;
use std::slice::SliceIndex;
use vtable_rs::VPtr;
use windows::core::PCWSTR;

use crate::cs::MapId;
use crate::dltx::DLString;
use crate::fd4::FD4Time;
use crate::position::{BlockPosition, HavokPosition};
use crate::rotation::Quaternion;
use crate::Vector;
use shared::{FSMatrix4x4, FSVector4, OwnedPtr};

use crate::cs::field_ins::{FieldInsBaseVmt, FieldInsHandle};
use crate::cs::gaitem::GaitemHandle;
use crate::cs::network_session::PlayerNetworkSession;
use crate::cs::player_game_data::{ChrAsm, PlayerGameData};
use crate::cs::session_manager::{SessionManagerPlayerEntry, SessionManagerPlayerEntryBase};
use crate::cs::sp_effect::{NpcSpEffectEquipCtrl, SpecialEffect};
use crate::cs::task::{CSEzRabbitNoUpdateTask, CSEzVoidTask};
use crate::cs::world_chr_man::{ChrSetEntry, WorldBlockChr};
use crate::cs::world_geom_man::{CSMsbParts, CSMsbPartsEne};
use crate::cs::CSPlayerMenuCtrl;
use crate::cs::ItemId;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Used for communicating about characters in the networking layer. This handle is essentially the
/// same as FieldInsHandle but has its MapID and selector swapped. In packets this might be packed
/// into map_id (4 bytes) + chr_selector (3 bytes). According to Sekiro's debug asserts the packed
/// version is referred to as the "whoid".
pub struct P2PEntityHandle {
    pub map_id: i32,
    pub chr_selector: i32,
}

#[repr(C)]
pub struct AtkParamLookupResult {
    behavior_param_id: u32,
    unk_param_def_meta: u32,
    is_player_atk_param: bool,
    _pad9: [u8; 7],
    param_row: usize,
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
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum OmissionMode {
    NoUpdate = -2,
    Normal = 0,
    OneFps = 1,
    FiveFps = 5,
    TwentyFps = 20,
    ThirtyFps = 30,
}

#[repr(C)]
/// Abstract base class to all characters. NPCs, Enemies, Players, Summons, Ghosts, even gesturing
/// character on bloodmessages inherit from this.
///
/// Source of name: RTTI
pub struct ChrIns {
    pub vftable: VPtr<dyn ChrInsVmt, Self>,
    pub field_ins_handle: FieldInsHandle,
    chr_set_entry: usize,
    unk18: usize,
    pub backread_state: u32,
    unk24: u32,
    chr_res: usize,
    pub map_id_1: MapId,
    pub map_id_origin_1: i32,
    pub block_origin_override: MapId,
    pub block_origin: MapId,
    pub chr_set_cleanup: u32,
    _pad44: u32,
    unk48: usize,
    pub chr_model_ins: OwnedPtr<CSChrModelIns>,
    pub chr_ctrl: OwnedPtr<ChrCtrl>,
    pub think_param_id: i32,
    pub npc_id_1: i32,
    pub chr_type: ChrType,
    pub team_type: u8,
    pad6d: [u8; 3],
    pub p2p_entity_handle: P2PEntityHandle,
    unk78: usize,
    /// Position in global map chunk coordinates.
    pub chunk_position: FSVector4,
    /// Initial position of the character when it was created.
    pub initial_position: HavokPosition,
    /// Initial orientation of the character when it was created (in euler angles).
    pub initial_orientation_euler: FSVector4,
    /// Time in seconds since last update ran for the ChrIns.
    pub chr_update_delta_time: f32,
    pub omission_mode: OmissionMode,
    /// Amount of frames between updates for this ChrIns.
    /// Uses same values as omission mode.
    pub frames_per_update: OmissionMode,
    unkbc: OmissionMode,
    pub target_velocity_recorder: usize,
    unkc8: u8,
    pub is_locked_on: bool,
    unkca: [u8; 0x6],
    pub lock_on_target_position: FSVector4,
    unke0: [u8; 0x80],
    /// Used by TAE's UseGoods to figure out what item to actually apply.
    pub tae_queued_use_item: ItemId,
    unk164: u32,
    unk168: u32,
    unk16c: u32,
    unk170: u32,
    unk174: u32,
    /// Container for the speffects applied to this character.
    pub special_effect: OwnedPtr<SpecialEffect>,
    /// Refers to what field ins you were last hit by.
    pub last_hit_by: FieldInsHandle,
    pub character_id: u32,
    unk18c: u32,
    pub module_container: OwnedPtr<ChrInsModuleContainer>,
    unk198: usize,
    unk1a0: f32,
    unk1a4: f32,
    unk1a8: f32,
    unk1ac: f32,
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
    // _pad1cb: u8,
    pub chr_flags1cc: ChrInsFlags1cc,
    unk1d0: FSVector4,
    unk1e0: u32,
    pub network_authority: u32,
    pub event_entity_id: u32,
    unk1ec: f32,
    unk1f0: usize,
    pub npc_sp_effect_equip_ctrl: OwnedPtr<NpcSpEffectEquipCtrl>,
    unk200: [u8; 0x18],
    pub character_creator_steam_id: u64,
    /// What asset to use for the mimic veil.
    pub mimicry_asset: i32,
    /// Row ID of the MAP_MIMICRY_ESTABLISHMENT_PARAM, determines stuff like entry and exit
    /// sfx.
    pub mimicry_establishment_param_id: i32,
    unk228: u32,
    unk22c: u32,
    pub chr_fade_multiplier: f32,
    pub chr_fade_multiplier_reset: f32,
    /// Transparency multiplier for the character
    /// Controlled by TAE Event 193 SetOpacityKeyframe
    pub opacity_keyframes_multiplier: f32,
    /// Transparency multiplier, applied to the previous frame.
    pub opacity_keyframes_multiplier_previous: f32,
    unk240: f32,
    unk244: f32,
    /// Camouflage transparency multiplier.
    /// Changed by ChrCamouflageSlot
    pub camouflage_transparency: f32,
    /// Base transparency of the character.
    pub base_transparency: f32,
    unk250: [u8; 0xc],
    unk25c: [u8; 0x20],
    unk27c: [u8; 0x84],
    chr_slot_sys: [u8; 0x40],
    unk340: usize,
    unk348: [u8; 0x40],
    last_received_packet60: u32,
    unk38c: [u8; 0xc],
    hka_pose_importer: usize,
    unk3a0: usize,
    anim_skeleton_to_model_modifier: usize,
    unk3b0: usize,
    cloth_state: [u8; 0x30],
    unk3e8: [u8; 0x28],
    update_data_module_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_chr_ctrl_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_chr_model_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_havok_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_replay_recorder_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_behavior_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    unk530_debug_flags: u64,
    unk538: u64,
    unk540: u32,
    pub role_param_id: i32,
    unk548: [u8; 0x38],
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrInsFlags1c4(u8);
    impl Debug;
    /// Skips omission mode updates
    pub skip_omission_mode_updates, set_skip_omission_mode_updates: 0;
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
    impl Debug;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrInsFlags1c8(u8);
    impl Debug;
    /// Request the fall death camera to be enabled.
    pub request_falldeath_camera, set_request_falldeath_camera: 2;
    /// This flag controls whether the character tag (name, hp, etc) should be rendered or not.
    pub enable_character_tag, set_enable_character_tag:         4;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct NetChrSyncFlags(u8);
    impl Debug;
    /// When set, the character will use distance-based network update authority.
    pub distance_based_network_update_authority, set_distance_based_network_update_authority: 5;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrInsFlags1ca(u8);
    impl Debug;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrInsFlags1cc(u32);
    impl Debug;
}

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
    fall: usize,
    ladder: usize,
    pub action_request: OwnedPtr<CSChrActionRequestModule>,
    pub throw: OwnedPtr<CSChrThrowModule>,
    hitstop: usize,
    damage: usize,
    material: usize,
    knockback: usize,
    sfx: usize,
    vfx: usize,
    behavior_data: usize,
    unkc8: usize,
    /// Describes a number of render-related inputs, like the color for the phantom effect and
    /// equipment coloring effects.
    pub model_param_modifier: OwnedPtr<CSChrModelParamModifierModule>,
    dripping: usize,
    unke0: usize,
    ride: usize,
    bonemove: usize,
    /// Describes if your character is wet for rendering as well as applying speffects.
    pub wet: OwnedPtr<CSChrWetModule>,
    auto_homing: usize,
    above_shadow_test: usize,
    sword_arts: usize,
    grass_hit: usize,
    wheel_rot: usize,
    cliff_wind: usize,
    navimesh_cost_effect: usize,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrActionRequestModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    action_requests: u64,
    unk18: [u8; 0x8],
    unk20: u64,
    unk28: [u8; 0x8],
    unk30: u64,
    unk38: [u8; 0x8],
    /// Controls what actions are currently can't be inputted by the player.
    pub disabled_action_inputs: ChrActions,
    unk48: [u8; 0x48],
    unk90: u32,
    unk94: [u8; 0x4],
    /// Controls what actions can be queued during current animation.
    pub possible_action_inputs: ChrActions,
    /// Controls what actions can interrupt current animation.
    pub possible_action_cancels: ChrActions,
    unka8: [u8; 0x8],
    /// Current action durations in seconds.
    /// Corresponds to how long each action button is held down.
    pub action_timers: ActionTimers,
    /// For how long movement request buttons are held down.
    /// Conflicting movement requests will be ignored (eg. W + S).
    pub movement_request_duration: f32,
    unkf4: [u8; 0x4],
    /// Param ID of the requested gesture from PadManipulator.
    pub requested_gesture: i32,
    unkfc: [u8; 0x4],
    pub ai_cancels: AiActionCancels,
    unk104: [u8; 0x3c],
}

#[repr(C)]
pub struct ActionTimers {
    /// Main hand light attack
    pub r1: f32,
    /// Main hand heavy attack
    pub r2: f32,
    /// Offhand light attack
    pub l1: f32,
    /// Offhand heavy attack
    pub l2: f32,
    /// Pouch slot submenu and weapon switch button (E)
    pub action: f32,
    /// Roll and Backstep
    pub roll: f32,
    /// Jump button
    pub jump: f32,
    /// Consumable item use
    pub use_item: f32,
    /// Spell switch
    pub switch_spell: f32,
    /// Change weapon in right hand
    pub change_weapon_r: f32,
    /// Change weapon in left hand
    pub change_weapon_l: f32,
    /// Change to next consumable
    pub change_item: f32,
    /// Right stick click
    pub r3: f32,
    /// Left stick click
    pub l3: f32,
    pub touch_r: f32,
    pub touch_l: f32,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct AiActionCancels(u32);
    impl Debug;

    /// Set by TAE Event 0 ChrActionFlag (action 32 CANCEL_JUMP_CROUCH_WEAPON_SWITCH)
    pub slot_switch, set_slot_switch:       3;
    /// Set by TAE Event 0 ChrActionFlag (action 4 CANCEL_RH_ATTACK & 23 CANCEL_AI_COMBOATTACK)
    pub rh_attack, set_rh_attack:           4;
    /// Set by TAE Event 0 ChrActionFlag (action 1 CANCEL_LS_MOVEMENT)
    pub ls_movement, set_ls_movement:       6;
    /// Set by TAE Event 0 ChrActionFlag (action 4 CANCEL_RH_ATTACK & 16 CANCEL_LH_ATTACK)
    pub action_general, set_action_general: 9;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrActions(u64);
    impl Debug;
    pub r1, set_r1:                           0;
    pub r2, set_r2:                           1;
    pub l1, set_l1:                           2;
    pub l2, set_l2:                           3;
    /// Pouch slot submenu and weapon switch button (E)
    pub action, set_action:                   4;
    /// Roll button
    pub sp_move, set_sp_move:                 5;
    pub jump, set_jump:                       6;
    pub use_item, set_use_item:               7;
    pub switch_spell, set_switch_spell:       8;
    pub change_weapon_r, set_change_weapon_r: 9;
    pub change_weapon_l, set_change_weapon_l: 10;
    pub change_item, set_change_item:         11;
    /// Lock on
    pub r3, set_r3:                           12;
    /// Crouch
    pub l3, set_l3:                           13;
    pub touch_r, set_touch_r:                 14;
    pub touch_l, set_touch_l:                 15;
    pub backstep, set_backstep:               16;
    pub rolling, set_rolling:                 17;
    /// Magic casted by the spellcasting item when in main hand.
    pub magic_r, set_magic_r:                 19;
    /// Magic casted by the spellcasting item when in off hand.
    pub magic_l, set_magic_l:                 20;
    pub gesture, set_gesture:                 21;
    pub ladderup, set_ladderup:               22;
    pub ladderdown, set_ladderdown:           23;
    pub guard, set_guard:                     24;
    pub emergencystep, set_emergencystep:     25;
    /// Forward + R1 + L1 (ds kick)
    pub light_kick, set_light_kick:           26;
    /// Forward + R2 + L2 (ds kick)
    pub heavy_kick, set_heavy_kick:           27;
    pub change_style_r, set_change_style_r:   28;
    pub change_style_l, set_change_style_l:   29;
    pub rideon, set_rideon:                   30;
    /// Torrent boost
    pub rideoff, set_rideoff:                 31;
    pub buddy_disappear, set_buddy_disappear: 32;
    /// Magic casted by the spellcasting sword when in main hand.
    pub magic_r2, set_magic_r2:               33;
    /// Magic casted by the spellcasting sword when in off hand.
    pub magic_l2, set_magic_l2:               34;
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrActionFlagModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub animation_action_flags: ChrActionAnimationFlags,
    unk14: u32,
    unk18_flags: u32,
    /// Damage level of the last received attack.
    /// Determines how much character will stagger when hit.
    pub damage_level: u8,
    // pad1d: [u8; 0x3],
    /// Guard level from the params of the equipped weapon.
    pub guard_level: u32,
    unk24: [u8; 0x10],
    /// Param ID of the last received attack.
    pub received_damage_type: u32,
    unk38: [u8; 0x8],
    pub action_modifiers_flags: ChrActionModifiersFlags,
    unk48: u64,
    unk50: u64,
    unk58: [u8; 0x10],
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model0_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model0_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model1_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model1_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model2_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model2_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model3_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub lh_model3_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model0_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model0_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model1_absorp_pos_param_condition: u8,
    /// /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model1_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model2_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model2_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model3_absorp_pos_param_condition: u8,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub rh_model3_change_type: WeaponModelChangeType,
    /// Set by TAE Event 712 OverrideWeaponModelLocations
    pub weapon_model_location_overridden: bool,
    unk79: [u8; 0xb],
    /// Set by TAE Event 224 SetTurnSpeed
    pub turn_speed: f32,
    /// Set by TAE Event 706 ChrTurnSpeedForLock
    pub lock_on_turn_speed: f32,
    /// Set by TAE Event 717 SetJointTurnSpeed
    pub joint_turn_speed: f32,
    pub global_turn_speed_priority: i8,
    pub turn_speed_priority: i8,
    pub lock_on_turn_speed_priority: i8,
    pub joint_turn_speed_priority: i8,
    /// Set by TAE Event 704 ChrTurnSpeedEX
    pub speed_default: f32,
    /// Set by TAE Event 704 ChrTurnSpeedEX
    pub speed_extra: f32,
    /// Set by TAE Event 704 ChrTurnSpeedEX
    pub speed_boost: f32,
    unka0: f32,
    unka4: f32,
    /// Set by TAE Event 705 FacingAngleCorrection
    pub facing_angle_correction_rad: f32,
    /// Set by TAE Event 760 BoostRootMotionToReachTarget
    pub root_motion_div: f32,
    /// Set by TAE Event 760 BoostRootMotionToReachTarget
    pub root_motion_mult_min_dist: f32,
    /// Set by TAE Event 760 BoostRootMotionToReachTarget
    pub root_motion_mult_max_dist: f32,
    /// Set by TAE Event 760 BoostRootMotionToReachTarget
    pub root_motion_mult_angle_from_target: f32,
    /// Set by TAE Event 760 BoostRootMotionToReachTarget
    pub root_motion_mult_target_radius: f32,
    unkc0: [u8; 0x110],
    /// Set by TAE Event 0 ChrActionFlag (action 5 SET_PARRYABLE_WINDOW)
    pub unused_parry_window_arg: u8,
    unk1d1: [u8; 0xf],
    unk1e0: f32,
    unk1e4: f32,
    unk1e8: [u8; 0x10],
    /// Set by TAE Event 800 SetMovementMultiplier
    pub mov_dist_multiplier: f32,
    /// Set by TAE Event 800 SetMovementMultiplier
    pub cam_turn_dist_multiplier: f32,
    /// Set by TAE Event 800 SetMovementMultiplier
    pub ladder_dist_multiplier: f32,
    /// Set by TAE Event 0 (3 SET_GUARD_TYPE)
    pub guard_behavior_judge_id: u32,
    /// Set by TAE Event 342 SetSaDurabilityMultiplier
    pub sa_durability_multiplier: f32,
    /// Set by TAE Event 511 SetSpEffectWetConditionDepth
    /// Controls what speffect will be applied by speffect param
    pub sp_effect_wet_condition_depth: SpEffectWetConditionDepth,
    unk20d: [u8; 0x7],
    unk214: u32,
    /// Set by TAE Event 0 ChrActionFlag (action 72 INVOKEKNOCKBACKVALUE)
    pub knockback_value: f32,
    pub action_flags: u32,
    unk220: [u8; 0x18],
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_up_limit: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_down_limit: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_right_limit: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_left_limit: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_up_dead_zone: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_down_dead_zone: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_right_dead_zone: i16,
    /// Set by TAE Event 238 SetBulletAimAngle
    pub bullet_aim_angle_left_dead_zone: i16,
    unk248: [u8; 0x10],
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpEffectWetConditionDepth {
    Default = 0,
    LowerBody = 1,
    FullBody = 2,
}

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeaponModelChangeType {
    MoveToDefaultLocation = -1,
    MoveTo1HRightWeaponLocation = 0,
    MoveTo1HLeftWeaponLocation = 1,
    MoveTo2HRightWeaponLocation = 2,
    MoveToSheathedLocation = 3,
    MaintainPreviousChange = 4,
    WeaponIdHardcoded = 5,
    Unknown6 = 6,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrActionAnimationFlags(u32);
    impl Debug;

    pub stay_state, set_stay_state: 0;
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrActionModifiersFlags(u64);
    impl Debug;

    /// Set by TAE Event 0 ChrActionFlag (action 94 PERFECT_INVINCIBILITY)
    pub perfect_invincibility, set_perfect_invincibility:                                             0;
    /// Set by TAE Event 0 ChrActionFlag (action 8 FLAG_AS_DODGING)
    pub dodging, set_dodging:                                                                         1;
    /// Set by TAE Event 0 ChrActionFlag (action 68 INVINCIBLE_DURING_THROW_ATTACKER)
    pub invincible_during_throw_attacker, set_invincible_during_throw_attacker:                       2;
    /// Set by TAE Event 0 ChrActionFlag (action 67 INVINCIBLE_EXCLUDING_THROW_ATTACKS_DEFENDER)
    pub invincible_excluding_throw_attacks_defender, set_invincible_excluding_throw_attacks_defender: 3;
    /// Set by TAE Event 0 ChrActionFlag (action 132 JUMP_FRAMES_LOWER_BODY_IFRAMES)
    pub jump_frames_lower_body_iframes, set_jump_frames_lower_body_iframes:                           4;
    /// Set by TAE Event 0 ChrActionFlag (action 143 PVE_ONLY_IFRAMES)
    pub pve_only_iframes, set_pve_only_iframes:                                                       5;
    /// Set by TAE Event 0 ChrActionFlag (action 3 SET_GUARD_TYPE)
    pub guard_type_set, set_guard_type_set:                                                           6;
    /// Set by TAE Event 0 ChrActionFlag (action 18 CANCEL_THROW)
    pub cancel_throw, set_cancel_throw:                                                               7;
    /// Set by TAE Event 0 ChrActionFlag (action 24 SUPER_ARMOR)
    pub super_armor, set_super_armor:                                                                 8;
    /// Set by TAE Event 0 ChrActionFlag (action 72 INVOKEKNOCKBACK)
    pub invokeknockbackvalue, set_invokeknockbackvalue:                                               9;
    /// Set by TAE Event 0 ChrActionFlag (action 5 SET_PARRYABLE_WINDOW)
    /// When set, the character can be parried.
    pub parryable, set_parryable:                                                                     10;
    /// Set by TAE Event 0 ChrActionFlag (action 42 SWEETSPOT_DEAL_12_5_MORE_DAMAGE)
    pub take_12_5_percent_more_damage, set_take_12_5_percent_more_damage:                             11;
    /// Set by TAE Event 0 ChrActionFlag (action 59 WEAKSPOT_DEAL_20_LESS_DAMAGE)
    pub weakspot_deal_20_less_damage, set_weakspot_deal_20_less_damage:                               12;
    /// Set by TAE Event 0 ChrActionFlag (action 56 DISABLE_WALL_ATTACK_BOUND)
    pub disable_wall_attack_bound, set_disable_wall_attack_bound:                                     13;
    /// Set by TAE Event 0 ChrActionFlag (action 57 DISABLE_NPC_WALL_ATTACK_BOUND)
    pub disable_npc_wall_attack_bound, set_disable_npc_wall_attack_bound:                             14;
    /// Set by TAE Event 0 ChrActionFlag (action 7 DISABLE_TURNING)
    pub disable_turning, set_disable_turning:                                                         15;
    /// Set by TAE Event 704 ChrTurnSpeedEX
    /// Additionally sets speed_default, speed_boost and speed_extra on CSChrActionFlagModule
    pub turn_speed_modified, set_turn_speed_modified:                                                 16;
    /// Set by TAE Event 0 ChrActionFlag (action 96 SET_IMMORTALITY)
    pub set_immortality, set_set_immortality:                                                         17;
    /// Set by TAE 760 BoostRootMotionToReachTarget
    pub root_motion_multiplier_set, set_root_motion_multiplier_set:                                   19;
    /// Set by TAE 760 BoostRootMotionToReachTarget
    /// Depends on `enable` argument of the event.
    pub root_motion_multiplier_enabled, set_root_motion_multiplier_enabled:                           20;
    /// Set by TAE Event 0 ChrActionFlag (action 102 POISE_FORCED_BREAK)
    pub poise_forced_break, set_poise_forced_break:                                                   21;
    /// Set by TAE Event 705 FacingAngleCorrection
    pub facing_angle_correction_set, set_facing_angle_correction_set:                                 24;
    /// Set by TAE Event 197 DS3FadeOut
    pub fade_out_applied, set_fade_out_applied:                                                       25;
    /// Set by TAE Event 0 ChrActionFlag (action 109 CAN_DOUBLE_CAST_ENV_331)
    pub can_double_cast, set_can_double_cast:                                                         26;
    /// Set by TAE Event 790 DisableDefaultWeaponTrail
    pub disable_default_weapon_trail, set_disable_default_weapon_trail:                               27;
    /// Set by TAE Event 791 PartDamageAdditiveBlendInvalid
    pub part_damage_additive_blend_invalid, set_part_damage_additive_blend_invalid:                   31;
    /// Set by TAE Event 0 ChrActionFlag (action 110 DISABLE_DIRECTION_CHANGE)
    pub disable_direction_change, set_disable_direction_change:                                       32;
    /// Set by TAE Event 0 ChrActionFlag (action 114 ENHANCED_CAMERA_TRACKING)
    pub enhanced_camera_tracking, set_enhanced_camera_tracking:                                       33;
    /// Set by TAE Event 0 ChrActionFlag (action 111 AI_PARRY_POSSIBLE_STATE)
    pub ai_parry_possible_state, set_ai_parry_possible_state:                                         35;
    /// Set by TAE Event 0 ChrActionFlag (action 63 AI_PARRY_SIGNAL)
    pub ai_parry_signal, set_ai_parry_signal:                                                         36;
    /// Set by TAE Event 0 ChrActionFlag (action 119 TRYTOINVOKEFORCEPARRYMODE)
    pub force_parry_mode, set_force_parry_mode:                                                       37;
    /// Set by TAE Event 782 AiReplanningCtrlReset
    pub ai_replanning_ctrl_reset, set_ai_replanning_ctrl_reset:                                       39;
    /// Set by TAE Event 707 ManualAttackAiming
    pub manual_attack_aiming, set_manual_attack_aiming:                                               40;
    /// Set by TAE Event 332 WeaponArtWeaponStyleCheck
    pub weapon_art_weapon_style_check, set_weapon_art_weapon_style_check:                             41;
    /// Set by TAE Event 0 ChrActionFlag (action 53 DISABLE_FLOATING_GAUGE_DISPLAY)
    pub disable_floating_gauge_display, set_disable_floating_gauge_display:                           44;
    /// Set by TAE Event 238 SetBulletAimAngle
    /// Additionally, sets bullet_aim_angle limits on CSChrActionFlagModule
    pub bullet_aim_angle_set, set_bullet_aim_angle_set:                                               45;
    /// Set by TAE Event 781
    pub turn_lower_body, set_turn_lower_body:                                                         46;
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrPhysicsModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: NonNull<ChrIns>,
    unk18: [u8; 0x8],
    pub data_module: NonNull<CSChrDataModule>,
    unk28: [u8; 0x28],
    pub orientation: Quaternion,
    /// Rotation, controlled by specifics of the character's movement,
    /// can be changed by tae and interpolated towards the target rotation
    pub interpolated_orientation: Quaternion,
    pub position: HavokPosition,
    pub last_update_position: HavokPosition,
    unk90: bool,
    pub chr_proxy_pos_update_requested: bool,
    pub standing_on_solid_ground: bool,
    pub touching_solid_ground: bool,
    unk94: [u8; 0x4],
    chr_proxy: usize,
    chr_proxy2: usize,
    unka8: [u8; 0x8],
    hk_collision_shape: usize,
    unkb8: [u8; 0x10],
    unkc8: f32,
    pub adjust_to_hi_collision: bool,
    unkcd: [u8; 0x3],
    root_motion: FSVector4,
    root_motion_unk: FSVector4,
    unkf0: FSVector4,
    unk100: [u8; 0x4],
    pub chr_push_up_factor: f32,
    ground_offset: f32,
    ground_offset_unk: f32,
    unk110: [u8; 0x10],
    gravity: FSVector4,
    gravity_unk: FSVector4,
    unk140: [u8; 0x10],
    unk150: FSVector4,
    unk160: FSVector4,
    unk170: FSVector4,
    unk180: FSVector4,
    pub additional_rotation: FSVector4,
    unk1a0: [u8; 0x8],
    unk1a8: FD4Time,
    unk1b8: f32,
    unk1bc: f32,
    /// Set by TAE Event 0 ChrActionFlag
    /// (action 124 EnableRotationInterpolationMultiplier or 125 SnapToTargetRotation)
    /// Controls how much the character's rotation is interpolated towards the target rotation.
    pub rotation_multiplier: f32,
    pub motion_multiplier: f32,
    unk1c8: [u8; 0x4],
    pub gravity_multiplier: f32,
    pub is_falling: bool,
    pub is_touching_ground: bool,
    unk1d2: u8,
    no_gravity_unk: bool,
    unk1d4: u8,
    /// Set by TAE Event 0 ChrActionFlag (action 27 DISABLE_GRAVITY)
    pub gravity_disabled: bool,
    unk1d6: u8,
    unk1d7: u8,
    unk1d8: u8,
    /// Set by TAE Event 0 ChrActionFlag (action 38 FLYING_CHARACTER_FALL)
    pub flying_character_fall_requested: bool,
    unk1da: u8,
    /// Should the character's rotation use world Y alignment logic.
    pub use_world_y_alignment_logic: bool,
    pub is_surface_constrained: bool,
    unk1dd: [u8; 0x4],
    /// Only true for Watcher Stones character (stone sphere catapillars).
    pub is_watcher_stones: bool,
    unk1e2: [u8; 0xe],
    /// Information about the slope the character is currently on
    pub slope_info: ChrPhysicsSlopeInfo,
    /// Information about character's sliding state
    pub slide_info: ChrPhysicsSlideInfo,
    unk290: [u8; 0x30],
    unkposition: FSVector4,
    pub orientation_euler: FSVector4,
    pub chr_hit_height: f32,
    pub chr_hit_radius: f32,
    unk2e8: [u8; 0x8],
    pub hit_height: f32,
    pub hit_radius: f32,
    unk2f8: [u8; 0x8],
    pub weight: f32,
    unk304: f32,
    unk308: f32,
    unk30c: [u8; 0x4],
    chr_push_up_factor2: f32,
    pub default_max_turn_rate: f32,
    unk318: f32,
    unk31c: [u8; 0x4],
    pub move_type_flags: MoveTypeFlags,
    unk324: [u8; 0x4],
    pub player_game_data: Option<NonNull<PlayerGameData>>,
    unk330: f32,
    unk334: f32,
    unk338: [u8; 0x8],
    unk340: f32,
    unk344: f32,
    unk348: [u8; 0x48],
    hk_frame_data: usize,
    unk398: f32,
    unk39c: [u8; 0x4],
    unk3a0: FSVector4,
    unk3b0: [u8; 0x10],
    unk3c0: FSVector4,
    unk3d0: FSVector4,
    unk3e0: [u8; 0x6],
    /// Loaded from NpcParam
    pub is_enable_step_disp_interpolate: bool,
    unk3e7: u8,
    /// Loaded from NpcParam
    pub step_disp_interpolate_time: f32,
    /// Loaded from NpcParam
    pub step_disp_interpolate_trigger_value: f32,
    unk3f0: u8,
    no_gravity_unk2: bool,
    unk3f2: u8,
    pub debug_draw_orientation: bool,
    unk3f4: u8,
    unk3f5: u8,
    pub debug_draw_character_slope_capsule: bool,
    unk3f7: [u8; 0x29],
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MoveTypeFlags(u8);
    impl Debug;
    pub use_world_y_alignment, set_use_world_y_alignment:   0;
    pub is_surface_constrained, set_is_surface_constrained: 1;
    pub is_pad_manipulated, set_is_pad_manipulated:         4;
}

#[repr(C)]
pub struct ChrPhysicsSlopeInfo {
    unk0: FSVector4,
    unk10: FSVector4,
    unk20: f32,
    unk24: f32,
    unk28: f32,
    unk2c: f32,
    unk30: FSVector4,
    /// Slope vector, used for slope detection.
    pub slope_vector: FSVector4,
    unk50: [u8; 0x20],
}
#[repr(C)]
pub struct ChrPhysicsSlideInfo {
    /// Slide direction vector
    pub slide_vector: FSVector4,
    /// Information about the slope the character is sliding on
    pub slope_info: NonNull<ChrPhysicsSlopeInfo>,
    /// Angle at which the character is starting to slide
    pub max_slide_angle: f32,
    /// Is character currently sliding?
    pub is_sliding: bool,
    unk1d: [u8; 0x3],
    unk20: f32,
    unk24: bool,
    unk25: [u8; 0xb],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrWetModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: [u8; 0x60],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrModelParamModifierModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub modifiers: Vector<CSChrModelParamModifierModuleEntry>,
}

#[repr(C)]
pub struct CSChrModelParamModifierModuleEntry {
    unk0: u8,
    unk1: [u8; 0x3],
    unk4: u32,
    unk8: u32,
    unkc: u32,
    unk10: u64,
    unk18: u32,
    unk1c: u32,
    pub name: PCWSTR,
    unk28: CSChrModelParamModifierModuleEntryValue,
    unk40: CSChrModelParamModifierModuleEntryValue,
    unk58: CSChrModelParamModifierModuleEntryValue,
    unk70: u32,
    unk74: u32,
    unk78: u32,
    unk7c: u32,
    unk80: u64,
    unk88: CSChrModelParamModifierModuleEntryValue,
    unka0: CSChrModelParamModifierModuleEntryValue,
    unkb0: [u8; 0x20],
}

unsafe impl Sync for CSChrModelParamModifierModuleEntry {}
unsafe impl Send for CSChrModelParamModifierModuleEntry {}

#[repr(C)]
pub struct CSChrModelParamModifierModuleEntryValue {
    unk0: u32,
    pub value1: f32,
    pub value2: f32,
    pub value3: f32,
    pub value4: f32,
    unk14: u32,
}

#[repr(C)]
pub struct CSChrTimeActModuleAnim {
    pub anim_id: i32,
    pub play_time: f32,
    play_time2: f32,
    pub anim_length: f32,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrTimeActModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    hvk_anim: usize,
    chr_tae_anim_event: usize,
    /// Circular buffer of animations to play.
    pub anim_queue: [CSChrTimeActModuleAnim; 10],
    /// Index of the next animation to play or update.
    pub write_idx: u32,
    /// Index of the last animation played or updated.
    pub read_idx: u32,
    unkc8: u32,
    unkcc: u32,
    unkd0: u32,
    unkd4: u32,
}

#[repr(C)]
pub struct CSChrBehaviorModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: usize,
    unk18: usize,
    unk20: usize,
    unk28: usize,
    pub root_motion: FSVector4,
    unk40: [u8; 0x20],
    unk60: [u8; 0xa48],
    unkaa8: [u8; 0x58],
    unkb00: [u8; 0xa48],
    unk1548: [u8; 0x68],
    unk15b0: FD4Time,
    unk15c0: [u8; 0xc0],
    /// controls IK
    /// Set to -1 by TAE Event 0 ChrActionFlag (action 28 DISABLE_FOOT_IK)
    pub ground_touch_state: u32,
    /// Read from NpcParam, PI by default.
    pub max_ankle_pitch_angle_rad: f32,
    /// Read from NpcParam, PI by default.
    pub max_ankle_roll_angle_rad: f32,
    unk168c: [u8; 0x104],
    unk1790: FSVector4,
    unk17a0: [u8; 0x10],
    chr_behavior_debug_anim_helper: usize,
    unk17b8: [u8; 0x10],
    pub animation_speed: f32,
    unk17cc: [u8; 0x1f4],
}

#[repr(C)]
pub struct CSChrEventModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: [u8; 0x8],
    /// Id of override animation that should be played on next frame.
    pub request_animation_id: i32,
    /// ID of default idle animation.
    pub idle_anim_id: i32,
    unk20: i32,
    unk24: u32,
    pub ez_state_request_ladder: i32,
    unk2c: [u8; 0xB],
    pub msg_map_list_call: i32,
    unk3c: u32,
    pub flags: u8, // bit in pos 1 is iframes
    unk41: [u8; 0xA],
    pub ez_state_request_ladder_output: i32,
    unk50: [u8; 0x27],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrSuperArmorModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    /// Current super armor of the character, related to poise.
    pub sa_durability: f32,
    /// Maximum super armor of the character.
    pub sa_durability_max: f32,
    unk18: u32,
    /// Time to lost super armor reset.
    pub recover_time: f32,
    unk20: u8,
    unk21: u8,
    /// Set by TAE Event 0 ChrActionFlag (action 71 POISE_BREAK_UNRECOVERABLE)
    pub poise_broken_state: bool,
    unk23: u8,
    unk24: u32,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrToughnessModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    /// Current toughness of the character, related to stance break
    pub toughness: f32,
    toughness_unk: f32,
    /// Maximum toughness of the character
    pub toughness_max: f32,
    /// Time to lost toughness reset.
    pub recover_time: f32,
    unk20: [u8; 0x108],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrDataModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub msb_parts: CSMsbPartsEne,
    msb_res_cap: usize,
    unk68: usize,
    unk70: u32,
    unk74: u32,
    unk78: u32,
    pub map_id_origin: u32,
    unk80: u32,
    unk84: u32,
    pub world_block_chr: NonNull<WorldBlockChr<ChrIns>>,
    unk90: [u8; 0x30],
    pub draw_params: u32,
    pub chara_init_param_id: u32,
    // wchar_t[6]
    unkc8: [u8; 0xc],
    unkd4: [u8; 0x64],
    pub hp: i32,
    pub max_hp: i32,
    pub max_uncapped_hp: i32,
    pub base_hp: i32,
    pub fp: i32,
    pub max_fp: i32,
    pub base_fp: i32,
    pub stamina: i32,
    pub max_stamina: i32,
    pub base_stamina: i32,
    recoverable_hp_1: f32,
    recoverable_hp_2: f32,
    pub recoverable_hp_time: f32,
    unk16c: f32,
    unk170: [u8; 0x28],
    unk198: [u8; 0x3],
    // 2nd bit makes you undamageable
    debug_flags: u8,
    unk19c: [u8; 0x8c],
    /// Name for character behavior.
    /// c0000 for player-like characters
    pub character_behavior_name: DLString,
    dl_string: [u8; 0x30],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSPairAnimNode {
    vftable: usize,
    unk8: usize,
    pub owner: OwnedPtr<ChrIns>,
    pub forwarding_recipient: FieldInsHandle,
    unk20: FSVector4,
    unk30: FSVector4,
    unk40: u32,
    unk44: [u8; 0xc],
}

#[repr(u32)]
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

#[repr(C)]
/// Source of name: RTTI
pub struct CSThrowNode {
    pub super_pair_anim_node: CSPairAnimNode,
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

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ThrowModuleFlags(u32);
    impl Debug;
    /// Set by TAE Event 0 ChrActionFlag (action 70 THROW_ESCAPE_TRANSITION_ATTACKER)
    pub escape_transition, set_escape_transition: 0;
    /// Set by TAE Event 0 ChrActionFlag (action 69 THROW_DEATH_TRANSITION_DEFENDER)
    pub death_transition, set_death_transition:   1;
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
    unk100: FSVector4,
    unk110: FSVector4,
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
    pub lock_on_chr_tag_dmypoly_offset: FSVector4,
    /// Stores the model matrix derived from `CSChrPhysicsModule::ConstructModelMatrix`.
    /// Constructed from `CSChrPhysicsModule::position` and `CSChrPhysicsModule::orientation`.
    pub physics_model_matrix: FSMatrix4x4,
    /// Stores the `raw_physics_model_matrix` multiplied by itself.
    pub physics_transform_matrix_squared: FSMatrix4x4,
    /// The primary model matrix for the character.
    /// It's initially constructed by combining:
    /// - Translation from `raw_physics_model_matrix` and `vertical_position_offset`.
    /// - Orientation from `raw_physics_model_matrix` combined with `additional_orientation_quat`.
    /// - Scaling from `scale_size_x`, `scale_size_y`, and `scale_size_z`.
    ///
    /// This matrix is then processed by ChrEasingModule,
    /// and the eased result is stored back into this field. It's the final matrix
    /// propagated to components like `locationMtx44ChrEntity`.
    pub model_matrix: FSMatrix4x4,
    /// Stores the `model_matrix` multiplied by itself after all modifications.
    pub model_matrix_squared: FSMatrix4x4,
    unk2b0: FSVector4,
    /// An additional orientation (quaternion) that is multiplied with the orientation
    /// derived from the `raw_physics_model_matrix` to produce the final orientation
    /// for the `model_matrix`.
    pub additional_orientation_quat: FSVector4,
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
    unk350: FSVector4,
    unk360: FSVector4,
    unk370: [u8; 0x10],
    unk380: FSVector4,
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
/// Source of name: RTTI
pub struct CSModelIns {
    vftable: usize,
    unk8: usize,
    pub model_item: usize,
    pub model_disp_entity: usize,
    pub location_entity: usize,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrModelIns {
    pub model_ins: CSModelIns,
}

#[repr(C)]
/// Source of name: RTTI
pub struct PlayerIns {
    pub chr_ins: ChrIns,
    pub player_game_data: OwnedPtr<PlayerGameData>,
    chr_manipulator: usize,
    unk590: usize,
    pub player_session_holder: PlayerSessionHolder,
    unk5c0: usize,
    replay_recorder: usize,
    unk5d0: u32,
    unk5d4: u32,
    pub snipe_mode_draw_alpha_fade_timer: f32,
    unk5bc: u32,
    unk5e0: usize,
    fg_model: usize,
    npc_param: usize,
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
    unk67c: [u8; 0x24],
    pub player_menu_ctrl: NonNull<CSPlayerMenuCtrl>,
    unk6b0: [u8; 0x8],
    pub locked_on_enemy: FieldInsHandle,
    pub session_manager_player_entry: OwnedPtr<SessionManagerPlayerEntryBase>,
    /// Position within the current block.
    pub block_position: BlockPosition,
    /// Angle as radians. Relative to the orientation of the current block.
    pub block_orientation: f32,
    unk6d4: [u8; 0x1c],
    unk6f0: usize,
    unk6f8: [u8; 0xb],
    unk703: bool,
    pub quickmatch_is_stalemate: bool,
    unk705: bool,
    unk706: u8,
    unk707: u8,
    pub opacity_keyframes_timer: FD4Time,
    /// When false, chr team type is 14 (Neutral) and chr is an NPC
    /// Will decrease `opacity_keyframes_timer` and set `ChrIns.opacity_keyframes_multiplier` to 0
    pub enable_neutral_npc_rendering: bool,
    unk718: [u8; 0x27],
}

impl AsRef<ChrIns> for PlayerIns {
    fn as_ref(&self) -> &ChrIns {
        &self.chr_ins
    }
}

#[repr(C)]
/// Source of name: RTTI
pub struct EnemyIns {
    pub chr_ins: ChrIns,
    pub com_manipulator: usize,
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
/// Role of character in PvP/PvE.
/// Changes a lot of things, like appearance, what items you can use, etc.
pub enum ChrType {
    None = -1,
    Local = 0,
    WhitePhantom = 1,
    BlackPhantom = 2,
    Ghost = 3,
    Ghost1 = 4,
    Npc = 5,
    GrayPhantom = 8,
    Arena = 13,
    Quickmatch = 14,
    Invader = 15,
    Invader2 = 16,
    BluePhantom = 17,
    Invader3 = 18,
}
