use std::fmt::Display;

use bitfield::bitfield;

bitfield! {
    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct BlockId(i32);
    impl Debug;

    u8;
    /// The area is generally used to indicate what type of map is being talked about.
    /// Above 60 is overworld and has special handling the engine for overworld map loading.
    pub area, _: 31, 24;
    _, set_area: 31, 24;

    /// The smallest loadable unit for a map. Region and index generally are used to refer to
    /// specific variants of the same map.
    pub block, _: 23, 16;
    _, set_block: 23, 16;

    // TODO: fact-check this term because I have no clue how region would be an appropriate name.
    pub region, _: 15, 8;
    _, set_region: 15, 8;

    pub index, _: 7, 0;
    _, set_index: 7, 0;
}

impl BlockId {
    /// BlockId -1 indicating that some entity is global or not segregated by map.
    pub const fn none() -> Self {
        Self(-1)
    }

    /// Constructs a BlockId from seperate parts.
    pub fn from_parts(area: u8, block: u8, region: u8, index: u8) -> Self {
        let mut blockid = BlockId(0);
        blockid.set_area(area);
        blockid.set_block(block);
        blockid.set_region(region);
        blockid.set_index(index);
        blockid
    }

    pub fn is_overworld(&self) -> bool {
        let area = self.area();
        (50..89).contains(&area)
    }
}

impl From<BlockId> for i32 {
    fn from(val: BlockId) -> Self {
        val.0
    }
}

impl From<i32> for BlockId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "m{:0>2}_{:0>2}_{:0>2}_{:0>2}",
            self.area(),
            self.block(),
            self.region(),
            self.index()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::cs::BlockId;

    #[test]
    fn test_bitfield() {
        let mut blockid = BlockId(0);
        blockid.set_area(61);
        blockid.set_block(57);
        blockid.set_region(39);
        blockid.set_index(3);

        assert_eq!(blockid.area(), 61);
        assert_eq!(blockid.block(), 57);
        assert_eq!(blockid.region(), 39);
        assert_eq!(blockid.index(), 3);

        assert_eq!(blockid.0, 0x3D392703);
    }
}
