use std::{
    cell::UnsafeCell,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use eldenring::fd4::FD4TaskData;
use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp},
    dlrf::DLRuntimeClass,
    fd4::FD4TaskBaseVmt,
};
use pelite::pe64::Pe;
use pelite::{pattern, pattern::Atom};
use shared::program::Program;
use std::sync::LazyLock;
use vtable_rs::VPtr;

const REGISTER_TASK_PATTERN: &[Atom] =
    pattern!("e8 ? ? ? ? 48 8b 0d ? ? ? ? 4c 8b c7 8b d3 e8 $ { ' }");

static REGISTER_TASK_VA: LazyLock<u64> = LazyLock::new(|| {
    let program = Program::current();
    let mut matches = [0u32; 2];

    if !program
        .scanner()
        .finds_code(REGISTER_TASK_PATTERN, &mut matches)
    {
        panic!("Could not find REGISTER_TASK_PATTERN or found duplicates.");
    }

    program
        .rva_to_va(matches[1])
        .expect("Call target for REGISTER_TASK_PATTERN was not in exe")
});

pub trait CSTaskImpExt {
    /// Registers the given closure as a task to the games task runtime.
    fn run_recurring<T: Into<RecurringTask>>(
        &self,
        execute: T,
        group: CSTaskGroupIndex,
    ) -> RecurringTaskHandle;
}

impl CSTaskImpExt for CSTaskImp {
    fn run_recurring<T: Into<RecurringTask>>(
        &self,
        task: T,
        group: CSTaskGroupIndex,
    ) -> RecurringTaskHandle {
        let register_task: extern "C" fn(&CSTaskImp, CSTaskGroupIndex, &RecurringTask) =
            unsafe { std::mem::transmute(*REGISTER_TASK_VA) };

        #[allow(clippy::arc_with_non_send_sync)]
        let task: Arc<RecurringTask> = Arc::new(task.into());
        // SAFETY: we hold a unique reference to the contents of `arc`
        unsafe {
            *task.self_ref.get() = Some(task.clone());
        }

        register_task(self, group, task.as_ref());

        RecurringTaskHandle { _task: task }
    }
}

pub struct RecurringTaskHandle {
    _task: Arc<RecurringTask>,
}

impl Drop for RecurringTaskHandle {
    fn drop(&mut self) {
        self._task.cancel();
    }
}

#[repr(C)]
pub struct RecurringTask {
    vftable: VPtr<dyn FD4TaskBaseVmt, Self>,
    unk8: usize,
    closure: Box<dyn FnMut(&FD4TaskData)>,
    unregister_requested: AtomicBool,
    self_ref: UnsafeCell<Option<Arc<Self>>>,
}

impl FD4TaskBaseVmt for RecurringTask {
    extern "C" fn get_runtime_class(&self) -> &DLRuntimeClass {
        unimplemented!();
    }

    extern "C" fn destructor(&mut self) {
        unimplemented!();
    }

    extern "C" fn execute(&mut self, data: &FD4TaskData) {
        // Run the task if cancellation wasn't requested.
        // if !self.unregister_requested.load(Ordering::Relaxed) {
        (self.closure)(data);
        // }

        // TODO: implement the games unregister fn to properly get the task removed from the task
        // pool instead of just not running the closure.

        // Drop if we got cancelled in the meanwhile.
        // if self.unregister_requested.load(Ordering::Relaxed) {
        //     self.self_ref.get_mut().take();
        // }
    }
}

impl RecurringTask {
    pub fn new<F: FnMut(&FD4TaskData) + 'static + Send>(closure: F) -> Self {
        Self {
            vftable: Default::default(),
            unk8: 0,
            closure: Box::new(closure),
            unregister_requested: AtomicBool::new(false),
            self_ref: UnsafeCell::new(None),
        }
    }

    pub fn cancel(&self) {
        self.unregister_requested.store(true, Ordering::Relaxed);
    }
}

impl<F: FnMut(&FD4TaskData) + 'static + Send> From<F> for RecurringTask {
    fn from(value: F) -> Self {
        Self::new(value)
    }
}
