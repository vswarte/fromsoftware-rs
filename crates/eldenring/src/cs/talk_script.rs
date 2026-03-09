use std::{mem::MaybeUninit, ptr::NonNull};

use crate::{
    cs::{
        BlockId, CSEzStateTalkEnv, CSEzStateTalkEvent, CSMenuManImp, FieldInsHandle, MenuJobBase,
        MenuType, WorldChrMan,
    },
    ez_state::{
        EzStateEnvironmentQuery, EzStateEvent, EzStateMachineImpl, EzStateRawValue, EzStateValue,
    },
};
use shared::{FromStatic, InstanceError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EzStateInvokeError {
    #[error("Failed to get WorldChrMan instance")]
    WorldChrManError(InstanceError),

    #[error("Failed to get MenuMan instance")]
    MenuManError(InstanceError),

    #[error("NPC associated with this talk script does not exist")]
    ChrError,
}

#[repr(C)]
/// Owner of all context relevant to a single execution of a talkscript. Typically, there is an
/// instance of this created per interactible NPC when a map is loaded.
pub struct TalkScript {
    pub machine_holder: Option<NonNull<TalkScriptMachineHolder>>,
    unk8: usize,
    pub env: Box<CSEzStateTalkEnv>,
    pub event: Box<CSEzStateTalkEvent>,
    pub talk_id: i32,
    pub npc_talk: Box<CSNpcTalkIns>,
    unk30: i32,
    unk34: i32,
    unk38: bool,
    unk3c: i32,
    unk40: i32,
    pub elapsed_frames: i32,
    pub elapsed_time: f32,
    unk50: usize,
    pub map_id: BlockId,
}

impl TalkScript {
    pub fn new(map_id: BlockId, talk_id: i32, field_ins_handle: FieldInsHandle) -> Self {
        let mut npc_talk = Box::new(CSNpcTalkIns::new(talk_id, field_ins_handle));
        npc_talk.menu_state.owner = Some(NonNull::from_ref(&*npc_talk));

        let env = Box::new(CSEzStateTalkEnv::new(talk_id, npc_talk.as_ref()));
        let event = Box::new(CSEzStateTalkEvent::new(talk_id, npc_talk.as_ref()));

        Self {
            machine_holder: None,
            unk8: 0,
            env,
            event,
            talk_id,
            npc_talk,
            unk30: -1,
            unk34: -1,
            unk38: true,
            unk3c: 0,
            unk40: 0,
            elapsed_frames: 0,
            elapsed_time: 0.0,
            unk50: 0,
            map_id,
        }
    }

    /// Execute a single ESD event with the given arguments.
    ///
    /// # Safety
    ///
    /// ESD events must only be called when WorldChrMan is initialized, CSMenuMan is initialized,
    /// and the NPC FieldInsHandle associated with this talk script exists. Some specific events
    /// have less restrictions.
    pub unsafe fn event_unchecked(&mut self, args: impl Into<EzStateEvent>) {
        let event_args = args.into();

        (self.event.vftable.invoke)(&mut self.event, &event_args);
    }

    /// Execute a single ESD environment query with the given arguments.
    ///
    /// # Safety
    ///
    /// ESD envs must only be called when WorldChrMan is initialized, CSMenuMan is initialized,
    /// and the NPC FieldInsHandle associated with this talk script exists. Some specific envs
    /// have less restrictions.
    pub unsafe fn env_unchecked(
        &mut self,
        args: impl Into<EzStateEnvironmentQuery>,
    ) -> EzStateValue {
        let env_args = args.into();

        let mut env_result = MaybeUninit::<EzStateRawValue>::uninit();
        (self.env.vftable.invoke)(&mut self.env, &mut env_result, &env_args);
        (&(unsafe { env_result.assume_init() })).into()
    }

    /// Execute a single ESD event with the given arguments, or return an error if the data
    /// accessed by the ESD system isn't initialized.
    pub fn event(&mut self, args: impl Into<EzStateEvent>) -> Result<(), EzStateInvokeError> {
        self.check_invoke_preconditions()?;
        unsafe { self.event_unchecked(args) }
        Ok(())
    }

    /// Execute a single ESD environment query with the given arguments, or return an error if the
    /// data accessed by the ESD system isn't initialized.
    pub fn env(
        &mut self,
        args: impl Into<EzStateEnvironmentQuery>,
    ) -> Result<EzStateValue, EzStateInvokeError> {
        self.check_invoke_preconditions()?;
        Ok(unsafe { self.env_unchecked(args) })
    }

    /// Verify that data accessed by CSEzStateTalkEvent and CSEzStateTalkEnv are available
    /// before invoking an ESD event or env.
    fn check_invoke_preconditions(&self) -> Result<(), EzStateInvokeError> {
        let _ = unsafe { CSMenuManImp::instance() }.map_err(EzStateInvokeError::MenuManError)?;

        let world_chr_man =
            unsafe { WorldChrMan::instance() }.map_err(EzStateInvokeError::WorldChrManError)?;

        let _ = world_chr_man
            .chr_ins_by_handle(&self.npc_talk.base.field_ins_handle)
            .ok_or(EzStateInvokeError::ChrError)?;

        Ok(())
    }
}

#[repr(C)]
pub struct TalkScriptMachineHolder {
    pub machine: NonNull<EzStateMachineImpl>,
    unk8: usize,
    unk10: usize,
    pub talk_id: i32,
    pub field_ins_handle: FieldInsHandle,
    unk24: bool,
    pub owner: NonNull<TalkScript>,
}

#[repr(C)]
#[derive(Default)]
pub struct OpenMenuJob {
    pub finalize_callback_job: Option<NonNull<MenuJobBase>>,
    pub input_data_count: u64,
}

#[repr(C)]
#[derive(Default)]
pub struct NpcMenuState {
    pub open_menu_job: OpenMenuJob,
    pub current_open_menu: MenuType,
    pub owner: Option<NonNull<CSNpcTalkIns>>,
}

#[repr(C)]
/// Base class of CSNpcTalkIns
pub struct CSTalkIns {
    pub talk_id: i32,
    unk4: i32,
    unk8: i32,
    unkc: i32,
    unk10: i32,
    unk14: u8,
    pub talk_interrupt_reason: i32,
    pub talk_param_id: i32,
    unk20: i32,
    unk24: f32,
    unk28: i32,
    unk2c: bool,
    unk2d: bool,
    unk2e: bool,
    unk2f: bool,
    unk30: bool,
    unk34: i32,
    unk38: bool,
    unk39: bool,
    unk3a: bool,
    unk3b: bool,
    pub field_ins_handle: FieldInsHandle,
    unk48: usize,
    unk50: f32,
    unk54: i32,
    unk58: f32,
    pub event_flag_id: i32,
    unk60: u8,
    unk64: i32,
    unk68: u8,
    unk6c: i32,
}

impl CSTalkIns {
    pub fn new(talk_id: i32, field_ins_handle: FieldInsHandle) -> Self {
        CSTalkIns {
            talk_id,
            unk4: -1,
            unk8: -1,
            unkc: -1,
            unk10: 0,
            unk14: 0u8,
            talk_interrupt_reason: -1,
            talk_param_id: -1,
            unk20: -1,
            unk24: -1.0,
            unk28: -1,
            unk2c: false,
            unk2d: false,
            unk2e: false,
            unk2f: false,
            unk30: false,
            unk34: -1,
            unk38: true,
            unk39: false,
            unk3a: false,
            unk3b: false,
            field_ins_handle,
            unk48: 0,
            unk50: -1.0,
            unk54: 0,
            unk58: 0.0,
            event_flag_id: -1,
            unk60: 0u8,
            unk64: 0,
            unk68: 0u8,
            unk6c: 0,
        }
    }
}

#[repr(C)]
/// Holds context about a talkscript being executed
///
/// Source of name: "CS::CSNpcTalkIns::_IsEventComp" event flag caller
pub struct CSNpcTalkIns {
    pub base: CSTalkIns,
    unk70: usize,
    unk78: i32,
    unk7c: f32,
    unk80: i32,
    unk84: f32,
    unk88: usize,
    unk90: usize,
    pub menu_state: Box<NpcMenuState>,
    unka0: usize,
    talk_dynamic_chr_ctrl: usize,
    unkb0: i16,
    unkb4: i32,
    unkb8: i32,
    unkbc: f32,
}

impl CSNpcTalkIns {
    pub fn new(talk_id: i32, field_ins_handle: FieldInsHandle) -> Self {
        Self {
            base: CSTalkIns::new(talk_id, field_ins_handle),
            unk70: 0,
            unk78: 0,
            unk7c: -1.0,
            unk80: 0,
            unk84: -1.0,
            unk88: 0,
            unk90: 0,
            menu_state: Box::default(),
            unka0: 0,
            talk_dynamic_chr_ctrl: 0,
            unkb0: 0,
            unkb4: 0,
            unkb8: 0,
            unkbc: 5.0,
        }
    }
}
