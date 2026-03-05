#![allow(non_camel_case_types)]
use std::ptr::NonNull;

use bitfield::bitfield;
use shared::{F32Vector3, OwnedPtr, Subclass, Superclass, singleton};
use vtable_rs::VPtr;

use crate::{
    DoublyLinkedList,
    cs::{BlockId, CSEzTask, CSEzVoidTask, DeathState, FullScreenMessage, SummonParamType},
    dlkr::DLAllocatorRef,
    dlrf::DLRuntimeClassImpl,
    dltx::{DLShiftJisStringKind, DLString, DLUTF16StringKind},
    fd4::FD4Time,
};
use num_enum::TryFromPrimitive;

#[repr(C)]
#[singleton("CSLuaEventMan")]
pub struct CSLuaEventManImp {
    pub lua_event_observer: OwnedPtr<CSLuaEventObserver>,
    pub lua_event_proxy: OwnedPtr<CSLuaEventProxy>,
    pub lua_event_script_imitation_class: NonNull<DLRuntimeClassImpl>,
    pub lua_event_script_imitation: Option<OwnedPtr<CSLuaEventScriptImitation>>,
    unk20: i32,
    unk24: i32,
    unk28: [u8; 0x18],
}

#[repr(C)]
pub struct CSScriptCallParam {
    pub is_net_message: bool,
    pub actor_character_event_id: i32,
    pub actor_steam_id: u64,
    pub event_id: u32,
    pub var1: i32,
    pub var2: i32,
}

#[repr(C)]
#[derive(Superclass)]
#[superclass(children(CSLuaEventMsgExec_Func, CSLuaEventMsgExec_String))]
pub struct CSLuaEventMsgExec {
    pub vftable: VPtr<dyn CSLuaEventMsgExecVmt, Self>,
}

#[vtable_rs::vtable]
pub trait CSLuaEventMsgExecVmt {
    fn destructor(&mut self, flags: u32) -> *mut CSLuaEventMsgExec;
    fn execute(&mut self, proxy: &mut CSLuaEventProxy, param: &CSScriptCallParam);
}

#[repr(C)]
#[derive(Subclass)]
pub struct CSLuaEventMsgExec_Func {
    pub base: CSLuaEventMsgExec,
    /// `std::function(CSLuaEventScriptImitation*)(CSLuaEventProxy*, CSScriptCallParam*)`
    func: [u8; 0x40],
}

#[repr(C)]
#[derive(Subclass)]
pub struct CSLuaEventMsgExec_String {
    pub base: CSLuaEventMsgExec,
    pub lua_script_imitation_class: NonNull<CSLuaEventScriptImitation>,
    pub event_msg: DLString<DLShiftJisStringKind>,
}

#[repr(C)]
pub struct EventMsgExecListEntry {
    pub event_group: u32,
    /// First argument to execute function.
    /// Usually it's just [LuaEventId] but could also be a event flag in case of `OnEvent_Bonfire`
    pub arg1: u32,
    /// Second argument to execute function.
    /// Used to pass bonfire entity id in case of `OnEvent_Bonfire`
    pub arg2: u32,
    /// Third argument to execute function.
    /// Usually unused and set to 0.
    pub arg3: u32,
    /// How repetition of the event execution should be handled
    pub repetition: LuaScriptExecuteRepetition,
    /// Whether this event is from a network message
    pub is_net_message: bool,
    pub is_repeat_message: bool,
    /// Whether this function is marked for deletion and
    /// should be removed on next game loop update.
    pub is_deleted: bool,
    pub lua_event_msg_exec: OwnedPtr<CSLuaEventMsgExec>,
}

