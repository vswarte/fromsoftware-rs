use std::ptr::NonNull;

use crate::{
    DLVector,
    cs::{MultiplayRole, MultiplayType, PlayerGameData, SummonParamType},
    dltx::DLString,
    fd4::{FD4StepBase, FD4StepBaseInterface, FD4Time},
    from_net::{FNString, FNVector},
    position::BlockPosition,
    stl::DLList,
};
use shared::{OwnedPtr, StepperStates};

use super::{BlockId, CSEzTask, CSEzUpdateTask};

#[repr(C)]
#[shared::singleton("CSNetMan")]
pub struct CSNetMan {
    vftable: usize,
    pub nat_type: u32,
    pub session_nat_type: u32,
    pub disable_multiplay: bool,
    unk11: bool,
    unk12: bool,
    unk13: bool,
    unk14: bool,
    freeze_game: bool,
    unk16: bool,
    unk17: bool,
    // True if fps is low, prevents you from online play.
    pub low_fps_penalty: bool,
    pub server_connection_lost: bool,
    unk1a: bool,
    unk1b: u8,
    pub block_id: BlockId,
    unk20: BlockId,
    pub play_region_id: u32,
    unk28: [u8; 0x40],
    sos_db: usize,
    wandering_ghost_db: usize,
    /// Keeps track of all all bloodmessages in the world as well as any rating and created
    /// bloodmessages.
    pub blood_message_db: OwnedPtr<CSNetBloodMessageDb>,
    bloodstain_db: usize,
    bonfire_db: usize,
    spiritual_statue_db: usize,
    unk98: usize,
    unka0: usize,
    pub breakin_manager: OwnedPtr<BreakInManager>,
    /// Keeps track of quickmatch gamemode state.
    pub quickmatch_manager: OwnedPtr<QuickmatchManager>,
    visitor_db: usize,
    penalty_manager: usize,
    /// Task that updates the structure (pulls in new data from server, spawn received signs,
    /// stains and messages, spawns ghost replays, etc)
    pub update_task: CSEzUpdateTask<CSEzTask, Self>,
    unkf0: u8,
    unkf1: u8,
    unkf2: u8,
    /// Makes all ghosts, blood messages, and bloodstains appear as if they were created by someone
    /// with the same group password as the local player.
    pub debug_group_password: bool,
    unkf4: u32, // Probably padding
    unkf8: usize,
}

#[repr(C)]
pub struct CSNetBloodMessageDb {
    vftable: usize,
    // Contains all CSNetBloodMessageDbItem?
    pub entries: DLList<OwnedPtr<CSNetBloodMessageDbItem>>,
    unk20: usize,
    /// Seemingly contains message data for messages created by local user
    pub created_data: DLList<usize>,
    // Contains ???
    unk40: DLList<usize>,
    unk58: usize,
    blood_message_ins_man_1: usize,
    blood_message_ins_man_2: usize,
    pub discovered_messages: DLList<OwnedPtr<OwnedPtr<CSNetBloodMessageDbItem>>>,
    unk88: [u8; 0xD0],
    /// Hosts any ongoing jobs for evaluations.
    evaluate_job: usize,
    unk160: usize,
}

#[repr(C)]
pub struct CSNetBloodMessageDbItem {
    vftable: usize,
    unk8: u32,
    unkc: u32,
    unk10: u32,
    pub block_id: BlockId,
    unk18: u32,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub angle: f32,
    pub template1: u16,
    pub gesture_param: u16,
    pub part1: u16,
    pub infix: u16,
    pub template2: u16,
    pub part2: u16,
    unk38: u16,
    unk3a: u16,
    unk3c: u16,
    unk3e: u16,
    pub message_id: u64,
    unk48: u32,
}

#[repr(C)]
pub struct BreakInData {
    pub block_id: BlockId,
    pub block_pos: BlockPosition,
    pub entryfilelist_id: i32,
    pub summon_param_type: SummonParamType,
    pub multiplay_role: MultiplayRole,
    pub has_password: bool,
    unk1e: u8,
    pub join_data: FNVector<u8>,
}

#[repr(C)]
pub struct BreakInPointManager {
    breakin_points: DLList<()>,
    unk18: [u8; 0x10],
}

