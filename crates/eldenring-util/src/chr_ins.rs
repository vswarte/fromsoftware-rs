use std::mem::transmute;

use eldenring::{cs::ChrIns, rva};
use pelite::pe64::Pe;

use shared::program::Program;

pub trait ChrInsExt {
    fn apply_speffect(&mut self, sp_effect: i32, dont_sync: bool);

    fn remove_speffect(&mut self, sp_effect: i32);
}

impl ChrInsExt for ChrIns {
    fn apply_speffect(&mut self, sp_effect: i32, dont_sync: bool) {
        let rva = Program::current()
            .rva_to_va(rva::get().chr_ins_apply_speffect)
            .unwrap();

        let call = unsafe { transmute::<u64, fn(&mut ChrIns, i32, bool) -> u64>(rva) };
        call(self, sp_effect, dont_sync);
    }

    fn remove_speffect(&mut self, sp_effect: i32) {
        let rva = Program::current()
            .rva_to_va(rva::get().chr_ins_remove_speffect)
            .unwrap();

        let call = unsafe { transmute::<u64, fn(&mut ChrIns, i32) -> u64>(rva) };
        call(self, sp_effect);
    }
}
