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
//!
//! # Part Two
//!
//! The tile floor in the lobby is meant to be a living art exhibit. Every day, the tiles are all flipped according to the following rules:
//!
//! Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
//! Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
//! Here, tiles immediately adjacent means the six tiles directly touching the tile in question.
//!
//! The rules are applied simultaneously to every tile; put another way, it is first determined which tiles need to be flipped, then they are all flipped at the same time.
//!
//! In the above example, the number of black tiles that are facing up after the given number of days has passed is as follows:
//!
//! ```text
//! Day 1: 15
//! Day 2: 12
//! Day 3: 25
//! Day 4: 14
//! Day 5: 23
//! Day 6: 28
//! Day 7: 41
//! Day 8: 37
//! Day 9: 49
//! Day 10: 37
//!
//! Day 20: 132
//! Day 30: 259
//! Day 40: 406
//! Day 50: 566
//! Day 60: 788
//! Day 70: 1106
//! Day 80: 1373
//! Day 90: 1844
//! Day 100: 2208
//! ```
//!
//! After executing this process a total of 100 times, there would be 2208 black tiles facing up.
//!
//! How many tiles will be black after 100 days?

use std::collections::HashMap;

use crate::common::Vec2;

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> usize {
    let mut grid = HexGrid::default();
    grid.follow_paths(parse_paths(INPUT_VALUES));

    grid.count_black_tiles()
}

/// Part two answer.
pub fn run_ex2() -> usize {
    let mut grid = HexGrid::default();
    grid.follow_paths(parse_paths(INPUT_VALUES));
    grid.run_steps(100);
    grid.count_black_tiles()
}

/// Hexagonal direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// East.
    East,
    /// South east.
    SouthEast,
    /// South west.
    SouthWest,
    /// West.
    West,
    /// North west.
    NorthWest,
    /// North east.
    NorthEast,
}

impl Direction {
    /// Get all directions.
    pub fn all() -> &'static [Direction] {
        &[
            Self::East,
            Self::SouthEast,
            Self::SouthWest,
            Self::West,
            Self::NorthWest,
            Self::NorthEast,
        ]
    }
}

/// Hexagonal tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    /// White tile.
    White,
    /// Black tile.
    Black,
}

