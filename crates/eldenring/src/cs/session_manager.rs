use std::ptr::NonNull;

use windows::Win32::Foundation::FILETIME;

use crate::{
    dlcr::{AESDecrypter, AESEncrypter, DLSerialCipherKey},
    dlkr::{DLAllocatorBase, DLPlainLightMutex},
    dltx::{DLCodedString, DLInplaceStr, DLUTF16StringKind},
    fd4::FD4Time,
    DoublyLinkedList, Vector,
};
use shared::{FSVector3, FSVector4, OwnedPtr};

use super::{CSEzTask, CSEzUpdateTask, MapId, P2PEntityHandle};

#[repr(u32)]
#[derive(Debug, PartialEq)]
/// Various states for an online lobby to be in.
///
/// Source of name: Sekiro Debug Menu
pub enum LobbyState {
    None = 0x0,
    TryToCreateSession = 0x1,
    FailedToCreateSession = 0x2,
    Host = 0x3,
    TryToJoinSession = 0x4,
    FailedToJoinSesion = 0x5,
    Client = 0x6,
    OnLeaveSession = 0x7,
    FailedToLeaveSession = 0x8,
}

#[repr(u32)]
#[derive(Debug)]
pub enum ProtocolState {
    Inactive = 0x0,
    Unk1 = 0x1,
    AwaitingWorldData = 0x2,
    Unk3 = 0x3,
    Unk4 = 0x4,
    Unk5 = 0x5,
    InWorld = 0x6,
    Unk7 = 0x7,
}

impl ProtocolState {
    /// Seems to be checked for packet 39,
    fn should_handle_some_packets(&self) -> bool {
        match self {
            ProtocolState::Inactive => false,
            ProtocolState::Unk1 => false,
            ProtocolState::AwaitingWorldData => false,
            ProtocolState::Unk3 => false,
            ProtocolState::Unk4 => true,
            ProtocolState::Unk5 => true,
            ProtocolState::InWorld => false,
            ProtocolState::Unk7 => true,
        }
    }
}

#[repr(C)]
#[dlrf::singleton("CSSessionManager")]
pub struct CSSessionManager {
    vftable: usize,
    unk8: u32,
    pub lobby_state: LobbyState,
    pub protocol_state: ProtocolState,
    unk14: f32,
    unk18: u8,
    unk19: u8,
    unk1a: u8,
    unk1b: u8,
    unk1c: u8,
    unk1d: u8,
    unk1e: u8,
    unk1f: u8,
    unk20: u32,
    unk24: u32,
    unk28: u32,
    unk2c: u32,
    unk30: usize,
    map_active_synchronizer: usize,
    voice_chat_manager: usize,
    allocator: NonNull<DLAllocatorBase>,
    unk50: NonNull<Self>,
    unk58: u32,
    unk5c: u32,
    manager_impl_steam: usize,
    unk68: usize,
    pub players: Vector<SessionManagerPlayerEntry>,
    pub host_player: SessionManagerPlayerEntryBase,
    unk160: usize,
    unk168: usize,
    /// Player limit for current session.
    /// Set to 4 in open world and 6 in quickmatch arena.
    pub session_player_limit: u32,
    unk174: u32,
    unk178: u8,
    unk179: u8,
    unk17a: u8,
    unk17b: u8,
    unk17c: u32,
    player_data_man: usize,
    /// Used to warp players back to the latest valid multiplay area in case they step out of it.
    pub stay_in_multiplay_area_warp_data: OwnedPtr<CSStayInMultiplayAreaWarpData>,
    unk190: usize,
    protocol_state_1_timeout: FD4Time,
    protocol_state_2_timeout: FD4Time,
    unk1b8: usize,
    unk1c0: u8,
    unk1c1: u8,
    unk1c2: u8,
    unk1c3: u8,
    unk1c4: f32,
    unk1c8: u16,
    unk1ca: u8,
    unk1cb: u8,
    unk1cc: f32,
    unk1d0: f32,
    unk1d4: f32,
    unk1d8: f32,
    unk1dc: u32,
    pub update_task: CSEzUpdateTask<CSEzTask, Self>,
    unk208: CSEzUpdateTask<CSEzTask, Self>,
    unk230: i8,
    unk231: u8,
    unk232: u8,
    unk233: u8,
    unk234: u32,
    pub serial_cipher_key: OwnedPtr<DLSerialCipherKey>,
    pub aes_encrypter: OwnedPtr<AESEncrypter>,
    pub aes_decrypter: OwnedPtr<AESDecrypter>,
    unk250: u32,
    unk254: u32,
    unk258: u32,
    /// This field is set to 1 on init and never changed.
    /// If it's not 1, all sessions will use this value instead of the default session player limit.
    pub session_player_limit_override: u32,
    /// P2P Send queue? Seems unused? Maybe left-over from DS2?
    p2p_send_queue: CSSessionManagerP2PSendQueue,
    pub mutex: DLPlainLightMutex,
    unk2d0: f32,
    unk2d4: f32,
    /// Contain statistics about the inbound packet queue, seems unused.
    p2p_inbound_queue_stats: Option<OwnedPtr<CSSessionManagerP2PInboundQueueStats>>,
    unk2e0: u32,
    /// Seems to be a total for the amount of packet bytes in some fashion?
    unk2e4: u32,
    unk2e8: u32,
    unk2ec: u8,
    unk2ed: bool,
    unk2ee: u8,
    unk2ef: u8,
    unk2f0: DoublyLinkedList<()>,
    unk308: u16,
    unk30a: u16,
    unk30c: u32,
    unk310: i32,
    unk314: u32,
    unk318: u32,
    unk31c: u32,
    unk320: u16,
    unk322: u16,
    unk324: i32,
    unk328: i32,
    unk32c: u32,
    /// Next fields seem to be some collection?
    unk330: NonNull<DLAllocatorBase>,
    unk338: Option<OwnedPtr<()>>,
    unk340: u32,
    unk344: u32,
    unk348: u16,
    unk34a: u16,
    unk34c: i32,
    unk350: f32,
    unk354: u32,
}

