use std::{marker::PhantomData, ptr::NonNull};

use shared::StepperStates;

use crate::dlkr::DLAllocatorRef;
use crate::dltx::DLString;
use crate::fd4::FD4Time;
use crate::Tree;

/// Source of name: RTTI
pub type FD4StepBase<Subject, Base, States> = FD4StepTemplateBase<Subject, Base, States>;

/// Source of name: RTTI
#[repr(C)]
pub struct FD4StepTemplateBase<Subject, Base, States: StepperStates> {
    base: FD4StepTemplateInterface<Base, Subject>,
    pub stepper_fns: NonNull<States::StepperFnArray<StepperFn<Subject>>>,
    pub attach: FD4ComponentAttachSystem_Step,

    /// Current state executing this frame.
    pub current_state: States,
    /// Target step for next frames execution.
    pub requested_state: States,
    unk48: u8,

    // Seemingly all debug stuff after this point.
    pub allocator: DLAllocatorRef,
    unk58: usize,
    unk60: i8,
    unk61: bool,
    unk68: DLString,
    /// State label seemingly used for debug tooling.
    /// Examples: "NotExecuting", "State Finished.(No StepMethod is Executing.)"
    pub debug_state_label: *const u16,
    unka0: bool,
    unka4: i32,
}

/// Source of name: RTTI
#[repr(C)]
pub struct FD4StepTemplateInterface<Base, Subject> {
    base: Base,
    _phantom_data: PhantomData<Subject>,
}

/// Source of name: RTTI
#[repr(C)]
pub struct FD4StepBaseInterface {
    vftable: *const (),
}

/// Single state for the stepper to be executing from.
#[repr(C)]
pub struct StepperFn<T> {
    pub executor: extern "C" fn(&mut T, &FD4Time),
    pub name: *const u16,
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
