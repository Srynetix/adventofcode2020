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

impl Into<(isize, isize)> for Vec2 {
    fn into(self) -> (isize, isize) {
        (self.x, self.y)
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

impl Into<(isize, isize, isize)> for Vec3 {
    fn into(self) -> (isize, isize, isize) {
        (self.x, self.y, self.z)
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

impl Into<(isize, isize, isize, isize)> for Vec4 {
    fn into(self) -> (isize, isize, isize, isize) {
        (self.x, self.y, self.z, self.t)
    }
}