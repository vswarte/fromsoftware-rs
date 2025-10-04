use super::{F32Matrix4x4, F32Vector3, F32Vector4};

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    /// The starting point of the segment.
    pub origin: F32Vector4,
    /// The direction and length of the segment from the origin.
    pub dir: F32Vector4,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    /// The starting point of the ray.
    pub origin: F32Vector4,
    /// The direction of the ray.
    pub dir: F32Vector4,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    /// A point on the line.
    pub origin: F32Vector4,
    /// The direction of the line.
    pub dir: F32Vector4,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    /// The first edge vector from the origin.
    pub edge1: F32Vector4,
    /// The second edge vector from the origin.
    pub edge2: F32Vector4,
    /// The origin point of the rectangle.
    pub origin: F32Vector4,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    /// The plane equation `(nx, ny, nz, d)`.
    pub plane: F32Vector4,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    /// The sphere's center `(x, y, z)` and radius `w`.
    pub sphere: F32Vector4,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    /// The minimum corner of the box.
    pub min: F32Vector4,
    /// The maximum corner of the box.
    pub max: F32Vector4,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Obb {
    /// The half-size of the box along its local axes.
    pub extents: F32Vector4,
    /// The transformation matrix from local to world space.
    pub xform: F32Matrix4x4,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lss {
    /// The central line segment.
    pub segment: Segment,
    /// The radius of the capsule.
    pub radius: f32,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rss {
    /// The central rectangle.
    pub rectangle: Rectangle,
    /// The radius of the swept sphere.
    pub radius: f32,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    /// The first vertex of the triangle.
    pub origin: F32Vector4,
    /// The vector from the origin to the second vertex.
    pub edge1: F32Vector4,
    /// The vector from the origin to the third vertex.
    pub edge2: F32Vector4,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle3 {
    /// The first vertex of the triangle.
    pub origin: F32Vector3,
    /// The vector from the origin to the second vertex.
    pub edge1: F32Vector3,
    /// The vector from the origin to the third vertex.
    pub edge2: F32Vector3,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frustum {
    /// The six planes that define the frustum volume.
    pub planes: [Plane; 6],
}
