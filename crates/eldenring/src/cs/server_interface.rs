use crate::{DLVector, dlkr::CSNetworkAllocator, dltx::DLString, from_net::FNAllocator};
use fromsoftware_shared::OwnedPtr;

#[repr(C)]
#[shared::singleton("CSServerInterface")]
pub struct CSServerInterface {
    fn_client: usize,
    unk8: u32,
    server_log_data_tracker: OwnedPtr<(), CSNetworkAllocator>,
    net_player_watcher: OwnedPtr<(), CSNetworkAllocator>,
    unk20: usize,
    unk28: usize,
    unk30: usize,
    unk38: usize,
    unk40: usize,
    unk48: [u8; 0xc0],
    unk108: i32,
    pub fn_allocator: FNAllocator,
    unk130: u32,
    pub system_clock: u64,
    pub channel_id: u32,
    /// Whether forced server log-in enabled or not.
    pub login_enabled: bool,
    pub primary_server_url: DLString,
    pub secondary_server_urls: DLVector<DLString>,
    unk198: usize,
    unk1a0: u32,
}
