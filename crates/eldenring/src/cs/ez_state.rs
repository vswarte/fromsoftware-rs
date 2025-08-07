use std::{collections::VecDeque, ptr::NonNull};

use thiserror::Error;
use vtable_rs::vtable;

use crate::DynamicSizeSpan;

#[vtable]
pub trait EzStateEventVmt {
    fn destructor(&mut self);

    fn unk08(&mut self);

    fn event_id(&self) -> u32;

    fn arg_count(&self) -> u32;

    fn arg(&self, index: u32) -> &EzStateExternalFuncArg;
}

#[repr(C)]
pub union EzStateExternalFuncArgValue {
    pub float32: f32,
    pub int32: u32,
    pub unk64: u64,
}

#[repr(C)]
pub struct EzStateExternalFuncArg {
    pub value: EzStateExternalFuncArgValue,
    pub value_type: u32,
}

/// Source of name: RTTI
#[repr(C)]
pub struct EzStateMachineImpl {
    vfptr: usize,
    unk8: [u8; 0x18],
    pub current_state: NonNull<EzStateState>,
    pub state_group: NonNull<EzStateStateGroup>,
}

#[repr(C)]
pub struct EzStateStateGroup {
    pub id: i32,
    pub states: DynamicSizeSpan<EzStateState>,
    pub initial_state: NonNull<EzStateState>,
}

#[repr(C)]
pub struct EzStateState {
    pub id: i32,
    /// Possible transitions from this state into others.
    pub transitions: DynamicSizeSpan<NonNull<EzStateTransition>>,
    /// Events to run while entering this state.
    pub entry_events: DynamicSizeSpan<EzStateEvent>,
    /// Events to run while exiting this state.
    pub exit_events: DynamicSizeSpan<EzStateEvent>,
    /// Events to run while being in this state.
    pub while_events: DynamicSizeSpan<EzStateEvent>,
}

#[repr(C)]
pub struct EzStateTransition {
    /// Target state to transition into
    pub target_state: Option<NonNull<EzStateState>>,
    /// Events to run while passing through this transition
    pub pass_events: DynamicSizeSpan<EzStateEvent>,
    pub sub_transitions: DynamicSizeSpan<NonNull<EzStateTransition>>,
    /// Check that runs to check if we should be applying this transition.
    pub evaluator: EzStateExpression,
}

#[repr(C)]
#[derive(PartialEq)]
pub struct EzStateEventCommand {
    pub bank: i32,
    pub id: i32,
}

#[repr(C)]
pub struct EzStateEvent {
    pub command: EzStateEventCommand,
    pub arguments: DynamicSizeSpan<EzStateExpression>,
}

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct EzStateExpression(pub DynamicSizeSpan<u8>);

impl EzStateExpression {
    pub const fn from_static_slice(v: &'static [u8]) -> Self {
        Self(DynamicSizeSpan::from_static_slice(v))
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub unsafe fn from_raw_parts(v: *const u8, length: usize) -> Self {
        Self(DynamicSizeSpan::from_raw_parts(v, length))
    }
}
