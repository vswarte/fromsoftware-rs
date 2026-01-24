use std::ptr::NonNull;
use bitfield::bitfield;

use shared::{F32Vector4, OwnedPtr};

use crate::{
    DoublyLinkedList, Tree, cs::{
        CSAiTargetingSystemOwner, CSChrInsHandleTargetAccessor, CSComThinkOwner, CSFixedPosTarget,
        CSRelativePosTarget, CSTargetVelocityRecorder, CSTargetingSystemBase, ChrIns, ChrSetEntry,
        GoalIns,
    }, dlut::DLFixedVector, fd4::FD4Time, param::{NPC_PARAM_ST, NPC_THINK_PARAM_ST}, position::HavokPosition
};

/// Source of name: RTTI
#[repr(C)]
pub struct CSAiFunc {
    vftable: isize,
    pub ai_ins: NonNull<AiIns>,
}

/// Source of name: RTTI
#[repr(C)]
pub struct AiIns {
    pub com_think_owner: Option<NonNull<CSComThinkOwner>>,
    pub ai_func: Option<NonNull<CSAiFunc>>,
    // WorldBlockInfo->0x30?
    unk10: i32,
    // WorldBlockInfo->0x34?
    unk14: i32,
    pub npc_param: NpcParamLookupResult,
    pub npc_think_param: NpcThinkParamLookupResult,
    pub goal: AiGoal,
    pub interrupts: AiInterruptFlags,
    unk80: u64,
    unk88: u32,

    // Some spooky lua shit
    // Seemingly storing some VM state to expose it to the hardcoded-end or perhaps the other way
    // around?
    pub lua_timers: [f32; 16],
    pub lua_id_timers: [AiIdTimer; 128],
    pub lua_numbers: [f32; 64],
    pub lua_string_indexed_numbers: [LuaStringIndexedNumber; 256],
    pub lua_string_indexed_number_count: u32,
    pub lua_string_indexed_number_arrays: LuaStringIndexedNumberArrays,

    pub force_battle_goal: bool,
    pub executing_attack_in_attack_goal: bool,
    pub want_to_move_to: HavokPosition,
    unkc3f0: F32Vector4,
    pub general_motion_multiplier: f32,
    pub motion_multiplier: F32Vector4,
    pub passive_move_modifier: f32,
    /// What walking type the AI is using.
    /// 0 = Standing still.
    /// 1 = Walking.
    /// 2 = Running.
    pub walk_type: i32,
    unkc428: u32,
    pub is_dashing: bool,
    pub fly_state: i32,
    pub action_request: AiActionRequest,
    unkc458: bool,
    pub is_in_attack_goal: bool,
    pub event_request: AiEventRequest,
    pub damage_last_frame: f32,
    pub touching_breakable_defense: i32,
    pub is_touching_breakable: i32,
    pub targeting: AiTargeting,
    unkc628: [u8; 0x310],
    pub targeting_system_owner: CSAiTargetingSystemOwner,
    pub targeting_system: NonNull<CSTargetingSystemBase>,
    pub self_target_accessor: SelfTarget,
    pub host_target_accessor: SelfTarget,
    pub target_velocity: AiTargetVelocity,
    pub fixed_pos_target_1: CSFixedPosTarget,
    pub chr_ins_handle_target_accessor: CSChrInsHandleTargetAccessor,
    unkd980: AiInsUnkd980,
    pub relative_pos_target: CSRelativePosTarget,
    pub is_in_battle: bool,
    unkd9c1: bool,
    pub path: OwnedPtr<AiPath>,
    pub has_new_path_data: bool,
    pub follow_path: AiFollowPath,
    pub is_on_ladder: bool,
    pub parallel_move: bool,
    pub mesh: AiMesh,
    pub back_to_home: AiBackToHome,
    pub turn_target: AiTargetPointType,
    pub emergency_turn: bool,
    unkdac0: AiDAC0,
    unkdb40: u8,
    pub fixed_pos_target_2: CSFixedPosTarget,
    pub team: AiTeam,
    self_ptr: Option<NonNull<NonNull<AiIns>>>,
    pub area_observer: AiAreaObserver,
    pub special_effect_observer: SpecialEffectObserver,
    pub fixed_pos_target_3: CSFixedPosTarget,
    unkdc50: i32,
    unkdc58: u64,
    pub ladder: AiLadder,

