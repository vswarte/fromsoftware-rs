use std::fmt::Display;

use bitfield::bitfield;

bitfield! {
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    // #FIXME: replace with i32 after
    // https://github.com/dzamlo/rust-bitfield/pull/59 merged
    pub struct MapId(u32);
    impl Debug;

    i32;
    pub area, _: 31, 24;
    _, set_area: 31, 24;

    pub block, _: 23, 16;
    _, set_block: 23, 16;

    pub region, _: 15, 8;
    _, set_region: 15, 8;

    pub index, _: 7, 0;
    _, set_index: 7, 0;
}

impl MapId {
    /// MapId -1 indicating that some entity is global or not segregated by map.
    pub const fn none() -> Self {
        Self(u32::MAX)
    }

    /// Constructs a MapId from seperate parts.
    pub fn from_parts(area: i8, block: i8, region: i8, index: i8) -> Self {
        let mut mapid = MapId(0);
        mapid.set_area(area as i32);
        mapid.set_block(block as i32);
        mapid.set_region(region as i32);
        mapid.set_index(index as i32);
        mapid
    }

    pub fn is_overworld(&self) -> bool {
        let area = self.area();
        (50..89).contains(&area)
    }
}

impl From<MapId> for i32 {
    fn from(val: MapId) -> Self {
        val.0 as i32
    }
}

impl From<i32> for MapId {
    fn from(value: i32) -> Self {
        Self(value as u32)
    }
}

impl Display for MapId {
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
    use crate::cs::MapId;

    #[test]
    fn test_bitfield() {
        let mut mapid = MapId(0);
        mapid.set_area(61);
        mapid.set_block(48);
        mapid.set_region(10);
        mapid.set_index(3);

        assert_eq!(mapid.area(), 61);
        assert_eq!(mapid.block(), 48);
        assert_eq!(mapid.region(), 10);
        assert_eq!(mapid.index(), 3);

        assert_eq!(mapid.0, 0x3d300a03);
    }
}
