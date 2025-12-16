use eldenring::cs::{ChrIns, HavokBehaviorCharacter};
use fromsoftware_shared::{OwnedPtr, Program};
use pelite::pe64::Pe;
use std::{iter, mem::transmute};

const PLAY_ANIMATION_BY_NAME_RVA: u32 = 0x00C14460;

// Define a trait so that we can attach this trait implementation to the ChrIns, without modifying the ChrIns crate.
pub trait ChrInsPlayAnim {
    fn play_animation_by_name(&self, animation: &str) -> bool;
}

impl ChrInsPlayAnim for ChrIns {
    // Sugar coat function to accept ChrIns and a Rust string.
    fn play_animation_by_name(&self, animation: &str) -> bool {
        // Havok Behavior Character
        let hkb_character = &self
            .module_container
            .behavior
            .beh_character
            .hkb_character;

        // Turn the Rust &str in to an UTF16 string that is NULL-Terminated.
        let wide_c_string: Vec<u16> = animation.encode_utf16().chain(iter::once(0)).collect();

        // Turn the Relative Virtual Address in to the memory address (as u64) of the function.
        let Some(va) = Program::current()
            .rva_to_va(PLAY_ANIMATION_BY_NAME_RVA)
            .ok()
        else {
            return false;
        };

        // Transmute the memory address to be interpreted as it's function instead.
        let play_animation_by_name = unsafe {
            transmute::<u64, extern "C" fn(&OwnedPtr<HavokBehaviorCharacter>, *const u16) -> u32>(va)
        };

        // Call the function using a reference to the hkb_character and a pointer to the animation name.
        let result = play_animation_by_name(hkb_character, wide_c_string.as_ptr());

        // Return wether the animation played successfully as a boolean.
        result != u32::MAX
    }
}
