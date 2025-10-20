use std::{
    cell::UnsafeCell,
    ffi::c_void,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use vtable_rs::VPtr;

/// The trait shared by task implementations across FSW games.
pub trait SharedTaskImp<TIndex, TTaskData: Send + 'static> {
    /// Directly calls the internal task registration function. Users should not
    /// call this directly.
    fn register_task_internal(&self, index: TIndex, task: &RecurringTask<TTaskData>);
}

/// An extension on each game's task implementation to allow users to easily
/// register custom tasks as Rust closures.
pub trait SharedTaskImpExt<TIndex, TTaskData: Send + 'static> {
    /// Registers the given closure as a task to the games task runtime.
    fn run_recurring<T: Into<RecurringTask<TTaskData>>>(
        &self,
        execute: T,
        group: TIndex,
    ) -> RecurringTaskHandle<TTaskData>;
}

impl<TIndex, TTaskData: Send + 'static, S: SharedTaskImp<TIndex, TTaskData>>
    SharedTaskImpExt<TIndex, TTaskData> for S
{
    fn run_recurring<T: Into<RecurringTask<TTaskData>>>(
        &self,
        task: T,
        group: TIndex,
    ) -> RecurringTaskHandle<TTaskData> {
        #[allow(clippy::arc_with_non_send_sync)]
        let task: Arc<RecurringTask<TTaskData>> = Arc::new(task.into());
        // SAFETY: we hold a unique reference to the contents of `arc`
        unsafe {
            *task.self_ref.get() = Some(task.clone());
        }

        self.register_task_internal(group, task.as_ref());

        RecurringTaskHandle { _task: task }
    }
}

/// A handle for the a task registered through `SharedTaskImpExt.run_recurring`
/// that allows users to cancel it later using `Drop.drop`.
pub struct RecurringTaskHandle<TTaskData: Send + 'static> {
    _task: Arc<RecurringTask<TTaskData>>,
}

impl<TTaskData: Send + 'static> Drop for RecurringTaskHandle<TTaskData> {
    fn drop(&mut self) {
        self._task.cancel();
    }
}

/// A custom task created by `fromsoftware-rs` that can masquerade as one of the
/// game's native tasks.
///
/// We assume that the subset of this structure that the games care (especially
/// the vftable) about is the same across games. So far, we know it works on both
/// DS3 and ER.
#[repr(C)]
pub struct RecurringTask<TTaskData: Send + 'static> {
    vftable: VPtr<dyn SharedTaskBaseVmt, Self>,
    unk8: usize,
    closure: Box<dyn FnMut(&TTaskData)>,
    unregister_requested: AtomicBool,
    self_ref: UnsafeCell<Option<Arc<Self>>>,
}

impl<TTaskData: Send + 'static> RecurringTask<TTaskData> {
    pub fn new<F: FnMut(&TTaskData) + 'static + Send>(closure: F) -> Self {
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

impl<TTaskData: Send + 'static> SharedTaskBaseVmt for RecurringTask<TTaskData> {
    extern "C" fn get_runtime_class(&self) -> usize {
        unimplemented!();
    }

    extern "C" fn destructor(&mut self) {
        unimplemented!();
    }

    extern "C" fn execute(&mut self, data: *const c_void) {
        // Run the task if cancellation wasn't requested.
        // if !self.unregister_requested.load(Ordering::Relaxed) {

        // SAFETY: We're declaring the type of the data in the first place.
        (self.closure)(unsafe { &*(data as *const TTaskData) });

        // }

        // TODO: implement the games unregister fn to properly get the task removed from the task
        // pool instead of just not running the closure.

        // Drop if we got cancelled in the meanwhile.
        // if self.unregister_requested.load(Ordering::Relaxed) {
        //     self.self_ref.get_mut().take();
        // }
    }
}

impl<TTaskData: Send + 'static, F: FnMut(&TTaskData) + 'static + Send> From<F>
    for RecurringTask<TTaskData>
{
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

#[vtable_rs::vtable]
trait SharedTaskBaseVmt {
    fn get_runtime_class(&self) -> usize;

    fn destructor(&mut self);

    // TODO: Make data generic once vtable-rs supports this.
    fn execute(&mut self, data: *const c_void);
}
