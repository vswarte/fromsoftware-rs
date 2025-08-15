use std::fmt::Display;

/// Refers to a part of the games overal map.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId(pub i32);

impl BlockId {
    /// BlockId -1 indicating that some entity is global or not segregated by map.
    pub const fn none() -> Self {
        Self::from_parts(-1, -1, -1, -1)
    }

    /// Constructs a BlockId from seperate parts.
    pub const fn from_parts(area: i8, block: i8, region: i8, index: i8) -> Self {
        Self((index as i32) | (region as i32) << 8 | (block as i32) << 16 | (area as i32) << 24)
    }

    /// The area is generally used to indicate what type of map is being talked about.
    /// Above 60 is overworld and has special handling the engine for overworld map loading.
    pub const fn area(&self) -> i32 {
        self.0 >> 24 & 0xFF
    }

    /// The smallest loadable unit for a map. Region and index generally are used to refer to
    /// specific variants of the same map.
    pub const fn block(&self) -> i32 {
        self.0 >> 16 & 0xFF
    }

    // TODO: fact-check this term because I have no clue how regio nwould be an appropriate name.
    pub const fn region(&self) -> i32 {
        self.0 >> 8 & 0xFF
    }

    /// Seemingly used for the 
    pub const fn index(&self) -> i32 {
        self.0 & 0xFF
    }

    pub const fn is_overworld(&self) -> bool {
        self.area() >= 50 && self.area() < 89
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
