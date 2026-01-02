use std::ptr::NonNull;

use windows::core::PCWSTR;

use crate::{
    cs::{DisplayGhostData, PasswordData},
    dltx::DLString,
    fd4::{FD4StepBaseInterface, FD4Time},
    position::BlockPosition,
    stl::{BasicVector, DoublyLinkedList, Vector},
};
use shared::OwnedPtr;

use super::{BlockId, CSEzTask, CSEzUpdateTask};

#[repr(C)]
#[shared::singleton("CSNetMan")]
pub struct CSNetMan {
    vftable: usize,
    unk8: u32,
    unkc: u32,
    unk10: [u8; 5],
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
    unka8: usize,
    /// Keeps track of quickmatch gamemode state.
    pub quickmatch_manager: OwnedPtr<QuickmatchManager>,
    visitor_db: usize,
    penalty_manager: usize,
    /// Task that updates the structure (pulls in new data from server, spawn received signs,
    /// stains and messages, spawns ghost replays, etc)
    pub update_task: CSEzUpdateTask<CSEzTask, Self>,
    unkf0: u32,
    unkf4: u32, // Probably padding
    unkf8: usize,
}

#[repr(C)]
pub struct CSNetBloodMessageDb {
    vftable: usize,
    // Contains all CSNetBloodMessageDbItem?
    pub entries: DoublyLinkedList<OwnedPtr<CSNetBloodMessageDbItem>>,
    unk20: usize,
    /// Seemingly contains message data for messages created by local user
    pub created_data: DoublyLinkedList<OwnedPtr<CSNetBloodMessageCreatedData>>,
    // Contains ???
    unk40: DoublyLinkedList<usize>,
    unk58: usize,
    blood_message_ins_man_1: usize,
    blood_message_ins_man_2: usize,
    pub discovered_messages: DoublyLinkedList<OwnedPtr<OwnedPtr<CSNetBloodMessageDbItem>>>,
    unk88: [u8; 0xD0],
    /// Hosts any ongoing jobs for evaluations.
    evaluate_job: usize,
    unk160: usize,
}