#[repr(C)]
pub struct CSLuaEventMsgMap {
    vftable: usize,
    pub event_msg_exec_list: DoublyLinkedList<OwnedPtr<EventMsgExecListEntry>>,
    /// Entries that were executed but retained to prevent immediate re-execution
    /// - The list is checked before scheduling to reject duplicates.
    /// - After execution, entries with [`EventMsgExecListEntry::repetition`] [`LuaScriptExecuteRepetition::Once`] are either deleted or moved
    ///   here depending on event id and [`EventMsgExecListEntry.is_repeat_message`].
    ///
    /// [`EventMsgExecListEntry.is_repeat_message`]: EventMsgExecListEntry::is_repeat_message
    pub deferred_event_exec_list: DoublyLinkedList<OwnedPtr<EventMsgExecListEntry>>,
    pub lua_script_imitation_class: NonNull<DLRuntimeClassImpl>,
}

#[repr(C)]
pub struct CSLuaEventScriptImitation {
    vftable: usize,
    /// Id of the boss that was just killed.
    /// 0 if not applicable.
    pub clear_boss_id: u32,
    pub clear_boss: bool,
    /// Whether current player killed the host or not.
    pub is_kill_host: bool,
    /// Special death conditions based on the items the player has equipped.
    pub death_state: DeathState,
    /// Whether the player has any of the items equipped that cause death penalty to be skipped or not.
    pub is_death_penalty_skip: bool,
    pub is_wait_reentry_to_map: bool,
    /// Whether to disable the map enter animation when
    /// loading into the map.
    /// Copied from [GameMan::disable_map_enter_anim](crate::cs::GameMan::disable_map_enter_anim).
    pub disable_map_enter_anim: bool,
    pub remo_flag: u32,
    pub lua_warp_bonfire_entity_id: i32,
    /// Offset that will be added to bonfire animation ids
    /// eg. to play different animation for the roundtable grace.
    pub bonfire_animation_id_offset: u32,
    pub bonfire_entity_id: u32,
    pub should_reset_world: bool,
    pub should_reset_character: bool,
    pub should_reset_magic_charges: bool,
    pub should_restore_estus: bool,
    pub world_reset_delay: f32,
    unk30: i32,
    unk34: F32Vector3,
    unk40: f32,
    unk44: bool,
    unk45: bool,
    pub leave_event: CSLeaveEvent,
    pub player_kill_event: CSPlayerKillEvent,
    pub death_restart_event: CSDeathRestartEvent,
}

#[repr(C)]
pub struct CSLeaveEvent {
    pub owner: NonNull<CSLuaEventScriptImitation>,
}

#[repr(C)]
pub struct CSPlayerKillEvent {
    pub owner: NonNull<CSLuaEventScriptImitation>,
}

#[repr(C)]
pub struct CSDeathRestartEvent {
    pub owner: NonNull<CSLuaEventScriptImitation>,
    pub full_screen_message_id: FullScreenMessage,
    pub character_event_id: i32,
    unk10: i32,
    pub is_death_penalty_skip: bool,
}

