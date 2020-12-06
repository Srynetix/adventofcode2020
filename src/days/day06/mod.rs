//! # Day 6: Custom Customs
//!
//! As your flight approaches the regional airport where you'll switch to a much larger plane, customs declaration forms are distributed to the passengers.
//!
//! The form asks a series of 26 yes-or-no questions marked a through z. All you need to do is identify the questions for which anyone in your group answers "yes". Since your group is just you, this doesn't take very long.
//!
//! However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help. For each of the people in their group, you write down the questions for which they answer "yes", one per line. For example:
//!
//! ```text
//! abcx
//! abcy
//! abcz
//! ```
//!
//! In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z. (Duplicate answers to the same question don't count extra; each question counts at most once.)
//!
//! Another group asks for your help, then another, and eventually you've collected answers from every group on the plane (your puzzle input). Each group's answers are separated by a blank line, and within each group, each person's answers are on a single line. For example:
//!
//! ```text
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! ```
//!
//! This list represents answers from five groups:
//!
//! - The first group contains one person who answered "yes" to 3 questions: a, b, and c.
//! - The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
//! - The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
//! - The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
//! - The last group contains one person who answered "yes" to only 1 question, b.
//!
//! In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.
//!
//! For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?
//!
//! # Part Two
//!
//! As you finish the last group's customs declaration, you notice that you misread one word in the instructions:
//!
//! You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which everyone answered "yes"!
//!
//! Using the same example as above:
//!
//! ```text
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! ```
//!
//! This list represents answers from five groups:
//!
//! - In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
//! - In the second group, there is no question to which everyone answered "yes".
//! - In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
//! - In the fourth group, everyone answered yes to only 1 question, a.
//! - In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
//!
//! In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.
//!
//! For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?

use std::collections::HashMap;

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> usize {
    INPUT_VALUES
        .split("\n\n")
        .map(count_unique_questions_for_anyone)
        .sum()
}

/// Part two answer.
pub fn run_ex2() -> usize {
    INPUT_VALUES
        .split("\n\n")
        .map(count_unique_questions_for_everyone)
        .sum()
}

/// Count unique questions where anyone answered 'yes' for group entries.
///
/// # Arguments
///
/// * `group_entries` - Group entries
pub fn count_unique_questions_for_anyone(group_entries: &str) -> usize {
    let mut entries: Vec<char> = group_entries
        .lines()
        .flat_map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    entries.sort_unstable();
    entries.dedup();
    entries.len()
}

/// Count unique questions where everyone answered 'yes' for group entries.
///
/// # Arguments
///
/// * `group_entries` - Group entries
pub fn count_unique_questions_for_everyone(group_entries: &str) -> usize {
    let lines: Vec<&str> = group_entries.lines().collect();
    let group_count = lines.len();

    let mut counter = HashMap::new();
    for l in lines {
        for c in l.chars() {
            let v = counter.entry(c).or_insert(0);
            *v += 1;
        }
    }

    counter.keys().filter(|k| counter[k] == group_count).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 6714;
    const EX2_OUTPUT: usize = 3435;

    #[test]
    pub fn test_run_ex1() {
        assert_eq!(run_ex1(), EX1_OUTPUT);
    }

    #[test]
    pub fn test_run_ex2() {
        assert_eq!(run_ex2(), EX2_OUTPUT);
    }
}
