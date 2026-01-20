use std::{ptr::NonNull, slice};

use thiserror::Error;

use super::FieldArea;
use crate::cxx_stl::CxxVec;
use shared::{FromStatic, OwnedPtr, UnknownStruct};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EventFlagError {
    /// The event flag is above the maximum value.
    #[error("Event flag {0} is higher than the maximum value 99999999")]
    TooHigh(u32),

    /// The event flag's area number is above the maximum value.
    #[error("Event flag area {0} must be less than 90")]
    InvalidArea(u8),
}

/// A handle pointing to a one-bit event flag in the game's event storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventFlag(u32);

/// A valid event flag
impl EventFlag {
    /// The index of this event in [EventWorld.regions]. Always in `0..10`.
    pub fn region(&self) -> u8 {
        ((self.0 / 10000000) % 10) as u8
    }

    /// The [WorldAreaInfo.area_number] for the area that this selects. Always
    /// less than 90.
    pub fn area(&self) -> u8 {
        ((self.0 / 100000) % 100) as u8
    }

    /// The [BlockInfo.group] for the area that this selects. Always in `0..10`.
    pub fn group(&self) -> u8 {
        ((self.0 / 10000) % 10) as u8
    }

    /// The index of this event in [EventBlock.zones]. Always in `0..10`.
    pub fn zone(&self) -> u8 {
        ((self.0 / 1000) % 10) as u8
    }

    /// The index of this event in [EventZone.words]. Always in `0..32`.
    pub fn word(&self) -> u8 {
        ((self.0 % 1000) / 32) as u8
    }

    /// The index of the bit that represents this event's value in its word.
    /// Always in `0..32`.
    ///
    /// This index is big-endian, in the sense that a lower index refers to a
    /// less-significant bit. This ensures that `1 << flag.bit()` will return
    /// the mask for the bit in question.
    pub fn bit(&self) -> u8 {
        // Flag IDs themselves are little-endian, so we have to subtract them
        // from 31 to make them usable in the more ergonomic big-endian way.
        31 - ((self.0 % 1000) % 32) as u8
    }

    /// Gets the block index to use when there's no global FieldArea currently
    /// available.
    fn global_block_index(&self) -> Option<u8> {
        Some(match (self.area(), self.group()) {
            (12, 1) => 0,
            (20, 1) => 1,
            (21, 0) => 3,
            (29, 0) => 4,
            (29, 1) => 5,
            (29, 2) => 6,
            _ => return None,
        })
    }
}

impl TryFrom<u32> for EventFlag {
    type Error = EventFlagError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 99999999 {
            return Err(EventFlagError::TooHigh(value));
        }

        let wrapped = EventFlag(value);
        if wrapped.area() >= 90 {
            Err(EventFlagError::InvalidArea(wrapped.area()))
        } else {
            Ok(wrapped)
        }
    }
}

#[repr(C)]
// Source of name: FD4Singleton error handling
/// The singleton that manages the game's event flags.
#[shared::singleton("SprjEventFlagMan")]
pub struct SprjEventFlagMan {
    /// A struct that owns the actual flag data.
    pub flags: FD4VirtualMemoryFlag,

    _unk230: [u8; 0x20],

    pub event_maker: EventMakerEx,
}

impl SprjEventFlagMan {
    /// Sets the state of the given [EventFlag]. Returns whether the flag was
    /// set successfully.
    pub fn set_flag(&mut self, flag: EventFlag, state: bool) -> bool {
        let word_index = flag.word() as usize;
        self.get_event_zone_mut(flag)
            .and_then(|z| {
                let old_word = z.words().get(word_index)?;
                z.words_mut()[word_index] = if state {
                    old_word | (1 << flag.bit())
                } else {
                    old_word & !(1 << flag.bit())
                };
                Some(true)
            })
            .unwrap_or_default()
    }

    /// Retrieves the state of the given [EventFlag]. Returns `false` for flags
    /// that don't exist.
    pub fn get_flag(&self, flag: EventFlag) -> bool {
        self.get_event_zone(flag)
            .and_then(|z| z.words().get(flag.word() as usize))
            .map(|word| (word >> flag.bit()) & 1 == 1)
            .unwrap_or_default()
    }

    /// Returns the [EventZone] that contains the data for the given
    /// [EventFlag].
    pub fn get_event_zone(&self, flag: EventFlag) -> Option<&EventZone> {
        self.flags
            .current_world()
            .regions
            .get(flag.region() as usize)?
            .blocks()
            .get(self.get_event_block_index(flag)? as usize)?
            .zones
            .get(flag.zone() as usize)
    }

    /// Returns the mutable [EventZone] that contains the data for the given
    /// [EventFlag].
    pub fn get_event_zone_mut(&mut self, flag: EventFlag) -> Option<&mut EventZone> {
        let block_index = self.get_event_block_index(flag)?;
        self.flags
            .current_world_mut()
            .regions
            .get_mut(flag.region() as usize)?
            .blocks_mut()
            .get_mut(block_index as usize)?
            .zones
            .get_mut(flag.zone() as usize)
    }

