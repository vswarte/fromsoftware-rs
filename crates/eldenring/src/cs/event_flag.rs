use std::mem::ManuallyDrop;

use crate::Tree;
use shared::OwnedPtr;

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
    allocator: usize,
    unk10: u32,
    unk14: u32,
    unk18: u32,
    /// Used to determine the event flag group.
    pub event_flag_divisor: u32,
    /// Size of an event flag group in bytes.
    pub event_flag_holder_size: u32,
    /// Amount of event flag groups.
    pub event_flag_holder_count: u32,
    /// Top of the flag block structure.
    pub flag_blocks: *mut FlagBlock,
    /// Describes where to find a flag block.
    pub flag_block_descriptors: Tree<FlagBlockDescriptor>,
    unk38: [u8; 0x30],
}

impl CSFD4VirtualMemoryFlag {
    /// Sets the event flag bit for a given event flag. Does not inherently network set flags.
    pub fn set_flag(&mut self, flag: impl Into<EventFlag>, state: bool) {
        let flag: EventFlag = flag.into();
        let Some(group) = self
            .flag_block_descriptors
            .iter()
            .find(|d| d.group == flag.group())
        else {
            return;
        };

        let Some(location) = self.flag_block(group) else {
            return;
        };

        location.set(flag, state)
    }

    /// Retrieves the event flag current state.
    pub fn get_flag(&self, flag: impl Into<EventFlag>) -> bool {
        let flag: EventFlag = flag.into();
        let Some(group) = self
            .flag_block_descriptors
            .iter()
            .find(|d| d.group == flag.group())
        else {
            return false;
        };

        let Some(location) = self.flag_block(group) else {
            return false;
        };

        location.get(flag)
    }

    /// Locates a flag block for a given FlagBlockDescriptor.
    fn flag_block<'a>(&self, descriptor: &'a mut FlagBlockDescriptor) -> Option<&'a mut FlagBlock> {
        Some(match descriptor.location_mode {
            1 => unsafe {
                self.flag_blocks
                    .add(descriptor.location.holder_offset as usize)
                    .as_mut()?
            },
            2 => unsafe { (*descriptor.location.external_location).as_mut() },
            _ => return None,
        })
    }
}

#[repr(C)]
/// Describes where to find a flag block.
pub struct FlagBlockDescriptor {
    pub group: u32,
    unk4: u32,
    pub location_mode: u32,
    unkc: u32,
    /// Describes the location of the flag block together with location_mode.
    location: FlagBlockLocationUnion,
}

union FlagBlockLocationUnion {
    holder_offset: u32,
    external_location: ManuallyDrop<OwnedPtr<FlagBlock>>,
}

pub enum FlagBlockLocation {
    HolderOffset(u32),
    External(OwnedPtr<FlagBlock>),
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
