//! Common code

/// Cell
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    /// Inactive
    Inactive,
    /// Active
    Active,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Active,
            '.' => Self::Inactive,
            c => panic!("Unknown cell char: {}", c),
        }
    }
}
