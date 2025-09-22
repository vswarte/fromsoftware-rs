use std::ptr::NonNull;

use windows::core::PCWSTR;

use crate::{dlkr::DLAllocatorBase, dltx::DLString, Tree};

use super::FD4TaskBase;

/// Source of name: RTTI
#[repr(C)]
pub struct FD4StepTemplateBase<const N: usize, T> {
    pub task: FD4TaskBase,
    pub stepper_fns: NonNull<[StepperFn<T>; N]>,
    unk18: FD4StepTemplateBase0x18,
    /// Index into the stepper_fns array.
    pub current_state: u32,
    /// Target step for next cycle.
    pub request_state: u32,
    unk50: bool,
    unk51: [u8; 7],
    allocator: NonNull<DLAllocatorBase>,
    unk60: usize,
    unk68: i8,
    unk69: bool,
    _pad6a: [u8; 6],
    unk70: DLString,
    state_description: PCWSTR,
    unka8: bool,
    unka9: [u8; 3],
}

impl<const N: usize, T> AsRef<FD4TaskBase> for FD4StepTemplateBase<N, T> {
    fn as_ref(&self) -> &FD4TaskBase {
        &self.task
    }
}

/// Single state for the stepper to be executing from.
#[repr(C)]
pub struct StepperFn<T> {
    pub executor: fn(&mut T, usize),
    name: PCWSTR,
}

impl<T> StepperFn<T> {
    pub fn name(&self) -> String {
        unsafe { self.name.to_string().unwrap() }
    }
}

#[repr(C)]
pub struct FD4StepTemplateBase0x18 {
    unk0: NonNull<()>,
    unk8: Tree<()>,
    unk20: NonNull<DLAllocatorBase>,
    unk28: NonNull<DLAllocatorBase>,
}

/// Source of name: RTTI
#[repr(C)]
pub struct FD4StepBaseInterface<const N: usize, T> {
    vftable: usize,
    pub stepper_fns: NonNull<[StepperFn<T>; N]>,
    unk10: NonNull<()>,
}