#[repr(C)]
pub struct CSLuaEventProxy {
    vftable: usize,
    pub lua_event_script_imitation_class: NonNull<DLRuntimeClassImpl>,
    unk10: DLAllocatorRef,
    unk18: [u8; 0x18],
    pub lua_event_msg_map: CSLuaEventMsgMap,
    /// Dummy field for the packet broadcast
    pub packet_send_canary: u8,
    unk71: u8,
    pub is_net_message: bool,
    pub disable_event_networking: bool,
    pub is_repeat_message: bool,
    pub is_load_wait: bool,
    unk78: FD4Time,
    pub control_flags: LuaEventControlFlags,
    pub summon_param_type: SummonParamType,
    pub is_lobby_state_client: bool,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    /// Lua event flags to control certain event behaviors.
    ///
    /// 40xx event flags in old `global_event.lua` scripts (DES, DS1)
    pub struct LuaEventControlFlags(u32);
    impl Debug;
    /// 4047 再読み込み関連イベントの割り込み防止フラグ
    ///
    /// English: Flag to prevent interruption of reload-related events
    pub pause_reload_events, set_pause_reload_events: 0;
    /// 4000 自分の死亡イベント
    ///
    /// English: Self death event
    pub pause_self_death_event, set_pause_self_death_event: 1;
    /// 4093 (not present in DES, so no comment there)
    ///
    /// Pauses various `OnDisconnect` events handling
    pub pause_disconnect_event, set_pause_disconnect_event: 2;
    /// 4092 (not present in DES, so no comment there)
    ///
    /// Flag to indicate whether the player was alive at the time of block clear
    /// so `SoloBlockClear` can clear [`pause_reload_events`] and [`pause_self_death_event`]
    ///
    /// [`pause_reload_events`]: LuaEventControlFlags::pause_reload_events
    /// [`pause_self_death_event`]: LuaEventControlFlags::pause_self_death_event
    pub was_alive_at_block_clear, set_was_alive_at_block_clear: 3;
    /// Set when current player is a red hunter in someone else's world.
    /// Makes the character able to receive rewards for killing players based on their role param.
    pub red_hunt_active, set_red_hunt_active: 4;
    /// 4079 (not present in DES, so no comment there)
    ///
    /// - Cleared at start of `Lua_BonfireLoopAnimBegin`
    /// - Set in `Lua_BonfireLoopAnimBegin` after playing sit-down anim
    /// - Signals sit-down animation has started
    pub bonfire_loop_begin_requested, set_bonfire_loop_begin_requested: 5;
    /// 4083 (not present in DES, so no comment there)
    ///
    /// - Checked in `Lua_BonfireLoopAnimBegin_1` - if set, skip loop and go to stand up
    /// - Set in `Lua_BonfireLoopAnimEnd` when stand up not yet allowed
    /// - Signals player requested exit but loop animation wasn't running yet
    pub bonfire_end_pending, set_bonfire_end_pending: 6;
    /// 4084 (not present in DES, so no comment there)
    ///
    /// - Set in `Lua_BonfireLoopAnimBegin_1` before playing loop anim
    /// - Checked in `Lua_BonfireLoopAnimEnd` - if set, proceed to stand up; if not set, set bit 6
    /// - Confirms loop animation is actively playing
    pub bonfire_sitting_loop_active, set_bonfire_sitting_loop_active: 7;
    /// 4079 with 4083 (when both are true) (not present in DES, so no comment there)
    ///
    /// - Set in `Lua_BonfireLoopAnimEnd` when bit 7 is set, before playing stand up anim
    /// - Checked in `Lua_BonfireLoopAnimBegin_1` at start - if set, early return (prevents re-entry)
    /// - Cleared in `Lua_BonfireLoopAnimEnd` after stand up starts
    pub bonfire_stand_up_in_progress, set_bonfire_stand_up_in_progress: 8;
    /// 4044 キックアウトしたのを通知
    ///
    /// English: Notify that someone was kicked out
    pub pause_player_leave_event, set_pause_player_leave_event: 9;
    /// (Not present in DES/DS1 lua scripts)
    ///
    /// Signals that the player was notified of block clear,
    /// so when the hosts disconnects, game shows "The Host of Fingers accomplished their objective in a distant location."
    /// instead of generic connection lost message.
    pub notified_of_block_clear, set_notified_of_block_clear: 10;
    /// (Not present in DES/DS1 lua scripts)
    ///
    ///  Set by `RegistReturnTitle`, prevents `CS::CSWorldTalkManImpl::Update` from running
    pub return_title_requested, set_return_title_requested: 11;
    /// (Not present in DES/DS1 lua scripts)
    ///
    /// Set by `STEP_WaitDialogOk` when in arena and player is dead
    /// forces BattleRoyale end-finalization (`SendQMResultsStats`, `InvokeLocalEvent`) instead of
    /// early-return on certain quickmatch states.
    pub arena_local_player_dead, set_arena_local_player_dead: 12;
    /// (Not present in DES/DS1 lua scripts)
    ///
    /// Set by `SoloPlayDeath_Arena` when not in duel, indicating that
    /// respawn should happen without returning to title and with arena-specific death restart behavior.
    pub arena_death_restart_flag, set_arena_death_restart_flag: 13;
    /// If set, forces kickout at the end of death restart in arena.
    pub arena_death_restart_kickout, set_arena_death_restart_kickout: 14;
    /// (Not present in DES/DS1 lua scripts)
    ///
    /// Set by `SoloPlayDeath_Arena`, prevents some of multiplayer connection lost handling until
    /// death restart ends.
    pub arena_death_restart_pending, set_arena_death_restart_pending: 15;
    /// (Not present in DES/DS1 lua scripts)
    ///
    /// Set by `StartCeremonyRestartWait` when in ceremony and it's ended.
    pub ceremony_restart_pending, set_ceremony_restart_pending: 16;
}

