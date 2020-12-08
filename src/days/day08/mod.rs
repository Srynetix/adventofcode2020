//! # Day 8: Handheld Halting
//!
//! Your flight to the major airline hub reaches cruising altitude without incident. While you consider checking the in-flight menu for one of those drinks that come with a little umbrella, you are interrupted by the kid sitting next to you.
//!
//! Their handheld game console won't turn on! They ask if you can take a look.
//!
//! You narrow the problem down to a strange infinite loop in the boot code (your puzzle input) of the device. You should be able to fix it, but first you need to be able to run the code in isolation.
//!
//! The boot code is represented as a text file with one instruction per line of text. Each instruction consists of an operation (acc, jmp, or nop) and an argument (a signed number like +4 or -20).
//!
//! - acc increases or decreases a single global value called the accumulator by the value given in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0. After an acc instruction, the instruction immediately below it is executed next.
//! - jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next instruction, jmp +1 would continue to the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.
//! - nop stands for No OPeration - it does nothing. The instruction immediately below it is executed next.
//!
//! For example, consider the following program:
//!
//! ```text
//! nop +0
//! acc +1
//! jmp +4
//! acc +3
//! jmp -3
//! acc -99
//! acc +1
//! jmp -4
//! acc +6
//! ```
//!
//! These instructions are visited in this order:
//!
//! ```text
//! nop +0  | 1
//! acc +1  | 2, 8(!)
//! jmp +4  | 3
//! acc +3  | 6
//! jmp -3  | 7
//! acc -99 |
//! acc +1  | 4
//! jmp -4  | 5
//! acc +6  |
//! ```
//!
//! First, the nop +0 does nothing. Then, the accumulator is increased from 0 to 1 (acc +1) and jmp +4 sets the next instruction to the other acc +1 near the bottom. After it increases the accumulator from 1 to 2, jmp -4 executes, setting the next instruction to the only acc +3. It sets the accumulator to 5, and jmp -3 causes the program to continue back at the first acc +1.
//!
//! This is an infinite loop: with this sequence of jumps, the program will run forever. The moment the program tries to run any instruction a second time, you know it will never terminate.
//!
//! Immediately before the program would run an instruction a second time, the value in the accumulator is 5.
//!
//! Run your copy of the boot code. Immediately before any instruction is executed a second time, what value is in the accumulator?
//!
//! # Part Two
//!
//! After some careful analysis, you believe that exactly one instruction is corrupted.
//!
//! Somewhere in the program, either a jmp is supposed to be a nop, or a nop is supposed to be a jmp. (No acc instructions were harmed in the corruption of this boot code.)
//!
//! The program is supposed to terminate by attempting to execute an instruction immediately after the last instruction in the file. By changing exactly one jmp or nop, you can repair the boot code and make it terminate correctly.
//!
//! For example, consider the same program from above:
//!
//! ```text
//! nop +0
//! acc +1
//! jmp +4
//! acc +3
//! jmp -3
//! acc -99
//! acc +1
//! jmp -4
//! acc +6
//! ```
//!
//! If you change the first instruction from nop +0 to jmp +0, it would create a single-instruction infinite loop, never leaving that instruction. If you change almost any of the jmp instructions, the program will still eventually find another jmp instruction and loop forever.
//!
//! However, if you change the second-to-last instruction (from jmp -4 to nop -4), the program terminates! The instructions are visited in this order:
//!
//! ```text
//! nop +0  | 1
//! acc +1  | 2
//! jmp +4  | 3
//! acc +3  |
//! jmp -3  |
//! acc -99 |
//! acc +1  | 4
//! nop -4  | 5
//! acc +6  | 6
//! ```
//!
//! After the last instruction (acc +6), the program terminates by attempting to run the instruction below the last instruction in the file. With this change, after the program terminates, the accumulator contains the value 8 (acc +1, acc +1, acc +6).
//!
//! Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp). What is the value of the accumulator after the program terminates?

#![allow(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::doc_markdown
)]

