use vtable_rs::{vtable, VPtr};

#[vtable]
pub trait EzStateEventVmt {
    fn destructor(&mut self);

    fn unk08(&mut self);

    /// Yields the event ID
    fn event_id(&self) -> u32;

    /// The amount of arguments for this event dispatch.
    fn arg_count(&self) -> u32;

    /// Yields the argument data for the argument referenced by its index.
    fn arg(&self, index: u32) -> &EzStateExternalFuncArg;
}

#[repr(C)]
pub struct EzStateEvent {
    vmt: VPtr<dyn EzStateEventVmt, Self>,
    event_id: u32,
    args: Vec<EzStateExternalFuncArg>,
}

impl EzStateEvent {
    pub fn new<'a>(
        event_id: u32,
        args: impl Iterator<Item = &'a EzStateExternalFuncArgSafe>,
    ) -> Self {
        let args = std::iter::once(EzStateExternalFuncArg {
            value: EzStateExternalFuncArgValue { int32: event_id },
            value_type: 1,
        })
        .chain(args.map(|f| (*f).into()))
        .collect();

        Self {
            vmt: Default::default(),
            event_id,
            args,
        }
    }
}

impl EzStateEventVmt for EzStateEvent {
    extern "C" fn destructor(&mut self) {
        unimplemented!()
    }

    extern "C" fn unk08(&mut self) {
        unimplemented!()
    }

    #[doc = "Yields the event ID"]
    extern "C" fn event_id(&self) -> u32 {
        tracing::info!("EzStateEvent::event_id");
        self.event_id
    }

    #[doc = "The amount of arguments for this event dispatch."]
    extern "C" fn arg_count(&self) -> u32 {
        tracing::info!("EzStateEvent::arg_count");
        TryInto::<u32>::try_into(self.args.len()).unwrap()
    }

    #[doc = "Yields the argument data for the argument referenced by its index."]
    extern "C" fn arg(&self, index: u32) -> &EzStateExternalFuncArg {
        tracing::info!("EzStateEvent::arg");
        self.args.get(index as usize).unwrap()
    }
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

#[derive(Clone, Copy)]
pub enum EzStateExternalFuncArgSafe {
    Float32(f32),
    Int32(u32),
    Unk64(u64),
}

impl From<EzStateExternalFuncArg> for EzStateExternalFuncArgSafe {
    fn from(value: EzStateExternalFuncArg) -> Self {
        match value.value_type {
            1 => Self::Float32(unsafe { value.value.float32 }),
            2 => Self::Int32(unsafe { value.value.int32 }),
            3 => Self::Unk64(unsafe { value.value.unk64 }),
            _ => unimplemented!(),
        }
    }
}

impl From<EzStateExternalFuncArgSafe> for EzStateExternalFuncArg {
    fn from(val: EzStateExternalFuncArgSafe) -> Self {
        match val {
            EzStateExternalFuncArgSafe::Float32(v) => EzStateExternalFuncArg {
                value: EzStateExternalFuncArgValue { float32: v },
                value_type: 1,
            },
            EzStateExternalFuncArgSafe::Int32(v) => EzStateExternalFuncArg {
                value: EzStateExternalFuncArgValue { int32: v },
                value_type: 2,
            },
            EzStateExternalFuncArgSafe::Unk64(v) => EzStateExternalFuncArg {
                value: EzStateExternalFuncArgValue { unk64: v },
                value_type: 3,
            },
        }
    }
}
