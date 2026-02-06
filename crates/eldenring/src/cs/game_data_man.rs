use crate::{
    Vector,
    cs::{CSGaitemGameData, ChrType, PlayerGameData},
    fd4::FD4Time,
};
use shared::{FromStatic, OwnedPtr, load_static_indirect};
use std::{borrow::Cow, ptr::NonNull};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RemotePlayerDataSlotState {
    /// Player data slot is free / unoccupied
    Free = 0,
    /// Player data slot is occupied but not yet synced
    Occupied = 1 << 0,
    /// Player data slot has base character data (packet 8)
    BaseData = 1 << 1,
    /// Player data slot has equipment data (packet 12)
    Equipment = 1 << 2,
    /// Player data slot has character type data (packet 11)
    Type = 1 << 3,
    /// Player data slot is fully synced
    FullySynced = 0xF,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Source of name: global_event.lua from DS3
pub enum DeathState {
    None = -1,
    Normal = 0,
    /// DS3 tears of denial style resurrection
    MagicResurrection = 1,
    /// Sacrificial Twig
    RingNormalResurrection = 2,
    /// Twiggy Cracked Tear
    RingCurseResurrection = 3,
}

#[repr(C)]
pub struct GameDataMan {
    trophy_equip_data: usize,
    pub main_player_game_data: OwnedPtr<PlayerGameData>,
    pub player_game_data_list: OwnedPtr<[PlayerGameData; 5]>,
    /// Pointer to the game data of the player used for the baseline
    /// for the arena match multiplay scaling in active match.
    pub quickmatch_scaling_baseline_game_data: Option<NonNull<PlayerGameData>>,
    pub remote_game_data_states: OwnedPtr<[RemotePlayerDataSlotState; 5]>,
    pub session_player_game_data_list: OwnedPtr<[Option<OwnedPtr<PlayerGameData>>; 40]>,
    pub gaitem_game_data: OwnedPtr<CSGaitemGameData>,
    tutorial_data: usize,
    unk40: [u8; 0x18],
    pub game_settings: OwnedPtr<GameSettings>,
    menu_system_save_load: usize,
    menu_profile_save_load: usize,
    key_config_save_load: usize,
    profile_summary: usize,
    pc_option_data: usize,
    unk88: [u8; 0x4],
    pub request_full_recovery: bool,
    unk8d: [u8; 0x2],
    /// Whether game should give the player the phantom great rune
    /// Will be true for some time during loading when Mogh's great rune is active and
    /// the player is invading someone else's world
    pub award_phantom_great_rune_requested: bool,
    /// Whether game should give the player the rebreak in item
    /// Will be true for some time during loading when the player is invading someone else's world
    pub award_rebreak_in_item_requested: bool,
    pub death_count: u32,
    /// Character type to switch to after loading a map
    /// [ChrType::None] if no switch is requested
    ///
    /// Set by `CS::CSLuaEventProxy::SetChrTypeDataGreyNext`
    pub post_map_load_chr_type: ChrType,
    unk9c: [u8; 0x4],
    /// Play time as milliseconds
    /// will be maxed out at 999:59:59.999
    pub play_time: u32,
    unka4: [u8; 0xC],
    /// Timer for tracking boss fight duration
    pub boss_fight_timer: FD4Time,
    /// Whether a boss fight is currently active
    pub boss_fight_active: bool,
    /// Count of white phantoms currently summoned
    /// Used to apply enemy level scaling
    pub white_phantom_count: u32,
    pub boss_health_bar_entity_id: u32,
    pub boss_health_bar_npc_param_id: u32,
    unkd0: [u8; 0x4],
    /// State of special death-related effects
    pub death_state: DeathState,
    /// Whether the player has a death preventing effect active
    pub has_death_preventing_effect: bool,
    /// Whether the player died recently
    pub just_died: bool,
    /// Leave request status for each player slot
    /// Used by lua script imitation to track on leave events
    pub leave_requests: [bool; 5],
    pub game_version_data: GameVersionData,
    unkf0: bool,
    /// Whether the DLC list is up to date and any pending DLCs have been applied ([`pending_dlc_list`] is empty)
    ///
    /// [`pending_dlc_list`]: Self::pending_dlc_list
    pub dlc_list_up_to_date: bool,
    /// Vector of indecies into `CSDlc` that are not applied to this game data yet
    pub pending_dlc_list: Vector<u32>,
    pub is_net_penalized: bool,
    pub net_penalty_requested: bool,
    pub net_penalty_points: u16,
    pub net_penalty_forgive_item_limit_time: f32,
    pub ng_lvl: u32,
    unk124: [u8; 0x34],
}

impl FromStatic for GameDataMan {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("GameDataMan")
    }

    unsafe fn instance() -> shared::InstanceResult<&'static mut Self> {
        unsafe { load_static_indirect(crate::rva::get().game_data_man) }
    }
}