use std::collections::HashSet;

use eyre::{eyre, Result};
use serde::Deserialize;

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> isize {
    if let StepOutput::LoopFound(e) = Interpreter::new_from_code(INPUT_VALUES)
        .expect("Bad code")
        .run()
    {
        e
    } else {
        panic!("Code should loop");
    }
}

/// Part two answer.
pub fn run_ex2() -> isize {
    if let StepOutput::Finished(e) = Interpreter::new_from_code(INPUT_VALUES)
        .expect("Bad code")
        .run_repair_mode()
    {
        e
    } else {
        panic!("Code should finish");
    }
}

/// Operation code
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OpCode {
    /// No operation
    Nop,
    /// Increases or decreases the accumulator
    Acc,
    /// Jump to relative instruction number
    Jmp,
}

/// Instruction
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
pub struct Instruction {
    opcode: OpCode,
    value: isize,
}

impl Instruction {
    /// Creates a new instruction.
    ///
    /// # Arguments
    ///
    /// * `opcode` - Instruction `OpCode`
    /// * `value` - Instruction value
    pub const fn new(opcode: OpCode, value: isize) -> Self {
        Self { opcode, value }
    }

    /// Convert instruction to fixed instruction by swapping `Nop` and `Jmp` `OpCode`s.
    pub fn to_fixed_instruction(&self) -> Self {
        match self.opcode {
            OpCode::Acc => self.clone(),
            OpCode::Nop => Self::new(OpCode::Jmp, self.value),
            OpCode::Jmp => Self::new(OpCode::Nop, self.value),
        }
    }
}

/// Instruction parser
pub struct Parser;

impl Parser {
    /// Parse instruction from input string.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    ///
    /// # Errors
    ///
    /// * Parse error
    pub fn parse_instruction(input: &str) -> Result<Instruction> {
        let mut tokens = input.split_whitespace();
        let opcode: OpCode = tokens
            .next()
            .ok_or_else(|| eyre!("Missing 'opcode' from instruction: {}", input))
            .and_then(|res| serde_plain::from_str(res).map_err(Into::into))?;
        let value: isize = tokens
            .next()
            .ok_or_else(|| eyre!("Missing 'value' from instruction: {}", input))
            .and_then(|res| serde_plain::from_str(res).map_err(Into::into))?;

        Ok(Instruction::new(opcode, value))
    }

    /// Parse code.
    ///
    /// # Arguments
    ///
    /// * `code` - Source code
    ///
    /// # Errors
    ///
    /// * Parse error
    pub fn parse_code(code: &str) -> Result<Vec<Instruction>> {
        code.lines().map(Self::parse_instruction).collect()
    }
}

/// Step output
#[derive(Debug, Eq, PartialEq)]
pub enum StepOutput {
    /// Normal execution
    Normal,
    /// Loop found, with current accumulator value
    LoopFound(isize),
    /// Finished, with current accumulator value
    Finished(isize),
    /// Error
    Error,
}

/// Interpreter
pub struct Interpreter {
    instructions: Vec<Instruction>,
    accumulator: isize,
    cursor: usize,
    seen_instructions: HashSet<usize>,
}

impl Interpreter {
    /// Creates interpreter from code.
    ///
    /// # Arguments
    ///
    /// * `code` - Source code
    ///
    /// # Errors
    ///
    /// * Parse error
    pub fn new_from_code(code: &str) -> Result<Self> {
        Ok(Self {
            instructions: Parser::parse_code(code)?,
            accumulator: 0,
            cursor: 0,
            seen_instructions: HashSet::new(),
        })
    }

    /// Reset state, conserving current instructions.
    pub fn reset_state(&mut self) {
        self.accumulator = 0;
        self.cursor = 0;
        self.seen_instructions = HashSet::new();
    }

