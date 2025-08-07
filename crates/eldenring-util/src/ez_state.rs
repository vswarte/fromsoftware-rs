use std::{collections::VecDeque, pin::Pin, ptr::NonNull};

use eldenring::{
    cs::{
        EzStateEvent, EzStateEventCommand, EzStateExpression, EzStateExternalFuncArg,
        EzStateExternalFuncArgValue, EzStateState, EzStateTransition,
    },
    DynamicSizeSpan,
};
use thiserror::Error;

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

pub trait EzStateExpressionExt {
    fn disassemble(&self) -> Result<EzStateDisassembly, EzStateDisassemblyError>;
}

impl EzStateExpressionExt for EzStateExpression {
    fn disassemble(&self) -> Result<EzStateDisassembly, EzStateDisassemblyError> {
        let bytes = self.0.as_slice();
        let mut instructions = Vec::new();
        let mut i = 0;

        // First pass to tokenize everything
        while i < bytes.len() {
            let byte = bytes[i];
            match byte {
                0x00..=0x7F => {
                    instructions.push(EzStateInstruction::PushInt((byte as i8) - 64));
                }
                0x80 => {
                    if let Some(value) = read_f32(bytes, i + 1) {
                        instructions.push(EzStateInstruction::PushFloat32(value));
                        i += 4;
                    } else {
                        instructions.push(EzStateInstruction::Unknown(byte));
                    }
                }
                0x81 => {
                    if let Some(value) = read_f64(bytes, i + 1) {
                        instructions.push(EzStateInstruction::PushFloat64(value));
                        i += 8;
                    } else {
                        instructions.push(EzStateInstruction::Unknown(byte));
                    }
                }
                0x82 => {
                    if let Some(val) = read_u32(bytes, i + 1) {
                        instructions.push(EzStateInstruction::PushInt32(val as i32));
                        i += 4;
                    } else {
                        instructions.push(EzStateInstruction::Unknown(byte));
                    }
                }
                0x84..=0x8A => {
                    let parameter_count = (byte - 0x84) as usize;
                    instructions.push(EzStateInstruction::RawCall { parameter_count });
                }
                0x8C => instructions.push(EzStateInstruction::Operation(EzStateOperation::Add)),
                0x8D => instructions.push(EzStateInstruction::Operation(EzStateOperation::Neg)),
                0x8E => instructions.push(EzStateInstruction::Operation(EzStateOperation::Sub)),
                0x8F => instructions.push(EzStateInstruction::Operation(EzStateOperation::Mul)),
                0x90 => instructions.push(EzStateInstruction::Operation(EzStateOperation::Div)),
                0x91 => instructions.push(EzStateInstruction::Operation(EzStateOperation::LessEq)),
                0x92 => {
                    instructions.push(EzStateInstruction::Operation(EzStateOperation::GreaterEq))
                }
                0x93 => instructions.push(EzStateInstruction::Operation(EzStateOperation::Less)),
                0x94 => instructions.push(EzStateInstruction::Operation(EzStateOperation::Greater)),
                0x95 => instructions.push(EzStateInstruction::Operation(EzStateOperation::Equal)),
                0x96 => {
                    instructions.push(EzStateInstruction::Operation(EzStateOperation::NotEqual))
                }
                0x98 => instructions.push(EzStateInstruction::Operation(EzStateOperation::And)),
                0x99 => instructions.push(EzStateInstruction::Operation(EzStateOperation::Or)),
                0x9A => instructions.push(EzStateInstruction::Operation(EzStateOperation::Not)),
                0xA1 => instructions.push(EzStateInstruction::ExpressionEnd),
                0xA7..=0xAE => {
                    let register_index = byte - 0xA7;
                    instructions.push(EzStateInstruction::SetRegister(register_index));
                }
                0xAF..=0xB6 => {
                    let register_index = byte - 0xAF;
                    instructions.push(EzStateInstruction::GetRegister(register_index));
                }
                0xB8 => instructions.push(EzStateInstruction::GetStateGroupArgument),
                0xB9 => instructions.push(EzStateInstruction::GetCallResult),
                0xBA => instructions.push(EzStateInstruction::Constant(0x7fffffff)),
                _ => instructions.push(EzStateInstruction::Unknown(byte)),
            }

            i += 1;
        }

        let mut condensed = Vec::new();
        let mut stack = VecDeque::new();

        // Second pass to refine output and do some light bookkeeping of the stack for function
        // calls.
        for instruction in instructions.iter() {
            match instruction {
                EzStateInstruction::PushInt(v) => stack.push_front(EzStateStackValue::Int8(*v)),
                EzStateInstruction::PushInt32(v) => stack.push_front(EzStateStackValue::Int32(*v)),
                EzStateInstruction::PushFloat32(v) => {
                    stack.push_front(EzStateStackValue::Float32(*v))
                }
                EzStateInstruction::PushFloat64(v) => {
                    stack.push_front(EzStateStackValue::Float64(*v))
                }
                EzStateInstruction::RawCall { parameter_count } => {
                    let mut arguments = Vec::with_capacity(*parameter_count);
                    for _ in 0..*parameter_count {
                        arguments.push(
                            stack
                                .pop_front()
                                .ok_or(EzStateDisassemblyError::StackEmpty)?,
                        );
                    }

                    let function_id = match stack
                        .pop_front()
                        .ok_or(EzStateDisassemblyError::StackEmpty)?
                    {
                        EzStateStackValue::Int8(v) => v as i32,
                        EzStateStackValue::Int32(v) => v,
                        EzStateStackValue::Float32(_) => return Err(EzStateDisassemblyError::IncorrectDataType),
                        EzStateStackValue::Float64(_) => return Err(EzStateDisassemblyError::IncorrectDataType),
                    };

                    condensed.push(EzStateInstruction::Call {
                        function: function_id.into(),
                        arguments,
                    });
                }
                EzStateInstruction::Call {
                    function: _,
                    arguments: _,
                } => return Err(EzStateDisassemblyError::HighLevelInstructionInLowLevelCode),
                _ => {} // EzStateInstruction::Operation(ez_state_operation) => todo!(),
                        // EzStateInstruction::Unknown(_) => todo!(),
                        // EzStateInstruction::SetRegister(_) => todo!(),
                        // EzStateInstruction::GetRegister(_) => todo!(),
                        // EzStateInstruction::Constant(_) => todo!(),
                        // EzStateInstruction::GetCallResult => todo!(),
                        // EzStateInstruction::GetStateGroupArgument => todo!(),
                        // EzStateInstruction::ExpressionEnd => todo!(),
            }
        }

        Ok(EzStateDisassembly {
            low: instructions,
            high: condensed,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum EzStateFunction {
    GetEventFlag,
    ComparePlayerInventoryNumber,
    PlayerHasTool,
    Unknown(i32),
}

impl From<i32> for EzStateFunction {
    fn from(value: i32) -> Self {
        match value {
            15 => EzStateFunction::GetEventFlag,
            47 => EzStateFunction::ComparePlayerInventoryNumber,
            230 => EzStateFunction::PlayerHasTool,
            v => EzStateFunction::Unknown(v),
        }
    }
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    if offset + 4 > bytes.len() {
        return None;
    }
    let mut buf = [0u8; 4];
    buf.copy_from_slice(&bytes[offset..offset + 4]);
    Some(u32::from_le_bytes(buf))
}

fn read_f32(bytes: &[u8], offset: usize) -> Option<f32> {
    if offset + 4 > bytes.len() {
        return None;
    }
    let mut buf = [0u8; 4];
    buf.copy_from_slice(&bytes[offset..offset + 4]);
    Some(f32::from_le_bytes(buf))
}

fn read_f64(bytes: &[u8], offset: usize) -> Option<f64> {
    if offset + 8 > bytes.len() {
        return None;
    }
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&bytes[offset..offset + 8]);
    Some(f64::from_le_bytes(buf))
}

pub const EZSTATE_COMMAND_SHOW_SHOP_MESSAGE: EzStateEventCommand =
    EzStateEventCommand { bank: 1, id: 10 };
pub const EZSTATE_COMMAND_CLOSE_SHOP_MESSAGE: EzStateEventCommand =
    EzStateEventCommand { bank: 1, id: 12 };
pub const EZSTATE_COMMAND_ADD_TALK_LIST_DATA: EzStateEventCommand =
    EzStateEventCommand { bank: 1, id: 19 };
pub const EZSTATE_COMMAND_CLEAR_TALK_LIST_DATA: EzStateEventCommand =
    EzStateEventCommand { bank: 1, id: 20 };
pub const EZSTATE_COMMAND_OPEN_REPOSITORY: EzStateEventCommand =
    EzStateEventCommand { bank: 1, id: 30 };

#[derive(PartialEq, Debug)]
pub enum EzStateOperation {
    Add,
    Neg,
    Sub,
    Mul,
    Div,
    LessEq,
    GreaterEq,
    Less,
    Greater,
    Equal,
    NotEqual,
    And,
    Or,
    Not,
}

#[derive(PartialEq, Debug)]
pub enum EzStateInstruction {
    PushInt(i8),
    PushInt32(i32),
    PushFloat32(f32),
    PushFloat64(f64),
    RawCall {
        parameter_count: usize,
    },
    Call {
        function: EzStateFunction,
        arguments: Vec<EzStateStackValue>,
    },
    Operation(EzStateOperation),
    Unknown(u8),
    SetRegister(u8),
    GetRegister(u8),
    Constant(i32),
    GetCallResult,
    GetStateGroupArgument,
    ExpressionEnd,
}

#[derive(PartialEq, Debug)]
pub enum EzStateStackValue {
    Int8(i8),
    Int32(i32),
    Float32(f32),
    Float64(f64),
}

#[derive(Debug, Error)]
pub enum EzStateDisassemblyError {
    #[error("Stack contains no further values")]
    StackEmpty,
    #[error("High level instruction found in low level code")]
    HighLevelInstructionInLowLevelCode,
    #[error("Incorrect data type for context")]
    IncorrectDataType,
}

#[derive(Debug)]
pub struct EzStateDisassembly {
    pub low: Vec<EzStateInstruction>,
    pub high: Vec<EzStateInstruction>,
}

pub struct CustomTalkScriptMenuOption {
    /// Unique index for this option in the menu. Used in the result comparison for transitions
    /// away.
    index: i32,
    /// EventTextForTalk fmg ID.
    menu_text: i32,
    /// Target state
    target_state: *const EzStateState,
}

impl CustomTalkScriptMenuOption {
    pub fn new(index: i32, menu_text: i32, target_state: *const EzStateState) -> Self {
        Self { index, menu_text, target_state }
    }
}

pub struct CustomTalkScriptMenu {
    /// Entry state to build out the presentation
    presentation_state: Pin<Box<EzStateState>>,
    /// Entry events to the presentation state
    presentation_state_entry_events: Pin<Box<[EzStateEvent]>>,
    /// Presentation state transitions
    presentation_state_transitions: Pin<Box<[NonNull<EzStateTransition>]>>,

    /// State to catch and branch based on the menu result
    branch_state: Pin<Box<EzStateState>>,
    branch_state_transitions: Pin<Box<[NonNull<EzStateTransition>]>>,

    /// Backing memory for expression spans
    code: Vec<Pin<Box<[u8]>>>,
    expressions: Pin<Box<[EzStateExpression]>>,
}

impl CustomTalkScriptMenu {
    pub fn from_options(state_index: i32, options: &[CustomTalkScriptMenuOption]) -> Self {
        let mut code: Vec<Pin<Box<[u8]>>> = Vec::with_capacity(options.len() * 3);
        let mut expressions_vec: Vec<EzStateExpression> = Vec::with_capacity(options.len() * 3);
        let mut entry_events_vec: Vec<EzStateEvent> = Vec::with_capacity(options.len() + 3);
        let mut branch_state_transitions_vec: Vec<NonNull<EzStateTransition>> =
            Vec::with_capacity(options.len());
        let mut presentation_state_transitions_vec: Vec<NonNull<EzStateTransition>> = Vec::new();

        // Add setup events
        entry_events_vec.push(Self::command(EZSTATE_COMMAND_CLOSE_SHOP_MESSAGE));
        entry_events_vec.push(Self::command(EZSTATE_COMMAND_CLEAR_TALK_LIST_DATA));

        for option in options {
            let expr_index = expressions_vec.len();

            for value in [option.index.to_le_bytes(), option.menu_text.to_le_bytes()] {
                let pinned = Box::into_pin(Self::make_i32_push(&value).into_boxed_slice());
                let span =
                    unsafe { DynamicSizeSpan::from_raw_parts(pinned.as_ptr(), pinned.len()) };
                expressions_vec.push(EzStateExpression(span));
                code.push(pinned);
            }

            {
                let pinned = Box::into_pin(vec![0x3f, 0xa1].into_boxed_slice());
                let span = unsafe { DynamicSizeSpan::from_raw_parts(pinned.as_ptr(), pinned.len()) };
                expressions_vec.push(EzStateExpression(span));
                code.push(pinned);
            }

            let args = unsafe {
                DynamicSizeSpan::from_raw_parts(
                    expressions_vec[expr_index..expr_index + 3].as_ptr(),
                    3,
                )
            };

            entry_events_vec.push(EzStateEvent {
                command: EZSTATE_COMMAND_ADD_TALK_LIST_DATA,
                arguments: args,
            });

            {
                let pinned = Box::into_pin(Self::make_talk_list_result_evaluator(option.index).into_boxed_slice());
                let span = unsafe { DynamicSizeSpan::from_raw_parts(pinned.as_ptr(), pinned.len()) };
                code.push(pinned);

                let transition = EzStateTransition {
                    target_state: Some(NonNull::from_ref(unsafe { option.target_state.as_ref() }.unwrap())),
                    pass_events: DynamicSizeSpan::empty(),
                    sub_transitions: DynamicSizeSpan::empty(),
                    evaluator: EzStateExpression(span),
                };
                branch_state_transitions_vec.push(NonNull::from_ref(Box::leak(Box::new(transition))));
            }
        }

        let branch_state_transitions: Pin<Box<[NonNull<EzStateTransition>]>> =
            branch_state_transitions_vec.into_boxed_slice().into();
        let branch_trans_span = unsafe {
            DynamicSizeSpan::from_raw_parts(
                branch_state_transitions.as_ptr(),
                branch_state_transitions.len(),
            )
        };

        let branch_state = Box::pin(EzStateState {
            id: 100 + state_index,
            transitions: branch_trans_span,
            entry_events: DynamicSizeSpan::empty(),
            exit_events: DynamicSizeSpan::empty(),
            while_events: DynamicSizeSpan::empty(),
        });

        {
            let transition = EzStateTransition {
                target_state: Some(NonNull::from_ref(&branch_state)),
                pass_events: DynamicSizeSpan::empty(),
                sub_transitions: DynamicSizeSpan::empty(),
                evaluator: EzStateExpression::from_static_slice(CLOSE_SHOP_MENU_EVALUATOR),
            };

            presentation_state_transitions_vec
                .push(NonNull::from_ref(Box::leak(Box::new(transition))));
        }

        entry_events_vec.push(EzStateEvent {
            command: EZSTATE_COMMAND_SHOW_SHOP_MESSAGE,
            arguments: DynamicSizeSpan::from_static_slice(SHOW_SHOP_MESSAGE_ARGS),
        });

        let presentation_state_entry_events: Pin<Box<[EzStateEvent]>> =
            entry_events_vec.into_boxed_slice().into();
        let expressions = expressions_vec.into_boxed_slice().into();
        let presentation_state_transitions: Pin<Box<[NonNull<EzStateTransition>]>> =
            presentation_state_transitions_vec.into_boxed_slice().into();

        // Create spans after pinning
        let entry_span = unsafe {
            DynamicSizeSpan::from_raw_parts(
                presentation_state_entry_events.as_ptr(),
                presentation_state_entry_events.len(),
            )
        };

        let presentation_trans_span = unsafe {
            DynamicSizeSpan::from_raw_parts(
                presentation_state_transitions.as_ptr(),
                presentation_state_transitions.len(),
            )
        };

        let presentation_state = Box::pin(EzStateState {
            id: state_index,
            transitions: presentation_trans_span,
            entry_events: entry_span,
            exit_events: DynamicSizeSpan::empty(),
            while_events: DynamicSizeSpan::empty(),
        });

        Self {
            presentation_state,
            presentation_state_entry_events,
            presentation_state_transitions,
            branch_state,
            branch_state_transitions,
            code,
            expressions,
        }
    }

    fn make_i32_push(bytes: &[u8]) -> Vec<u8> {
        [vec![0x82], bytes.to_vec(), vec![0xa1]].concat()
    }

    fn make_talk_list_result_evaluator(index: i32) -> Vec<u8> {
        [
            vec![0x57, 0x84, 0x82],
            index.to_le_bytes().to_vec(),
            vec![0x95, 0xa1],
        ]
        .concat()
    }

    fn command(command: EzStateEventCommand) -> EzStateEvent {
        EzStateEvent {
            command,
            arguments: DynamicSizeSpan::empty(),
        }
    }

    pub fn entry_state(&self) -> &EzStateState {
        &self.presentation_state
    }
}

const EZSTATE_PUSH_1: &[u8] = &[0x82, 0x01, 0x00, 0x00, 0x00, 0xA1];

const CLOSE_SHOP_MENU_EVALUATOR: &[u8] = &[
    0x7b,  // 59 (CheckSpecificPersonMenuIsOpen)
    0x41,  // 1
    0x40,  // 0
    0x86,  // call with 2 args
    0x41,  // 1
    0x95,  // ==
    0x7a,  // 58 (CheckSpecificPersonGenericDialogIsOpen)
    0x40,  // 0
    0x85,  // call with 1 arg
    0x40,  // 0
    0x95,  // ==
    0x98,  // &&
    0x40,  // 0
    0x95,  // ==
    0xa1   // end
];

const SHOW_SHOP_MESSAGE_ARGS: &[EzStateExpression] = &[EzStateExpression::from_static_slice(EZSTATE_PUSH_1)];