#[repr(C)]
pub struct CSNetBloodMessageCreatedData {
    pub player_id: u32,
    unk4: [u8; 8],
    pub message_id: u64,
    pub block_id: BlockId,
    unk1c: u32,
    pub position: BlockPosition,
    pub template1: u16,
    pub gesture_param: u16,
    pub part1: u16,
    pub infix: u16,
    pub template2: u16,
    pub part2: u16,
    unk3c: u16,
    pub display_ghost: DisplayGhostData,
    pub character_name: [u16; 20],
    pub positive_rating: u16,
    pub negative_rating: u16,
    unkfc: u16,
    unkfe: [u8; 2],
    pub group_passwords: [PasswordData; 5],
    pub net_blood_message_db_item: OwnedPtr<CSNetBloodMessageDbItem>,
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
pub struct QuickmatchManager {
    /// Stepper that updates the games quickmatch state.
    pub quickmatching_ctrl: OwnedPtr<CSQuickMatchingCtrl>,
    /// Keeps track of quickmatch settings as well as any participants.
    pub battle_royal_context: OwnedPtr<CSBattleRoyalContext>,
    /// Populated during creation of the QM lobby locally. Either by joining or creating a room.
    active_battle_royal_context: Option<NonNull<CSBattleRoyalContext>>,
    unk18: u32,
    /// List of speffects applied to the players during battle.
    /// Source of names: debug strings
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
    // TODO: more fields up to 0xd8
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CSQuickMatchingCtrlState {
    None = 0x0,
    SearchRegister = 0x1,
    SearchRegisterWait = 0x2,
    // Waiting for lobby to gain enough people to start.
    GuestInviteWait = 0x3,
    GuestWaitSession = 0x4,
    GuestReadyWait = 0x5,
    // Moving to arena map.
    GuestMoveMap = 0x6,
    // People are loaded into the map and match is running or has errored.
    GuestInGame = 0x7,
    HostWaitSession = 0x8,
    // Hosting and allowing other people to join the room before starting.
    HostInvite = 0x9,
    HostReadyWait = 0xa,
    HostReadyWaitBlockList = 0xb,
    // Moving to arena map.
    HostMoveMap = 0xc,
    // People are loaded into the map and match is running or has errored.
    HostInGame = 0xd,
    // Match has ended either by completion or error.
    Unregister = 0xe,
}

/// Source of name: RTTI
#[repr(C)]
pub struct CSQuickMatchingCtrl {
    pub base: FD4StepBaseInterface<15, Self>,
    unk18: [u8; 0x28],
    pub current_state: CSQuickMatchingCtrlState,
    pub requested_state: CSQuickMatchingCtrlState,
    unk48: [u8; 0x50],
    /// FD4Step state string.
    state_string: PCWSTR,
    unka0: bool,
    unka1: bool,
    unka2: bool,
    unka3: bool,
    unka4: u32,
    pub context: NonNull<CSBattleRoyalContext>,
    menu_job: usize,
    unkb8: FD4Time,
    unkc8: bool,
    unkc9: bool,
    unkca: bool,
    unkcb: bool,
    unkcc: bool,
    unkcd: bool,
    unkce: [u8; 5],
    unkd3: bool,
    /// Set to true if the client doesn't send the QM "ready" packet in time.
    pub move_map_timed_out: bool,
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
    unkf1: u8,
    unkf2: u8,
    unkf3: u8,
    unkf4: u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Enum describing various quickmatch (arena) gamemode settings
pub enum QuickMatchSettings {
    Duel = 0,
    Brawl1v1 = 1,
    Brawl2v2 = 2,
    Brawl3v3 = 3,
    Team1v1 = 4,
    Team2v2 = 5,
    Team3v3 = 6,
    AlliesPasswordTeam1v1 = 7,
    AlliesPasswordTeam2v2 = 8,
    AlliesPasswordTeam3v3 = 9,
    SpiritAshesDuel = 10,
    SpiritAshesBrawl1v1 = 11,
    SpiritAshesBrawl2v2 = 12,
    SpiritAshesBrawl3v3 = 13,
    SpiritAshesTeam1v1 = 14,
    SpiritAshesTeam2v2 = 15,
    SpiritAshesTeam3v3 = 16,
    SpiritAshesAlliesPasswordTeam1v1 = 17,
    SpiritAshesAlliesPasswordTeam2v2 = 18,
    SpiritAshesAlliesPasswordTeam3v3 = 19,
}

impl QuickMatchSettings {
    /// Whether or not this gamemode allows spirit ashes summoning.
    pub const fn spirit_ashes_allowed(&self) -> bool {
        matches!(
            self,
            QuickMatchSettings::SpiritAshesDuel
                | QuickMatchSettings::SpiritAshesBrawl1v1
                | QuickMatchSettings::SpiritAshesBrawl2v2
                | QuickMatchSettings::SpiritAshesBrawl3v3
                | QuickMatchSettings::SpiritAshesTeam1v1
                | QuickMatchSettings::SpiritAshesTeam2v2
                | QuickMatchSettings::SpiritAshesTeam3v3
                | QuickMatchSettings::SpiritAshesAlliesPasswordTeam1v1
                | QuickMatchSettings::SpiritAshesAlliesPasswordTeam2v2
                | QuickMatchSettings::SpiritAshesAlliesPasswordTeam3v3
        )
    }
    /// Whether or not this gamemode is team-based.
    pub const fn is_team_mode(&self) -> bool {
        matches!(
            self,
            QuickMatchSettings::Team1v1
                | QuickMatchSettings::Team2v2
                | QuickMatchSettings::Team3v3
                | QuickMatchSettings::AlliesPasswordTeam1v1
                | QuickMatchSettings::AlliesPasswordTeam2v2
                | QuickMatchSettings::AlliesPasswordTeam3v3
                | QuickMatchSettings::SpiritAshesTeam1v1
                | QuickMatchSettings::SpiritAshesTeam2v2
                | QuickMatchSettings::SpiritAshesTeam3v3
                | QuickMatchSettings::SpiritAshesAlliesPasswordTeam1v1
                | QuickMatchSettings::SpiritAshesAlliesPasswordTeam2v2
                | QuickMatchSettings::SpiritAshesAlliesPasswordTeam3v3
        )
    }
    /// Whether or not this gamemode uses password for match you with your allies.
    /// Compared to just being password protected lobby where password doesn't affect team composition.
    pub const fn is_allies_password_mode(&self) -> bool {
        matches!(
            self,
            QuickMatchSettings::AlliesPasswordTeam1v1
                | QuickMatchSettings::AlliesPasswordTeam2v2
                | QuickMatchSettings::AlliesPasswordTeam3v3
                | QuickMatchSettings::SpiritAshesAlliesPasswordTeam1v1
                | QuickMatchSettings::SpiritAshesAlliesPasswordTeam2v2
                | QuickMatchSettings::SpiritAshesAlliesPasswordTeam3v3
        )
    }
    /// Whether or not this gamemode is a brawl (free-for-all) mode.
    pub const fn is_brawl_mode(&self) -> bool {
        matches!(
            self,
            QuickMatchSettings::Brawl1v1
                | QuickMatchSettings::Brawl2v2
                | QuickMatchSettings::Brawl3v3
                | QuickMatchSettings::SpiritAshesBrawl1v1
                | QuickMatchSettings::SpiritAshesBrawl2v2
                | QuickMatchSettings::SpiritAshesBrawl3v3
        )
    }
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
    pub arena_list: BasicVector<QuickMatchArena>,
    unk40: Vector<usize>,
    unk60: Vector<usize>,
    /// All quickmatch participants.
    pub participants: DoublyLinkedList<QuickmatchParticipant>,
    unk98: u8,
    /// Seems to be indicative of why some QM lobby failed
    pub error_state: u8,
    unk9a: u8,
    unk9b: u8,
    pub venue: QuickMatchVenue,
    unka0: u32,
    unka4: u32,
    unka8: u32,
    unkac: u32,
}

#[repr(C)]
pub struct QuickmatchSpawnData {
    pub block_id: BlockId,
    pub block_position: BlockPosition,
    pub role: u32,
}

#[repr(C)]
pub struct QuickmatchParticipant {}

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
