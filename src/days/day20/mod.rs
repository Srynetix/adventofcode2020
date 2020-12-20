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

use std::{collections::HashMap, convert::TryFrom};

use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;

const INPUT_VALUES: &str = include_str!("input.txt");
static TILE_ID_RGX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"Tile (\d+):"#).unwrap());

/// Part one answer.
pub fn run_ex1() -> usize {
    let tiles = TileParser::parse_multiple_from_input(INPUT_VALUES).unwrap();
    TileMatcher::reorder_tiles_and_get_corners(&tiles)
        .iter()
        .product()
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

/// Tile.
pub struct Tile {
    id: usize,
    data: Vec<Vec<bool>>,
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

impl Tile {
    /// Show tile in console.
    ///
    /// # Errors
    ///
    /// * IO errors
    pub fn show(&self) -> std::io::Result<()> {
        use std::io::Write;

        let size = self.data.len();
        let visible_c = '█';
        let hidden_c = ' ';
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        for i in 0..size {
            for j in 0..size {
                if self.data[i][j] {
                    write!(handle, "{}", visible_c)?;
                } else {
                    write!(handle, "{}", hidden_c)?;
                }
            }
            writeln!(handle)?;
        }

        handle.flush()
    }
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
    pub fn rotated(data: &[Vec<bool>], rotation: TileRotation) -> Vec<Vec<bool>> {
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
    /// * `data` - Date
    pub fn flip_vertical(data: &[Vec<bool>]) -> Vec<Vec<bool>> {
        let size = data.len();
        let mut output = Self::allocate_empty_grid(size);

        for i in 0..size {
            for j in 0..size {
                output[i][j] = data[size - i - 1][j];
            }
        }

        output
    }

    /// Flip data horizontally.
    ///
    /// # Arguments
    ///
    /// * `data` - Data
    pub fn flip_horizontal(data: &[Vec<bool>]) -> Vec<Vec<bool>> {
        let size = data.len();
        let mut output = Self::allocate_empty_grid(size);

        for i in 0..size {
            for j in 0..size {
                output[i][j] = data[i][size - j - 1];
            }
        }

        output
    }

    #[allow(clippy::needless_range_loop)]
    fn rotated_90_cw(data: &[Vec<bool>]) -> Vec<Vec<bool>> {
        let size = data.len();
        let mut output = Self::allocate_empty_grid(size);

        for i in 0..size {
            for j in 0..size {
                output[i][j] = data[size - j - 1][i];
            }
        }

        output
    }

    fn rotated_180_cw(data: &[Vec<bool>]) -> Vec<Vec<bool>> {
        Self::rotated_90_cw(&Self::rotated_90_cw(data))
    }

    #[allow(clippy::needless_range_loop)]
    fn rotated_90_ccw(data: &[Vec<bool>]) -> Vec<Vec<bool>> {
        let size = data.len();
        let mut output = Self::allocate_empty_grid(size);

        for i in 0..size {
            for j in 0..size {
                output[i][j] = data[j][size - i - 1];
            }
        }

        output
    }

    fn allocate_empty_grid(size: usize) -> Vec<Vec<bool>> {
        let mut output = Vec::new();
        output.resize(size, vec![]);
        for v in &mut output {
            v.resize(size, false);
        }

        output
    }

    fn invert_side(s: &str) -> String {
        s.chars().rev().collect()
    }

    fn extract_borders(data: &[Vec<bool>]) -> HashMap<BorderDirection, String> {
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

    fn extract_single_border(data: &[Vec<bool>], direction: BorderDirection) -> String {
        Self::border_to_string(match direction {
            BorderDirection::Top => data[0][..].to_vec(),
            BorderDirection::Bottom => data[data.len() - 1][..].to_vec(),
            BorderDirection::Left => data.iter().map(|x| x[0]).collect(),
            BorderDirection::Right => data.iter().map(|x| x[data.len() - 1]).collect(),
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    fn border_to_string(border: Vec<bool>) -> String {
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

/// Tile matcher.
pub struct TileMatcher;

impl TileMatcher {
    /// Find all matches.
    ///
    /// # Arguments
    ///
    /// * `tiles` - Tiles
    pub fn find_all_matches(
        tiles: &[Tile],
    ) -> HashMap<usize, Vec<(usize, BorderDirection, BorderDirection)>> {
        let mut matches = HashMap::new();

        for (idx, t1) in tiles.iter().enumerate() {
            for t2 in &tiles[idx + 1..] {
                let t1_borders = TileManipulator::extract_borders(&t1.data);
                let t2_borders = TileManipulator::extract_borders(&t2.data);

                for (t1_d, t1_v) in &t1_borders {
                    for (t2_d, t2_v) in &t2_borders {
                        if t1_v == t2_v || t1_v == &TileManipulator::invert_side(t2_v) {
                            let v = matches.entry(t1.id).or_insert(vec![]);
                            v.push((t2.id, *t1_d, *t2_d));

                            let v = matches.entry(t2.id).or_insert(vec![]);
                            v.push((t1.id, *t2_d, *t1_d));
                        }
                    }
                }
            }
        }

        matches
    }

    /// Reorder tiles and get corners.
    ///
    /// # Arguments
    ///
    /// * `tiles` - Tiles
    pub fn reorder_tiles_and_get_corners(tiles: &[Tile]) -> Vec<usize> {
        let matches = Self::find_all_matches(tiles);

        // Corners only have two matches
        matches
            .iter()
            .filter_map(|(&k, v)| if v.len() == 2 { Some(k) } else { None })
            .collect()
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
    ///
    /// # Errors
    ///
    /// * Parse errors
    pub fn parse_from_input(input: &str) -> Result<Tile, DayError> {
        let mut lines = input.lines();
        let id =
            Self::parse_tile_id(lines.next().ok_or_else(|| {
                DayError::ParseError("Could not fetch tile ID line.".to_string())
            })?)?;

        let data = Self::parse_tile_data(lines)?;

        Ok(Tile { id, data })
    }

    /// Parse multiple tiles from input.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    ///
    /// # Errors
    ///
    /// * Parse errors
    pub fn parse_multiple_from_input(input: &str) -> Result<Vec<Tile>, DayError> {
        input
            .split("\n\n")
            .map(|block| Self::parse_from_input(block))
            .collect()
    }

    fn parse_tile_id(input: &str) -> Result<usize, DayError> {
        if let Some(captures) = TILE_ID_RGX.captures(input.trim()) {
            captures.get(1).unwrap().as_str().parse().map_err(|e| {
                DayError::ParseError(format!(
                    "Could not parse tile ID from line {}: {}",
                    input, e
                ))
            })
        } else {
            Err(DayError::ParseError(format!(
                "Could not parse tile ID from line {}. No match for regex.",
                input
            )))
        }
    }

    fn parse_tile_data<'a, I>(input: I) -> Result<Vec<Vec<bool>>, DayError>
    where
        I: Iterator<Item = &'a str>,
    {
        let data = input
            .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        if data.is_empty() {
            return Err(DayError::ParseError("Empty tile data".to_string()));
        }

        Ok(data)
    }
}

impl TryFrom<&str> for Tile {
    type Error = DayError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TileParser::parse_from_input(value)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use super::*;

    const EX1_OUTPUT: usize = 140_656_720_229_539;

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
        let tile: Tile = SINGLE_SAMPLE.try_into().unwrap();
        assert_eq!(tile.id, 2311);
        assert_eq!(tile.data.len(), 10);
    }

    #[test]
    fn test_tile_parser_multiple() {
        let tiles = TileParser::parse_multiple_from_input(MULTIPLE_SAMPLE).unwrap();
        assert_eq!(tiles.len(), 9);
    }

    #[test]
    fn test_tile_rotation() {
        let tile: Tile = SMALL_SAMPLE.try_into().unwrap();
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
        let tile: Tile = SMALL_SAMPLE.try_into().unwrap();

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
        let tile: Tile = SMALL_SAMPLE.try_into().unwrap();

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
        let tiles = TileParser::parse_multiple_from_input(MULTIPLE_SAMPLE).unwrap();
        let mut corners = TileMatcher::reorder_tiles_and_get_corners(&tiles);
        corners.sort_unstable();

        assert_eq!(corners, vec![1171, 1951, 2971, 3079]);
        assert_eq!(corners.iter().product::<usize>(), 20_899_048_083_289);
    }

    #[test]
    fn test_run_ex1() {
        assert_eq!(run_ex1(), EX1_OUTPUT);
    }
}
