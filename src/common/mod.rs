//! Common types

use std::cmp::Ordering;

/// Vec3
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vec2 {
    /// X coordinate
    pub x: isize,
    /// Y coordinate
    pub y: isize,
}

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vec2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            o => o,
        }
    }
}

impl From<(isize, isize)> for Vec2 {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl From<Vec2> for (isize, isize) {
    fn from(vec: Vec2) -> Self {
        (vec.x, vec.y)
    }
}

/// Vec3
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vec3 {
    /// X coordinate
    pub x: isize,
    /// Y coordinate
    pub y: isize,
    /// Z coordinate
    pub z: isize,
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vec3 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.z.cmp(&other.z) {
            Ordering::Equal => match self.y.cmp(&other.y) {
                Ordering::Equal => self.x.cmp(&other.x),
                o => o,
            },
            o => o,
        }
    }
}

impl From<(isize, isize, isize)> for Vec3 {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Self { x, y, z }
    }
}

impl From<Vec3> for (isize, isize, isize) {
    fn from(vec: Vec3) -> Self {
        (vec.x, vec.y, vec.z)
    }
}

/// Vec4
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Vec4 {
    /// X coordinate
    pub x: isize,
    /// Y coordinate
    pub y: isize,
    /// Z coordinate
    pub z: isize,
    /// T coordinate
    pub t: isize,
}

impl PartialOrd for Vec4 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vec4 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.t.cmp(&other.t) {
            Ordering::Equal => match self.z.cmp(&other.z) {
                Ordering::Equal => match self.y.cmp(&other.y) {
                    Ordering::Equal => self.x.cmp(&other.x),
                    o => o,
                },
                o => o,
            },
            o => o,
        }
    }
}

impl From<(isize, isize, isize, isize)> for Vec4 {
    fn from((x, y, z, t): (isize, isize, isize, isize)) -> Self {
        Self { x, y, z, t }
    }
}

impl From<Vec4> for (isize, isize, isize, isize) {
    fn from(vec: Vec4) -> Self {
        (vec.x, vec.y, vec.z, vec.t)
    }
}
