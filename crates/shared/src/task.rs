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
pub trait SharedTaskImp<IndexType: Send + 'static, DataType: Send + 'static> {
    /// Directly calls the internal task registration function. Users should not
    /// call this directly.
    fn register_task_internal(&self, index: IndexType, task: &RecurringTask<DataType>);
}

/// An extension on each game's task implementation to allow users to easily
/// register custom tasks as Rust closures.
pub trait SharedTaskImpExt<IndexType: Send + 'static, DataType: Send + 'static> {
    /// Registers the given closure as a task to the games task runtime.
    fn run_recurring<T: Into<RecurringTask<DataType>>>(
        &self,
        execute: T,
        group: IndexType,
    ) -> RecurringTaskHandle<DataType>;
}

impl<
        IndexType: Send + 'static,
        DataType: Send + 'static,
        S: SharedTaskImp<IndexType, DataType>,
    > SharedTaskImpExt<IndexType, DataType> for S
{
    fn run_recurring<T: Into<RecurringTask<DataType>>>(
        &self,
        task: T,
        group: IndexType,
    ) -> RecurringTaskHandle<DataType> {
        #[allow(clippy::arc_with_non_send_sync)]
        let task: Arc<RecurringTask<DataType>> = Arc::new(task.into());
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
pub struct RecurringTaskHandle<DataType: Send + 'static> {
    _task: Arc<RecurringTask<DataType>>,
}

impl<DataType: Send + 'static> Drop for RecurringTaskHandle<DataType> {
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
pub struct RecurringTask<DataType: Send + 'static> {
    vftable: VPtr<dyn SharedTaskBaseVmt, Self>,
    unk8: usize,
    closure: Box<dyn FnMut(&DataType)>,
    unregister_requested: AtomicBool,
    self_ref: UnsafeCell<Option<Arc<Self>>>,
}

impl<DataType: Send + 'static> RecurringTask<DataType> {
    pub fn new<F: FnMut(&DataType) + 'static + Send>(closure: F) -> Self {
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

impl<DataType: Send + 'static> SharedTaskBaseVmt for RecurringTask<DataType> {
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
        (self.closure)(unsafe { &*(data as *const DataType) });

        // }

        // TODO: implement the games unregister fn to properly get the task removed from the task
        // pool instead of just not running the closure.

        // Drop if we got cancelled in the meanwhile.
        // if self.unregister_requested.load(Ordering::Relaxed) {
        //     self.self_ref.get_mut().take();
        // }
    }
}

impl<DataType: Send + 'static, F: FnMut(&DataType) + 'static + Send> From<F>
    for RecurringTask<DataType>
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
