use std::mem::transmute;

use pelite::pe64::Pe;
use eldenring::cs::ChrIns;

use crate::{program::Program, rva::{RVA_CHR_INS_APPLY_SPEFFECT, RVA_CHR_INS_REMOVE_SPEFFECT}};

pub trait ChrInsExt {
    fn apply_speffect(&mut self, sp_effect: i32, sync: bool);

    fn remove_speffect(&mut self, sp_effect: i32);
}

impl ChrInsExt for ChrIns {
    fn apply_speffect(&mut self, sp_effect: i32, sync: bool) {
        let rva = Program::current()
            .rva_to_va(RVA_CHR_INS_APPLY_SPEFFECT)
            .unwrap();

        let call = unsafe { transmute::<u64, fn(&mut ChrIns, i32, bool) -> u64>(rva) };
        call(self, sp_effect, sync);
    }

    fn remove_speffect(&mut self, sp_effect: i32) {
        let rva = Program::current()
            .rva_to_va(RVA_CHR_INS_REMOVE_SPEFFECT)
            .unwrap();

        let call = unsafe { transmute::<u64, fn(&mut ChrIns, i32) -> u64>(rva) };
        call(self, sp_effect);
    }
}
