//! # Day 20: Jurassic Jigsaw
//!
//! The high-speed train leaves the forest and quickly carries you south. You can even see a desert in the distance! Since you have some spare time, you might as well see if there was anything interesting in the image the Mythical Information Bureau satellite captured.
//!
//! After decoding the satellite messages, you discover that the data actually contains many small images created by the satellite's camera array. The camera array consists of many cameras; rather than produce a single square image, they produce many smaller square image tiles that need to be reassembled back into a single image.
//!
//! Each camera in the camera array returns a single monochrome image tile with a random unique ID number. The tiles (your puzzle input) arrived in a random order.
//!
//! Worse yet, the camera array appears to be malfunctioning: each image tile has been rotated and flipped to a random orientation. Your first task is to reassemble the original image by orienting the tiles so they fit together.
//!
//! To show how the tiles should be reassembled, each tile's image data includes a border that should line up exactly with its adjacent tiles. All tiles have this border, and the border lines up exactly when the tiles are both oriented correctly. Tiles at the edge of the image also have this border, but the outermost edges won't line up with any other tiles.
//!
//! For example, suppose you have the following nine tiles:
//!
//! ```text
//! Tile 2311:
//! ..##.#..#.
//! ##..#.....
//! #...##..#.
//! ####.#...#
//! ##.##.###.
//! ##...#.###
//! .#.#.#..##
//! ..#....#..
//! ###...#.#.
//! ..###..###
//!
//! Tile 1951:
//! #.##...##.
//! #.####...#
//! .....#..##
//! #...######
//! .##.#....#
//! .###.#####
//! ###.##.##.
//! .###....#.
//! ..#.#..#.#
//! #...##.#..
//!
//! Tile 1171:
//! ####...##.
//! #..##.#..#
//! ##.#..#.#.
//! .###.####.
//! ..###.####
//! .##....##.
//! .#...####.
//! #.##.####.
//! ####..#...
//! .....##...
//!
//! Tile 1427:
//! ###.##.#..
//! .#..#.##..
//! .#.##.#..#
//! #.#.#.##.#
//! ....#...##
//! ...##..##.
//! ...#.#####
//! .#.####.#.
//! ..#..###.#
//! ..##.#..#.
//!
//! Tile 1489:
//! ##.#.#....
//! ..##...#..
//! .##..##...
//! ..#...#...
//! #####...#.
//! #..#.#.#.#
//! ...#.#.#..
//! ##.#...##.
//! ..##.##.##
//! ###.##.#..
//!
//! Tile 2473:
//! #....####.
//! #..#.##...
//! #.##..#...
//! ######.#.#
//! .#...#.#.#
//! .#########
//! .###.#..#.
//! ########.#
//! ##...##.#.
//! ..###.#.#.
//!
//! Tile 2971:
//! ..#.#....#
//! #...###...
//! #.#.###...
//! ##.##..#..
//! .#####..##
//! .#..####.#
//! #..#.#..#.
//! ..####.###
//! ..#.#.###.
//! ...#.#.#.#
//!
//! Tile 2729:
//! ...#.#.#.#
//! ####.#....
//! ..#.#.....
//! ....#..#.#
//! .##..##.#.
//! .#.####...
//! ####.#.#..
//! ##.####...
//! ##..#.##..
//! #.##...##.
//!
//! Tile 3079:
//! #.#.#####.
//! .#..######
//! ..#.......
//! ######....
//! ####.#..#.
//! .#...#.##.
//! #.#####.##
//! ..#.###...
//! ..#.......
//! ..#.###...
//! ```
//!
//! By rotating, flipping, and rearranging them, you can find a square arrangement that causes all adjacent borders to line up:
//!
//! ```text
//! #...##.#.. ..###..### #.#.#####.
//! ..#.#..#.# ###...#.#. .#..######
//! .###....#. ..#....#.. ..#.......
//! ###.##.##. .#.#.#..## ######....
//! .###.##### ##...#.### ####.#..#.
//! .##.#....# ##.##.###. .#...#.##.
//! #...###### ####.#...# #.#####.##
//! .....#..## #...##..#. ..#.###...
//! #.####...# ##..#..... ..#.......
//! #.##...##. ..##.#..#. ..#.###...
//!
//! #.##...##. ..##.#..#. ..#.###...
//! ##..#.##.. ..#..###.# ##.##....#
//! ##.####... .#.####.#. ..#.###..#
//! ####.#.#.. ...#.##### ###.#..###
//! .#.####... ...##..##. .######.##
//! .##..##.#. ....#...## #.#.#.#...
//! ....#..#.# #.#.#.##.# #.###.###.
//! ..#.#..... .#.##.#..# #.###.##..
//! ####.#.... .#..#.##.. .######...
//! ...#.#.#.# ###.##.#.. .##...####
//!
//! ...#.#.#.# ###.##.#.. .##...####
//! ..#.#.###. ..##.##.## #..#.##..#
//! ..####.### ##.#...##. .#.#..#.##
//! #..#.#..#. ...#.#.#.. .####.###.
//! .#..####.# #..#.#.#.# ####.###..
//! .#####..## #####...#. .##....##.
//! ##.##..#.. ..#...#... .####...#.
//! #.#.###... .##..##... .####.##.#
//! #...###... ..##...#.. ...#..####
//! ..#.#....# ##.#.#.... ...##.....
//! ```
//!
//! For reference, the IDs of the above tiles are:
//!
//! ```text
//! 1951    2311    3079
//! 2729    1427    2473
//! 2971    1489    1171
//! ```
//!
//! To check that you've assembled the image correctly, multiply the IDs of the four corner tiles together. If you do this with the assembled tiles from the example above, you get 1951 * 3079 * 2971 * 1171 = 20899048083289.
//!
//! Assemble the tiles into an image. What do you get if you multiply together the IDs of the four corner tiles?
//!
//! # Part Two
//!
//! Now, you're ready to check the image for sea monsters.
//!
//! The borders of each tile are not part of the actual image; start by removing them.
//!
//! In the example above, the tiles become:
//!
//! ```text
//! .#.#..#. ##...#.# #..#####
//! ###....# .#....#. .#......
//! ##.##.## #.#.#..# #####...
//! ###.#### #...#.## ###.#..#
//! ##.#.... #.##.### #...#.##
//! ...##### ###.#... .#####.#
//! ....#..# ...##..# .#.###..
//! .####... #..#.... .#......
//!
//! #..#.##. .#..###. #.##....
//! #.####.. #.####.# .#.###..
//! ###.#.#. ..#.#### ##.#..##
//! #.####.. ..##..## ######.#
//! ##..##.# ...#...# .#.#.#..
//! ...#..#. .#.#.##. .###.###
//! .#.#.... #.##.#.. .###.##.
//! ###.#... #..#.##. ######..
//!
//! .#.#.### .##.##.# ..#.##..
//! .####.## #.#...## #.#..#.#
//! ..#.#..# ..#.#.#. ####.###
//! #..####. ..#.#.#. ###.###.
//! #####..# ####...# ##....##
//! #.##..#. .#...#.. ####...#
//! .#.###.. ##..##.. ####.##.
//! ...###.. .##...#. ..#..###
//! ```
//!
//! Remove the gaps to form the actual image:
//!
//! ```text
//! .#.#..#.##...#.##..#####
//! ###....#.#....#..#......
//! ##.##.###.#.#..######...
//! ###.#####...#.#####.#..#
//! ##.#....#.##.####...#.##
//! ...########.#....#####.#
//! ....#..#...##..#.#.###..
//! .####...#..#.....#......
//! #..#.##..#..###.#.##....
//! #.####..#.####.#.#.###..
//! ###.#.#...#.######.#..##
//! #.####....##..########.#
//! ##..##.#...#...#.#.#.#..
//! ...#..#..#.#.##..###.###
//! .#.#....#.##.#...###.##.
//! ###.#...#..#.##.######..
//! .#.#.###.##.##.#..#.##..
//! .####.###.#...###.#..#.#
//! ..#.#..#..#.#.#.####.###
//! #..####...#.#.#.###.###.
//! #####..#####...###....##
//! #.##..#..#...#..####...#
//! .#.###..##..##..####.##.
//! ...###...##...#...#..###
//! ```
//!
//! Now, you're ready to search for sea monsters! Because your image is monochrome, a sea monster will look like this:
//!
//! ```text
//!                   #
//! #    ##    ##    ###
//!  #  #  #  #  #  #
//! ```
//!
//! When looking for this pattern in the image, the spaces can be anything; only the # need to match. Also, you might need to rotate or flip your image before it's oriented correctly to find sea monsters. In the above image, after flipping and rotating it to the appropriate orientation, there are two sea monsters (marked with O):
//!
//! ```text
//! .####...#####..#...###..
//! #####..#..#.#.####..#.#.
//! .#.#...#.###...#.##.O#..
//! #.O.##.OO#.#.OO.##.OOO##
//! ..#O.#O#.O##O..O.#O##.##
//! ...#.#..##.##...#..#..##
//! #.##.#..#.#..#..##.#.#..
//! .###.##.....#...###.#...
//! #.####.#.#....##.#..#.#.
//! ##...#..#....#..#...####
//! ..#.##...###..#.#####..#
//! ....#.##.#.#####....#...
//! ..##.##.###.....#.##..#.
//! #...#...###..####....##.
//! .#.##...#.##.#.#.###...#
//! #.###.#..####...##..#...
//! #.###...#.##...#.##O###.
//! .O##.#OO.###OO##..OOO##.
//! ..O#.O..O..O.#O##O##.###
//! #.#..##.########..#..##.
//! #.#####..#.#...##..#....
//! #....##..#.#########..##
//! #...#.....#..##...###.##
//! #..###....##.#...##.##.#
//! ```
//!
//! Determine how rough the waters are in the sea monsters' habitat by counting the number of # that are not part of a sea monster. In the above example, the habitat's water roughness is 273.