    /// Step on next instruction.
    pub fn step(&mut self) -> StepOutput {
        if self.cursor >= self.instructions.len() {
            // End !
            return StepOutput::Finished(self.accumulator);
        }

        let instr = &self.instructions[self.cursor];
        let mut next_cursor = self.cursor + 1;
        match instr.opcode {
            OpCode::Acc => self.accumulator += instr.value,
            OpCode::Nop => (),
            OpCode::Jmp => next_cursor = (self.cursor as isize + instr.value) as usize,
        }

        // Update cursor and seen instructions
        self.seen_instructions.insert(self.cursor);
        self.cursor = next_cursor;

        if self.seen_instructions.contains(&self.cursor) {
            StepOutput::LoopFound(self.accumulator)
        } else {
            StepOutput::Normal
        }
    }

    /// Run interpreter.
    /// Breaks on Finished or LoopFound.
    pub fn run(&mut self) -> StepOutput {
        loop {
            match self.step() {
                StepOutput::Normal => (),
                step => {
                    return step;
                }
            }
        }
    }

    /// Run on repair mode.
    /// Breaks on Finished, or return Error.
    pub fn run_repair_mode(&mut self) -> StepOutput {
        let original_instructions = self.instructions.clone();
        let reparation_choices: Vec<usize> = original_instructions
            .iter()
            .enumerate()
            .filter_map(|(idx, ins)| {
                if ins.opcode == OpCode::Acc {
                    None
                } else {
                    Some(idx)
                }
            })
            .collect();
        let reparation_limit = reparation_choices.len();
        let mut reparation_cursor = 0;

        while reparation_cursor < reparation_limit {
            let reparation = reparation_choices[reparation_cursor];
            let mut new_instructions = original_instructions.clone();
            let instr_to_repair = &new_instructions[reparation];
            new_instructions[reparation] = instr_to_repair.to_fixed_instruction();

            // Set instructions as current and reset state
            self.instructions = new_instructions;
            self.reset_state();

            if let StepOutput::Finished(i) = self.run() {
                return StepOutput::Finished(i);
            }

            reparation_cursor += 1;
        }

        StepOutput::Error
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: isize = 1930;
    const EX2_OUTPUT: isize = 1688;

    const CODE_SAMPLE: &str = r#"nop +0
    acc +1
    jmp +4
    acc +3
    jmp -3
    acc -99
    acc +1
    jmp -4
    acc +6"#;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            Parser::parse_instruction("jmp +4").unwrap(),
            Instruction::new(OpCode::Jmp, 4)
        );
        assert_eq!(
            Parser::parse_instruction("jmp -4").unwrap(),
            Instruction::new(OpCode::Jmp, -4)
        );
        assert_eq!(
            Parser::parse_instruction("nop +0").unwrap(),
            Instruction::new(OpCode::Nop, 0)
        );
        assert!(Parser::parse_instruction("toto").is_err());
    }

    #[test]
    fn test_parse_code() {
        assert_eq!(
            Parser::parse_code("jmp +4\nnop +0").unwrap(),
            vec![
                Instruction::new(OpCode::Jmp, 4),
                Instruction::new(OpCode::Nop, 0)
            ]
        );
    }

    #[test]
    fn test_interpreter_run() {
        assert_eq!(
            Interpreter::new_from_code(CODE_SAMPLE).unwrap().run(),
            StepOutput::LoopFound(5)
        );
    }

    #[test]
    fn test_interpreter_run_repair_mode() {
        assert_eq!(
            Interpreter::new_from_code(CODE_SAMPLE)
                .unwrap()
                .run_repair_mode(),
            StepOutput::Finished(8)
        );
    }

    #[test]
    fn test_run_ex1() {
        assert_eq!(
            Interpreter::new_from_code(INPUT_VALUES).unwrap().run(),
            StepOutput::LoopFound(EX1_OUTPUT)
        );
    }

    #[test]
    fn test_run_ex2() {
        assert_eq!(
            Interpreter::new_from_code(INPUT_VALUES)
                .unwrap()
                .run_repair_mode(),
            StepOutput::Finished(EX2_OUTPUT)
        );
    }
}
