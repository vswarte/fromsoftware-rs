use vtable_rs::vtable;

#[vtable]
pub trait EzStateEventVmt {
    fn destructor(&mut self);

    fn unk08(&mut self);

    fn event_id(&self) -> u32;

    fn arg_count(&self) -> u32;

    fn arg(&self, index: u32) -> &EzStateExternalFuncArg;
}

#[repr(C)]
pub union EzStateExternalFuncArgValue {
    pub float32: f32,
    pub int32: u32,
    pub unk64: u64,
}

#[repr(C)]
pub struct EzStateExternalFuncArg {
    pub value: EzStateExternalFuncArgValue,
    pub value_type: u32,
}
