//! Day 3: Toboggan Trajectory
//!
//! With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.
//!
//! Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:
//!
//! ```text
//! ..##.......
//! #...#...#..
//! .#....#..#.
//! ..#.#...#.#
//! .#...##..#.
//! ..#.##.....
//! .#.#.#....#
//! .#........#
//! #.##...#...
//! #...##....#
//! .#..#...#.#
//! ```
//!
//! These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:
//!
//! ```text
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........#.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...##....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).
//!
//! The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by counting all the trees you would encounter for the slope right 3, down 1:
//!
//! From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.
//!
//! The locations you'd check in the above example are marked here with O where there was an open square and X where there was a tree:
//!
//! ```text
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........X.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...#X....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! In this example, traversing the map using this slope would cause you to encounter 7 trees.
//!
//! Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?
//!
//! # Part Two
//!
//! Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.
//!
//! Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:
//!
//! - Right 1, down 1.
//! - Right 3, down 1. (This is the slope you already checked.)
//! - Right 5, down 1.
//! - Right 7, down 1.
//! - Right 1, down 2.
//!
//! In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.
//!
//! What do you get if you multiply together the number of trees encountered on each of the listed slopes?

use eyre::{eyre, Result};
use once_cell::sync::Lazy;

const INPUT_VALUES: &str = include_str!("input.txt");
const EX1_SLOPE: (usize, usize) = (3, 1);
static EX2_SLOPES: Lazy<Vec<(usize, usize)>> =
    Lazy::new(|| vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);

/// Part one answer.
pub fn run_ex1() -> usize {
    let (x, y) = EX1_SLOPE;
    TobogganMap::from_input(INPUT_VALUES).follow_slope(x, y)
}

/// Part two answer.
pub fn run_ex2() -> usize {
    let map = TobogganMap::from_input(INPUT_VALUES);

    EX2_SLOPES
        .iter()
        .map(|(offset_x, offset_y)| map.follow_slope(*offset_x, *offset_y))
        .product()
}

/// Handle map state, with an empty cell (`.`, or a tree `#`)
pub enum MapCell {
    /// Empty cell: `.`
    Empty,
    /// Tree cell: `#`
    Tree,
}

impl MapCell {
    /// Try to convert char to `MapCell`.
    ///
    /// # Arguments
    ///
    /// * `c` - Character
    ///
    /// # Errors
    ///
    /// * Unsupported char
    pub fn try_from_char(c: char) -> Result<Self> {
        Ok(match c {
            '.' => Self::Empty,
            '#' => Self::Tree,
            o => return Err(eyre!("Bad character {}", o)),
        })
    }
}

/// Handle toboggan map data
pub struct TobogganMap {
    data: Vec<MapCell>,
    width: usize,
}

impl TobogganMap {
    /// Get map cell at `x` and `y` position.
    ///
    /// # Arguments
    ///
    /// * `x` - X position
    /// * `y` - Y position
    pub fn get_xy(&self, x: usize, y: usize) -> Option<&MapCell> {
        let x = x % self.width;
        let pos = x + y * self.width;

        if pos < self.data.len() {
            Some(&self.data[pos])
        } else {
            None
        }
    }

    /// Follow slope using `offset_x` and `offset_y` offsets.
    ///
    /// # Arguments
    ///
    /// * `offset_x` - X offset
    /// * `offset_y` - Y offset
    pub fn follow_slope(&self, offset_x: usize, offset_y: usize) -> usize {
        let (mut x, mut y, mut tree_counter) = (0, 0, 0);
        while let Some(cell) = self.get_xy(x, y) {
            if let MapCell::Tree = cell {
                tree_counter += 1;
            }

            x += offset_x;
            y += offset_y;
        }

        tree_counter
    }

    /// Parse toboggan map from input text.
    ///
    /// # Arguments
    ///
    /// * `input` - Input text
    pub fn from_input(input: &str) -> Self {
        let width = input.lines().next().expect("Needs at least one line").len();

        let data: Vec<MapCell> = input
            .lines()
            .flat_map(|l| l.chars().filter_map(|c| MapCell::try_from_char(c).ok()))
            .collect();

        Self { data, width }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 299;
    const EX2_OUTPUT: usize = 3_621_285_278;

    #[test]
    fn test_run_ex1() {
        assert_eq!(run_ex1(), EX1_OUTPUT);
    }

    #[test]
    fn test_run_ex2() {
        assert_eq!(run_ex2(), EX2_OUTPUT);
    }
}
