//! Common code

use std::cmp::Ordering;
use std::convert::TryFrom;

use thiserror::Error;

/// Day error.
#[derive(Debug, Error)]
pub enum DayError {
    /// Parse error.
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Cell
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    /// Inactive
    Inactive,
    /// Active
    Active,
}

impl TryFrom<char> for Cell {
    type Error = DayError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '#' => Self::Active,
            '.' => Self::Inactive,
            c => return Err(DayError::ParseError(format!("Unknown cell char: {}", c))),
        })
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
