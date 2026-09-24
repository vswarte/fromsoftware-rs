use std::ptr::NonNull;

use crate::{DLMap, dlkr::MainHeapAllocator};

#[repr(transparent)]
pub struct EventFlag(u32);

impl From<u32> for EventFlag {
    fn from(value: u32) -> Self {
        EventFlag(value)
    }
}

impl EventFlag {
    pub fn group(&self) -> u32 {
        self.0 / 1000
    }
    pub fn byte(&self) -> u32 {
        (self.0 % 1000) / 8
    }
    pub fn bit(&self) -> u32 {
        7 - ((self.0 % 1000) % 8)
    }
}

#[repr(C)]
/// Manages the event flags for the game.
///
/// Source of name: DLRF RuntimeClass metadata
#[shared::singleton("CSEventFlagMan")]
pub struct CSEventFlagMan {
    pub virtual_memory_flag: CSFD4VirtualMemoryFlag,
    pub world_type: u32,
    unk7c: [u8; 0x1f4],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSFD4VirtualMemoryFlag {
    vftable: usize,
    pub allocator: &'static MainHeapAllocator,
    /// Exponent (base 10) used to derive [`Self::group_count`].
    ///
    /// 7 by default.
    pub group_digit_count: u32,
    /// `10^group_digit_count`. Upper bound on event flag group indices.
    pub group_count: u32,
    /// Exponent (base 10) used to derive [`Self::flags_per_block`] and [`Self::event_flag_holder_size`].
    ///
    /// 3 by default.
    pub flag_digit_count: u32,
    /// `10^flag_digit_count`. Amount of individual flags packed into one flag group.
    pub flags_per_block: u32,
    /// `(flags_per_block + 7) >> 3`. Size of one flag group in bytes.
    pub event_flag_holder_size: u32,
    /// Amount of event flag groups.
    pub event_flag_holder_count: u32,
    /// Top of the flag block structure.
    pub flag_blocks: *mut FlagBlock,
    /// Describes where to find a flag block.
    pub flag_block_descriptors: DLMap<u32, FlagBlockDescriptor>,
    unk38: [u8; 0x30],
}

impl CSFD4VirtualMemoryFlag {
    /// Sets the event flag bit for a given event flag. Does not inherently network set flags.
    pub fn set_flag(&mut self, flag: impl Into<EventFlag>, state: bool) {
        let flag: EventFlag = flag.into();
        // Copy flag_blocks pointer before the immutable borrow of self via find
        let base = self.flag_blocks;
        let ptr = self
            .flag_block_descriptors
            .find(&flag.group())
            .and_then(|d| unsafe { d.resolve(base) });

        if let Some(block) = ptr {
            unsafe { (*block).set(flag, state) };
        }
    }

    /// Retrieves the event flag current state.
    pub fn get_flag(&self, flag: impl Into<EventFlag>) -> bool {
        let flag: EventFlag = flag.into();
        let base = self.flag_blocks;
        self.flag_block_descriptors
            .find(&flag.group())
            .and_then(|d| unsafe { d.resolve(base) })
            .map(|block| unsafe { (*block).get(flag) })
            .unwrap_or(false)
    }
}

#[repr(C)]
/// Describes where to find a flag block.
pub struct FlagBlockDescriptor {
    pub location_mode: u32,
    /// Describes the location of the flag block together with location_mode.
    location: FlagBlockLocationUnion,
}

impl FlagBlockDescriptor {
    /// Resolves the descriptor to a raw `*mut FlagBlock`
    ///
    /// # Safety
    ///
    /// `base` must be a valid pointer to the flag block array
    unsafe fn resolve(&self, base: *mut FlagBlock) -> Option<*mut FlagBlock> {
        match self.location_mode {
            1 => Some(unsafe { base.add(self.location.holder_offset as usize) }),
            2 => Some(unsafe { self.location.external_location.as_ptr() }),
            _ => None,
        }
    }
}

union FlagBlockLocationUnion {
    holder_offset: u32,
    external_location: NonNull<FlagBlock>,
}

#[repr(C)]
/// Contains the actual flag bits
pub struct FlagBlock {
    bytes: [u8; 125],
}

impl FlagBlock {
    pub fn set(&mut self, flag: EventFlag, state: bool) {
        let byte = &mut self.bytes[flag.byte() as usize];
        let mask = 0b00000001 << flag.bit();

        *byte = match state {
            true => *byte | mask,
            false => *byte & !mask,
        }
    }

    pub fn get(&self, flag: EventFlag) -> bool {
        let byte = &self.bytes[flag.byte() as usize];
        let mask = 0b00000001 << flag.bit();

        (*byte & mask) != 0
    }
}
