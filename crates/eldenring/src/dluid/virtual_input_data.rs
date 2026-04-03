use core::slice;
use std::ptr::NonNull;

use crate::Vector;

/// Source of name: RTTI
#[repr(C)]
pub struct DLVirtualInputData {
    vftable: *const (),
    /// Corresponds to movement inputs such as Mouse, Stick and character movement keys.
    pub analog_key_info: DLVirtualAnalogKeyInfo<f32>,
    /// Corresponds to action inputs such as jump, crouch and attacks.
    pub dynamic_bitset: DynamicBitset,
}

impl DLVirtualInputData {
    pub fn get_analog(&self, index: usize) -> f32 {
        let vector = &self.analog_key_info.vector;
        if index < vector.len() {
            let items = self.analog_key_info.vector.items();
            return items[index];
        }

        0.0
    }

    pub fn set_analog(&mut self, index: usize, state: f32) {
        let vector = &mut self.analog_key_info.vector;
        if index < vector.len() {
            let items = vector.items_mut();
            items[index] = state;
        }
    }

    pub fn get_digital(&self, index: usize) -> bool {
        self.dynamic_bitset.get(index)
    }

    pub fn set_digital(&mut self, index: usize, state: bool) {
        self.dynamic_bitset.set(index, state);
    }
}

/// Source of name: RTTI
#[repr(C)]
pub struct DLVirtualAnalogKeyInfo<T> {
    vftable: *const (),
    pub vector: Vector<T>,
}

/// Source of name: RTTI
#[repr(C)]
pub struct DynamicBitset {
    vftable: *const (),
    /// Corresponds to the amount of integers (32 bit-size) required to store the bitfield.
    ///
    /// Calculated during creation as:
    ///
    /// integer_count = bit_count // 32 * 4.
    pub integer_count: usize,
    /// Bitfield that this [DynamicBitset] corresponds to.
    ///
    /// It's allocated as an array of integers with the size of `integer_count`.
    ///
    /// # SAFETY
    ///
    /// We assume the `integer_count` field is always accurate to access this.
    pub bitset: NonNull<u32>,
    allocator: *const (),
}

impl DynamicBitset {
    pub fn as_slice(&self) -> &[u32] {
        unsafe {
            let data = self.bitset.as_ptr();
            slice::from_raw_parts(data, self.integer_count)
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u32] {
        unsafe {
            let data = self.bitset.as_ptr();
            slice::from_raw_parts_mut(data, self.integer_count)
        }
    }

    pub fn get(&self, bit_index: usize) -> bool {
        let slice: &[u32] = self.as_slice();

        let index: usize = bit_index / 32;
        let row: u32 = slice[index];
        let shift: usize = bit_index & 31;

        ((row >> shift) & 1) == 1
    }

    pub fn set(&mut self, bit_index: usize, state: bool) {
        let slice = self.as_slice_mut();

        let index = bit_index / 32;
        let row = &mut slice[index];
        let shift = bit_index & 31;

        let mask = 1u32 << shift;

        *row = (*row & !mask) | ((state as u32) << shift);
    }
}