    /// Returns the index of the [EventBlock] that contains `flag`.
    pub fn get_event_block_index(&self, flag: EventFlag) -> Option<u32> {
        if flag.area() == 0 && flag.group() == 0 {
            Some(0)
            // Safety: If the event man is being accessed safely, the field area
            // should be accessible as well.
        } else if let Ok(field_area) = unsafe { FieldArea::instance() } {
            field_area
                .world_res()?
                .super_world_info
                .area_info()
                .iter()
                .find(|ai| ai.area_number == flag.area())
                .and_then(|ai| {
                    ai.block_info().iter().find(|bi| {
                        bi.block_id.group() == flag.group() && bi.block_id.area() == flag.area()
                    })
                })
                .map(|bi| bi.world_block_index + 1)
        } else {
            Some((flag.global_block_index()? + 1).into())
        }
    }
}

#[repr(C)]
// Source of name: Elden Ring RTTI
/// The container for the actual event data.
pub struct FD4VirtualMemoryFlag {
    /// Raw backing data for event flags. This is not guaranteed to be organized
    /// in any particular way; access events through the dedicated methods
    /// instead of directly through this buffer.
    pub data: [OwnedPtr<u32>; 2],

    /// The length in bytes of the corresponding buffers in [data](Self.data).
    pub data_length: [usize; 2],

    /// The array of event blocks. The length is stored as a u64 immediately
    /// before the head of the array.
    pub blocks: OwnedPtr<EventBlock>,

    /// The event worlds. Only one is active at a time.
    pub worlds: [EventWorld; 2],

    /// The [EventWorld] that's currently active.
    pub current_world: NonNull<EventWorld>,

    /// The index of [self.current_world] in [self.worlds].
    pub current_world_index: u32,

    pub _unk224: u32,

    /// Whether this class's data has been initialized.
    pub is_initialized: bool,
}

impl FD4VirtualMemoryFlag {
    /// Returns the currently-active [EventWorld].
    pub fn current_world(&self) -> &EventWorld {
        unsafe { self.current_world.as_ref() }
    }

    /// Returns the mutable currently-active [EventWorld].
    pub fn current_world_mut(&mut self) -> &mut EventWorld {
        unsafe { self.current_world.as_mut() }
    }
}

#[repr(C)]
// Source of name: RTTI
pub struct EventMakerEx {
    _unk00: CxxVec<UnknownStruct<0x30>>,
    _unk20: u64, // debug related?
}

#[repr(C)]
pub struct EventWorld {
    pub regions: [EventRegion; 10],

    /// The length (in bytes) of the [FD4VirtualMemoryFlag] corresponding to
    /// this world.
    pub data_length: usize,
}

#[repr(C)]
pub struct EventRegion {
    /// A pointer to the list of event blocks that are part of this region.
    pub blocks: Option<NonNull<EventBlock>>,

    /// The length of the [blocks](Self.blocks) array.
    pub blocks_length: u32,

    _blocks_length_times_1280: u64,
}

impl EventRegion {
    /// Returns the list of blocks that belong to this region.
    pub fn blocks(&self) -> &[EventBlock] {
        self.blocks
            .map(|blocks| unsafe {
                slice::from_raw_parts(blocks.as_ptr(), self.blocks_length as usize)
            })
            .unwrap_or_default()
    }

    /// Returns the mutable list of blocks that belong to this region.
    pub fn blocks_mut(&mut self) -> &mut [EventBlock] {
        self.blocks
            .map(|mut blocks| unsafe {
                slice::from_raw_parts_mut(blocks.as_mut(), self.blocks_length as usize)
            })
            .unwrap_or_default()
    }
}

#[repr(C)]
pub struct EventBlock {
    pub zones: [EventZone; 10],
    _unka0: u64,
}

#[repr(C)]
pub struct EventZone {
    pub words: NonNull<[u32; 32]>,
    _unka0: u64,
}

impl EventZone {
    /// The words in this zone.
    pub fn words(&self) -> &[u32] {
        unsafe { self.words.as_ref() }
    }

    /// The mutable words in this zone.
    pub fn words_mut(&mut self) -> &mut [u32] {
        unsafe { self.words.as_mut() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0xf8, size_of::<EventWorld>());
        assert_eq!(0x18, size_of::<EventRegion>());
        assert_eq!(0xa8, size_of::<EventBlock>());
        assert_eq!(0x10, size_of::<EventZone>());
        assert_eq!(0x28, size_of::<EventMakerEx>());
        assert_eq!(0x230, size_of::<FD4VirtualMemoryFlag>());
        assert_eq!(0x278, size_of::<SprjEventFlagMan>());
    }
}