#[repr(C)]
pub struct BreakInAreaList {
    pub areas: DLVector<u32>,
    pub count: u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BreakInSearchState {
    Idle = 0,
    InSearch = 1,
    ConnectionAttempt = 2,
    CheckingResponse = 3,
}

#[repr(C)]
pub struct BreakInTarget {
    pub player_id: u32,
    pub external_id: FNString,
    pub play_region: u32,
}

#[repr(C)]
pub struct BreakInManager {
    pub multiplay_type: MultiplayType,
    pub targets: FNVector<BreakInTarget>,
    unk20: FNVector<()>,
    /// Data from breakin push
    pub data: BreakInData,
    pub point_manager: BreakInPointManager,
    rebreakin_pos_step: usize,
    pub error_code: i32,
    pub areas: BreakInAreaList,
    unkd0: usize,
    unkd8: usize,
    pub invasion_search_state: BreakInSearchState,
    pub last_update_invasion_search_state: BreakInSearchState,
    pub attempt_interval_timer: FD4Time,
    pub time_out_timer: FD4Time,
    pub is_yellow_costume_region: bool,
    pub is_multi_region: bool,
}

#[repr(C)]
pub struct QuickmatchManager {
    /// Stepper that updates the games quickmatch state.
    pub quickmatching_ctrl: OwnedPtr<CSQuickMatchingCtrl>,
    /// Keeps track of quickmatch settings as well as any participants.
    pub battle_royal_context: OwnedPtr<CSBattleRoyalContext>,
    /// Populated during creation of the QM lobby locally. Either by joining or creating a room.
    pub active_battle_royal_context: Option<NonNull<CSBattleRoyalContext>>,
    unk18: f32,
    /// List of speffects applied to the players during battle.
    /// Source of names: debug strings
    ///
    /// ```text
    /// 1110 Team A Summon/Respawn                            チームＡ用召喚・リスポン時
    /// 1111 Team B Summon/Respawn                            チームＢ用召喚・リスポン時
    /// 1130 Death                                            死亡時
    /// 1100 Kill                                             撃破時
    /// 1140 Crown for 1st Place                              一位時王冠
    /// 1150 Crown for Tied 1st Place                         同率一位時王冠
    /// 1160 Notification to remove 1st place special effects 一位時の特殊効果を消す通知用
    /// 1200 Heal when killing 1st place player               一位者殺害時回復
    /// 1300 Heal when killing tied 1st place player          同率一位者殺害時回復
    /// 1210 Heal when in 1st place                           一位時回復
    /// 1310 Heal when in tied 1st place                      同率一位時回復
    /// ```
    pub utility_sp_effects: [u32; 11],
    /// Skips the `LeaveMultiplayLog` server request when quickmatch ends.
    pub skip_leave_multiplay_log: bool,
    pub battle_session_data: QuickMatchBattleSessiontData,
    pub my_team_eliminations: u32,
    pub other_team_eliminations: u32,
    /// Set once battle_session_data/eliminations are computed, so a retried
    /// `CSQuickMatchRankingEndBattleJob` reuses the cached result instead of recomputing.
    pub results_computed: bool,
    unk5d: u8,
    pub character_id: u32,
    pub quickmatch_united_combat_rank: i32,
    pub quickmatch_duel_rank: i32,
    pub quickmatch_spirit_ashes_rank: i32,
    pub quickmatch_united_combat_points: i32,
    pub quickmatch_duel_points: i32,
    pub quickmatch_spirit_ashes_points: i32,
    unk80: usize,
    unk88: f32,
    pub debug_settings: QuickmatchManagerDebugSettings,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuickmatchResult {
    Win = 0,
    Lose = 1,
    Draw = 2,
    Error = 3,
}

#[repr(C)]
pub struct QuickMatchBattleSessiontData {
    pub result: QuickmatchResult,
    pub elimination_count: u8,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuickmatchDesiredTeam {
    None = 0,
    TeamA = 1,
    TeamB = 2,
    Any = 3,
}

#[repr(C)]
pub struct QuickmatchManagerDebugSettings {
    pub settings: QuickMatchSettings,
    pub venue: QuickMatchVenue,
    pub desired_team: QuickmatchDesiredTeam,
    pub password: DLString,
    unk40: u8,
}

/// State machine for [`CSQuickMatchingCtrl`]. Splits into a guest track and a host track
/// depending on [`QuickmatchSpawnData::role`].
///
/// Guest: `SearchRegister` -> `SearchRegisterWait` -> `GuestInviteWait` -> `GuestWaitSession` ->
/// `GuestReadyWait` -> `GuestMoveMap` -> `GuestInGame`. If the host rejects the join, falls back
/// to `SearchRegister` and retries after [`CSQuickMatchingCtrl::guest_research_retry_timer`].
///
/// Host: `SearchRegister` -> `SearchRegisterWait` -> `HostWaitSession` -> `HostInvite` ->
/// `HostReadyWait`/`HostReadyWaitBlockList` -> `HostMoveMap` -> `HostInGame`. `HostInvite`
/// accepts or rejects pending joins based on lobby capacity and the local block list.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, StepperStates)]
pub enum CSQuickMatchingCtrlState {
    /// Stepper is not running.
    NotExecuting = -1,
    /// No quickmatch is active.
    None = 0,
    /// Looking up existing rooms that match the quickmatch settings.
    SearchRegister = 1,
    /// Waiting for a response for the SearchRegister request.
    SearchRegisterWait = 2,
    /// Guest: waiting for the host to accept or reject a `JoinQuickMatch` request.
    GuestInviteWait = 3,
    GuestWaitSession = 4,
    GuestReadyWait = 5,
    GuestMoveMap = 6,
    /// People are loaded into the map and match is running.
    GuestInGame = 7,
    HostWaitSession = 8,
    /// Host: accepts or rejects pending join requests.
    HostInvite = 9,
    /// Host: waiting for participants to become ready.
    HostReadyWait = 10,
    /// Like `HostReadyWait`, but also handles allied-password team resolution.
    HostReadyWaitBlockList = 11,
    HostMoveMap = 12,
    /// People are loaded into the map and match is running.
    HostInGame = 13,
    /// Match has ended either by completion or error.
    Unregister = 14,
}

/// Source of name: RTTI
#[repr(C)]
pub struct CSQuickMatchingCtrl {
    pub stepper: FD4StepBase<Self, FD4StepBaseInterface, CSQuickMatchingCtrlState>,
    pub context: NonNull<CSBattleRoyalContext>,
    menu_job: usize,
    /// How long to wait before re-searching after a search cycle or a rejected join.
    /// Based on [`crate::param::NETWORK_PARAM_ST::guest_update_time`].
    pub guest_research_retry_timer: FD4Time,
    pub can_edit_settings: bool,
    /// Per-slot team assignment: indices 0-4 for team-match mode,
    /// indices 5-9 for brawl mode.
    pub quickmatch_team_types: [u8; 10],
    /// Set once every participant is ready.
    pub all_participants_ready: bool,
    /// True once the move map is requested, eather by everyone being ready to start the match or by
    /// error occuring.
    pub move_map_requested: bool,
    pub local_move_map_ready: bool,
    pub received_world_flag_sync: bool,
    /// Reset every update; gates a session state check to only happen once per update.
    pub checked_session_state_this_update: bool,
    pub join_retry_pending: bool,
    pub success_full_end: bool,
    pub should_start_join: bool,
    pub recompute_lead_requested: bool,
    pub sent_move_map_ready: bool,
    pub send_ranking_result: bool,
    pub death_unregister: bool,
    /// Forces [`CSQuickMatchingCtrlState::GuestInviteWait`] to bail back to
    /// [`CSQuickMatchingCtrlState::SearchRegisterWait`] instead of checking pending invites.
    pub pause_guest_invites: bool,
    pub sent_desired_team_packet: bool,
    /// Set once the host's allied-password team-stagger packet is received.
    pub received_allied_password_team_stagger_packet: bool,
    /// Password used for other team in allied-password mode.
    pub enemy_team_password: DLString,
    pub ally_team_elimination_count: u32,
    pub enemy_team_elimination_count: u32,
    /// Host's `UpdateQuickMatch` heartbeat interval.
    /// Based on [`crate::param::NETWORK_PARAM_ST::host_register_update_time`].
    pub host_registration_update_timer: FD4Time,
    /// Interval for the periodic "still searching" message.
    /// Based on [`crate::param::NETWORK_PARAM_ST::summon_message_interval`].
    pub summon_message_interval_timer: FD4Time,
    /// How long the host or guest waits for an accepted participant to finish loading before
    /// evicting them.
    /// Based on [`crate::param::NETWORK_PARAM_ST::host_player_no_time_out_time`] and
    /// [`crate::param::NETWORK_PARAM_ST::guest_player_no_time_out_time`].
    pub move_map_timeout_timer: FD4Time,
    /// Overall timeout for the whole attempt.
    /// Based on [`crate::param::NETWORK_PARAM_ST::quick_match_search_timeout`].
    pub wait_session_timeout_timer: FD4Time,
    pub allied_password_assembly_timeout_time: FD4Time,
    /// Allied-password mode only. Staggers when each teammate is allowed to re-register.
    /// Based on `player_index * 5` seconds.
    pub allied_password_team_stagger_timer: FD4Time,
    pub allied_password_joined_count_prev: u8,
    pub allied_password_joined_count: u8,
    pub enemy_password_joined_count: u8,
    pub active_state_elapsed_time: f32,
    pub join_target_host_external_id: FNString,
    /// Prevents the host from accepting new join requests.
    pub pause_accepting_join_requests: bool,
    pub sent_world_enter_packet: bool,
}

/// Source of name: RTTI
#[repr(C)]
pub struct CSBattleRoyalContext {
    pub quickmatch_context: CSQuickMatchContext,
    /// Required players to be in lobby before quickmatch can kick-off.
    pub match_player_count: u32,
    pub setting: QuickMatchSettings,
    /// Current number of players in the quickmatch lobby.
    pub current_player_count: u32,
    /// Selected arena enum.
    pub venue: QuickMatchVenue,
    /// Password used for the quickmatch lobby.
    pub password: DLString,
    /// Whether or not the quickmatch uses a fixed map instead of random.
    pub is_fixed_map: bool,
    /// Whether or not the quickmatch uses any format (duel, brawl, team) instead of just one.
    pub is_any_format: bool,
    pub session_nat_type_override: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuickMatchSize {
    /// Special case for Duel.
    Duel,
    /// One player vs one player in brawl or team mode with no additional allies.
    OneVsOne,
    TwoVsTwo,
    ThreeVsThree,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct QuickMatchSettings(pub u32);

impl QuickMatchSettings {
    /// Whether or not this gamemode allows spirit ashes summoning.
    pub const fn spirit_ashes_allowed(self) -> bool {
        self.0 >= 10
    }
    /// Whether or not this gamemode is brawl mode.
    pub const fn is_brawl_mode(self) -> bool {
        matches!(self.0 % 10, 1..=3)
    }
    /// Whether or not this gamemode is team-based.
    pub const fn is_team_mode(self) -> bool {
        matches!(self.0 % 10, 4..=9)
    }
    /// Whether or not this gamemode uses password for match you with your allies.
    /// Compared to just being password protected lobby where password doesn't affect team composition.
    pub const fn is_allies_password_mode(self) -> bool {
        matches!(self.0 % 10, 7..=9)
    }
    pub const fn match_size(self) -> QuickMatchSize {
        match self.0 % 10 {
            1 | 4 | 7 => QuickMatchSize::OneVsOne,
            2 | 5 | 8 => QuickMatchSize::TwoVsTwo,
            3 | 6 | 9 => QuickMatchSize::ThreeVsThree,
            _ => QuickMatchSize::Duel,
        }
    }
}

/// Values written to [`CSQuickMatchContext::error_state`].
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuickMatchErrorState {
    None = 0,
    /// The registration job failed.
    RegistrationJobFailed = 1,
    /// A guest's join-session job failed without being a retry.
    JoinSessionJobFailed = 2,
    /// Session isn't in the expected state for the current role.
    SessionStateMismatch = 3,
    /// Guest-side session singleton check failed.
    GuestSessionUnavailable = 4,
    /// Set by `CancelMatch` when there's no role assigned yet.
    CancelledNoRole = 5,
    /// Set by `CancelMatch` when hosting.
    CancelledHost = 6,
    /// `CancelledHost`, refined for allied-password matches specifically.
    CancelledHostAlliedPassword = 7,
    /// Set by `CancelMatch` when guesting.
    CancelledGuest = 8,
    Unk9 = 9,
    /// Allied-password team assembly timed out.
    AlliedPasswordAssemblyTimedOut = 10,
    Unk11 = 11,
    /// Set by `CancelMatch` on Steam disconnect, or by `SearchRegister` on a failed
    /// registration.
    SteamDisconnected = 12,
    /// `CSServerInterface` wasn't in its expected connected state when submitting the
    /// end-of-match ranking result.
    ConnectionErrorOccured = 13,
}

/// Source of name: RTTI
#[repr(C)]
pub struct CSQuickMatchContext {
    vtable: usize,
    /// Encodes the battle type (1v1, 2v2, 3v3, etc)
    pub match_settings: QuickMatchSettings,
    /// Map for this map as an integer, 45000000 as an example.
    pub match_map: QuickMatchArena,
    /// Spawn data for the local player.
    pub spawn_data: QuickmatchSpawnData,
    /// Vector of arenas available for quickmatch to randomly select from.
    pub arena_list: FNVector<QuickMatchArena>,
    /// Pending search results a guest can pop from to attempt a join.
    pub join_candidate_stack: DLVector<QuickMatchSearchResultEntry>,
    /// Pending search results for an allied-password opponent search.
    pub allied_password_opponent_candidates: DLVector<QuickMatchSearchResultEntry>,
    /// All quickmatch participants.
    pub participants: DLList<QuickmatchParticipant>,
    pub lobby_search_timed_out: bool,
    /// Seems to be indicative of why some QM lobby failed
    pub error_state: QuickMatchErrorState,
    pub result_submition_state: QuickMatchRankingSubmissionState,
    pub venue: QuickMatchVenue,
    unka0: [u8; 9],
}

#[repr(C)]
pub struct QuickMatchSearchResultEntry {
    pub host_player_id: u32,
    pub host_external_id: FNString,
    pub arena_id: QuickMatchArena,
    pub target_team: QuickmatchTeamSlot,
    pub match_player_count: u32,
    pub settings: QuickMatchSettings,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuickMatchRankingSubmissionState {
    Wait = 0,
    Dispatched = 1,
    Skipped = 2,
    NoContext = 3,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuickmatchRole {
    /// Not yet assigned a role.
    None = 0,
    Host = 1,
    Guest = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuickmatchTeamSlot {
    Ally = 0,
    Enemy = 1,
}

#[repr(C)]
pub struct QuickmatchSpawnData {
    pub block_id: BlockId,
    pub block_position: BlockPosition,
    pub role: QuickmatchRole,
}

/// Readiness state for a [`QuickmatchParticipant`].
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ParticipantReadyState {
    /// Set right after the host accepts a join.
    Accepted = 0,
    /// Counted as ready by `HostReadyWait`.
    ReadyStage = 1,
    /// Fully ready.
    Ready = 2,
    /// Marked for eviction by the host due to not being ready in time.
    PendingKick = 3,
}

/// Source of name: RTTI. Node data for [`CSQuickMatchContext::participants`].
#[repr(C)]
pub struct QuickmatchParticipant {
    /// Server-side unique id for this player.
    pub player_id: u32,
    /// Unique platform id for this player. SteamID for PC.
    pub external_id: FNString,
    unk28: u32,
    unk2c: u32,
    /// Server-side id of character this participant is playing as.
    pub character_id: u32,
    pub ready_state: ParticipantReadyState,
    /// How long the host waits for this participant to become ready before evicting them.
    /// Based on [`crate::param::NETWORK_PARAM_ST::host_time_out_time`].
    pub ready_state_kickout_timer: FD4Time,
    pub team: QuickmatchTeamSlot,
    pub player_game_data: Option<NonNull<PlayerGameData>>,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuickMatchVenue {
    Invalid = 0,
    RoyalColosseum = 1,
    LimgraveColosseum = 2,
    CaelidColosseum = 3,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QuickMatchArena {
    Invalid = 0,
    RoyalColosseum = 4500000,
    LimgraveColosseum = 4502000,
    CaelidColosseum = 4501000,
}