#[repr(C)]
pub struct SessionManagerPlayerEntryBase {
    internal_thread_steam_connection: usize,
    internal_thread_steam_socket: usize,
    pub steam_id: u64,
    pub steam_name: DLInplaceStr<DLUTF16StringKind, 64>,
    connection_ref_info: usize,
    voice_chat_member_ref_info: usize,
}

#[repr(C)]
pub struct SessionManagerPlayerEntry {
    pub base: SessionManagerPlayerEntryBase,
    /// Index in networked player game data list, will be -1 for host.
    pub game_data_index: i32,
    unkd4: u32,
    unkd8: usize,
    pub p2p_entity_handle: P2PEntityHandle,
    unke8: u8,
    pub is_host: bool,
    unkea: [u8; 0x16],
}

#[repr(C)]
pub struct StayInMultiplayFadeTrackerEntry {
    /// Steam ID of the player who is currently in warp and should not be rendered.
    pub steam_id: u64,
    /// Time in seconds until the player should be rendered again.
    pub fade_time: f32,
    _pad: [u8; 4],
}

#[repr(C)]
/// Object used to warp players back to the latest valid multiplay area in case they step out of it.
pub struct CSStayInMultiplayAreaWarpData {
    /// Vector of remote player warp trackers.
    /// Used to check when player rendering should be disabled using bitflag on ChrIns at 0x1c5.
    /// When warp time is 0, player rendering is enabled back.
    pub player_fade_tracker: Vector<StayInMultiplayFadeTrackerEntry>,
    /// Sent by host to clients on connect in packet 90 (0x5A).
    /// Contains the ID of the play area the host was in when the client connected.
    /// If current multiplay area ID has different boss ID than this one, player will be warped to latest stored position.
    /// Setting this to 0 will disable this.
    pub multiplay_start_area_id: u32,
    unk24: u32,
    stay_in_multiplay_area_warp_step: [u8; 0xc8],
    /// Last position player was before stepping out of the multiplay area, relative to the map.
    /// Read from GameMan and uses same logic as bloodstains.
    pub saved_position: FSVector3,
    /// Last MapId player was before stepping out of the multiplay area.
    /// Read from GameMan and uses same logic as bloodstains.
    pub saved_map_id: MapId,
    /// Delay before the player is warped back to the safe position.
    /// Used for fadeout effect and updated by task.
    /// This is set to 0 when the player is warped back.
    pub warp_request_delay: f32,
    /// Setting this to true will completely disable multiplay area restrictions,
    /// allowing player to go anywhere on the map.
    pub disable_multiplay_restriction: bool,
    /// Controls if the player should be warped back to the saved position.
    /// False when falling, or doing something else that can mess up the warp.
    pub is_warp_possible: bool,
    _pad: [u8; 0x2],
}

#[repr(C)]
pub struct CSSessionManagerP2PSendQueue {
    pub queue: Vector<CSSessionManagerP2PSendQueueEntry>,
    unk20: CSSessionManager0x20,
    rand_xor_shift: usize,
    unk38: u32,
    unk3c: u32,
}

#[repr(C)]
pub struct CSSessionManagerP2PSendQueueEntry {
    /// Recipient's steam ID.
    pub recipient: u64,
    pub packet_bytes: *mut u8,
    pub packet_length: u32,
    pub packet_type: u8,
    unk15: u8,
    _pad16: u16,
}

#[repr(C)]
pub struct CSSessionManager0x20 {
    time_1: FILETIME,
    time_2: FILETIME,
}

#[repr(C)]
pub struct CSSessionManagerP2PInboundQueueStats {
    /// Seems to keep track of the amount of packets waiting in-queue.
    pending_packet_count: u32,
    /// Seems to keep track of the amount of raw bytes in the queue.
    pending_packet_bytes: u32,
    unk8: u32,
    unkc: u32,
}
