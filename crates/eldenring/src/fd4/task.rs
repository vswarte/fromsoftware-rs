use std::{
    cell::UnsafeCell,
    ptr::NonNull,
    sync::{atomic::AtomicBool, Arc},
};

use vtable_rs::VPtr;

use crate::{dlrf::DLRuntimeClass, Tree, Vector};

use super::FD4Time;

#[repr(C)]
#[derive(Debug)]
pub struct FD4TaskData {
    pub delta_time: FD4Time,
    pub task_group_id: u32,
    pub seed: i32,
}

#[vtable_rs::vtable]
pub trait FD4TaskBaseVmt {
    fn get_runtime_class(&self) -> &DLRuntimeClass;

    fn destructor(&mut self);

    fn execute(&mut self, data: &FD4TaskData);
}

#[repr(C)]
pub struct FD4TaskBase {
    pub vftable: VPtr<dyn FD4TaskBaseVmt, Self>,
    unk8: u32,
    _padc: u32,
}

impl FD4TaskBaseVmt for FD4TaskBase {
    extern "C" fn get_runtime_class(&self) -> &DLRuntimeClass {
        unimplemented!()
    }

    extern "C" fn destructor(&mut self) {
        unimplemented!()
    }

    extern "C" fn execute(&mut self, data: &FD4TaskData) {
        unimplemented!()
    }
}

#[repr(C)]
pub struct FD4TaskQueue {
    vftable: usize,
    allocator: usize,
    pub entries_tree: Tree<FD4TaskGroup>,
    pub entries_vector: Vector<FD4TaskGroup>,
}

#[repr(C)]
pub struct FD4TaskGroup {
    vftable: usize,
}

#[repr(C)]
pub struct FD4TaskRequestEntry {
    pub task: NonNull<FD4TaskBase>,
}
