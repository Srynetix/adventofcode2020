//! # Day 17: Conway Cubes
//!
//! As your flight slowly drifts through the sky, the Elves at the Mythical Information Bureau at the North Pole contact you. They'd like some help debugging a malfunctioning experimental energy source aboard one of their super-secret imaging satellites.
//!
//! The experimental energy source is based on cutting-edge technology: a set of Conway Cubes contained in a pocket dimension! When you hear it's having problems, you can't help but agree to take a look.
//!
//! The pocket dimension contains an infinite 3-dimensional grid. At every integer 3-dimensional coordinate (x,y,z), there exists a single cube which is either active or inactive.
//!
//! In the initial state of the pocket dimension, almost all cubes start inactive. The only exception to this is a small flat region of cubes (your puzzle input); the cubes in this region start in the specified active (#) or inactive (.) state.
//!
//! The energy source then proceeds to boot up by executing six cycles.
//!
//! Each cube only ever considers its neighbors: any of the 26 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3, its neighbors include the cube at x=2,y=2,z=2, the cube at x=0,y=2,z=3, and so on.
//!
//! During a cycle, all cubes simultaneously change their state according to the following rules:
//!
//! If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
//! If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
//! The engineers responsible for this experimental energy source would like you to simulate the pocket dimension and determine what the configuration of cubes should be at the end of the six-cycle boot process.
//!
//! For example, consider the following initial state:
//!
//! ```text
//! .#.
//! ..#
//! ###
//! ```
//!
//! Even though the pocket dimension is 3-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1 region of the 3-dimensional space.)
//!
//! Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z coordinate (and the frame of view follows the active cells in each cycle):
//!
//! ```text
//! Before any cycles:
//!
//! z=0
//! .#.
//! ..#
//! ###
//!
//!
//! After 1 cycle:
//!
//! z=-1
//! #..
//! ..#
//! .#.
//!
//! z=0
//! #.#
//! .##
//! .#.
//!
//! z=1
//! #..
//! ..#
//! .#.
//!
//!
//! After 2 cycles:
//!
//! z=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1
//! ..#..
//! .#..#
//! ....#
//! .#...
//! .....
//!
//! z=0
//! ##...
//! ##...
//! #....
//! ....#
//! .###.
//!
//! z=1
//! ..#..
//! .#..#
//! ....#
//! .#...
//! .....
//!
//! z=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//!
//! After 3 cycles:
//!
//! z=-2
//! .......
//! .......
//! ..##...
//! ..###..
//! .......
//! .......
//! .......
//!
//! z=-1
//! ..#....
//! ...#...
//! #......
//! .....##
//! .#...#.
//! ..#.#..
//! ...#...
//!
//! z=0
//! ...#...
//! .......
//! #......
//! .......
//! .....##
//! .##.#..
//! ...#...
//!
//! z=1
//! ..#....
//! ...#...
//! #......
//! .....##
//! .#...#.
//! ..#.#..
//! ...#...
//!
//! z=2
//! .......
//! .......
//! ..##...
//! ..###..
//! .......
//! .......
//! .......
//! ```
//!
//! After the full six-cycle boot process completes, 112 cubes are left in the active state.
//!
//! Starting with your given initial configuration, simulate six cycles. How many cubes are left in the active state after the sixth cycle?
//!
//! --- Part Two ---
//!
//! For some reason, your simulated results don't match what the experimental energy source engineers expected. Apparently, the pocket dimension actually has four spatial dimensions, not three.
//!
//! The pocket dimension contains an infinite 4-dimensional grid. At every integer 4-dimensional coordinate (x,y,z,w), there exists a single cube (really, a hypercube) which is still either active or inactive.
//!
//! Each cube only ever considers its neighbors: any of the 80 other cubes where any of their coordinates differ by at most 1. For example, given the cube at x=1,y=2,z=3,w=4, its neighbors include the cube at x=2,y=2,z=3,w=3, the cube at x=0,y=2,z=3,w=4, and so on.
//!
//! The initial state of the pocket dimension still consists of a small flat region of cubes. Furthermore, the same rules for cycle updating still apply: during each cycle, consider the number of active neighbors of each cube.
//!
//! For example, consider the same initial state as in the example above. Even though the pocket dimension is 4-dimensional, this initial state represents a small 2-dimensional slice of it. (In particular, this initial state defines a 3x3x1x1 region of the 4-dimensional space.)
//!
//! Simulating a few cycles from this initial state produces the following configurations, where the result of each cycle is shown layer-by-layer at each given z and w coordinate:
//!
//! ```text
//! Before any cycles:
//!
//! z=0, w=0
//! .#.
//! ..#
//! ###
//!
//!
//! After 1 cycle:
//!
//! z=-1, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=0, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=1, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=-1, w=0
//! #..
//! ..#
//! .#.
//!
//! z=0, w=0
//! #.#
//! .##
//! .#.
//!
//! z=1, w=0
//! #..
//! ..#
//! .#.
//!
//! z=-1, w=1
//! #..
//! ..#
//! .#.
//!
//! z=0, w=1
//! #..
//! ..#
//! .#.
//!
//! z=1, w=1
//! #..
//! ..#
//! .#.
//!
//!
//! After 2 cycles:
//!
//! z=-2, w=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1, w=-2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=-2
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=1, w=-2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-2, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-1, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-2, w=0
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=-1, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=0
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=-2, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-1, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-2, w=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1, w=2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=2
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=1, w=2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//! ```
//!
//! After the full six-cycle boot process completes, 848 cubes are left in the active state.
//!
//! Starting with your given initial configuration, simulate six cycles in a 4-dimensional space. How many cubes are left in the active state after the sixth cycle?

