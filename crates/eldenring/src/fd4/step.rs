use std::ptr::NonNull;

use shared::StepperStates;
use windows::core::PCWSTR;

use crate::{Tree, dlkr::DLAllocatorRef, dltx::DLString, fd4::FD4Time};

/// Source of name: RTTI
#[repr(C)]
pub struct FD4StepTemplateBase<TStates: StepperStates, TSubject> {
    // Inheritance chain: FD4ComponentBase -> FD4StepBaseInterface -> FD4StepTemplateInterface<FD4StepBaseInterface>
    vftable: *const (),
    pub stepper_fns: NonNull<TStates::StepperFnArray<StepperFn<TSubject>>>,
    pub attach: FD4ComponentAttachSystem_Step,
    /// Current state executing this frame.
    pub current_state: TStates,
    /// Target step for next frames execution.
    pub requested_state: TStates,
    unk48: u8,

    // Seemingly all debug stuff after this point.
    pub allocator: DLAllocatorRef,
    unk58: usize,
    unk60: i8,
    unk61: bool,
    unk68: DLString,
    /// State label seemingly used for debug tooling.
    /// Examples: "NotExecuting", "State Finished.(No StepMethod is Executing.)"
    pub debug_state_label: PCWSTR,
    unka0: bool,
    unka4: i32,
}

/// Single state for the stepper to be executing from.
#[repr(C)]
pub struct StepperFn<T> {
    pub executor: extern "C" fn(&mut T, &FD4Time),
    pub name: PCWSTR,
}

/// Source of name: RTTI
#[repr(C)]
pub struct FD4ComponentAttachSystem {
    vftable: *const (),
    unk8: Tree<()>,
    pub allocator: DLAllocatorRef,
}

/// Source of name: RTTI
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct FD4ComponentAttachSystem_Step {
    pub base: FD4ComponentAttachSystem,
    pub allocator: DLAllocatorRef,
}