#[repr(C)]
pub struct GameVersionData {
    /// Current version of the game data structure
    pub game_data_version: u32,
    /// Version of the game data read from the last save
    pub last_saved_game_data_version: u32,
    /// Whether the saved game data version is the latest
    pub saved_game_data_version_is_the_latest: bool,
    pub unused: u32,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DisplayBlood {
    Off = 0,
    On = 1,
    Mild = 2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PerformanceSetting {
    PrioritizeQuality = 0,
    PrioritizeFramerate = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HudType {
    Off = 0,
    On = 1,
    Auto = 2,
}

#[repr(C)]
pub struct GameSettings {
    /// Camera rotation speed
    /// Range: 0-10
    /// Default value is read from Game.Option.Control.RotSpeed property
    pub camera_speed: u8,
    /// Controls the strength of controller rumble
    /// Range: 0-10
    /// Default value is read from Game.Option.Control.Rumble property
    pub controller_rumble_strength: u8,
    /// Controls the brightness of the game
    /// Range: 0-10
    /// Default value is read from Game.Option.Disp.Brightness property
    pub brightness: u8,
    /// Range: 0-10
    /// Default value is read from Game.Option.Sound.SoundType property
    pub sound_type: u8,
    /// Controls the volume of the music
    /// Range: 0-10
    /// Default value is read from Game.Option.Sound.MusicVol property
    pub music_volume: u8,
    /// Controls the volume of sound effects
    /// Range: 0-10
    /// Default value is read from Game.Option.Sound.SeVol property
    pub sfx_volume: u8,
    /// Controls the volume of the voice chat
    /// Range: 0-10
    /// Default value is read from Game.Option.Sound.VoiceVol property
    pub voice_volume: u8,
    /// Controls how blood is displayed
    /// Default value is read from Game.Option.Disp.Blood property
    pub display_blood: DisplayBlood,
    /// Controls whether subtitles are shown
    /// Default value is read from Game.Option.Disp.Subtitle property
    pub show_subtitles: bool,
    /// Type of HUD display
    /// Default value is read from Game.Option.Disp.HUD property
    pub hud_type: HudType,
    /// Controls whether the camera X axis is reversed
    /// Default value is read from Game.Option.Control.RotLR property
    pub reverse_camera_xaxis: bool,
    /// Controls whether the camera Y axis is reversed
    /// Default value is read from Game.Option.Control.RotUD property
    pub reverse_camera_yaxis: bool,
    /// Controls whether camera should automatically lock on to the next target
    /// after the current target is defeated or lost
    pub auto_lock_on: bool,
    /// Controls whether camera automatically adjusts when near walls
    pub camera_auto_wall_recovery: bool,
    unke: u8,
    /// Unused, but read from
    /// Game.Option.Control.JumpButtonL3 property
    pub jump_button_l3: bool,
    /// Controls whether camera recenters vertically when resetting
    /// Default value is read from Game.Option.Control.CameraResetUD property
    pub reset_camera_yaxis: bool,
    /// Controls whether game allowed or not to take control of camera during
    /// certain cinematic moments
    /// Default value is read from Game.Option.Control.CameraDirection property
    pub cinematic_effects: bool,
    unk12: u8,
    /// Controls whether cross-region play is enabled
    /// Doesn't work on PC version
    pub enable_cross_region_play: bool,
    /// Controls whether voice chat is enabled
    /// Locked behind release flag 51 on PC release
    pub voice_chat: bool,
    /// Controls whether gamer tags are shown instead of character names
    /// Locked behind release flag 49 on PC release
    pub show_gamer_tags: bool,
    /// Controls whether manual attack aiming is enabled
    /// Only works on Ringed Finger weapon
    pub manual_attack_aiming: bool,
    /// Controls whether camera automatically targets enemies when attacking
    /// with no lock-on
    pub auto_target: bool,
    /// Controls whether game starts in offline mode
    pub start_offline: bool,
    /// Default value is read from Game.Option.Network.HideWhiteSignInSignEnemyWorld property
    pub send_summon_signs: bool,
    /// Unused setting enabled by release flag 37
    /// Uses GR System Message 103000 for the name
    /// and 3001 for the description
    pub unused_gr_system_103000: bool,
    unk1b: u8,
    /// Controls HDR brightness level
    /// Range: 0-10
    pub hdr_brightness: u8,
    /// Controls HDR max brightness level
    /// Range: 0-10
    pub hdr_max_brightness: u8,
    /// Controls HDR contrast level
    /// Range: 0-10
    pub hdr_contrast: u8,
    /// Controls how game utilizes system resources
    /// Locked behind release flag 39 on PC release
    pub performance_setting: PerformanceSetting,
    /// Controls the master volume
    /// Range: 0-10
    pub master_volume: u8,
    /// Controls whether ray tracing is enabled
    /// Locked behind release flag 38 on PC release
    pub enable_ray_tracing: bool,
    /// Controls whether newly acquired items are marked in inventory
    pub mark_new_items: bool,
    /// Controls whether recent items tab is shown in inventory
    pub show_recent_items: bool,
    unka4: [u8; 10],
    /// Controls whether tutorials are shown
    pub show_tutorials: bool,
    /// Controls whether camera automatically rotates to follow player movement
    pub camera_auto_rotation: bool,
    /// Unused space, will allways be memset on deserialization
    pub unused_space: [u8; 0x110],
}
