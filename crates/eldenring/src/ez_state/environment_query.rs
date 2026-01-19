use pelite::pe64::Pe;
use shared::Program;
use std::fmt::Debug;

use crate::{
    ez_state::{EzStateRawValue, EzStateValue},
    rva,
};

#[repr(C)]
/// Holds the arguments for an invocation of an ESD query (i.e. a function call that returns a
/// value). This contains between 1 and 8 values, since the ID is at index 0 and there is a maximum
/// capacity of 7 arguments.
///
/// Source of name: RTTI
pub struct EzStateEnvironmentQuery {
    vftable: usize,
    arity: u32,
    id: EzStateRawValue,
    args: [EzStateRawValue; 7],
}

impl EzStateEnvironmentQuery {
    pub fn id(&self) -> i32 {
        let value: EzStateValue = self.id.into();
        value.into()
    }

    pub fn arg(&self, index: u32) -> Option<EzStateValue> {
        if index >= self.arity {
            None
        } else {
            Some(self.args[index as usize].into())
        }
    }

    pub fn arg_count(&self) -> u32 {
        self.arity - 1
    }
}

impl Default for EzStateEnvironmentQuery {
    fn default() -> Self {
        Self {
            vftable: Program::current()
                .rva_to_va(rva::get().ez_state_environment_query_impl_vmt)
                .unwrap() as usize,
            arity: 1,
            id: EzStateValue::Int32(0).into(),
            args: [EzStateValue::Int32(0).into(); 7],
        }
    }
}

impl Debug for EzStateEnvironmentQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EzStateEnvironmentQuery(")?;
        self.id.fmt(f)?;
        f.write_str(", ")?;
        self.args[..self.arity as usize].fmt(f)?;
        f.write_str(")")
    }
}

impl From<i32> for EzStateEnvironmentQuery {
    fn from(id: i32) -> Self {
        Self {
            id: EzStateValue::Int32(id).into(),
            ..Self::default()
        }
    }
}

impl<I> From<(i32, I)> for EzStateEnvironmentQuery
where
    I: IntoIterator<Item = EzStateValue>,
{
    fn from((id, args): (i32, I)) -> Self {
        let mut new = Self::from(id);
        for arg in args {
            new.args[new.arg_count() as usize] = arg.into();
            new.arity += 1;
            if new.arg_count() as usize == new.args.len() {
                break;
            }
        }
        new
    }
}
