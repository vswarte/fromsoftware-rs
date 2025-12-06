use pelite::pe64::{Pe, Va};
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
    vftable: Va,
    arity: u32,
    values: [EzStateRawValue; 8],
}

impl EzStateEnvironmentQuery {
    pub fn id(&self) -> i32 {
        let value: EzStateValue = self.values[0].into();
        value.into()
    }

    pub fn arg(&self, index: u32) -> Option<EzStateValue> {
        if index >= self.arity {
            None
        } else {
            Some(self.values[(index + 1) as usize].into())
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
                .unwrap(),
            arity: 0,
            values: [EzStateValue::Unk64(0u64).into(); 8],
        }
    }
}

impl Debug for EzStateEnvironmentQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("EzStateEnvironmentQuery(")?;
        self.values[..self.arity as usize].fmt(f)?;
        f.write_str(")")
    }
}

macro_rules! from_slice {
    ($n:literal) => {
        impl From<[EzStateValue; $n]> for EzStateEnvironmentQuery {
            fn from(values: [EzStateValue; $n]) -> Self {
                let mut new = Self::default();
                new.arity = $n;
                for (index, value) in values.iter().enumerate() {
                    new.values[index] = (*value).into();
                }
                new
            }
        }
    };
}

from_slice!(1);
from_slice!(2);
from_slice!(3);
from_slice!(4);
from_slice!(5);
from_slice!(6);
from_slice!(7);
from_slice!(8);
