use vtable_rs::VPtr;

#[vtable_rs::vtable]
pub trait CCallbackBaseVmt<P> {
    /// Called when the callback is triggered
    fn run(&mut self, pv_param: *mut P);

    /// Called when the callback is triggered as a result of a specific API call
    fn run_call(&mut self, pv_param: *mut P, io_failure: bool, api_call: u64);

    /// Returns the size of the parameter struct (P)
    fn get_callback_size_bytes(&mut self) -> i32;

    fn destructor(&mut self, should_free: bool);
}

#[repr(C)]
pub struct CCallbackBase<P: 'static> {
    pub vftable: VPtr<dyn CCallbackBaseVmt<P>, Self>,
    /// Internal Steam flags
    pub callback_flags: CallbackFlags,
    /// The unique ID for the callback type
    /// (e.g., 1101 for UserStatsReceived_t)
    pub callback_id: i32,
}

#[repr(C)]
pub struct CCallback<T, P: 'static> {
    pub base: CCallbackBase<P>,
    pub obj: *mut T,
    pub func: extern "C" fn(&mut T, *mut P),
}

bitflags::bitflags! {
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct CallbackFlags: u8 {
        const REGISTERED = 0x01;
        const GAME_SERVER = 0x02;
    }
}
