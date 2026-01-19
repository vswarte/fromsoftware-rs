use pelite::pe64::Pe;
use shared::Program;
use std::fmt::Debug;

use crate::{
    dlut::DLFixedVector,
    ez_state::{EzStateRawValue, EzStateValue},
    rva,
};

#[repr(C)]
/// Holds the arguments for an invocation of an ESD event (i.e. a function call with a side effect).
/// This contains an ID and between 0 and 60 argument values.
///
/// Source of name: RTTI
pub struct EzStateEvent {
    vftable: usize,
    pub id: EzStateRawValue,
    pub args: DLFixedVector<EzStateRawValue, 60>,
    unk3d8: usize,
    unk3f0: usize,
}

impl EzStateEvent {
    pub fn id(&self) -> i32 {
        let value: EzStateValue = self.id.into();
        value.into()
    }

    pub fn arg(&self, index: u32) -> Option<EzStateValue> {
        if index as usize > self.args.len() {
            None
        } else {
            Some(self.args[index as usize].into())
        }
    }
}

impl Default for EzStateEvent {
    fn default() -> Self {
        Self {
            vftable: Program::current()
                .rva_to_va(rva::get().ez_state_detail_external_event_temp_vmt)
                .unwrap() as usize,
            id: Default::default(),
            args: Default::default(),
            unk3d8: 0,
            unk3f0: 0,
        }
    }
}

impl Debug for EzStateEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EzStateEvent(")?;
        self.id.fmt(f)?;
        f.write_str(", ")?;
        self.args.as_slice().fmt(f)?;
        f.write_str(")")
    }
}

impl From<i32> for EzStateEvent {
    fn from(id: i32) -> Self {
        Self {
            id: EzStateValue::Int32(id).into(),
            ..Self::default()
        }
    }
}

impl<I> From<(i32, I)> for EzStateEvent
where
    I: IntoIterator<Item = EzStateValue>,
{
    fn from((id, args): (i32, I)) -> Self {
        let mut new = Self::from(id);
        for arg in args {
            let _ = new.args.push(arg.into());
            if new.args.len() == new.args.capacity() {
                break;
            }
        }
        new
    }
}
