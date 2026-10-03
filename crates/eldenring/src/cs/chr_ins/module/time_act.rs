mod events;
mod tae;
pub use events::*;
pub use tae::*;
use windows::core::PCWSTR;

use std::{fmt::Debug, ptr::NonNull};

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
    /// Time in seconds between the animation starting and the last update.
    pub prev_local_time: f32,
    /// Time in seconds between the animation starting and the current frame.
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
    /// Pointer to `animation_count` instances of `HvkAnimTaeBinding`
    animations: NonNull<()>,
    pub tae_dat: OwnedPtr<TaeDat>,
    /// Name of the animbnd that the data belongs to, eg `c0000` for the player
    pub name: PCWSTR,
    unkb8: bool,
}

impl HvkAnim {
    pub fn animbnd_name(&self) -> String {
        unsafe {
            self.name
                .to_string()
                .unwrap_or_else(|_| "Invalid UTF-16".to_string())
        }
    }
}

impl Debug for HvkAnim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HvkAnim")
            .field("anim_containers", &self.anim_containers)
            .field("animation_count", &self.animation_count)
            .field("tae_dat", &self.tae_dat)
            .field("name", &self.animbnd_name())
            .finish()
    }
}

impl Debug for HvkAnimContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HvkAnimContainer")
            .field("hka_skeleton", &self.hka_skeleton)
            .field("hk_root_level_container", &self.hk_root_level_container)
            .finish()
    }
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
    pub tae_files: [Option<NonNull<TaeHeader>>; 999],
    pub tae_resolvers: [Option<OwnedPtr<TaeFileResolver>>; 999],
    /// Not sure what's this about; True when BND4 file entry unk1 is not 0
    pub file_states: [bool; 999],
}

#[repr(C)]
/// Class that resolves relative file offsets to pointers in raw data,
pub struct TaeFileResolver {
    vftable: usize,
    /// Resolved tae file with most offsets replaced with actual pointers.
    pub tae_file: Option<NonNull<TaeHeader>>,
}