#[repr(C)]
pub struct CSLuaEventObserver {
    vftable: usize,
    pub lua_event_observees: DoublyLinkedList<OwnedPtr<CSLuaEventCondition>>,
    pub bonfire_event_observees: DoublyLinkedList<OwnedPtr<CSLuaEventCondition>>,
    pub bonfire_near_enemy_update_task: CSEzVoidTask<CSEzTask, Self>,
    unk60: i32,
}

#[repr(C)]
pub struct CSLuaEventCondition {
    pub vftable: VPtr<dyn CSLuaEventConditionVmt, Self>,
    pub owner: NonNull<CSLuaEventObserver>,
    /// Id of the event condition, used to identify the event in Lua scripts.
    /// Usually it's just [LuaEventId] but could also be a event flag in case of `CSLuaEventConditionBonfire`
    /// or "disable" event flag in case of `CSLuaEventConditionDistance` with the ladder distance check
    /// type.
    pub condition_id: i32,
    pub event_group: u32,
    pub stop_execution: bool,
    pub execute_repetition: LuaScriptExecuteRepetition,
    /// Whether this event condition is marked for deletion and
    /// should be removed on next game loop update.
    pub is_deleted: bool,
}

#[vtable_rs::vtable]
pub trait CSLuaEventConditionVmt {
    /// Called every time new block is loaded
    fn on_block_load(&mut self, block_id: &BlockId);
    /// Called every time block is unloaded
    fn on_block_unload(&mut self, block_id: &BlockId);
    fn destructor(&mut self, flags: u32) -> *mut CSLuaEventCondition;
    fn check_condition(&mut self, proxy: &mut CSLuaEventProxy, frame_delta: &FD4Time) -> bool;
    /// Called before update and [`check_condition`] but after invoking event handler for this condition
    /// when packet 14 is received.
    /// Returns whether the event was handled or not and execution of the event should be stopped.
    ///
    /// [`check_condition`]: CSLuaEventConditionVmt::check_condition
    fn on_remote_event(&mut self, event_group: u32, arg1: u32, arg2: u32, arg3: u32) -> bool;
    /// Returns whether the event condition matches the given condition id and group.
    /// For example, it is indirectly called by EMEVD `2009[3]` [`RegisterBonfire`] to check if bonfire with these ids alredy registered.
    ///
    /// [`RegisterBonfire`]: https://soulsmods.github.io/emedf/er-emedf.html#RegisterBonfire
    fn check_id_and_group(&mut self, condition_id: u32, event_group: u32) -> bool;
    /// Called on every game loop update for every event condition after `check_condition` is called for all event conditions.
    fn post_update(&mut self);
    /// Returns a string for debug menu depending on the display type.
    /// 0 will return "Condition:{condition_type}",
    /// 1-3 will return debug string for the arguments of this event condition.
    /// Eg. for `CSLuaEventConditionDistance` it will return `Condition:Distance`, `<%d> <PlayerEntityID`, `<%d> <TargetEntityID>`, `<%d> <ActionButtonParamID>` for display types 0-3 respectively with %d being replaced with actual argument values.
    fn debug_string(
        &mut self,
        string_out: *mut DLString<DLUTF16StringKind>,
        display_type: u8,
    ) -> Option<NonNull<DLString<DLUTF16StringKind>>>;
    /// Stripped debug function
    fn unk8(&mut self);
    /// Debug function to trigger the event, without checking any conditions. Returns whether the event was triggered or not.
    fn debug_trigger_condition(&mut self) -> bool;
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, TryFromPrimitive)]
pub enum LuaScriptExecuteRepetition {
    /// Make the event execute every time it is triggered.
    Everytime = 0,
    /// Make the event execute only once.
    Once = 1,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, TryFromPrimitive)]
