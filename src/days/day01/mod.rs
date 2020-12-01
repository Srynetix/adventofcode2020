//! # Day 1: Report Repair
//!
//! After saving Christmas five years in a row, you've decided to take a vacation at a nice resort on a tropical island. Surely, Christmas will go on without you.
//!
//! The tropical island has its own currency and is entirely cash-only. The gold coins used there have a little picture of a starfish; the locals just call them stars. None of the currency exchanges seem to have heard of them, but somehow, you'll need to find fifty of these coins by the time you arrive so you can pay the deposit on your room.
//!
//! To save your vacation, you need to get all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
//!
//! Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.
//!
//! Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.
//!
//! For example, suppose your expense report contained the following:
//!
//! ```text
//!     1721
//!     979
//!     366
//!     299
//!     675
//!     1456
//! ```
//!
//! In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.
//!
//! Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
//!
//!
//! # Part Two
//!
//! The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.
//!
//! Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.
//!
//! In your expense report, what is the product of the three entries that sum to 2020?

use itertools::Itertools;

const INPUT_VALUES: &str = include_str!("input.txt");

pub fn run_ex1() -> u64 {
    multiply_values(search_if_eq_2020(INPUT_VALUES, 2))
}

pub fn run_ex2() -> u64 {
    multiply_values(search_if_eq_2020(INPUT_VALUES, 3))
}

fn multiply_values(values: Vec<u64>) -> u64 {
    values.iter().product()
}

fn search_if_eq_2020(entries_content: &str, combinations: usize) -> Vec<u64> {
    search_if_eq(entries_content, combinations, 2020)
}

fn search_if_eq(entries_content: &str, combinations: usize, target: u64) -> Vec<u64> {
    entries_content
        .lines()
        .filter_map(|s| s.parse::<u64>().ok())
        .combinations(combinations)
        .find(|v| v.iter().sum::<u64>() == target)
        .unwrap_or_else(Vec::new)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: u64 = 987339;
    const EX2_OUTPUT: u64 = 259521570;

    #[test]
    fn test_multiply_values() {
        assert_eq!(multiply_values(vec![]), 1);
        assert_eq!(multiply_values(vec![2020, 0]), 0);
        assert_eq!(multiply_values(vec![2020, 2]), 4040);
    }

    #[test]
    fn test_search_if_eq_2020() {
        assert_eq!(search_if_eq_2020("1234\n5678\n2020\n0", 2), vec![2020, 0]);
        assert_eq!(search_if_eq_2020("1234\n5678", 2), vec![]);
        assert_eq!(search_if_eq_2020("", 2), vec![]);
        assert_eq!(search_if_eq_2020("1234", 2), vec![]);
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
