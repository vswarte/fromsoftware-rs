mod events;
mod tae;
pub use events::*;
pub use tae::*;

use std::ptr::NonNull;

use shared::{OwnedPtr, Subclass, Superclass};

use crate::cs::ChrIns;

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrTimeActModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub hvk_anim: Option<NonNull<HvkAnim>>,
    pub chr_tae_anim_event: OwnedPtr<CSChrTaeAnimEvent>,
    /// Circular buffer of animations to play.
    pub anim_queue: [CSChrTimeActModuleAnim; 10],
    /// Index of the next animation to play or update.
    pub write_idx: u32,
    /// Index of the last animation played or updated.
    pub read_idx: u32,
    unkc8: i32,
    unkcc: f32,
    unkd0: f32,
    unkd4: u8,
    unkd5: u8,
}

#[repr(C)]
pub struct CSChrTimeActModuleAnim {
    pub anim_id: i32,
    /// Time in seconds since the animation started up to the last update.
    pub prev_local_time: f32,
    /// Time in seconds since the animation started up to the current frame.
    pub local_time: f32,
    /// Total length of the animation in seconds.
    pub anim_length: f32,
}

#[repr(C)]
#[derive(Superclass)]
#[superclass(children(CSChrTaeAnimEvent))]
pub struct CSTaeAnimEvent {
    vftable: usize,
    unk8: Option<NonNull<()>>,
    pub current_anim_id: u32,
    pub current_anim_duration: f32,
}

#[repr(C)]
#[derive(Subclass)]
pub struct CSChrTaeAnimEvent {
    pub base: CSTaeAnimEvent,
    pub owner: NonNull<ChrIns>,
}

#[repr(C)]
pub struct HvkAnim {
    vftable: usize,
    pub anim_containers: [HvkAnimContainer; 2],
    /// Total animation count loaded for this character
    pub animation_count: u32,
    /// Pointer to `HvkAnimTaeBinding` of `animation_count` amount
    animations: NonNull<()>,
    pub tae_dat: OwnedPtr<TaeDat>,
    /// Name of the animbnd data belongs to, eg `c0000` for the player
    pub name: NonNull<u16>,
    unkb8: bool,
}

#[repr(C)]
pub struct HvkAnimContainer {
    hka_skeleton: Option<NonNull<()>>,
    hk_root_level_container: Option<NonNull<()>>,
    unk10: [u8; 0x38],
}

#[repr(C)]
pub struct TaeDat {
    vftable: usize,
    pub tae_files: [Option<NonNull<TAE_Header_Main>>; 999],
    pub tae_resolvers: [Option<OwnedPtr<TaeFileResolver>>; 999],
    /// Not sure what's this about; True when BND4 file entry unk1 is not 0
    pub file_states: [bool; 999],
}

#[repr(C)]
/// Class that resolves relative file offsets to pointers in raw data,
pub struct TaeFileResolver {
    vftable: usize,
    /// Resolved tae file with most offsets replaced with actuall pointers,
    /// safe to read and traverse if non-null
    pub tae_file: Option<NonNull<TAE_Header_Main>>,
}
