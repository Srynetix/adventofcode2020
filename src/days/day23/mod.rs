//! # Day 23: Crab Cups
//!
//! The small crab challenges you to a game! The crab is going to mix up some cups, and you have to predict where they'll end up.
//!
//! The cups will be arranged in a circle and labeled clockwise (your puzzle input). For example, if your labeling were 32415, there would be five cups in the circle; going clockwise around the circle from the first cup, the cups would be labeled 3, 2, 4, 1, 5, and then back to 3 again.
//!
//! Before the crab starts, it will designate the first cup in your list as the current cup. The crab is then going to do 100 moves.
//!
//! Each move, the crab does the following actions:
//!
//! - The crab picks up the three cups that are immediately clockwise of the current cup. They are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
//! - The crab selects a destination cup: the cup with a label equal to the current cup's label minus one. If this would select one of the cups that was just picked up, the crab will keep subtracting one until it finds a cup that wasn't just picked up. If at any point in this process the value goes below the lowest value on any cup's label, it wraps around to the highest value on any cup's label instead.
//! - The crab places the cups it just picked up so that they are immediately clockwise of the destination cup. They keep the same order as when they were picked up.
//! - The crab selects a new current cup: the cup which is immediately clockwise of the current cup.
//!
//! For example, suppose your cup labeling were 389125467. If the crab were to do merely 10 moves, the following changes would occur:
//!
//! ```text
//! -- move 1 --
//! cups: (3) 8  9  1  2  5  4  6  7
//! pick up: 8, 9, 1
//! destination: 2
//!
//! -- move 2 --
//! cups:  3 (2) 8  9  1  5  4  6  7
//! pick up: 8, 9, 1
//! destination: 7
//!
//! -- move 3 --
//! cups:  3  2 (5) 4  6  7  8  9  1
//! pick up: 4, 6, 7
//! destination: 3
//!
//! -- move 4 --
//! cups:  7  2  5 (8) 9  1  3  4  6
//! pick up: 9, 1, 3
//! destination: 7
//!
//! -- move 5 --
//! cups:  3  2  5  8 (4) 6  7  9  1
//! pick up: 6, 7, 9
//! destination: 3
//!
//! -- move 6 --
//! cups:  9  2  5  8  4 (1) 3  6  7
//! pick up: 3, 6, 7
//! destination: 9
//!
//! -- move 7 --
//! cups:  7  2  5  8  4  1 (9) 3  6
//! pick up: 3, 6, 7
//! destination: 8
//!
//! -- move 8 --
//! cups:  8  3  6  7  4  1  9 (2) 5
//! pick up: 5, 8, 3
//! destination: 1
//!
//! -- move 9 --
//! cups:  7  4  1  5  8  3  9  2 (6)
//! pick up: 7, 4, 1
//! destination: 5
//!
//! -- move 10 --
//! cups: (5) 7  4  1  8  3  9  2  6
//! pick up: 7, 4, 1
//! destination: 3
//!
//! -- final --
//! cups:  5 (8) 3  7  4  1  9  2  6
//! ```
//!
//! In the above example, the cups' values are the labels as they appear moving clockwise around the circle; the current cup is marked with ( ).
//!
//! After the crab is done, what order will the cups be in? Starting after the cup labeled 1, collect the other cups' labels clockwise into a single string with no extra characters; each number except 1 should appear exactly once. In the above example, after 10 moves, the cups clockwise from 1 are labeled 9, 2, 6, 5, and so on, producing 92658374. If the crab were to complete all 100 moves, the order after cup 1 would be 67384529.
//!
//! Using your labeling, simulate 100 moves. What are the labels on the cups after cup 1?
//!
//! --- Part Two ---
//!
//! Due to what you can only assume is a mistranslation (you're not exactly fluent in Crab), you are quite surprised when the crab starts arranging many cups in a circle on your raft - one million (1000000) in total.
//!
//! Your labeling is still correct for the first few cups; after that, the remaining cups are just numbered in an increasing fashion starting from the number after the highest number in your list and proceeding one by one until one million is reached. (For example, if your labeling were 54321, the cups would be numbered 5, 4, 3, 2, 1, and then start counting up from 6 until one million is reached.) In this way, every number from one through one million is used exactly once.
//!
//! After discovering where you made the mistake in translating Crab Numbers, you realize the small crab isn't going to do merely 100 moves; the crab is going to do ten million (10000000) moves!
//!
//! The crab is going to hide your stars - one each - under the two cups that will end up immediately clockwise of cup 1. You can have them if you predict what the labels on those cups will be when the crab is finished.
//!
//! In the above example (389125467), this would be 934001 and then 159792; multiplying these together produces 149245887792.
//!
//! Determine which two cups will end up immediately clockwise of cup 1. What do you get if you multiply their labels together?

use std::collections::VecDeque;

/// Part one answer.
pub fn run_ex1() -> String {
    let mut cups = parse_cups(include_str!("input.txt"));
    run_steps(&mut cups, 100, 0);
    string_cupslice(&pick_cups_after_1(&cups))
}

/// Part two answer.
pub const fn run_ex2() -> usize {
    0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cup(usize);

#[derive(PartialEq, Eq)]
struct Cups(VecDeque<Cup>);

struct CupSlice(Vec<Cup>);

impl std::fmt::Debug for Cups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", string_cups(&self))
    }
}

impl std::fmt::Debug for CupSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", string_cupslice(&self))
    }
}

fn string_cupslice(cups: &CupSlice) -> String {
    cups.0.iter().fold(String::new(), |mut acc, c| {
        acc.push_str(&c.0.to_string());
        acc
    })
}