use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

const INPUT_VALUES: &str = include_str!("input.txt");

static MONSTER: Lazy<Vec<Vec<usize>>> = Lazy::new(|| {
    vec![
        vec![18],
        vec![0, 5, 6, 11, 12, 17, 18, 19],
        vec![1, 4, 7, 10, 13, 16],
    ]
});
static TILE_ID_RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"Tile (\d+):"#).unwrap());

/// Part one answer.
pub fn run_ex1() -> usize {
    let tiles = TileParser::parse_multiple_from_input(INPUT_VALUES);
    TileMatcher::find_puzzle_corners(&tiles).iter().product()
}

/// Part two answer.
pub fn run_ex2() -> usize {
    let tiles = TileParser::parse_multiple_from_input(INPUT_VALUES);
    let puzzle = TileMatcher::build_puzzle(&tiles);
    let puzzle = TileMatcher::find_and_replace_sea_monsters(&puzzle).unwrap();

    puzzle.0.iter().fold(0, |acc, x| {
        acc + x
            .iter()
            .fold(0, |acc, x| if *x == '#' { acc + 1 } else { acc })
    })
}

/// Tile.
#[derive(Clone)]
pub struct Tile {
    id: usize,
    data: Vec<Vec<bool>>,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in &self.data {
            for y in x {
                write!(f, "{}", if *y { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Tile {
    /// Clone tile with new data.
    ///
    /// # Arguments
    ///
    /// * `new_data` - New data
    pub fn clone_with_data(&self, new_data: Vec<Vec<bool>>) -> Self {
        Self {
            id: self.id,
            data: new_data,
        }
    }
}

/// Border direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BorderDirection {
    /// Top.
    Top,
    /// Left.
    Left,
    /// Right.
    Right,
    /// Bottom.
    Bottom,
}

/// Tile manipulator
pub struct TileManipulator;

impl TileManipulator {
    /// Rotate tile.
    ///
    /// # Arguments
    ///
    /// * `data` - Data
    /// * `rotation` - Rotation
    pub fn rotated<T>(data: &[Vec<T>], rotation: TileRotation) -> Vec<Vec<T>>
    where
        T: Clone + Default,
    {
        match rotation {
            TileRotation::R0 => data.to_vec(),
            TileRotation::R90 => Self::rotated_90_ccw(data),
            TileRotation::R180 => Self::rotated_180_cw(data),
            TileRotation::R270 => Self::rotated_90_cw(data),
        }
    }

    /// Flip data vertically.
    ///
    /// # Arguments
    ///
    /// * `data` - Data
    pub fn flip_vertical<T>(data: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Clone + Default,
    {
        let size = data.len();
        let mut output = Self::allocate_empty_grid(size);

        for i in 0..size {
            for j in 0..size {
                output[i][j] = data[size - i - 1][j].clone();
            }
        }

        output
    }

    /// Flip data horizontally.
    ///
    /// # Arguments
    ///
    /// * `data` - Data
    pub fn flip_horizontal<T>(data: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Clone + Default,
    {
        let size = data.len();
        let mut output = Self::allocate_empty_grid(size);

        for i in 0..size {
            for j in 0..size {
                output[i][j] = data[i][size - j - 1].clone();
            }
        }

        output
    }

    /// Rotate data 90° clockwise = 270° counter-clockwise.
    ///
    /// # Arguments
    ///
    /// * `data` - Data
    #[allow(clippy::needless_range_loop)]
    pub fn rotated_90_cw<T>(data: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Clone + Default,
    {
        let size = data.len();
        let mut output = Self::allocate_empty_grid(size);

        for i in 0..size {
            for j in 0..size {
                output[i][j] = data[size - j - 1][i].clone();
            }
        }

        output
    }

    /// Rotate data 180°.
    ///
    /// # Arguments
    ///
    /// * `data` - Data
    pub fn rotated_180_cw<T>(data: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Clone + Default,
    {
        Self::rotated_90_cw(&Self::rotated_90_cw(data))
    }

    /// Rotate data 90° counter-clockwise = 270° clockwise.
    ///
    /// # Arguments
    ///
    /// * `data` - Data
    #[allow(clippy::needless_range_loop)]
    fn rotated_90_ccw<T>(data: &[Vec<T>]) -> Vec<Vec<T>>
    where
        T: Clone + Default,
    {
        let size = data.len();
        let mut output = Self::allocate_empty_grid(size);

        for i in 0..size {
            for j in 0..size {
                output[i][j] = data[j][size - i - 1].clone();
            }
        }

        output
    }

    /// Allocate an empty `size` x `size` grid.
    ///
    /// # Arguments
    ///
    /// * `size` - Size
    pub fn allocate_empty_grid<T>(size: usize) -> Vec<Vec<T>>
    where
        T: Clone + Default,
    {
        let mut output = Vec::new();
        output.resize(size, vec![]);
        for v in &mut output {
            v.resize(size, T::default());
        }

        output
    }

    /// Invert a tile border.
    ///
    /// # Arguments
    ///
    /// * `s` - Border
    pub fn invert_side(s: &str) -> String {
        s.chars().rev().collect()
    }

    /// Extract all borders from tile data.
    ///
    /// # Arguments
    ///
    /// * `data` - Tile data
    pub fn extract_borders(data: &[Vec<bool>]) -> HashMap<BorderDirection, String> {
        let mut output = HashMap::new();

        for dir in &[
            BorderDirection::Top,
            BorderDirection::Bottom,
            BorderDirection::Left,
            BorderDirection::Right,
        ] {
            output.insert(*dir, Self::extract_single_border(data, *dir));
        }

        output
    }

    /// Extract a single borders from tile data at a direction.
    ///
    /// # Arguments
    ///
    /// * `data` - Tile data
    /// * `direction` - Direction
    pub fn extract_single_border(data: &[Vec<bool>], direction: BorderDirection) -> String {
        Self::border_to_string(match direction {
            BorderDirection::Top => data[0][..].to_vec(),
            BorderDirection::Bottom => data[data.len() - 1][..].to_vec(),
            BorderDirection::Left => data.iter().map(|x| x[0]).collect(),
            BorderDirection::Right => data.iter().map(|x| x[data.len() - 1]).collect(),
        })
    }

    /// Convert a border to a string.
    ///
    /// # Arguments
    ///
    /// * `border` - Border
    pub fn border_to_string(border: Vec<bool>) -> String {
        border
            .into_iter()
            .map(|x| if x { '#' } else { '.' })
            .collect()
    }
}

/// Tile rotation (counter-clockwise)
#[derive(Debug, Clone, Copy)]
pub enum TileRotation {
    /// No rotation.
    R0,
    /// 90° rotation
    R90,
    /// 180° rotation
    R180,
    /// 270° rotation
    R270,
}

/// Tile flip.
#[derive(Debug, Clone, Copy)]
pub enum TileFlip {
    /// No flip.
    None,
    /// Horizontal flip.
    Horizontal,
    /// Vertical flip.
    Vertical,
}

/// Puzzle builder.
pub struct PuzzleBuilder(Vec<Vec<Option<Tile>>>);

impl PuzzleBuilder {
    /// Build puzzle.
    pub fn build(&self) -> Puzzle {
        let puzzle_size = self.0.len();
        let tile_size = self.0[0][0].as_ref().unwrap().data.len();
        let total_size = puzzle_size * tile_size;
        let total_puzzle_size = total_size - puzzle_size * 2;
        let mut puzzle_data = Vec::new();
        puzzle_data.resize(total_puzzle_size, vec![]);
        for l in &mut puzzle_data {
            l.resize(total_puzzle_size, ' ');
        }

        let mut puzzle_y = 0;

        for y in 0..total_size {
            let mut puzzle_x = 0;
            let tile_id_y = y / tile_size;
            let tile_pos_y = y % tile_size;

            if tile_pos_y == 0 || tile_pos_y == tile_size - 1 {
                continue;
            }

            for x in 0..total_size {
                let tile_id_x = x / tile_size;
                let tile_pos_x = x % tile_size;

                // Strip borders
                if tile_pos_x == 0 || tile_pos_x == tile_size - 1 {
                    continue;
                }

                if let Some(tile) = &self.0[tile_id_y][tile_id_x] {
                    let value = &tile.data[tile_pos_y][tile_pos_x];
                    puzzle_data[puzzle_y][puzzle_x] = if *value { '#' } else { '.' };
                } else {
                    puzzle_data[puzzle_y][puzzle_x] = '-';
                }

                puzzle_x += 1;
            }

            puzzle_y += 1;
        }

        Puzzle(puzzle_data)
    }
}

/// Puzzle.
pub struct Puzzle(Vec<Vec<char>>);

impl Puzzle {
    /// Create puzzle from input string.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn from_input(input: &str) -> Self {
        Self(
            input
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }

    /// Show puzzle.
    pub fn show(&self) {
        let total_size = self.0.len();

        for y in 0..total_size {
            for x in 0..total_size {
                print!("{}", self.0[y][x]);
            }

            println!();
        }
    }

    /// Extract line from puzzle.
    ///
    /// # Arguments
    ///
    /// * `idx` - Line index
    pub fn extract_line(&self, idx: usize) -> &[char] {
        &self.0[idx][..]
    }
}

/// Tile matcher.
pub struct TileMatcher;

/// Tile matches.
pub type TileMatches = HashMap<usize, Vec<(usize, BorderDirection, BorderDirection, bool)>>;

impl TileMatcher {
    /// Find all matches.
    ///
    /// # Arguments
    ///
    /// * `tiles` - Tiles
    pub fn find_all_matches(tiles: &[Tile]) -> TileMatches {
        let mut matches = HashMap::new();

        for (idx, t1) in tiles.iter().enumerate() {
            for t2 in &tiles[idx + 1..] {
                let t1_borders = TileManipulator::extract_borders(&t1.data);
                let t2_borders = TileManipulator::extract_borders(&t2.data);

                for (t1_d, t1_v) in &t1_borders {
                    for (t2_d, t2_v) in &t2_borders {
                        let mut is_matching = false;
                        let mut is_inverted = false;

                        if t1_v == t2_v {
                            is_matching = true;
                        } else if t1_v == &TileManipulator::invert_side(t2_v) {
                            is_matching = true;
                            is_inverted = true;
                        }

                        if is_matching {
                            let v = matches.entry(t1.id).or_insert(vec![]);
                            v.push((t2.id, *t1_d, *t2_d, is_inverted));

                            let v = matches.entry(t2.id).or_insert(vec![]);
                            v.push((t1.id, *t2_d, *t1_d, is_inverted));
                        }
                    }
                }
            }
        }

        matches
    }

    /// Find puzzle corners.
    ///
    /// # Arguments
    ///
    /// * `tiles` - Tiles
    pub fn find_puzzle_corners(tiles: &[Tile]) -> Vec<usize> {
        let matches = Self::find_all_matches(tiles);

        Self::find_corners_from_matches(&matches)
    }

    /// Find corners from matches.
    ///
    /// # Arguments
    ///
    /// * `matches` - Matches
    pub fn find_corners_from_matches(matches: &TileMatches) -> Vec<usize> {
        // Corners only have two matches
        matches
            .iter()
            .filter_map(|(&k, v)| if v.len() == 2 { Some(k) } else { None })
            .collect()
    }

    /// Get tile from ID.
    ///
    /// # Arguments
    ///
    /// * `tiles` - Tiles
    /// * `id` - Tile ID
    pub fn get_tile_from_id(tiles: &[Tile], id: usize) -> &Tile {
        tiles.iter().find(|&t| t.id == id).unwrap()
    }

    /// Rotate a corner to be top-left.
    ///
    /// # Arguments
    ///
    /// * `tile` - Tile
    /// * `matches` - Matches
    pub fn rotate_corner_top_left(tile: &Tile, matches: &TileMatches) -> Tile {
        use BorderDirection::{Bottom, Left, Right, Top};

        let tile_matches = matches.get(&tile.id).unwrap();
        let (_, t1d, _, _) = tile_matches[0];
        let (_, t2d, _, _) = tile_matches[1];

        match (t1d, t2d) {
            (Top, Right) | (Right, Top) => {
                tile.clone_with_data(TileManipulator::rotated_90_cw(&tile.data))
            }
            (Right, Bottom) | (Bottom, Right) => tile.clone(),
            (Bottom, Left) | (Left, Bottom) => {
                tile.clone_with_data(TileManipulator::rotated_90_ccw(&tile.data))
            }
            (Left, Top) | (Top, Left) => {
                tile.clone_with_data(TileManipulator::rotated_180_cw(&tile.data))
            }
            _ => unreachable!(),
        }
    }

    /// Rotate a tile to match another on a direction.
    ///
    /// # Arguments
    ///
    /// * `tile` - Tile
    /// * `other_border` - Other tile border
    /// * `direction` - Direction
    pub fn rotate_tile_to_match(
        tile: &Tile,
        other_border: &str,
        direction: BorderDirection,
    ) -> Option<Tile> {
        // Rotate and/or flip `tile` to match `other_border` on direction `direction`
        let mut current_tile = tile.clone();
        let mut this_border = TileManipulator::extract_single_border(&current_tile.data, direction);
        let mut rotations = 0;

        loop {
            // OK
            if this_border == other_border {
                break;
            } else if this_border == TileManipulator::invert_side(&other_border) {
                match direction {
                    BorderDirection::Left | BorderDirection::Right => {
                        current_tile = current_tile
                            .clone_with_data(TileManipulator::flip_vertical(&current_tile.data));
                    }
                    _ => {
                        current_tile = current_tile
                            .clone_with_data(TileManipulator::flip_horizontal(&current_tile.data));
                    }
                }
            } else {
                current_tile = current_tile
                    .clone_with_data(TileManipulator::rotated_90_ccw(&current_tile.data));
                rotations += 1;
            }

            if rotations == 4 {
                // Abort
                return None;
            }

            this_border = TileManipulator::extract_single_border(&current_tile.data, direction);
        }

        Some(current_tile)
    }

    /// Position tile at right of another.
    ///
    /// # Arguments
    ///
    /// * `source` - Source tile
    /// * `matches` - Matches
    /// * `tiles` - Tiles
    pub fn position_tile_at_right(
        source: &Tile,
        matches: &TileMatches,
        tiles: &[Tile],
    ) -> Option<Tile> {
        // Iterate on matches from tile
        let tile_right_border =
            TileManipulator::extract_single_border(&source.data, BorderDirection::Right);
        let tile_matches = matches.get(&source.id).unwrap();
        for &(other_tile_id, _, _, _) in tile_matches {
            let other_tile = Self::get_tile_from_id(tiles, other_tile_id);
            if let Some(other_rotated_tile) =
                Self::rotate_tile_to_match(other_tile, &tile_right_border, BorderDirection::Left)
            {
                return Some(other_rotated_tile);
            }
        }

        None
    }

    /// Position tile at bottom of another.
    ///
    /// # Arguments
    ///
    /// * `source` - Source tile
    /// * `matches` - Matches
    /// * `tiles` - Tiles
    pub fn position_tile_at_bottom(
        source: &Tile,
        matches: &TileMatches,
        tiles: &[Tile],
    ) -> Option<Tile> {
        // Iterate on matches from tile
        let tile_right_border =
            TileManipulator::extract_single_border(&source.data, BorderDirection::Bottom);
        let tile_matches = matches.get(&source.id).unwrap();
        for &(other_tile_id, _, _, _) in tile_matches {
            let other_tile = Self::get_tile_from_id(tiles, other_tile_id);
            if let Some(other_rotated_tile) =
                Self::rotate_tile_to_match(other_tile, &tile_right_border, BorderDirection::Top)
            {
                return Some(other_rotated_tile);
            }
        }

        None
    }

    /// Build puzzle from tiles.
    ///
    /// # Arguments
    ///
    /// * `tiles` - Tiles
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss
    )]
    pub fn build_puzzle(tiles: &[Tile]) -> Puzzle {
        let puzzle_size = (tiles.len() as f64).sqrt().round() as usize;
        let matches = Self::find_all_matches(tiles);
        let corners = Self::find_corners_from_matches(&matches);
        let first_corner_tile = Self::get_tile_from_id(tiles, corners[0]);

        // Prepare output
        let mut output: Vec<Vec<Option<Tile>>> = Vec::new();
        output.resize(puzzle_size, vec![]);
        for x in &mut output {
            x.resize(puzzle_size, None);
        }

        // Place top-left
        let rotated_corner = Self::rotate_corner_top_left(first_corner_tile, &matches);
        output[0][0] = Some(rotated_corner);

        for y in 0..puzzle_size {
            if y != 0 {
                // Insert at bottom
                let last_tile = &output[y - 1][0].as_ref().unwrap();
                if let Some(tile) = Self::position_tile_at_bottom(&last_tile, &matches, tiles) {
                    output[y][0] = Some(tile);
                } else {
                    panic!("Tile not found (bottom) for position (0, {})", y);
                }
            }

            for x in 1..puzzle_size {
                let last_tile = &output[y][x - 1].as_ref().unwrap();
                if let Some(tile) = Self::position_tile_at_right(&last_tile, &matches, tiles) {
                    output[y][x] = Some(tile);
                } else {
                    panic!("Tile not found (right) for position ({}, {})", x, y);
                }
            }
        }

        PuzzleBuilder(output).build()
    }

    /// Check if a line slice match a monster line.
    ///
    /// # Arguments
    ///
    /// * `line` - Line slice
    /// * `monster_line` - Monster line index
    pub fn line_match_monster(line: &[char], monster_line: usize) -> bool {
        if line.len() < 20 {
            return false;
        }

        MONSTER[monster_line].iter().all(|&i| line[i] == '#')
    }

    /// Replace monster components with a 'O' character.
    ///
    /// # Arguments
    ///
    /// * `puzzle` - Puzzle
    /// * `x` - Starting X position
    /// * `y` - Starting Y position
    pub fn replace_monsters(puzzle: &mut Puzzle, x: usize, y: usize) {
        puzzle.0[y][x + 18] = 'O';
        for i in &[0, 5, 6, 11, 12, 17, 18, 19] {
            puzzle.0[y + 1][x + i] = 'O';
        }
        for i in &[1, 4, 7, 10, 13, 16] {
            puzzle.0[y + 2][x + i] = 'O';
        }
    }

    /// Find and replace sea monsters lines.
    ///
    /// # Arguments
    ///
    /// * `puzzle` - Puzzle
    pub fn find_and_replace_sea_monsters_lines(puzzle: &Puzzle) -> Option<Puzzle> {
        let puzzle_size = puzzle.0.len();
        let mut replaced_puzzle = None;

        // Scan lines
        for y in 0..puzzle_size - 3 {
            let line1 = puzzle.extract_line(y);
            for x in 0..puzzle_size - 20 {
                if Self::line_match_monster(&line1[x..x + 20], 0) {
                    let line2 = puzzle.extract_line(y + 1);
                    if Self::line_match_monster(&line2[x..x + 20], 1) {
                        let line3 = puzzle.extract_line(y + 2);
                        if Self::line_match_monster(&line3[x..x + 20], 2) {
                            if replaced_puzzle.is_none() {
                                replaced_puzzle = Some(Puzzle(puzzle.0.clone()));
                            }

                            Self::replace_monsters(&mut replaced_puzzle.as_mut().unwrap(), x, y);
                        }
                    }
                }
            }
        }

        replaced_puzzle
    }

    /// Find and replace sea monsters.
    ///
    /// # Arguments
    ///
    /// * `puzzle` - Puzzle
    fn find_and_replace_sea_monsters(puzzle: &Puzzle) -> Option<Puzzle> {
        let mut current_puzzle = Puzzle(puzzle.0.clone());

        for flip_state in &[TileFlip::None, TileFlip::Horizontal, TileFlip::Vertical] {
            match flip_state {
                TileFlip::None => (),
                TileFlip::Horizontal => {
                    current_puzzle = Puzzle(TileManipulator::flip_horizontal(&current_puzzle.0));
                }
                TileFlip::Vertical => {
                    current_puzzle = Puzzle(TileManipulator::flip_vertical(&current_puzzle.0));
                }
            }

            for _ in 0..4 {
                if let Some(puzzle) = Self::find_and_replace_sea_monsters_lines(&current_puzzle) {
                    return Some(puzzle);
                }

                current_puzzle = Puzzle(TileManipulator::rotated_90_ccw(&current_puzzle.0));
            }
        }

        None
    }
}

/// Tile parser.
pub struct TileParser;

impl TileParser {
    /// Parse tile from input.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn parse_from_input(input: &str) -> Tile {
        let mut lines = input.lines();
        let id = Self::parse_tile_id(lines.next().unwrap());
        let data = Self::parse_tile_data(lines);

        Tile { id, data }
    }

    /// Parse multiple tiles from input.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn parse_multiple_from_input(input: &str) -> Vec<Tile> {
        input
            .split("\n\n")
            .map(|block| Self::parse_from_input(block))
            .collect()
    }

    /// Parse a tile ID.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn parse_tile_id(input: &str) -> usize {
        TILE_ID_RGX
            .captures(input.trim())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap()
    }

    /// Parse tile data.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn parse_tile_data<'a, I>(input: I) -> Vec<Vec<bool>>
    where
        I: Iterator<Item = &'a str>,
    {
        input
            .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }
}

impl From<&str> for Tile {
    fn from(value: &str) -> Self {
        TileParser::parse_from_input(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 140_656_720_229_539;
    const EX2_OUTPUT: usize = 1885;

    const SMALL_SAMPLE: &str = indoc::indoc! {"
        Tile 1:
        .#.#
        ....
        .###
        .#..
    "};

    const SINGLE_SAMPLE: &str = indoc::indoc! {"
        Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###
    "};

    const MULTIPLE_SAMPLE: &str = indoc::indoc! {"
        Tile 2311:
        ..##.#..#.
        ##..#.....
        #...##..#.
        ####.#...#
        ##.##.###.
        ##...#.###
        .#.#.#..##
        ..#....#..
        ###...#.#.
        ..###..###

        Tile 1951:
        #.##...##.
        #.####...#
        .....#..##
        #...######
        .##.#....#
        .###.#####
        ###.##.##.
        .###....#.
        ..#.#..#.#
        #...##.#..

        Tile 1171:
        ####...##.
        #..##.#..#
        ##.#..#.#.
        .###.####.
        ..###.####
        .##....##.
        .#...####.
        #.##.####.
        ####..#...
        .....##...

        Tile 1427:
        ###.##.#..
        .#..#.##..
        .#.##.#..#
        #.#.#.##.#
        ....#...##
        ...##..##.
        ...#.#####
        .#.####.#.
        ..#..###.#
        ..##.#..#.

        Tile 1489:
        ##.#.#....
        ..##...#..
        .##..##...
        ..#...#...
        #####...#.
        #..#.#.#.#
        ...#.#.#..
        ##.#...##.
        ..##.##.##
        ###.##.#..

        Tile 2473:
        #....####.
        #..#.##...
        #.##..#...
        ######.#.#
        .#...#.#.#
        .#########
        .###.#..#.
        ########.#
        ##...##.#.
        ..###.#.#.

        Tile 2971:
        ..#.#....#
        #...###...
        #.#.###...
        ##.##..#..
        .#####..##
        .#..####.#
        #..#.#..#.
        ..####.###
        ..#.#.###.
        ...#.#.#.#

        Tile 2729:
        ...#.#.#.#
        ####.#....
        ..#.#.....
        ....#..#.#
        .##..##.#.
        .#.####...
        ####.#.#..
        ##.####...
        ##..#.##..
        #.##...##.

        Tile 3079:
        #.#.#####.
        .#..######
        ..#.......
        ######....
        ####.#..#.
        .#...#.##.
        #.#####.##
        ..#.###...
        ..#.......
        ..#.###...
    "};

    const SAMPLE_BUILT: &str = indoc::indoc! {"
        .#.#..#.##...#.##..#####
        ###....#.#....#..#......
        ##.##.###.#.#..######...
        ###.#####...#.#####.#..#
        ##.#....#.##.####...#.##
        ...########.#....#####.#
        ....#..#...##..#.#.###..
        .####...#..#.....#......
        #..#.##..#..###.#.##....
        #.####..#.####.#.#.###..
        ###.#.#...#.######.#..##
        #.####....##..########.#
        ##..##.#...#...#.#.#.#..
        ...#..#..#.#.##..###.###
        .#.#....#.##.#...###.##.
        ###.#...#..#.##.######..
        .#.#.###.##.##.#..#.##..
        .####.###.#...###.#..#.#
        ..#.#..#..#.#.#.####.###
        #..####...#.#.#.###.###.
        #####..#####...###....##
        #.##..#..#...#..####...#
        .#.###..##..##..####.##.
        ...###...##...#...#..###
    "};

    #[allow(clippy::needless_range_loop, clippy::needless_pass_by_value)]
    fn data_to_ascii(data: Vec<Vec<bool>>) -> String {
        let mut output = String::new();
        let size = data.len();

        for i in 0..size {
            for j in 0..size {
                if data[i][j] {
                    output.push('#');
                } else {
                    output.push('.');
                }
            }
            output.push('\n');
        }

        output
    }

    #[test]
    fn test_tile_parser_single() {
        let tile: Tile = SINGLE_SAMPLE.into();
        assert_eq!(tile.id, 2311);
        assert_eq!(tile.data.len(), 10);
    }

    #[test]
    fn test_tile_parser_multiple() {
        let tiles = TileParser::parse_multiple_from_input(MULTIPLE_SAMPLE);
        assert_eq!(tiles.len(), 9);
    }

    #[test]
    fn test_tile_rotation() {
        let tile: Tile = SMALL_SAMPLE.into();
        let original_tile = indoc::indoc! {"
            .#.#
            ....
            .###
            .#..
        "};

        let r90_tile = indoc::indoc! {"
            #.#.
            ..#.
            #.##
            ....
        "};

        let r180_tile = indoc::indoc! {"
            ..#.
            ###.
            ....
            #.#.
        "};

        let r270_tile = indoc::indoc! {"
            ....
            ##.#
            .#..
            .#.#
        "};

        assert_eq!(
            data_to_ascii(TileManipulator::rotated(&tile.data, TileRotation::R0)),
            original_tile
        );
        assert_eq!(
            data_to_ascii(TileManipulator::rotated(&tile.data, TileRotation::R90)),
            r90_tile
        );
        assert_eq!(
            data_to_ascii(TileManipulator::rotated(&tile.data, TileRotation::R180)),
            r180_tile
        );
        assert_eq!(
            data_to_ascii(TileManipulator::rotated(&tile.data, TileRotation::R270)),
            r270_tile
        );
    }

    #[test]
    fn test_tile_flip() {
        let tile: Tile = SMALL_SAMPLE.into();

        let vert_flip = indoc::indoc! {"
            .#..
            .###
            ....
            .#.#
        "};

        let horiz_flip = indoc::indoc! {"
            #.#.
            ....
            ###.
            ..#.
        "};

        assert_eq!(
            data_to_ascii(TileManipulator::flip_vertical(&tile.data)),
            vert_flip
        );
        assert_eq!(
            data_to_ascii(TileManipulator::flip_horizontal(&tile.data)),
            horiz_flip
        );
    }

    #[test]
    fn test_tile_borders() {
        let tile: Tile = SMALL_SAMPLE.into();

        assert_eq!(
            &TileManipulator::extract_single_border(&tile.data, BorderDirection::Top),
            ".#.#"
        );
        assert_eq!(
            &TileManipulator::extract_single_border(&tile.data, BorderDirection::Left),
            "...."
        );
        assert_eq!(
            &TileManipulator::extract_single_border(&tile.data, BorderDirection::Right),
            "#.#."
        );
        assert_eq!(
            &TileManipulator::extract_single_border(&tile.data, BorderDirection::Bottom),
            ".#.."
        );
    }

    #[test]
    fn test_resolve_sample() {
        let tiles = TileParser::parse_multiple_from_input(MULTIPLE_SAMPLE);
        let mut corners = TileMatcher::find_puzzle_corners(&tiles);
        corners.sort_unstable();

        assert_eq!(corners, vec![1171, 1951, 2971, 3079]);
        assert_eq!(corners.iter().product::<usize>(), 20_899_048_083_289);
    }

    #[test]
    fn test_build_puzzle() {
        let tiles = TileParser::parse_multiple_from_input(MULTIPLE_SAMPLE);
        TileMatcher::build_puzzle(&tiles);
    }

    #[test]
    fn test_find_and_replace_sea_monsters_from_built() {
        let puzzle = Puzzle::from_input(SAMPLE_BUILT.trim());

        // Rotate and flip puzzle to forward match
        let puzzle = Puzzle(TileManipulator::flip_horizontal(
            &TileManipulator::rotated_90_cw(&puzzle.0),
        ));
        assert!(TileMatcher::find_and_replace_sea_monsters(&puzzle).is_some());

        // Re-rotate
        let puzzle = Puzzle(TileManipulator::rotated_90_cw(&puzzle.0));
        assert!(TileMatcher::find_and_replace_sea_monsters(&puzzle).is_some());

        // Re-rotate
        let puzzle = Puzzle(TileManipulator::rotated_90_cw(&puzzle.0));
        assert!(TileMatcher::find_and_replace_sea_monsters(&puzzle).is_some());

        // Re-rotate
        let puzzle = Puzzle(TileManipulator::rotated_90_cw(&puzzle.0));
        assert!(TileMatcher::find_and_replace_sea_monsters(&puzzle).is_some());
    }

    #[test]
    fn test_find_and_replace_sea_monsters_from_sample() {
        let tiles = TileParser::parse_multiple_from_input(MULTIPLE_SAMPLE);
        let puzzle = TileMatcher::build_puzzle(&tiles);
        let puzzle = TileMatcher::find_and_replace_sea_monsters(&puzzle).unwrap();
        puzzle.show();
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
