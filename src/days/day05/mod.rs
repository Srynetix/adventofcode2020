//! # Day 5: Binary Boarding
//!
//! You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.
//!
//! You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.
//!
//! Instead of zones or groups, this airline uses binary space partitioning to seat people. A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".
//!
//! The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the front (0 through 63) or the back (64 through 127). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.
//!
//! For example, consider just the first seven characters of FBFBBFFRLR:
//!
//! - Start by considering the whole range, rows 0 through 127.
//! - F means to take the lower half, keeping rows 0 through 63.
//! - B means to take the upper half, keeping rows 32 through 63.
//! - F means to take the lower half, keeping rows 32 through 47.
//! - B means to take the upper half, keeping rows 40 through 47.
//! - B keeps rows 44 through 47.
//! - F keeps rows 44 through 45.
//! - The final F keeps the lower of the two, row 44.
//!
//! The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7). The same process as above proceeds again, this time with only three steps. L means to keep the lower half, while R means to keep the upper half.
//!
//! For example, consider just the last 3 characters of FBFBBFFRLR:
//!
//! - Start by considering the whole range, columns 0 through 7.
//! - R means to take the upper half, keeping columns 4 through 7.
//! - L means to take the lower half, keeping columns 4 through 5.
//! - The final R keeps the upper of the two, column 5.
//!
//! So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.
//!
//! Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID 44 * 8 + 5 = 357.
//!
//! Here are some other boarding passes:
//!
//! - BFFFBBFRRR: row 70, column 7, seat ID 567.
//! - FFFBBBFRRR: row 14, column 7, seat ID 119.
//! - BBFFBBFRLL: row 102, column 4, seat ID 820.
//!
//! As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass?
//!
//! # Part Two
//!
//! Ding! The "fasten seat belt" signs have turned on. Time to find your seat.
//!
//! It's a completely full flight, so your seat should be the only missing boarding pass in your list. However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.
//!
//! Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.
//!
//! What is the ID of your seat?

use eyre::Result;

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> usize {
    BoardingPass::from_entries(INPUT_VALUES)
        .expect("should work")
        .iter()
        .map(BoardingPass::get_seat_id)
        .max()
        .expect("Should contain at least one value")
}

/// Part two answer.
pub fn run_ex2() -> usize {
    let passes = BoardingPass::from_entries(INPUT_VALUES).expect("should work");

    let mut seats: Vec<usize> = passes.iter().map(BoardingPass::get_seat_id).collect();
    seats.sort_unstable();

    let mut last = 0_usize;
    for s in seats {
        if last != 0 && last != s - 1 {
            return s - 1;
        }

        last = s;
    }

    panic!("Seat not found");
}

/// Boarding pass.
#[derive(Debug, PartialEq, Eq)]
pub struct BoardingPass {
    row: usize,
    column: usize,
}

impl BoardingPass {
    /// Convert entries to boarding passes.
    ///
    /// # Arguments
    ///
    /// * `entries` - Boarding entries
    ///
    /// # Errors
    ///
    /// * Parse error
    pub fn from_entries(entries: &str) -> Result<Vec<Self>> {
        entries.lines().map(Self::from_entry).collect()
    }

    /// Convert entry to boarding pass.
    ///
    /// # Arguments
    ///
    /// * `entry` - Boarding entry
    ///
    /// # Errors
    //
    /// * Parse error
    pub fn from_entry(entry: &str) -> Result<Self> {
        // Convert row as binary to decimal (7 first letters)
        let row = usize::from_str_radix(
            &entry
                .chars()
                .take(7)
                .map(|x| match x {
                    'F' => '0',
                    'B' => '1',
                    e => panic!("Unknown letter {}", e),
                })
                .collect::<String>(),
            2,
        )?;

        // Convert column as binary to decimal (3 last letters)
        let column = usize::from_str_radix(
            &entry
                .chars()
                .skip(7)
                .take(3)
                .map(|x| match x {
                    'L' => '0',
                    'R' => '1',
                    e => panic!("Unknown letter {}", e),
                })
                .collect::<String>(),
            2,
        )?;

        Ok(Self { row, column })
    }

    /// Get seat ID from boarding pass.
    pub const fn get_seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 818;
    const EX2_OUTPUT: usize = 559;

    #[test]
    fn test_from_entry() {
        assert_eq!(
            BoardingPass::from_entry("FBFBBFFRLR").unwrap(),
            BoardingPass { row: 44, column: 5 }
        );
        assert_eq!(
            BoardingPass::from_entry("BFFFBBFRRR").unwrap(),
            BoardingPass { row: 70, column: 7 }
        );
        assert_eq!(
            BoardingPass::from_entry("FFFBBBFRRR").unwrap(),
            BoardingPass { row: 14, column: 7 }
        );
        assert_eq!(
            BoardingPass::from_entry("BBFFBBFRLL").unwrap(),
            BoardingPass {
                row: 102,
                column: 4
            }
        );
    }

    #[test]
    fn test_get_seat_id() {
        assert_eq!(BoardingPass { row: 44, column: 5 }.get_seat_id(), 357);
        assert_eq!(BoardingPass { row: 70, column: 7 }.get_seat_id(), 567);
        assert_eq!(BoardingPass { row: 14, column: 7 }.get_seat_id(), 119);
        assert_eq!(
            BoardingPass {
                row: 102,
                column: 4
            }
            .get_seat_id(),
            820
        );
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
