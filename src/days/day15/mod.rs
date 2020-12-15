//! # Day 15: Rambunctious Recitation
//!
//! You catch the airport shuttle and try to book a new flight to your vacation island. Due to the storm, all direct flights have been cancelled, but a route is available to get around the storm. You take it.
//!
//! While you wait for your flight, you decide to check in with the Elves back at the North Pole. They're playing a memory game and are ever so excited to explain the rules!
//!
//! In this game, the players take turns saying numbers. They begin by taking turns reading from a list of starting numbers (your puzzle input). Then, each turn consists of considering the most recently spoken number:
//!
//! - If that was the first time the number has been spoken, the current player says 0.
//! - Otherwise, the number had been spoken before; the current player announces how many turns apart the number is from when it was previously spoken.
//!
//! So, after the starting numbers, each turn results in that player speaking aloud either 0 (if the last number is new) or an age (if the last number is a repeat).
//!
//! For example, suppose the starting numbers are 0,3,6:
//!
//! - Turn 1: The 1st number spoken is a starting number, 0.
//! - Turn 2: The 2nd number spoken is a starting number, 3.
//! - Turn 3: The 3rd number spoken is a starting number, 6.
//! - Turn 4: Now, consider the last number spoken, 6. Since that was the first time the number had been spoken, the 4th number spoken is 0.
//! - Turn 5: Next, again consider the last number spoken, 0. Since it had been spoken before, the next number to speak is the difference between the turn number when it was last spoken (the previous turn, 4) and the turn number of the time it was most recently spoken before then (turn 1). Thus, the 5th number spoken is 4 - 1, 3.
//! - Turn 6: The last number spoken, 3 had also been spoken before, most recently on turns 5 and 2. So, the 6th number spoken is 5 - 2, 3.
//! - Turn 7: Since 3 was just spoken twice in a row, and the last two turns are 1 turn apart, the 7th number spoken is 1.
//! - Turn 8: Since 1 is new, the 8th number spoken is 0.
//! - Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is the difference between them, 4.
//! - Turn 10: 4 is new, so the 10th number spoken is 0.
//!
//! (The game ends when the Elves get sick of playing or dinner is ready, whichever comes first.)
//!
//! Their question for you is: what will be the 2020th number spoken? In the example above, the 2020th number spoken will be 436.
//!
//! Here are a few more examples:
//!
//! - Given the starting numbers 1,3,2, the 2020th number spoken is 1.
//! - Given the starting numbers 2,1,3, the 2020th number spoken is 10.
//! - Given the starting numbers 1,2,3, the 2020th number spoken is 27.
//! - Given the starting numbers 2,3,1, the 2020th number spoken is 78.
//! - Given the starting numbers 3,2,1, the 2020th number spoken is 438.
//! - Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
//!
//! Given your starting numbers, what will be the 2020th number spoken?
//!
//! # Part Two
//!
//! Impressed, the Elves issue you a challenge: determine the 30000000th number spoken. For example, given the same starting numbers as above:
//!
//! - Given 0,3,6, the 30000000th number spoken is 175594.
//! - Given 1,3,2, the 30000000th number spoken is 2578.
//! - Given 2,1,3, the 30000000th number spoken is 3544142.
//! - Given 1,2,3, the 30000000th number spoken is 261214.
//! - Given 2,3,1, the 30000000th number spoken is 6895259.
//! - Given 3,2,1, the 30000000th number spoken is 18.
//! - Given 3,1,2, the 30000000th number spoken is 362.
//!
//! Given your starting numbers, what will be the 30000000th number spoken?

use std::collections::VecDeque;

use eyre::{eyre, Result};

const INPUT_VALUES: &str = include_str!("input.txt");
const INITIAL_BUFFER_SIZE: usize = 1_048_576; // 1 MB

/// Part one answer.
pub fn run_ex1() -> usize {
    MemoryGame::from_str_input(INPUT_VALUES)
        .expect("Bad input values")
        .run_steps(2020)
        .expect("Should work")
}

/// Part two answer.
pub fn run_ex2() -> usize {
    MemoryGame::from_str_input(INPUT_VALUES)
        .expect("Bad input values")
        .run_steps(30_000_000)
        .expect("Should work")
}

/// Memory game
pub struct MemoryGame {
    memory: Vec<usize>,
    input: VecDeque<usize>,
    turn: usize,
}

impl MemoryGame {
    /// Creates new game from string.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    ///
    /// # Errors
    ///
    /// * Parse error
    pub fn from_str_input(input: &str) -> Result<Self> {
        let input: Result<_> = input
            .split(',')
            .map(|x| x.parse::<usize>().map_err(Into::into))
            .collect();

        Ok(Self::from_vec(input?))
    }

    /// Creates new game from vec.
    ///
    /// # Arguments
    ///
    /// * `input` - Input vec
    pub fn from_vec(input: Vec<usize>) -> Self {
        Self {
            memory: vec![0; INITIAL_BUFFER_SIZE],
            input: input.into(),
            turn: 1,
        }
    }

    /// Step.
    ///
    /// # Errors
    ///
    /// * If input is empty
    pub fn step(&mut self) -> Result<usize> {
        let i = self.input.pop_front().unwrap_or(0);

        if i >= self.memory.len() {
            self.memory.resize(i * 2, 0);
        }

        let t = self.memory[i];
        if t > 0 {
            self.input.push_back(self.turn - t);
        }

        self.memory[i] = self.turn;
        self.turn += 1;
        Ok(i)
    }

    /// Run for `n` steps.
    ///
    /// # Errors
    ///
    /// * If input is empty
    /// * If n is 0
    pub fn run_steps(&mut self, n: usize) -> Result<usize> {
        let mut result = 0;

        if n == 0 {
            return Err(eyre!("You need at least one step."));
        }

        for _ in 0..n {
            result = self.step()?;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 206;
    const EX2_OUTPUT: usize = 955;

    #[test]
    fn test_first_sample_steps() {
        let mut game = MemoryGame::from_vec(vec![0, 3, 6]);
        assert_eq!(game.step().unwrap(), 0);
        assert_eq!(game.step().unwrap(), 3);
        assert_eq!(game.step().unwrap(), 6);
        assert_eq!(game.step().unwrap(), 0);
        assert_eq!(game.step().unwrap(), 3);
        assert_eq!(game.step().unwrap(), 3);
        assert_eq!(game.step().unwrap(), 1);
        assert_eq!(game.step().unwrap(), 0);
        assert_eq!(game.step().unwrap(), 4);
        assert_eq!(game.step().unwrap(), 0);
    }

    #[test]
    fn test_first_sample_run() {
        let mut game = MemoryGame::from_vec(vec![0, 3, 6]);
        assert_eq!(game.run_steps(2020).unwrap(), 436);
    }

    #[test]
    fn test_more_samples() {
        assert_eq!(
            MemoryGame::from_vec(vec![1, 3, 2]).run_steps(2020).unwrap(),
            1
        );
        assert_eq!(
            MemoryGame::from_vec(vec![2, 1, 3]).run_steps(2020).unwrap(),
            10
        );
        assert_eq!(
            MemoryGame::from_vec(vec![1, 2, 3]).run_steps(2020).unwrap(),
            27
        );
        assert_eq!(
            MemoryGame::from_vec(vec![2, 3, 1]).run_steps(2020).unwrap(),
            78
        );
        assert_eq!(
            MemoryGame::from_vec(vec![3, 2, 1]).run_steps(2020).unwrap(),
            438
        );
        assert_eq!(
            MemoryGame::from_vec(vec![3, 1, 2]).run_steps(2020).unwrap(),
            1836
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
