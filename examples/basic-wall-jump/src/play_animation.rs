use eldenring::cs::{ChrIns, HavokBehaviorCharacter};
use fromsoftware_shared::{OwnedPtr, Program};
use pelite::pe64::Pe;
use std::{iter, mem::transmute};

const PLAY_ANIMATION_BY_NAME_RVA: u32 = 0x00C14460;

pub trait ChrInsPlayAnim {
    fn play_animation_by_name(&self, animation: &str) -> bool;
}

impl ChrInsPlayAnim for ChrIns { 
    fn play_animation_by_name(&self, animation: &str) -> bool {
        let hkb_character = &self
            .module_container
            .behavior
            .beh_character
            .hkb_character;

        let wide_c_string: Vec<u16> = animation.encode_utf16().chain(iter::once(0)).collect();

        let Some(va) = Program::current()
            .rva_to_va(PLAY_ANIMATION_BY_NAME_RVA)
            .ok()
        else {
            return false;
        };

        let play_animation_by_name = unsafe {
            transmute::<u64, extern "C" fn(&OwnedPtr<HavokBehaviorCharacter>, *const u16) -> u32>(va)
        };

        let result = play_animation_by_name(hkb_character, wide_c_string.as_ptr());

        result != u32::MAX
    }
}