fn string_cups(cups: &Cups) -> String {
    cups.0.iter().fold(String::new(), |mut acc, c| {
        acc.push_str(&c.0.to_string());
        acc
    })
}

fn parse_cups(input: &str) -> Cups {
    Cups(
        input
            .trim()
            .chars()
            .map(|c| Cup(c.to_digit(10).unwrap() as usize))
            .collect(),
    )
}

fn prepare_million_cups(cups: &mut Cups) {
    cups.0.resize(1_000_000, Cup(0));

    for i in 10..1_000_000 {
        cups.0.push_back(Cup(i));
    }
}

fn find_destination_cup_position(cups: &Cups, max_cups_num: usize, source_cup_id: usize) -> usize {
    let prev_cup_id = {
        if source_cup_id == 1 {
            max_cups_num
        } else {
            source_cup_id - 1
        }
    };

    if let Some(position) = cups.0.iter().position(|&cup| cup.0 == prev_cup_id) {
        position
    } else {
        find_destination_cup_position(cups, max_cups_num, prev_cup_id)
    }
}

fn get_cup_position(cups: &Cups, cup_id: usize) -> usize {
    cups.0.iter().position(|&cup| cup.0 == cup_id).unwrap()
}

fn remove_three_cups(cups: &mut Cups, cursor: usize) -> CupSlice {
    let mut drained_cups = vec![];
    for _ in 0..=2 {
        let element = cups.0.remove(cursor).or_else(|| cups.0.pop_front());
        drained_cups.push(element.unwrap());
    }

    CupSlice(drained_cups)
}

fn push_three_cups_after(cups: &mut Cups, new_cups: CupSlice, position: usize) {
    let insert_position = position + 1;

    for cup in new_cups.0.into_iter().rev() {
        cups.0.insert(insert_position, cup);
    }
}

fn game_step(cups: &mut Cups, cups_len: usize, cursor: usize) -> usize {
    let current_cup = cups.0[cursor];
    let cursor_str = format!("[C: {}]", cursor + 1);
    // println!(
    //     "{} Taking cup {:?} from {:?}",
    //     cursor_str, current_cup, cups
    // );
    let next_three = remove_three_cups(cups, cursor + 1);
    // println!(
    //     "{} Picking three cups {:?}, remaining {:?}",
    //     cursor_str, next_three, cups
    // );
    let destination_cup_index = find_destination_cup_position(cups, cups_len + 2, current_cup.0);
    // println!(
    //     "{} Destination cup index {} ({:?})",
    //     cursor_str, destination_cup_index, cups.0[destination_cup_index]
    // );
    push_three_cups_after(cups, next_three, destination_cup_index);
    // println!(
    //     "{} Pushing cups at index {}, now {:?}",
    //     cursor_str, destination_cup_index, cups
    // );
    let new_cursor = (get_cup_position(cups, current_cup.0) + 1).rem_euclid(cups_len);
    // println!("{} New cursor {}", cursor_str, new_cursor);
    // println!();

    new_cursor
}

fn pick_cups_after_1(cups: &Cups) -> CupSlice {
    let cups_len = cups.0.len();
    let mut output = Vec::new();
    let cup_position = get_cup_position(cups, 1);
    let mut cursor = (cup_position + 1).rem_euclid(cups_len);

    while cursor != cup_position {
        output.push(cups.0[cursor]);
        cursor = (cursor + 1).rem_euclid(cups_len);
    }

    CupSlice(output)
}

fn run_steps(cups: &mut Cups, n: usize, cursor: usize) -> usize {
    let cups_len = cups.0.len();
    let mut current_cursor = cursor;
    let mut c = n / 100;
    if c == 0 {
        c = 1;
    }

    for s in 0..n {
        if s % c == 0 {
            println!("Step {} / {}", s + 1, n);
        }

        current_cursor = game_step(cups, cups_len, current_cursor);
    }

    current_cursor
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: &str = "27865934";

    const SAMPLE: &str = "389125467";

    #[test]
    fn test_parse_cups() {
        let cups = parse_cups(SAMPLE);
        assert_eq!(string_cups(&cups), SAMPLE);
    }

    #[test]
    fn test_one_game_step() {
        let mut cups = parse_cups(SAMPLE);
        let cups_len = cups.0.len();

        // Run one step
        let new_cursor = game_step(&mut cups, cups_len, 0);
        assert_eq!(new_cursor, 1);
        assert_eq!(string_cups(&cups), "328915467");
    }

    #[test]
    fn test_10_game_steps() {
        let mut cups = parse_cups(SAMPLE);

        run_steps(&mut cups, 10, 0);
        assert_eq!(string_cupslice(&pick_cups_after_1(&cups)), "92658374");
    }

    #[test]
    fn test_100_game_steps() {
        let mut cups = parse_cups(SAMPLE);

        run_steps(&mut cups, 100, 0);
        assert_eq!(string_cupslice(&pick_cups_after_1(&cups)), "67384529");
    }

    // #[test]
    // fn test_million() {
    //     let mut cups = parse_cups(SAMPLE);
    //     println!("Preparing 1_000_000 cups");
    //     prepare_million_cups(&mut cups);

    //     println!("Running 10_000_000 steps");
    //     run_steps(&mut cups, 10_000_000, 0);

    //     println!("Searching 1...");
    //     let one = get_cup_position(&cups, 1);
    //     let one_one = (one + 1).rem_euclid(1_000_000);
    //     let one_two = (one + 2).rem_euclid(1_000_000);

    //     assert_eq!(cups.0[one_one], Cup(934_001));
    //     assert_eq!(cups.0[one_two], Cup(159_792));
    // }

    #[test]
    fn test_run_ex1() {
        assert_eq!(run_ex1(), EX1_OUTPUT);
    }
}
