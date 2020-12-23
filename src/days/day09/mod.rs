//! # Day 9: Encoding Error
//!
//! With your neighbor happily enjoying their video game, you turn your attention to an open data port on the little screen in the seat in front of you.
//!
//! Though the port is non-standard, you manage to connect it to your computer through the clever use of several paperclips. Upon connection, the port outputs a series of numbers (your puzzle input).
//!
//! The data appears to be encrypted with the eXchange-Masking Addition System (XMAS) which, conveniently for you, is an old cypher with an important weakness.
//!
//! XMAS starts by transmitting a preamble of 25 numbers. After that, each number you receive should be the sum of any two of the 25 immediately previous numbers. The two numbers will have different values, and there might be more than one such pair.
//!
//! For example, suppose your preamble consists of the numbers 1 through 25 in a random order. To be valid, the next number must be the sum of two of those numbers:
//!
//! - 26 would be a valid next number, as it could be 1 plus 25 (or many other pairs, like 2 and 24).
//! - 49 would be a valid next number, as it is the sum of 24 and 25.
//! - 100 would not be valid; no two of the previous 25 numbers sum to 100.
//! - 50 would also not be valid; although 25 appears in the previous 25 numbers, the two numbers in the pair must be different.
//!
//! Suppose the 26th number is 45, and the first number (no longer an option, as it is more than 25 numbers ago) was 20. Now, for the next number to be valid, there needs to be some pair of numbers among 1-19, 21-25, or 45 that add up to it:
//!
//! - 26 would still be a valid next number, as 1 and 25 are still within the previous 25 numbers.
//! - 65 would not be valid, as no two of the available numbers sum to it.
//! - 64 and 66 would both be valid, as they are the result of 19+45 and 21+45 respectively.
//!
//! Here is a larger example which only considers the previous 5 numbers (and has a preamble of length 5):
//!
//! ```text
//! 35
//! 20
//! 15
//! 25
//! 47
//! 40
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this example, after the 5-number preamble, almost every number is the sum of two of the previous 5 numbers; the only number that does not follow this rule is 127.
//!
//! The first step of attacking the weakness in the XMAS data is to find the first number in the list (after the preamble) which is not the sum of two of the 25 numbers before it. What is the first number that does not have this property?
//!
//! # Part Two
//!
//! The final step in breaking the XMAS encryption relies on the invalid number you just found: you must find a contiguous set of at least two numbers in your list which sum to the invalid number from step 1.
//!
//! Again consider the above example:
//!
//! ```text
//! 35
//! 20
//! 15
//! 25
//! 47
//! 40
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this list, adding up all of the numbers from 15 through 40 produces the invalid number from step 1, 127. (Of course, the contiguous set of numbers in your actual list might be much longer.)
//!
//! To find the encryption weakness, add together the smallest and largest number in this contiguous range; in this example, these are 15 and 47, producing 62.
//!
//! What is the encryption weakness in your XMAS-encrypted list of numbers?

use std::cmp::Ordering;

use itertools::Itertools;

const EX1_RING_SIZE: usize = 25;
const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> usize {
    XmasScanner::parse_and_find_error(INPUT_VALUES, EX1_RING_SIZE).unwrap()
}

/// Part two answer.
pub fn run_ex2() -> usize {
    let target = run_ex1();
    XmasScanner::find_weakness(INPUT_VALUES, target)
        .map(|x| x.get_sum())
        .unwrap()
}

/// Xmas Weakness output
pub struct XmasWeaknessOutput {
    data: Vec<usize>,
    start_cursor: usize,
    end_cursor: usize,
}

impl XmasWeaknessOutput {
    /// Create output from data, start and end cursor.
    ///
    /// # Arguments
    ///
    /// * `data` - Data
    /// * `start_cursor` - Start cursor
    /// * `end_cursor` - End cursor
    pub fn new(data: Vec<usize>, start_cursor: usize, end_cursor: usize) -> Self {
        Self {
            data,
            start_cursor,
            end_cursor,
        }
    }

    /// Find smallest and largest value in data.
    pub fn get_min_max(&self) -> (usize, usize) {
        let range = &self.data[self.start_cursor..self.end_cursor];
        (*range.iter().min().unwrap(), *range.iter().max().unwrap())
    }

    /// Get sum of min and max value.
    pub fn get_sum(&self) -> usize {
        let min_max = self.get_min_max();
        min_max.0 + min_max.1
    }
}

/// Xmas scanner
pub struct XmasScanner;

impl XmasScanner {
    /// Parse input string and find error if any.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn parse_and_find_error(input: &str, ring_size: usize) -> Option<usize> {
        let mut data = Vec::with_capacity(ring_size);
        // Parse initial data
        let mut initial_data = input
            .lines()
            .take(ring_size)
            .filter_map(|x| x.trim().parse::<usize>().ok())
            .collect();
        data.append(&mut initial_data);

        // Now parse remaining items
        let remaining_items: Vec<usize> = input
            .lines()
            .skip(ring_size)
            .filter_map(|x| x.trim().parse::<usize>().ok())
            .collect();

        for x in remaining_items {
            if Self::find_sum(&data, x).is_some() {
                data.remove(0);
                data.push(x);
            } else {
                return Some(x);
            }
        }

        None
    }

    /// Find a sum of number from the ring equals to the target number.
    ///
    /// # Arguments
    ///
    /// * `data` - Data
    /// * `target` - Target number
    pub fn find_sum(data: &[usize], target: usize) -> Option<(usize, usize)> {
        data.iter().combinations(2).find_map(|v| {
            if v[0] + v[1] == target {
                Some((*v[0], *v[1]))
            } else {
                None
            }
        })
    }

    /// Find weakness.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    /// * `invalid_number` - Invalid number
    pub fn find_weakness(input: &str, invalid_number: usize) -> Option<XmasWeaknessOutput> {
        let numbers: Vec<usize> = input
            .lines()
            .filter_map(|x| x.trim().parse().ok())
            .collect();

        for start_i in 0..numbers.len() {
            let mut acc = numbers[start_i];

            for end_i in start_i + 1..numbers.len() {
                acc += numbers[end_i];

                match acc.cmp(&invalid_number) {
                    Ordering::Equal => {
                        return Some(XmasWeaknessOutput::new(numbers, start_i, end_i))
                    }
                    Ordering::Greater => {
                        break;
                    }
                    Ordering::Less => (),
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 556_543_474;
    const EX2_OUTPUT: usize = 76_096_372;

    const SAMPLE: &str = r#"35
    20
    15
    25
    47
    40
    62
    55
    65
    95
    102
    117
    150
    182
    127
    219
    299
    277
    309
    576"#;

    #[test]
    fn test_parse_and_find_error() {
        assert_eq!(XmasScanner::parse_and_find_error(SAMPLE, 5), Some(127));
    }

    #[test]
    fn test_find_weakness() {
        let weakness = XmasScanner::find_weakness(SAMPLE, 127).unwrap();
        assert_eq!(weakness.start_cursor, 2);
        assert_eq!(weakness.end_cursor, 5);
        assert_eq!(weakness.get_min_max(), (15, 47));
        assert_eq!(weakness.get_sum(), 62);
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