use std::{collections::HashMap, convert::TryFrom, convert::TryInto};

use thiserror::Error;

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> usize {
    let mut game = Conway3D::try_from(INPUT_VALUES).expect("Bad input");
    game.run_steps(6);
    game.count_active_cells()
}

/// Part two answer.
pub const fn run_ex2() -> usize {
    0
}

/// Day error.
#[derive(Debug, Error)]
pub enum DayError {
    /// Parse error.
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Vec3
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Vec3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3 {
    /// Creates a new Vec3 from X, Y and Z coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    /// * `z` - Z coordinate
    pub const fn from_xyz(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
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

/// Game of Life in an 'infinite' 3D grid
#[derive(Debug, Default)]
pub struct Conway3D {
    map: HashMap<Vec3, Cell>,
    buffer: HashMap<Vec3, Cell>,
}

impl Conway3D {
    /// Creates new game.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            buffer: HashMap::new(),
        }
    }

    /// Set cell state at position.
    ///
    /// # Arguments
    ///
    /// * `position` - Grid position (x, y, z)
    /// * `state` - Cell state
    pub fn set_cell_state(&mut self, position: Vec3, state: Cell) {
        self.map.insert(position, state);
        self.buffer.insert(position, state);
    }

    /// Set multiple cell states.
    ///
    /// # Arguments
    ///
    /// * `stream` - `(Vec3, Cell)` tuple vec
    pub fn set_cell_states(&mut self, stream: Vec<(Vec3, Cell)>) {
        for (position, state) in stream {
            self.set_cell_state(position, state);
        }
    }

    /// Get cell at position.
    ///
    /// # Arguments
    ///
    /// * `position` - Position
    pub fn get_cell_at_position(&self, position: Vec3) -> Cell {
        self.map.get(&position).cloned().unwrap_or(Cell::Inactive)
    }

    /// Get active neighbors count from position.
    ///
    /// # Arguments
    ///
    /// * `position` - Position
    pub fn get_active_neighbors_count(&self, position: Vec3) -> usize {
        let mut count = 0;
        let (x, y, z) = position.into();

        for &i in &[-1, 0, 1] {
            for &j in &[-1, 0, 1] {
                for &k in &[-1, 0, 1] {
                    // Ignore current position
                    if i == 0 && j == 0 && k == 0 {
                        continue;
                    }

                    if let Cell::Active = self.get_cell_at_position((x + i, y + j, z + k).into()) {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Returns minimum and maximum bounds.
    pub fn get_bounds(&self) -> (Vec3, Vec3) {
        (
            self.map.keys().min().copied().unwrap(),
            self.map.keys().max().copied().unwrap(),
        )
    }

    /// Count active cells.
    pub fn count_active_cells(&self) -> usize {
        self.map.values().filter(|&&x| x == Cell::Active).count()
    }

    /// Execute a simulation step.
    pub fn step(&mut self) {
        let (min_bounds, max_bounds) = self.get_bounds();

        for z in min_bounds.z - 1..=max_bounds.z + 1 {
            for y in min_bounds.y - 1..=max_bounds.y + 1 {
                for x in min_bounds.x - 1..=max_bounds.x + 1 {
                    let position = (x, y, z).into();
                    let old_state = self.get_cell_at_position(position);
                    let neighbors_count = self.get_active_neighbors_count(position);

                    let new_state = match old_state {
                        Cell::Active if neighbors_count < 2 || neighbors_count > 3 => {
                            Cell::Inactive
                        }
                        Cell::Inactive if neighbors_count == 3 => Cell::Active,
                        _ => old_state,
                    };

                    self.buffer.insert(position, new_state);
                }
            }
        }

        // Swap buffers
        for (k, v) in &self.buffer {
            self.map.insert(*k, *v);
        }
    }

    /// Run multiple steps.
    ///
    /// # Arguments
    ///
    /// * `n` - Step count
    pub fn run_steps(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }
}

impl TryFrom<&str> for Conway3D {
    type Error = DayError;

    #[allow(clippy::cast_possible_wrap)]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut game = Self::new();

        game.set_cell_states(
            value
                .trim()
                .lines()
                .enumerate()
                .map(|(y, l)| {
                    Ok(l.trim()
                        .chars()
                        .enumerate()
                        .map(|(x, c)| Ok(((x as isize, y as isize, 0).into(), c.try_into()?)))
                        .collect::<Result<Vec<(Vec3, Cell)>, _>>()?)
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flatten()
                .collect(),
        );

        Ok(game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 269;

    const SAMPLE: &str = indoc::indoc! {"
        .#.
        ..#
        ###
    "};

    #[test]
    fn test_set_cell_states_from_str() {
        let game = Conway3D::try_from(SAMPLE).unwrap();
        assert_eq!(game.get_cell_at_position((0, 0, 0).into()), Cell::Inactive);
        assert_eq!(game.get_cell_at_position((1, 0, 0).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((2, 0, 0).into()), Cell::Inactive);
        assert_eq!(game.get_cell_at_position((0, 1, 0).into()), Cell::Inactive);
        assert_eq!(game.get_cell_at_position((1, 1, 0).into()), Cell::Inactive);
        assert_eq!(game.get_cell_at_position((2, 1, 0).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((0, 2, 0).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((1, 2, 0).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((2, 2, 0).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((3, 2, 0).into()), Cell::Inactive);
        assert_eq!(game.get_cell_at_position((0, 0, 1).into()), Cell::Inactive);
    }

    #[test]
    fn test_get_active_neighbors_count() {
        let game = Conway3D::try_from(SAMPLE).unwrap();
        assert_eq!(game.get_active_neighbors_count((0, 0, 0).into()), 1);
        assert_eq!(game.get_active_neighbors_count((1, 1, 0).into()), 5);
        assert_eq!(game.get_active_neighbors_count((1, 1, 1).into()), 5);
        assert_eq!(game.get_active_neighbors_count((2, 1, 1).into()), 4);
    }

    #[test]
    fn test_step() {
        let mut game = Conway3D::try_from(SAMPLE).unwrap();
        assert_eq!(game.get_bounds(), ((0, 0, 0).into(), (2, 2, 0).into()));
        game.step();
        assert_eq!(game.get_bounds(), ((-1, -1, -1).into(), (3, 3, 1).into()));

        assert_eq!(game.get_cell_at_position((0, 1, -1).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((2, 2, -1).into()), Cell::Active);
        assert_eq!(game.get_cell_at_position((1, 3, -1).into()), Cell::Active);
    }

    #[test]
    fn test_run_6_steps() {
        let mut game = Conway3D::try_from(SAMPLE).unwrap();
        game.run_steps(6);
        assert_eq!(game.count_active_cells(), 112);
    }

    #[test]
    fn test_run_ex1() {
        assert_eq!(run_ex1(), EX1_OUTPUT);
    }
}
