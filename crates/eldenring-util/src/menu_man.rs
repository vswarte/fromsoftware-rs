use pelite::pe::Pe;

use eldenring::cs::CSMenuManImp;

use crate::{program::Program, rva};

pub trait CSMenuManImpExt {
    fn display_status_message(&mut self, message: i32) -> bool;
}

pub const STATUS_MESSAGE_DEMIGOD_FELLED: i32 = 1;
pub const STATUS_MESSAGE_LEGEND_FELLED: i32 = 2;
pub const STATUS_MESSAGE_GREAT_ENEMY_FELLED: i32 = 3;
pub const STATUS_MESSAGE_ENEMY_FELLED: i32 = 4;
pub const STATUS_MESSAGE_YOU_DIED: i32 = 5;
pub const STATUS_MESSAGE_HOST_VANQUISHED: i32 = 7;
pub const STATUS_MESSAGE_BLOOD_FINGER_VANQUISHED: i32 = 8;
pub const STATUS_MESSAGE_DUTY_FULL_FILLED: i32 = 9;
pub const STATUS_MESSAGE_LOST_GRACE_DISCOVERED: i32 = 11;
pub const STATUS_MESSAGE_COMMENCE: i32 = 13;
pub const STATUS_MESSAGE_VICTORY: i32 = 14;
pub const STATUS_MESSAGE_STALEMATE: i32 = 15;
pub const STATUS_MESSAGE_DEFEAT: i32 = 16;
pub const STATUS_MESSAGE_MAP_FOUND: i32 = 17;
pub const STATUS_MESSAGE_GREAT_RUNE_RESTORED: i32 = 21;
pub const STATUS_MESSAGE_GOD_SLAIN: i32 = 22;
pub const STATUS_MESSAGE_DUELIST_VANQUISHED: i32 = 23;
pub const STATUS_MESSAGE_RECUSANT_VANQUISHED: i32 = 24;
pub const STATUS_MESSAGE_INVADER_VANQUISHED: i32 = 25;
pub const STATUS_MESSAGE_FURLED_FINGER_RANK_ADVANCED: i32 = 30;
pub const STATUS_MESSAGE_FURLED_FINGER_RANK_ADVANCED2: i32 = 31;
pub const STATUS_MESSAGE_DUELIST_RANK_ADVANCED: i32 = 32;
pub const STATUS_MESSAGE_DUELIST_RANK_ADVANCED2: i32 = 33;
pub const STATUS_MESSAGE_BLOODY_FINGER_RANK_ADVANCED: i32 = 34;
pub const STATUS_MESSAGE_BLOODY_FINGER_RANK_ADVANCED2: i32 = 35;
pub const STATUS_MESSAGE_RECUSANT_RANK_ADVANCED: i32 = 36;
pub const STATUS_MESSAGE_RECUSANT_RANK_ADVANCED2: i32 = 37;
pub const STATUS_MESSAGE_HUNTER_RANK_ADVANCED: i32 = 38;
pub const STATUS_MESSAGE_HUNTER_RANK_ADVANCED2: i32 = 39;
pub const STATUS_MESSAGE_HEART_STOLEN: i32 = 40;
pub const STATUS_MESSAGE_MENU_TEXT: i32 = 41;

impl CSMenuManImpExt for CSMenuManImp {
    // "You died", "Great enemy felled", etc
    fn display_status_message(&mut self, message: i32) -> bool {
        let rva = Program::current()
            .rva_to_va(rva::get().cs_menu_man_imp_display_status_message)
            .unwrap();

        let target = unsafe { std::mem::transmute::<u64, fn(&mut CSMenuManImp, i32) -> bool>(rva) };
        target(self, message)
    }
}
