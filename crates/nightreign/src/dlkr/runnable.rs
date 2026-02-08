#[vtable_rs::vtable]
pub trait DLRunnableVmt {
    /// Runs the runnable.
    fn run(&mut self);

    fn destructor(&mut self);
}