pub enum LuaEventId {
    HostDead = 4000,
    OnEvent_4001 = 4001,
    OnEvent_4002 = 4002,
    OnEvent_4003 = 4003,
    OnEvent_4004 = 4004,
    OnEvent_4005 = 4005,
    DeathRestartEvent = 4006,
    InGameStart = 4013,
    SynchroAnim_4014 = 4014,
    g_Initialize = 4030,
    SelfBloodStain = 4032,
    SummonInfoMsg = 4041,
    DeadInfoMsg = 4042,
    /// Not present in ER as registered function, but in Sekiro
    OnLeavePlayer = 4043,
    OnKickOut = 4044,
    OnThxKickOut = 4045,
    SelfLeaveMessage = 4046,
    OnLeave_Limit_BattleRoyal_Wait = 4048,
    NotifyClientLeaveAroundHost = 4049,
    BlockClear2Start = 4050,
    BlockClear_LeaveAction2 = 4051,
    BlockClearSynchroAnime = 4052,
    BlockClearSynchroInvalid = 4053,
    NotifyGuestOfBlockClear = 4054,
    NotifyWarningClientLeaveAroundHost = 4056,
    NotifyClientLeavingAroundHost = 4057,
    Call_Sos = 4058,
    OnBeJoinStart = 4059,
    SummonTimeOut = 4060,
    FailedSummon = 4061,
    OnGameLeave = 4063,
    OnDisableInvincible = 4064,
    OnEnterRideObj = 4065,
    OnLeaveRideObj = 4066,
    OnEnableDraw = 4069,
    OnMatchingCheck = 4071,
    OnMatchingError = 4072,
    SelfHeroBloodStain = 4077,
    MediumBossDestroy = 4078,
    OnEvent_BonfireFirstLvUp_Client = 4079,
    OnEvent_BonfireFirstLvUp = 4080,
    OnEvent_BonfireLvUp = 4081,
    OnEvent_BonfireRespawn = 4082,
    Lua_Warp_1 = 4085,
    Lua_BonfireLoopAnimBegin = 4086,
    SummonMotion_DelayEvent = 4087,
    OnDeadEvent = 4090,
    ReportBossArea = 4110,
    DeportBlackGhost_InBossArea = 4111,
    ReportClearedBossArea = 4112,
    ReportHostLeaveArea = 4113,
    /// Only used as id of a "caller" function in undocumented EMEVD `2007[0]`
    EmevdMessage2007_0 = 4120,
    FinishDmyMulti = 4130,
    OnDead_ClienInCeremony = 4140,
    OnLeave_ClienInCeremony = 4150,
    SummonInfoMsg_Ceremony = 4160,
    OnGuestDead = 4200,
    OnEvent_LadderUp = 5000,
    /// Typo is from original string.
    OnEvent_LadderDawn = 5010,
    OnBattleRoyaleEnd = 5102,
    OnBerserkerEnd = 5103,
    OnBerserkerEndMsg = 5104,
    OnRedHunterEnd = 5110,
    OnRedHunterEndMsg = 5111,
    RedHuntDutyFulfilled_HostInBossArea = 5112,
    RedHuntDutyFulfilled = 5113,
    RedHuntDutyFulfilled_BlackDisconnect = 5114,
    RedHuntDeport = 5115,
    EnableVoiceChat = 5120,
    OnLeave = 5130,
    MissionFailed = 5131,
}
