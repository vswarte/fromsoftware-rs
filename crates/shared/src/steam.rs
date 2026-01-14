use vtable_rs::VPtr;

use crate::OwnedPtr;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct CCallback {
    vftable: VPtr<dyn CCallbackVmt, Self>,
    unk8: u8,
    unkc: u32,
    /// Pointer to the structure passed down to the callback.
    subject: OwnedPtr<()>,
    /// Pointer to the function that should be called.
    function: isize,
}

#[vtable_rs::vtable]
trait CCallbackVmt {
    fn run(&mut self, data: isize);

    fn run_other(&mut self, data: isize, p3: u64, p4: bool);

    fn get_callback_size_bytes(&mut self) -> u32;

    fn destructor(&mut self);
}