    // unkc404: [u8; 0x2574],
    // /// When true character will move left and right exclusively.
    // pub move_lr_only: bool,
    // pub move_backwards_only: bool,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AiDirectionType {
    Center = 0,
    F = 1,
    B = 2,
    L = 3,
    R = 4,
    ToF = 5,
    ToB = 6,
    ToL = 7,
    ToR = 8,
    Top = 9,
    FL = 10,
    FR = 11,
    BL = 12,
    BR = 13,
    ToFL = 14,
    ToFR = 15,
    ToBL = 16,
    ToBR = 17,
}

#[repr(C)]
pub struct AiLadder {
    pub owner: NonNull<AiIns>,
    pub npc_think_param: NonNull<NpcThinkParamLookupResult>,
    unk10: u8,
    unk18: u64,
    unk20: f32,
    unk24: f32,
    unk28: u32,
    unk2c: u8,
    unk30: u32,
    unk34: u32,
}

#[repr(C)]
/// Source of name: RTTI
pub struct SpecialEffectObserver {
    pub owner: NonNull<AiIns>,
    pub entries: DoublyLinkedList<SpecialEffectObserverNode>,
}

pub struct SpecialEffectObserverNode {
    pub owner: NonNull<AiIns>,
    pub target: i32,
    pub sp_effect_id: i32,
    unk10: i32,
    pub observed_status: i32,
}

