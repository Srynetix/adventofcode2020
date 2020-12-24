//! # Day 24: Lobby Layout
//!
//! Your raft makes it to the tropical island; it turns out that the small crab was an excellent navigator. You make your way to the resort.
//!
//! As you enter the lobby, you discover a small problem: the floor is being renovated. You can't even reach the check-in desk until they've finished installing the new tile floor.
//!
//! The tiles are all hexagonal; they need to be arranged in a hex grid with a very specific color pattern. Not in the mood to wait, you offer to help figure out the pattern.
//!
//! The tiles are all white on one side and black on the other. They start with the white side facing up. The lobby is large enough to fit whatever pattern might need to appear there.
//!
//! A member of the renovation crew gives you a list of the tiles that need to be flipped over (your puzzle input). Each line in the list identifies a single tile that needs to be flipped by giving a series of steps starting from a reference tile in the very center of the room. (Every line starts from the same reference tile.)
//!
//! Because the tiles are hexagonal, every tile has six neighbors: east, southeast, southwest, west, northwest, and northeast. These directions are given in your list, respectively, as e, se, sw, w, nw, and ne. A tile is identified by a series of these directions with no delimiters; for example, esenee identifies the tile you land on if you start at the reference tile and then move one tile east, one tile southeast, one tile northeast, and one tile east.
//!
//! Each time a tile is identified, it flips from white to black or from black to white. Tiles might be flipped more than once. For example, a line like esew flips a tile immediately adjacent to the reference tile, and a line like nwwswee flips the reference tile itself.
//!
//! Here is a larger example:
//!
//! ```text
//! sesenwnenenewseeswwswswwnenewsewsw
//! neeenesenwnwwswnenewnwwsewnenwseswesw
//! seswneswswsenwwnwse
//! nwnwneseeswswnenewneswwnewseswneseene
//! swweswneswnenwsewnwneneseenw
//! eesenwseswswnenwswnwnwsewwnwsene
//! sewnenenenesenwsewnenwwwse
//! wenwwweseeeweswwwnwwe
//! wsweesenenewnwwnwsenewsenwwsesesenwne
//! neeswseenwwswnwswswnw
//! nenwswwsewswnenenewsenwsenwnesesenew
//! enewnwewneswsewnwswenweswnenwsenwsw
//! sweneswneswneneenwnewenewwneswswnese
//! swwesenesewenwneswnwwneseswwne
//! enesenwswwswneneswsenwnewswseenwsese
//! wnwnesenesenenwwnenwsewesewsesesew
//! nenewswnwewswnenesenwnesewesw
//! eneswnwswnwsenenwnwnwwseeswneewsenese
//! neswnwewnwnwseenwseesewsenwsweewe
//! wseweeenwnesenwwwswnew
//! ```
//!
//! In the above example, 10 tiles are flipped once (to black), and 5 more are flipped twice (to black, then back to white). After all of these instructions have been followed, a total of 10 tiles are black.
//!
//! Go through the renovation crew's list and determine which tiles they need to flip. After all of the instructions have been followed, how many tiles are left with the black side up?

use std::collections::HashMap;

use crate::common::Vec2;

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> usize {
    let mut grid = HexGrid::default();
    for p in INPUT_VALUES.trim().lines().map(str::trim).map(parse_path) {
        follow_path(&mut grid, p);
    }

    grid.count_black_tiles()
}

/// Part two answer.
pub const fn run_ex2() -> usize {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    White,
    Black,
}

impl Tile {
    fn flipped(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Debug, Default)]
struct HexGrid {
    data: HashMap<Vec2, Tile>,
}

impl HexGrid {
    fn next_pos(pos: Vec2, direction: Direction) -> Vec2 {
        let (x, y) = pos.into();

        match direction {
            Direction::West => (x - 1, y).into(),
            Direction::East => (x + 1, y).into(),
            Direction::NorthWest => (x, y - 1).into(),
            Direction::NorthEast => (x + 1, y - 1).into(),
            Direction::SouthWest => (x - 1, y + 1).into(),
            Direction::SouthEast => (x, y + 1).into(),
        }
    }

