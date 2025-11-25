use std::ptr::NonNull;

use vtable_rs::{vtable, VPtr};

use crate::DynamicSizeSpan;

#[vtable]
pub trait EzStateEventVmt {
    fn destructor(&mut self);

    fn unk08(&mut self);

    /// Yields the event ID
    fn event_id(&self) -> u32;

    /// The amount of arguments for this event dispatch.
    fn arg_count(&self) -> u32;

    /// Yields the argument data for the argument referenced by its index.
    fn arg(&self, index: u32) -> &EzStateExternalFuncArg;
}

#[repr(C)]
pub struct EzStateEvent {
    vmt: VPtr<dyn EzStateEventVmt, Self>,
    event_id: u32,
    args: Vec<EzStateExternalFuncArg>,
}

impl EzStateEvent {
    pub fn new<'a>(
        event_id: u32,
        args: impl Iterator<Item = &'a EzStateExternalFuncArgSafe>,
    ) -> Self {
        let args = std::iter::once(EzStateExternalFuncArg {
            value: EzStateExternalFuncArgValue { int32: event_id },
            value_type: 1,
        })
        .chain(args.map(|f| (*f).into()))
        .collect();

        Self {
            vmt: Default::default(),
            event_id,
            args,
        }
    }
}

impl EzStateEventVmt for EzStateEvent {
    extern "C" fn destructor(&mut self) {
        unimplemented!()
    }

    extern "C" fn unk08(&mut self) {
        unimplemented!()
    }

    #[doc = "Yields the event ID"]
    extern "C" fn event_id(&self) -> u32 {
        tracing::info!("EzStateEvent::event_id");
        self.event_id
    }

    #[doc = "The amount of arguments for this event dispatch."]
    extern "C" fn arg_count(&self) -> u32 {
        tracing::info!("EzStateEvent::arg_count");
        TryInto::<u32>::try_into(self.args.len()).unwrap()
    }

    #[doc = "Yields the argument data for the argument referenced by its index."]
    extern "C" fn arg(&self, index: u32) -> &EzStateExternalFuncArg {
        tracing::info!("EzStateEvent::arg");
        self.args.get(index as usize).unwrap()
    }
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

#[derive(Clone, Copy)]
pub enum EzStateExternalFuncArgSafe {
    Float32(f32),
    Int32(u32),
    Unk64(u64),
}

impl From<EzStateExternalFuncArg> for EzStateExternalFuncArgSafe {
    fn from(value: EzStateExternalFuncArg) -> Self {
        match value.value_type {
            1 => Self::Float32(unsafe { value.value.float32 }),
            2 => Self::Int32(unsafe { value.value.int32 }),
            3 => Self::Unk64(unsafe { value.value.unk64 }),
            _ => unimplemented!(),
        }
    }
}

impl From<EzStateExternalFuncArgSafe> for EzStateExternalFuncArg {
    fn from(val: EzStateExternalFuncArgSafe) -> Self {
        match val {
            EzStateExternalFuncArgSafe::Float32(v) => EzStateExternalFuncArg {
                value: EzStateExternalFuncArgValue { float32: v },
                value_type: 1,
            },
            EzStateExternalFuncArgSafe::Int32(v) => EzStateExternalFuncArg {
                value: EzStateExternalFuncArgValue { int32: v },
                value_type: 2,
            },
            EzStateExternalFuncArgSafe::Unk64(v) => EzStateExternalFuncArg {
                value: EzStateExternalFuncArgValue { unk64: v },
                value_type: 3,
            },
        }
    }
}

/// Source of name: RTTI
#[repr(C)]
pub struct EzStateMachineImpl {
    vfptr: usize,
    unk8: [u8; 0x18],
    pub current_state: NonNull<EzStateMachineState>,
    pub state_group: NonNull<EzStateMachineStateGroup>,
}

#[repr(C)]
pub struct EzStateMachineStateGroup {
    pub id: i32,
    pub states: DynamicSizeSpan<EzStateMachineState>,
    pub initial_state: NonNull<EzStateMachineState>,
}

#[repr(C)]
pub struct EzStateMachineState {
    pub id: i32,
    /// Possible transitions from this state into others.
    pub transitions: DynamicSizeSpan<NonNull<EzStateMachineTransition>>,
    /// Events to run while entering this state.
    pub entry_events: DynamicSizeSpan<EzStateEvent>,
    /// Events to run while exiting this state.
    pub exit_events: DynamicSizeSpan<EzStateEvent>,
    /// Events to run while being in this state.
    pub while_events: DynamicSizeSpan<EzStateEvent>,
}

#[repr(C)]
pub struct EzStateMachineTransition {
    /// Target state to transition into
    pub target_state: Option<NonNull<EzStateMachineState>>,
    /// Events to run while passing through this transition
    pub pass_events: DynamicSizeSpan<EzStateEvent>,
    pub sub_transitions: DynamicSizeSpan<NonNull<EzStateMachineTransition>>,
    /// Check that runs to check if we should be applying this transition.
    pub evaluator: EzStateExpression,
}

#[repr(C)]
#[derive(PartialEq)]
pub struct EzStateMachineEventCommand {
    pub bank: i32,
    pub id: i32,
}

#[repr(C)]
pub struct EzStateMachineEvent {
    pub command: EzStateMachineEventCommand,
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

    /// # Safety
    ///
    /// Caller must ensure that v argument is pointing to a valid EzState expression and that the
    /// length is exactly the length of the expression in bytes.
    pub unsafe fn from_raw_parts(v: *const u8, length: usize) -> Self {
        Self(DynamicSizeSpan::from_raw_parts(v, length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<EzStateMachineImpl>(), 0x140);
    }
}
