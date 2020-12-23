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

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> String {
    let mut cups = parse_cups(INPUT_VALUES);
    run_steps(&mut cups, 100);
    cups.to_string_from_one()
}

/// Part two answer.
pub fn run_ex2() -> usize {
    let mut cups = prepare_million_cups(INPUT_VALUES);
    run_steps(&mut cups, 10_000_000);

    let a = cups.next(1);
    let b = cups.next(a);
    a * b
}

/// Cup.
pub type Cup = usize;

/// Cups.
#[derive(Debug)]
pub struct Cups {
    head: Cup,
    data: Vec<Cup>,
}

impl Cups {
    /// Get next cup from `cup`.
    ///
    /// # Arguments
    ///
    /// * `cup` - Cup
    pub fn next(&self, cup: Cup) -> Cup {
        self.data[cup]
    }

    /// Set next cup link.
    ///
    /// # Arguments
    ///
    /// * `cup` - Source cup
    /// * `next` - Target cup
    pub fn set_next(&mut self, cup: Cup, next: Cup) {
        self.data[cup] = next;
    }

    /// Get previous cup.
    ///
    /// # Arguments
    ///
    /// * `cup` - Target cup
    pub fn prev_cup(&self, cup: Cup) -> Cup {
        let dest = cup - 1;
        if dest == 0 {
            self.data.len() - 1
        } else {
            dest
        }
    }

    /// Join cups to string, starting from cup one (and ignoring it).
    pub fn to_string_from_one(&self) -> String {
        cups_to_string(&self, 1)[1..].to_string()
    }
}

impl ToString for Cups {
    fn to_string(&self) -> String {
        cups_to_string(&self, self.head)
    }
}

/// Join cups to string, starting with `head`.
///
/// # Arguments
///
/// * `cups` - Cups
/// * `head` - Starting head
pub fn cups_to_string(cups: &Cups, head: Cup) -> String {
    let mut output = String::new();
    let mut cursor = head;
    output.push_str(&cursor.to_string());

    for _ in 1..cups.data.len() - 1 {
        cursor = cups.next(cursor);
        if cursor == head {
            break;
        }

        output.push_str(&cursor.to_string());
    }

    output
}

/// Parse cups.
///
/// # Arguments
///
/// * `input` - Input string
fn parse_cups(input: &str) -> Cups {
    let mut output = Vec::new();
    let chars: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Cup)
        .collect();
    output.resize(chars.len() + 1, 0);

    // Link each other
    for w in chars.windows(2) {
        output[w[0]] = w[1];
    }

    // Link last with first
    output[chars[chars.len() - 1]] = chars[0];

    Cups {
        head: chars[0],
        data: output,
    }
}

/// Prepare million cups.
///
/// # Arguments
///
/// * `input` - Input string
#[allow(clippy::needless_range_loop)]
pub fn prepare_million_cups(input: &str) -> Cups {
    let mut output = Vec::new();
    let chars: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as Cup)
        .collect();
    output.resize(1_000_000 + 1, 0);

    for w in chars.windows(2) {
        output[w[0]] = w[1];
    }

    // Now add millions
    for i in 10..=1_000_000 {
        output[i] = i + 1
    }

    // Link last char with 10
    output[chars[chars.len() - 1]] = 10;

    // Link last with first
    let output_len = output.len();
    output[output_len - 1] = chars[0];

    Cups {
        head: chars[0],
        data: output,
    }
}

/// Execute a game step.
///
/// # Arguments
///
/// * `cups` - Cups
pub fn game_step(cups: &mut Cups) {
    // Get current cursor
    let head = cups.head;

    // Get next three
    let t0 = cups.next(head);
    let t1 = cups.next(t0);
    let t2 = cups.next(t1);
    let next = cups.next(t2);

    // Get destination cup
    let mut dest = cups.prev_cup(head);
    while dest == t0 || dest == t1 || dest == t2 {
        dest = cups.prev_cup(dest);
    }

    // Update links
    cups.set_next(head, next);
    cups.set_next(t2, cups.next(dest));
    cups.set_next(dest, t0);
    cups.head = next;
}

/// Run `n` steps of simulation.
///
/// # Arguments
///
/// * `cups` - Cups
/// * `n` - Steps
pub fn run_steps(cups: &mut Cups, n: usize) {
    for _ in 0..n {
        game_step(cups);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: &str = "27865934";
    const EX2_OUTPUT: usize = 170_836_011_000;

    const SAMPLE: &str = "389125467";

    #[test]
    fn test_parse_cups() {
        let cups = parse_cups(SAMPLE);
        assert_eq!(cups.to_string(), SAMPLE);
    }

    #[test]
    fn test_10_game_steps() {
        let mut cups = parse_cups(SAMPLE);

        run_steps(&mut cups, 10);
        assert_eq!(cups.to_string_from_one(), "92658374");
    }

    #[test]
    fn test_100_game_steps() {
        let mut cups = parse_cups(SAMPLE);

        run_steps(&mut cups, 100);
        assert_eq!(cups.to_string_from_one(), "67384529");
    }

    #[test]
    fn test_million() {
        let mut cups = prepare_million_cups(SAMPLE);
        run_steps(&mut cups, 10_000_000);

        assert_eq!(cups.next(1), 934_001);
        assert_eq!(cups.next(cups.next(1)), 159_792);
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
