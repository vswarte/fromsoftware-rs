use pelite::pe64::Pe;
use shared::Program;
use std::{
    mem::{MaybeUninit, transmute},
    ptr::NonNull,
};
use vtable_rs::VPtr;

use crate::{
    cs::CSNpcTalkIns,
    ez_state::{EzStateEnvironmentQuery, EzStateEvent, EzStateRawValue},
    rva,
};

#[vtable_rs::vtable]
pub trait CSEzStateTalkEventVmt {
    fn destructor(&mut self);

    fn invoke(&mut self, args: &EzStateEvent);
}

#[repr(C)]
/// An ESD function call, which can be invoked multiple times with different arguments and
/// generally has a side effect in the menu or game world
///
/// Source of name: RTTI
pub struct CSEzStateTalkEvent {
    pub vftable: VPtr<dyn CSEzStateTalkEventVmt, Self>,
    unk8: usize,
    pub npc_talk_ins: NonNull<CSNpcTalkIns>,
    pub talk_id: i32,
}

impl CSEzStateTalkEvent {
    pub fn new(talk_id: i32, npc_talk_ins: &CSNpcTalkIns) -> Self {
        let vftable: VPtr<dyn CSEzStateTalkEventVmt, Self> = unsafe {
            transmute(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_state_talk_event_vmt)
                    .unwrap(),
            )
        };

        Self {
            vftable,
            unk8: 0,
            npc_talk_ins: NonNull::from(npc_talk_ins),
            talk_id,
        }
    }
}

#[vtable_rs::vtable]
pub trait CSEzStateTalkEnvVmt {
    fn destructor(&mut self);

    fn invoke<'a>(
        &mut self,
        result: &'a mut MaybeUninit<EzStateRawValue>,
        args: &EzStateEnvironmentQuery,
    ) -> &'a EzStateRawValue;
}

#[repr(C)]
/// An ESD environment query call, which can be invoked multiple times with different arguments,
/// returning a single value each time
///
/// Source of name: RTTI
pub struct CSEzStateTalkEnv {
    pub vftable: VPtr<dyn CSEzStateTalkEnvVmt, Self>,
    unk8: usize,
    pub talk_id: i32,
    pub npc_talk_ins: NonNull<CSNpcTalkIns>,
    unk20: bool,
    pub is_client_player: bool,
    pub is_character_disabled: bool,
}

impl CSEzStateTalkEnv {
    pub fn new(talk_id: i32, npc_talk_ins: &CSNpcTalkIns) -> Self {
        let vftable: VPtr<dyn CSEzStateTalkEnvVmt, Self> = unsafe {
            transmute(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_state_talk_env_vmt)
                    .unwrap(),
            )
        };

        Self {
            vftable,
            unk8: 0,
            npc_talk_ins: NonNull::from(npc_talk_ins),
            talk_id,
            unk20: false,
            is_client_player: false,
            is_character_disabled: false,
        }
    }
}