#[repr(C)]
pub struct AiAreaObserver {
    pub owner: NonNull<AiIns>,
    pub entries: DoublyLinkedList<CSAiAreaObserveBase>,
    pub has_entered_an_area: bool,
    pub has_left_an_area: bool,
    unk22: u8,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CSAiAreaObserveType {
    Standard = 1,
    Custom = 2,
    Region = 3,
    DmyObserveSphere = 4,
}

#[repr(C)]
pub struct CSAiAreaObserveBase {
    vftable: usize,
    pub observe_slot: u8,
    /// 1 = standard, 2 = custom, 3 = region, 4 = dmy observe sphere
    pub observe_type: CSAiAreaObserveType,
    pub is_in_area: bool,
    pub was_in_area: bool,
}

#[repr(C)]
pub struct CSAiAreaObserve {
    pub base: CSAiAreaObserveBase,
    pub origin: AiTargetPointType,
    pub observee: AiTargetPointType,
    pub angle_start: AiDirectionType,
    pub angle_width: f32,
    pub distance: f32,
    unk2c: f32,
}

#[repr(C)]
pub struct AiTeam {
    pub owner: NonNull<AiIns>,
    team_ai: usize,
    unk10: [f32; 2],
    pub waiting_for_call_validation: bool,
    pub waiting_for_reply_validation: bool,
    unk20: u64,
    unk28: usize,
}

#[repr(C)]
pub struct AiDAC0 {
    pub owner: NonNull<AiIns>,
    unk8: AiDAC08,
    unk18: u64,
    unk20: u8,
    unk24: u32,
    pub fixed_pos_target: CSFixedPosTarget,
    unk70: u32,
    unk74: u8,
}

#[repr(C)]
pub struct AiDAC08 {
    unk0: u64,
    unk8: u32,
    unkc: u16,
}

bitfield! {
    pub struct AiInterruptFlags(u64);
    impl Debug;

    pub find_enemy_first_interrupt, set_find_enemy_first_interrupt: 0;
    pub find_attack, set_find_attack: 1;
    pub damaged, set_damaged: 2;
    pub damaged_stranger, set_damaged_stranger: 3;
    pub find_missile, set_find_missile: 4;
    pub success_guard, set_success_guard: 5;
    pub miss_swing, set_miss_swing: 6;
    pub guard_begin, set_guard_begin: 7;
    pub guard_finish, set_guard_finish: 8;
    pub guard_break, set_guard_break: 9;
    pub shoot, set_shoot: 10;
    pub shoot_ready, set_shoot_ready: 11;
    pub use_item, set_use_item: 12;
    pub enter_battle_area, set_enter_battle_area: 13;
    pub leave_battle_area, set_leave_battle_area: 14;
    pub cannot_move, set_cannot_move: 15;
    pub inside_observe_area, set_inside_observe_area: 16;
    pub rebound_by_opponent_guard, set_rebound_by_opponent_guard: 17;
    pub forget_target, set_forget_target: 18;
    pub friend_request_support, set_friend_request_support: 19;
    pub target_is_guard, set_target_is_guard: 20;
    pub hit_enemy_wall, set_hit_enemy_wall: 21;
    pub success_parry, set_success_parry: 22;
    pub cannot_move_disable_interrupt, set_cannot_move_disable_interrupt: 23;
    pub parry_timing, set_parry_timing: 24;
    pub ride_node_ladder_bottom, set_ride_node_ladder_bottom: 25;
    pub ride_node_door, set_ride_node_door: 26;
    pub straight_by_path, set_straight_by_path: 27;
    pub changed_anim_id_offset, set_changed_anim_id_offset: 28;
    pub success_throw, set_success_throw: 29;
    pub looked_target, set_looked_target: 30;
    pub lose_sight_target, set_lose_sight_target: 31;
    pub ride_node_inside_wall, set_ride_node_inside_wall: 32;
    pub miss_swing_self, set_miss_swing_self: 33;
    pub guard_break_blow, set_guard_break_blow: 34;
    pub target_out_of_range, set_target_out_of_range: 35;
    pub unstable_floor, set_unstable_floor: 36;
    pub break_floor, set_break_floor: 37;
    pub break_observe_obj, set_break_observe_obj: 38;
    pub event_request, set_event_request: 39;
    pub outside_observe_area, set_outside_observe_area: 40;
    pub target_out_of_angle, set_target_out_of_angle: 41;
    pub platoon_ai_order, set_platoon_ai_order: 42;
    pub activate_special_effect, set_activate_special_effect: 43;
    pub deactivate_special_effect, set_deactivate_special_effect: 44;
    pub moved_end_on_failed_path, set_moved_end_on_failed_path: 45;
    pub change_sound_target, set_change_sound_target: 46;
    pub on_create_damage, set_on_create_damage: 47;
    pub invade_trigger_region, set_invade_trigger_region: 48;
    pub leave_trigger_region, set_leave_trigger_region: 49;
    pub ai_guard_broken, set_ai_guard_broken: 50;
    pub ai_rebound_by_opponent_guard, set_ai_rebound_by_opponent_guard: 51;
    pub backstab_risk, set_backstab_risk: 52;
    pub ladder_wait, set_ladder_wait: 53;
    pub ai_jump, set_ai_jump: 54;
    pub find_unfavorable_failed_point, set_find_unfavorable_failed_point: 55;
    pub unfavorable_attack_last_interrupt, set_unfavorable_attack_last_interrupt: 56;
}

#[repr(C)]
pub struct AiPath {
    pub unk0: HavokPosition,
    pub unk10: HavokPosition,
    pub follow_path_1: AiFollowPath,
    pub follow_path_2: AiFollowPath,
    pub pathing_result: i32,
    unk74: u32,
    unk78: u64,
    pub cannot_move: bool,
    unk90: AiPathUnkStruct,
    unkb0: AiPathUnkStruct,
    unkd0: AiPathUnkStruct,
    unkf0: u32,
    unkf4: f32,
    unkf8: u64,
    unk100: u8,
    unk101: u8,
    // TODO: check
    pub is_not_on_ladder: bool,
    unk103: bool,
    pub use_path: bool,
    unk108: usize,
    pub owner: NonNull<AiIns>,
    unk118: usize,
    unk120: F32Vector4,
    unk130: usize,
    unk138: usize,
    unk140: u8,
}

#[repr(C)]
pub struct AiPathUnkStruct {
    unk0: F32Vector4,
    unk10: u32,
    unk14: u32,
    unk18: u32,
    unk1c: u8,
}

#[repr(C)]
pub struct AiBackToHome {
    pub ai_ins: NonNull<AiIns>,
    unk8: u32,
    unk10: FD4Time,
    unk20: bool,
    unk24: u32,
    unk28: u64,
    unk30: u8,
    unk34: u32,
    unk38: u8,
    unk3c: u32,
}

#[repr(C)]
pub struct AiMesh {
    pub ai_ins: NonNull<AiIns>,
    pub normal_direction: F32Vector4,
    pub target_position: F32Vector4,
    pub starting_position: F32Vector4,
    pub ending_position: F32Vector4,
    pub line_thickness: f32,
}

#[repr(C)]
pub struct AiFollowPath {
    pub ai_ins: Option<NonNull<AiIns>>,
    pub target: AiTargetPointType,
    pub orientation_from_target: AiAngleBundle,
    pub stop_distance: f32,
    pub directional_distance: f32,
    pub hit_radius: f32,
    pub xz_distance_only: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct AiAngleBundle {
    pub angle: f32,
    pub angle_type: AiAngleType,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AiAngleType {
    Radians = 0x1,
    Degrees = 0x3,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AiTargetPointType {
    None = -2,
    SelfTarget = -1,

    Enemy0 = 0,
    Friend0 = 10,
    Event = 20,
    LocalPlayer = 21,
    LowHpFriend = 22,
    IntermediatePoint = 23,
    TeamFormation = 24,
    TeamLeader = 25,
    MemorizedRelativeTarget = 26,

    EnemyAvatarHome = 50,
    PersonalAvatarHome = 51,
    BoidsCommand = 60,

    Ride0 = 70,

    Search = 80,
    Sound = 81,
    HostPlayer = 82,

    PointInitial = 100,
    PointSnipe = 101,
    PointEvent = 102,
    MovePoint = 103,
    NearNavimesh = 104,
    FarNavigate = 105,
    NearNavigate = 106,
    AiFixedPosition = 107,
    FarLanding = 108,
    NearLanding = 109,
    SecondNearLanding = 110,
    InitialPose = 111,
    HitObstacle = 112,
    CurrentRequestedPosition = 114,
    NearMovePoint = 115,
    NearObjectActionPoint = 116,
    SecondNearObjectActionPoint = 117,
    LastSightPosition = 118,
    NearCorpsePosition = 119,
    AutoWalkAroundTest = 120,
    WalkAroundHome = 121,
    UnstableFloorCausePosition = 122,
    WalkAroundFree = 123,
    AiPredictionTargetPosition = 124,
    HorseRouteTarget = 125,
    HorseRouteStartDirectionPoint = 126,
    FlyRouteCruiseBoundary = 129,
    AiJumpTargetPathMove = 130,
    WaitAndSee = 131,
    TeamDefeat = 132,
    StragglerAfterDefeat = 133,

    EventTarget0 = 1000,
    EventTarget1 = 1001,
    EventTarget2 = 1002,
    EventTarget3 = 1003,
    EventTarget4 = 1004,
    EventTarget5 = 1005,
    EventTarget6 = 1006,
    EventTarget7 = 1007,
    EventTarget8 = 1008,
    EventTarget9 = 1009,
    EventTarget10 = 1010,

    TeamMember1 = 1101,
    TeamMember2 = 1102,
    TeamMember3 = 1103,
    TeamMember4 = 1104,
    TeamMember5 = 1105,
    TeamMember6 = 1106,
    TeamMember7 = 1107,
    TeamMember8 = 1108,
    TeamMember9 = 1109,
    TeamMember10 = 1110,
    TeamMember11 = 1111,
    TeamMember12 = 1112,
    TeamMember13 = 1113,
    TeamMember14 = 1114,
    TeamMember15 = 1115,
    TeamMember16 = 1116,
    TeamMember17 = 1117,
    TeamMember18 = 1118,
    TeamMember19 = 1119,
    TeamMember20 = 1120,
    TeamMember21 = 1121,
    TeamMember22 = 1122,
    TeamMember23 = 1123,
    TeamMember24 = 1124,
    TeamMember25 = 1125,
    TeamMember26 = 1126,
    TeamMember27 = 1127,
    TeamMember28 = 1128,
    TeamMember29 = 1129,
    TeamMember30 = 1130,
    TeamMember31 = 1131,
}

#[repr(C)]
pub struct AiInsUnkd980 {
    pub ai_ins: NonNull<AiIns>,
    unk8: u16,
    unkc: u32,
}

#[repr(C)]
pub struct AiGoal {
    pub ai_ins: NonNull<AiIns>,
    pub npc_think_param: NonNull<NpcThinkParamLookupResult>,
    pub logic_id: i32,
    pub battle_goal_id: i32,
    // TODO: wtf is this?
    pub npc_attack_counter: u8,
    unk1c: i32,
    /// Storage for all the GoalInses associated with this AiIns.
    pub goals: NonNull<[GoalIns; 32]>,
    unk28: u64,
    /// Top goal is usually the common top goal (id 100) and spawns all other goals as children.
    pub top_goal: NonNull<GoalIns>,
}

#[repr(C)]
pub struct AiActionRequest {
    ai_ins: NonNull<AiIns>,
    pub ez_action_id: i32,
    unkc: bool,
    pub is_request: bool,
    pub is_finished: bool,
    unk10: i32,
    pub request_ez_action_id_1: i32,
    pub request_ez_action_id_2: i32,
}

#[repr(C)]
pub struct AiEventRequest {
    requests: [f32; 4],
    unk10: i32,
    received_event_requests: u32,
}

#[repr(C)]
pub struct AiTargeting {
    targeting_system: NonNull<CSTargetingSystemBase>,
    unk8: [u8; 0x1a0],
}

#[repr(C)]
pub struct NpcParamLookupResult {
    pub param_row: Option<NonNull<NPC_PARAM_ST>>,
    pub row_id: u32,
    unkc: i32,
}

#[repr(C)]
pub struct NpcThinkParamLookupResult {
    pub row_id: u32,
    pub param_row: Option<NonNull<NPC_THINK_PARAM_ST>>,
    pub battle_goal_id: i32,
    pub logic_id: i32,
}

#[repr(C)]
pub struct AiIdTimer {
    pub id: i32,
    pub start_offset: f32,
    pub elapsed: f32,
}

#[repr(C)]
pub struct LuaStringIndexedNumber {
    pub hash: i32,
    pub value: f32,
    pub key: [u16; 64],
}

#[repr(C)]
pub struct LuaStringIndexedNumberArrays {
    pub descriptors: [LuaStringIndexedNumberArrayDescriptor; 64],
    pub items: [f32; 1024],
    pub descriptor_count: u32,
    pub total_item_count: u32,
}

#[repr(C)]
pub struct LuaStringIndexedNumberArrayDescriptor {
    pub index: i32,
    pub hash: i32,
    pub ptr: NonNull<f32>,
    pub key: [u16; 64],
}

#[repr(C)]
pub struct AiTargetVelocity {
    ai_ins: NonNull<AiIns>,
    unk8: u64,
    pub target_velocity_recorder: CSTargetVelocityRecorder,
}

#[repr(C)]
pub struct SelfTarget {
    vtable: isize,
    pub chr_set_entry: Option<NonNull<ChrSetEntry<ChrIns>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<CSAiFunc>(), 0x10);
    }
}
