use std::ops::{Deref, DerefMut};

use shared::{Subclass, Superclass};

use crate::fd4::FD4BaseKeyAssign;

/// Source of names: RTTI
#[repr(C)]
#[derive(Superclass)]
#[superclass(children(CSInGameKeyAssign, CSMenuViewerKeyAssign))]
pub struct CSKeyAssign {
    base: FD4BaseKeyAssign,
}

impl CSKeyAssign {
    pub fn get_virtual_input_index(&self, mapped_input: i32) -> Option<i32> {
        let virtual_input_index_map = unsafe { self.virtual_input_data_index_map.as_ref() };
        virtual_input_index_map
            .iter()
            .find(|pair| pair.key == mapped_input)
            .filter(|pair| pair.value != -1)
            .map(|pair| pair.value)
    }
}

impl Deref for CSKeyAssign {
    type Target = FD4BaseKeyAssign;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for CSKeyAssign {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

#[repr(C)]
#[derive(Subclass)]
pub struct CSInGameKeyAssign {
    base: CSKeyAssign,
}

#[repr(C)]
#[derive(Subclass)]
pub struct CSMenuViewerKeyAssign {
    base: CSKeyAssign,
}
