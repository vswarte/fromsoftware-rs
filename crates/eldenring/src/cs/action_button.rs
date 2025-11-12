use pelite::pe64::Pe;
use shared::program::Program;

use crate::dlkr::DLPlainLightMutex;
use crate::rva;

#[repr(C)]
#[fromsoftware_shared::singleton("CSActionButtonMan")]
pub struct CSActionButtonManImp {
    vftable: usize,
    unk8: [u8; 0x88],
    pub mutex: DLPlainLightMutex,
}

#[repr(C)]
struct UnkActionButtonStruct {
    unk0: f32,
    unk4: i32,
    unk8: i32,
    unkc: i32,
    unk10: u64,
    unk18: u64,
    unk20: u64,
    unk28: u64,
}

type FnExecuteActionButton = extern "C" fn(
    *const CSActionButtonManImp,
    i32,
    i8,
    i8,
    i8,
    i8,
    i8,
    i32,
    *const UnkActionButtonStruct,
) -> bool;

impl CSActionButtonManImp {
    pub fn present_action_button(&mut self, action_button_param_id: i32) -> bool {
        let target = unsafe {
            std::mem::transmute::<u64, FnExecuteActionButton>(
                Program::current()
                    .rva_to_va(rva::get().cs_action_button_man_execute_action_button)
                    .unwrap(),
            )
        };

        target(
            self,
            action_button_param_id,
            0,
            0,
            0,
            0,
            0,
            0,
            &UnkActionButtonStruct {
                unk0: 0.0,
                unk4: 0,
                unk8: 0,
                unkc: 0,
                unk10: 0,
                unk18: 0,
                unk20: 0,
                unk28: 0,
            },
        )
    }
}