    // fn get(&self, pos: Vec2) -> Option<Tile> {
    //     self.data.get(&pos).copied()
    // }

    fn create_or_flip(&mut self, pos: Vec2) -> Tile {
        let tile = self.data.entry(pos).or_insert(Tile::White);
        *tile = tile.flipped();
        *tile
    }

    fn count_black_tiles(&self) -> usize {
        self.data.values().filter(|&&t| t == Tile::Black).count()
    }
}

fn parse_direction(input: &str) -> Option<Direction> {
    Some(match input {
        "se" => Direction::SouthEast,
        "sw" => Direction::SouthWest,
        "ne" => Direction::NorthEast,
        "nw" => Direction::NorthWest,
        "e" => Direction::East,
        "w" => Direction::West,
        _ => return None,
    })
}

fn parse_path(input: &str) -> Vec<Direction> {
    let mut output = vec![];
    let mut c1 = 0;

    for c2 in 0..input.len() {
        let slice = &input[c1..=c2];
        if let Some(dir) = parse_direction(slice) {
            output.push(dir);

            // Update c1
            c1 = c2 + 1;
        }
    }

    output
}

fn follow_path(grid: &mut HexGrid, path: Vec<Direction>) -> (Vec2, Tile) {
    let mut next_pos = Vec2::from((0, 0));
    for dir in path {
        next_pos = HexGrid::next_pos(next_pos, dir);
    }

    (next_pos, grid.create_or_flip(next_pos))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 228;

    const SAMPLE: &str = indoc::indoc!(
        "
        sesenwnenenewseeswwswswwnenewsewsw
        neeenesenwnwwswnenewnwwsewnenwseswesw
        seswneswswsenwwnwse
        nwnwneseeswswnenewneswwnewseswneseene
        swweswneswnenwsewnwneneseenw
        eesenwseswswnenwswnwnwsewwnwsene
        sewnenenenesenwsewnenwwwse
        wenwwweseeeweswwwnwwe
        wsweesenenewnwwnwsenewsenwwsesesenwne
        neeswseenwwswnwswswnw
        nenwswwsewswnenenewsenwsenwnesesenew
        enewnwewneswsewnwswenweswnenwsenwsw
        sweneswneswneneenwnewenewwneswswnese
        swwesenesewenwneswnwwneseswwne
        enesenwswwswneneswsenwnewswseenwsese
        wnwnesenesenenwwnenwsewesewsesesew
        nenewswnwewswnenesenwnesewesw
        eneswnwswnwsenenwnwnwwseeswneewsenese
        neswnwewnwnwseenwseesewsenwsweewe
        wseweeenwnesenwwwswnew
    "
    );

    #[test]
    fn test_parse_path() {
        use Direction::{East, NorthEast, NorthWest, SouthEast, SouthWest, West};

        assert_eq!(parse_path("esenee"), vec![East, SouthEast, NorthEast, East]);
        assert_eq!(parse_path("esew"), vec![East, SouthEast, West]);

        assert_eq!(
            parse_path("nwwswee"),
            vec![NorthWest, West, SouthWest, East, East]
        )
    }

    #[test]
    fn test_follow_path() {
        let mut grid = HexGrid::default();

        assert_eq!(
            follow_path(&mut grid, parse_path("esenee")),
            (Vec2::from((3, 0)), Tile::Black)
        );
        assert_eq!(
            follow_path(&mut grid, parse_path("esenee")),
            (Vec2::from((3, 0)), Tile::White)
        );

        assert_eq!(
            follow_path(&mut grid, parse_path("esew")),
            (Vec2::from((0, 1)), Tile::Black)
        );

        assert_eq!(
            follow_path(&mut grid, parse_path("nwwswee")),
            (Vec2::from((0, 0)), Tile::Black)
        );
    }

    #[test]
    fn test_follow_paths() {
        let mut grid = HexGrid::default();

        for path in SAMPLE.trim().lines().map(str::trim).map(parse_path) {
            follow_path(&mut grid, path);
        }

        assert_eq!(grid.count_black_tiles(), 10);
    }

    #[test]
    fn test_run_ex1() {
        assert_eq!(run_ex1(), EX1_OUTPUT);
    }
}
