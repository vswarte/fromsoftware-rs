/// The game has a few different coordinate spaces and it's constantly translating between them.
///
/// The most notable ones probably are:
/// - block position (which is how assets, regions, etc are placed)
/// - "global" world position (which seems used primarily used by map cleanup and LOD code).
/// - havok position (seems to be AABB broadphase space, often used where a lot of collision checking happens like the effective player position, the camera position, anything that needs raycasting, etc).
/// - map position (positions on the in-game map, used for pins and the like).
///
/// Converting from one space to another space usually requires some additional data about the new
/// space the coordinates are moving into or from. For example going from block local to world "global"
/// coords requires knowing the world coordinates of the block center and going from havok position
/// to block position requires either the block position of the havok aabb center or reference
/// coordinate where both block and havok position are known.
///
/// One nice thing about both havok and block space is that they both operate on meters and 1 meter
/// represents the same distance. Therefor displacements can be made in one system and then applied
/// to another.
use std::ops::{Add, Sub};

/// Represents a position relative to some block center and character's yaw.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BlockPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
}

impl BlockPosition {
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, yaw: 0.0 }
    }
}

impl Sub for BlockPosition {
    type Output = PositionDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        PositionDelta {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add<PositionDelta> for BlockPosition {
    type Output = Self;

    fn add(self, rhs: PositionDelta) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            yaw: 0.0,
        }
    }
}

impl Sub<PositionDelta> for BlockPosition {
    type Output = Self;

    fn sub(self, rhs: PositionDelta) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            yaw: 0.0,
        }
    }
}

/// Represents a position in havok physics space
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HavokPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Sub for HavokPosition {
    type Output = PositionDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        PositionDelta {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add<PositionDelta> for HavokPosition {
    type Output = Self;

    fn add(self, rhs: PositionDelta) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w,
        }
    }
}

impl Sub<PositionDelta> for HavokPosition {
    type Output = Self;

    fn sub(self, rhs: PositionDelta) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w,
        }
    }
}

impl HavokPosition {
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }
}

/// Represents a delta or displacement that applies to either coordinate system.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositionDelta {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PositionDelta {
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

/// A (potentially non-normal) directional vector.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectionalVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl DirectionalVector {
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }
}

#[cfg(test)]
mod test {
    use crate::position::{BlockPosition, PositionDelta};

    use super::HavokPosition;

    #[test]
    fn havok_position_sub_works() {
        assert_eq!(
            PositionDelta::from_xyz(-1.0, -1.0, -1.0),
            HavokPosition::from_xyz(1.0, 1.0, 1.0) - HavokPosition::from_xyz(2.0, 2.0, 2.0)
        );
    }

    #[test]
    fn block_position_sub_works() {
        assert_eq!(
            PositionDelta::from_xyz(-1.0, -1.0, -1.0),
            BlockPosition::from_xyz(1.0, 1.0, 1.0) - BlockPosition::from_xyz(2.0, 2.0, 2.0)
        );
    }

    #[test]
    fn position_displacement_can_be_applied_to_both_systems() {
        let delta = BlockPosition::from_xyz(2.0, 2.0, 2.0) - BlockPosition::from_xyz(1.0, 1.0, 1.0);

        assert_eq!(
            HavokPosition::from_xyz(3.0, 3.0, 3.0),
            HavokPosition::from_xyz(2.0, 2.0, 2.0) + delta,
        );
        assert_eq!(
            HavokPosition::from_xyz(1.0, 1.0, 1.0),
            HavokPosition::from_xyz(2.0, 2.0, 2.0) - delta,
        );
        assert_eq!(
            BlockPosition::from_xyz(3.0, 3.0, 3.0),
            BlockPosition::from_xyz(2.0, 2.0, 2.0) + delta,
        );
        assert_eq!(
            BlockPosition::from_xyz(1.0, 1.0, 1.0),
            BlockPosition::from_xyz(2.0, 2.0, 2.0) - delta,
        );
    }
}