impl Tile {
    /// Returns a flipped tile.
    pub fn flipped(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

/// Direction path.
pub type DirPath = Vec<Direction>;

/// Hexagonal grid.
#[derive(Debug, Default)]
pub struct HexGrid {
    data: HashMap<Vec2, Tile>,
}

impl HexGrid {
    /// Get next position from position `pos` towards direction `direction`.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position
    /// * `direction` - Direction
    pub fn next_pos(pos: Vec2, direction: Direction) -> Vec2 {
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

    /// Get tile at position `pos`.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position
    pub fn get(&self, pos: Vec2) -> Tile {
        self.data.get(&pos).copied().unwrap_or(Tile::White)
    }

    /// Create or flip tile at position `pos`.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position
    pub fn create_or_flip(&mut self, pos: Vec2) -> Tile {
        let tile = self.data.entry(pos).or_insert(Tile::White);
        *tile = tile.flipped();
        *tile
    }

    /// Get grid bounds.
    pub fn get_bounds(&self) -> (Vec2, Vec2) {
        let (mut min_x, mut min_y) = (isize::MAX, isize::MAX);
        let (mut max_x, mut max_y) = (isize::MIN, isize::MIN);

        for p in self.data.keys() {
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);

            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }

        ((min_x, min_y).into(), (max_x, max_y).into())
    }

    /// Execute a step.
    pub fn life_step(&mut self) {
        let (min_bounds, max_bounds) = self.get_bounds();
        let mut tmp_grid = self.data.clone();

        for y in min_bounds.y - 1..=max_bounds.y + 1 {
            for x in min_bounds.x - 1..=max_bounds.x + 1 {
                let position = (x, y).into();
                let old_state = self.get(position);
                let neighbors_count = self.count_neighbors(position);

                let new_state = match old_state {
                    Tile::Black if neighbors_count == 0 || neighbors_count > 2 => Tile::White,
                    Tile::White if neighbors_count == 2 => Tile::Black,
                    _ => old_state,
                };

                tmp_grid.insert(position, new_state);
            }
        }

        // Swap
        self.data = tmp_grid;
    }

    /// Run `steps` steps.
    ///
    /// # Arguments
    ///
    /// * `steps` - Step count
    pub fn run_steps(&mut self, steps: usize) {
        for _ in 0..steps {
            self.life_step();
        }
    }

    /// Count black tiles neighbors from position `pos`.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position
    pub fn count_neighbors(&self, pos: Vec2) -> usize {
        Direction::all()
            .iter()
            .filter(|&&d| self.get(HexGrid::next_pos(pos, d)) == Tile::Black)
            .count()
    }

    /// Count black tiles.
    pub fn count_black_tiles(&self) -> usize {
        self.data.values().filter(|&&t| t == Tile::Black).count()
    }

    /// Follow direction path `path`.
    ///
    /// # Arguments
    ///
    /// * `path` - Direction path
    pub fn follow_path(&mut self, path: DirPath) -> (Vec2, Tile) {
        let next_pos = path.into_iter().fold(Vec2::from((0, 0)), HexGrid::next_pos);
        (next_pos, self.create_or_flip(next_pos))
    }

    /// Follow multiple direction paths `paths`.
    ///
    /// # Arguments
    ///
    /// * `paths` - Paths
    pub fn follow_paths(&mut self, paths: Vec<DirPath>) -> (Vec2, Tile) {
        paths
            .into_iter()
            .map(|p| self.follow_path(p))
            .last()
            .unwrap()
    }
}

/// Parse direction from string.
///
/// # Arguments
///
/// * `input` - Input string
pub fn parse_direction(input: &str) -> Option<Direction> {
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

/// Parse path from input.
///
/// # Arguments
///
/// * `input` - Input string
pub fn parse_path(input: &str) -> DirPath {
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

/// Parse multiple paths from input.
///
/// # Arguments
///
/// * `input` - Input string
pub fn parse_paths(input: &str) -> Vec<DirPath> {
    input
        .trim()
        .lines()
        .map(str::trim)
        .map(parse_path)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 228;
    const EX2_OUTPUT: usize = 3672;

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
            grid.follow_path(parse_path("esenee")),
            (Vec2::from((3, 0)), Tile::Black)
        );
        assert_eq!(
            grid.follow_path(parse_path("esenee")),
            (Vec2::from((3, 0)), Tile::White)
        );

        assert_eq!(
            grid.follow_path(parse_path("esew")),
            (Vec2::from((0, 1)), Tile::Black)
        );

        assert_eq!(
            grid.follow_path(parse_path("nwwswee")),
            (Vec2::from((0, 0)), Tile::Black)
        );
    }

    #[test]
    fn test_follow_paths() {
        let mut grid = HexGrid::default();
        grid.follow_paths(parse_paths(SAMPLE));

        assert_eq!(grid.count_black_tiles(), 10);
    }

    #[test]
    fn test_life_step() {
        let mut grid = HexGrid::default();
        grid.follow_paths(parse_paths(SAMPLE));

        assert_eq!(
            (0..4)
                .map(|_| {
                    grid.life_step();
                    grid.count_black_tiles()
                })
                .collect::<Vec<_>>(),
            vec![15, 12, 25, 14]
        );
    }

    #[test]
    fn test_run_steps() {
        let mut grid = HexGrid::default();
        grid.follow_paths(parse_paths(SAMPLE));

        grid.run_steps(100);
        assert_eq!(grid.count_black_tiles(), 2208);
    }

    #[test]
    fn test_run_ex1() {
        assert_eq!(run_ex1(), EX1_OUTPUT);
    }

    #[test]
    fn test_run_ex2() {
        assert_eq!(run_ex2(), EX2_OUTPUT);
    }
}
