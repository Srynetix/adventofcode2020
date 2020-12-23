//! # Day 14: Docking Data
//!
//! As your ferry approaches the sea port, the captain asks for your help again. The computer system that runs this port isn't compatible with the docking program on the ferry, so the docking parameters aren't being correctly initialized in the docking program's memory.
//!
//! After a brief inspection, you discover that the sea port's computer system uses a strange bitmask system in its initialization program. Although you don't have the correct decoder chip handy, you can emulate it in software!
//!
//! The initialization program (your puzzle input) can either update the bitmask or write a value to memory. Values and memory addresses are both 36-bit unsigned integers. For example, ignoring bitmasks for a moment, a line like mem\[8\] = 11 would write the value 11 to memory address 8.
//!
//! The bitmask is always given as a string of 36 bits, written with the most significant bit (representing 2^35) on the left and the least significant bit (2^0, that is, the 1s bit) on the right. The current bitmask is applied to values immediately before they are written to memory: a 0 or 1 overwrites the corresponding bit in the value, while an X leaves the bit in the value unchanged.
//!
//! For example, consider the following program:
//!
//! ```text
//! mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
//! mem[8] = 11
//! mem[7] = 101
//! mem[8] = 0
//! ```
//!
//! This program starts by specifying a bitmask (mask = ....). The mask it specifies will overwrite two bits in every written value: the 2s bit is overwritten with 0, and the 64s bit is overwritten with 1.
//!
//! The program then attempts to write the value 11 to memory address 8. By expanding everything out to individual bits, the mask is applied as follows:
//!
//! ```text
//! value:  000000000000000000000000000000001011  (decimal 11)
//! mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
//! result: 000000000000000000000000000001001001  (decimal 73)
//! ```
//!
//! So, because of the mask, the value 73 is written to memory address 8 instead. Then, the program tries to write 101 to address 7:
//!
//! ```text
//! value:  000000000000000000000000000001100101  (decimal 101)
//! mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
//! result: 000000000000000000000000000001100101  (decimal 101)
//! ```
//!
//! This time, the mask has no effect, as the bits it overwrote were already the values the mask tried to set. Finally, the program tries to write 0 to address 8:
//!
//! ```text
//! value:  000000000000000000000000000000000000  (decimal 0)
//! mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
//! result: 000000000000000000000000000001000000  (decimal 64)
//! ```
//!
//! 64 is written to address 8 instead, overwriting the value that was there previously.
//!
//! To initialize your ferry's docking program, you need the sum of all values left in memory after the initialization program completes. (The entire 36-bit address space begins initialized to the value 0 at every address.) In the above example, only two values in memory are not zero - 101 (at address 7) and 64 (at address 8) - producing a sum of 165.
//!
//! Execute the initialization program. What is the sum of all values left in memory after it completes?
//!
//! # Part Two
//!
//! For some reason, the sea port's computer system still can't communicate with your ferry's docking program. It must be using version 2 of the decoder chip!
//!
//! A version 2 decoder chip doesn't modify the values being written at all. Instead, it acts as a memory address decoder. Immediately before a value is written to memory, each bit in the bitmask modifies the corresponding bit of the destination memory address in the following way:
//!
//! - If the bitmask bit is 0, the corresponding memory address bit is unchanged.
//! - If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
//! - If the bitmask bit is X, the corresponding memory address bit is floating.
//!
//! A floating bit is not connected to anything and instead fluctuates unpredictably. In practice, this means the floating bits will take on all possible values, potentially causing many memory addresses to be written all at once!
//!
//! For example, consider the following program:
//!
//! ```text
//! mask = 000000000000000000000000000000X1001X
//! mem[42] = 100
//! mask = 00000000000000000000000000000000X0XX
//! mem[26] = 1
//! ```
//!
//! When this program goes to write to memory address 42, it first applies the bitmask:
//!
//! ```text
//! address: 000000000000000000000000000000101010  (decimal 42)
//! mask:    000000000000000000000000000000X1001X
//! result:  000000000000000000000000000000X1101X
//! ```
//!
//! After applying the mask, four bits are overwritten, three of which are different, and two of which are floating. Floating bits take on every possible combination of values; with two floating bits, four actual memory addresses are written:
//!
//! ```text
//! 000000000000000000000000000000011010  (decimal 26)
//! 000000000000000000000000000000011011  (decimal 27)
//! 000000000000000000000000000000111010  (decimal 58)
//! 000000000000000000000000000000111011  (decimal 59)
//! ```
//!
//! Next, the program is about to write to memory address 26 with a different bitmask:
//!
//! ```text
//! address: 000000000000000000000000000000011010  (decimal 26)
//! mask:    00000000000000000000000000000000X0XX
//! result:  00000000000000000000000000000001X0XX
//! ```
//!
//! This results in an address with three floating bits, causing writes to eight memory addresses:
//!
//! ```text
//! 000000000000000000000000000000010000  (decimal 16)
//! 000000000000000000000000000000010001  (decimal 17)
//! 000000000000000000000000000000010010  (decimal 18)
//! 000000000000000000000000000000010011  (decimal 19)
//! 000000000000000000000000000000011000  (decimal 24)
//! 000000000000000000000000000000011001  (decimal 25)
//! 000000000000000000000000000000011010  (decimal 26)
//! 000000000000000000000000000000011011  (decimal 27)
//! ```
//!
//! The entire 36-bit address space still begins initialized to the value 0 at every address, and you still need the sum of all values left in memory at the end of the program. In this example, the sum is 208.
//!
//! Execute the initialization program using an emulator for a version 2 decoder chip. What is the sum of all values left in memory after it completes?

use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

const INPUT_VALUES: &str = include_str!("input.txt");

static RGX_MASK: Lazy<Regex> = Lazy::new(|| Regex::new(r"mask = (?P<mask>[01X]{36})").unwrap());

static RGX_MEM: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"mem\[(?P<mem_idx>\d+)\] = (?P<mem_value>\d+)").unwrap());

/// Part one answer.
pub fn run_ex1() -> usize {
    let mut mem = BitmaskMemory::new();
    for l in INPUT_VALUES.lines() {
        mem.parse_line(l, false);
    }

    mem.get_memory_sum()
}

/// Part two answer.
pub fn run_ex2() -> usize {
    let mut mem = BitmaskMemory::new();
    for l in INPUT_VALUES.lines() {
        mem.parse_line(l, true);
    }

    mem.get_memory_sum()
}

/// Bitmask memory
#[derive(Debug, Default)]
pub struct BitmaskMemory {
    current_mask: Vec<Option<bool>>,
    memory: HashMap<usize, usize>,
}

impl BitmaskMemory {
    /// Creates a new bitmask memory.
    pub fn new() -> Self {
        Self {
            current_mask: vec![None; 36],
            memory: HashMap::new(),
        }
    }

    /// Set mask from str.
    ///
    /// # Arguments
    ///
    /// * `mask` - Mask
    pub fn set_mask_from_str(&mut self, mask: &str) {
        if mask.len() != 36 {
            panic!("Mask str should be 36 characters");
        }

        for (idx, c) in mask.chars().enumerate() {
            match c {
                'X' => self.current_mask[idx] = None,
                '0' => self.current_mask[idx] = Some(false),
                '1' => self.current_mask[idx] = Some(true),
                _ => panic!("Invalid mask character: {}", c),
            }
        }

        // Reverse mask
        self.current_mask.reverse();
    }

    /// Set value in memory using value mask.
    ///
    /// # Arguments
    ///
    /// * `addr` - Address
    /// * `value` - Value
    pub fn set_value_in_memory_using_value_mask(&mut self, addr: usize, value: usize) -> usize {
        let output = Self::apply_mask_on_value(&self.current_mask, value);
        self.memory.insert(addr, output);
        output
    }

    /// Set value in memory using value mask.
    ///
    /// # Arguments
    ///
    /// * `addr` - Address
    /// * `value` - Value
    pub fn set_value_in_memory_using_address_mask(&mut self, addr: usize, value: usize) {
        let addresses = Self::apply_mask_on_address(&self.current_mask, addr);
        for addr in addresses {
            self.memory.insert(addr, value);
        }
    }

    /// Get memory sum.
    pub fn get_memory_sum(&self) -> usize {
        self.memory.values().filter(|&&x| x != 0).sum()
    }

    /// Parse an input line.
    ///
    /// # Arguments
    ///
    /// * `input` - Input line
    /// * `use_address_mask` - Use address mask
    pub fn parse_line(&mut self, input: &str, use_address_mask: bool) {
        if let Some(captures) = RGX_MASK.captures(input.trim()) {
            let mask = captures.name("mask").unwrap().as_str();
            self.set_mask_from_str(mask);
        } else {
            let captures = RGX_MEM.captures(input.trim()).unwrap();
            let mem_idx = captures
                .name("mem_idx")
                .map(|x| x.as_str().parse::<usize>().unwrap())
                .unwrap();
            let mem_value = captures
                .name("mem_value")
                .map(|x| x.as_str().parse::<usize>().unwrap())
                .unwrap();
            if use_address_mask {
                self.set_value_in_memory_using_address_mask(mem_idx, mem_value);
            } else {
                self.set_value_in_memory_using_value_mask(mem_idx, mem_value);
            }
        }
    }

    fn apply_mask_on_value(mask: &[Option<bool>], value: usize) -> usize {
        let mut output = value;
        for (idx, x) in mask.iter().enumerate() {
            match x {
                Some(true) => output |= 1 << idx,
                Some(false) => output &= !(1 << idx),
                None => (),
            }
        }

        output
    }

    fn apply_mask_on_address(mask: &[Option<bool>], addr: usize) -> Vec<usize> {
        let mut addresses = vec![];
        let mut first_value = addr;

        for (idx, x) in mask.iter().enumerate() {
            match x {
                Some(true) => first_value |= 1 << idx,
                Some(false) => (),
                None => first_value &= !(1 << idx),
            }
        }
        addresses.push(first_value);

        let bits: Vec<usize> = mask
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| if x.is_none() { Some(idx) } else { None })
            .collect();
        for num in Self::generate_numbers_for_bits(&bits) {
            addresses.push(first_value + num);
        }

        addresses
    }

    #[allow(clippy::cast_possible_truncation)]
    fn generate_numbers_for_bits(bits: &[usize]) -> Vec<usize> {
        let mut output = vec![];

        let max_v = 2_u64.pow(bits.len() as u32);
        for i in 1..max_v {
            let mut number = 0;
            let bin = format!("{:b}", i);
            for (idx, n) in bin.chars().rev().enumerate() {
                if n == '1' {
                    number += 2_u64.pow(bits[idx] as u32);
                }
            }

            output.push(number as usize);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 8_471_403_462_063;
    const EX2_OUTPUT: usize = 2_667_858_637_669;

    const SAMPLE: &str = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0"#;

    const SAMPLE_2: &str = r#"mask = 000000000000000000000000000000X1001X
    mem[42] = 100
    mask = 00000000000000000000000000000000X0XX
    mem[26] = 1"#;

    #[test]
    #[allow(clippy::shadow_unrelated)]
    fn test_sample() {
        let mut memory = BitmaskMemory::new();

        let mut lines = SAMPLE.lines();
        let mask = lines.next().unwrap();

        memory.parse_line(mask, false);
        assert_eq!(memory.current_mask[0], None);
        assert_eq!(memory.current_mask[1], Some(false));
        assert_eq!(memory.current_mask[6], Some(true));

        let mem8 = lines.next().unwrap();
        memory.parse_line(mem8, false);
        assert_eq!(memory.memory[&8], 73);

        let mem7 = lines.next().unwrap();
        memory.parse_line(mem7, false);
        assert_eq!(memory.memory[&7], 101);

        let mem8 = lines.next().unwrap();
        memory.parse_line(mem8, false);
        assert_eq!(memory.memory[&8], 64);

        assert_eq!(memory.get_memory_sum(), 165);
    }

    #[test]
    #[allow(clippy::shadow_unrelated)]
    fn test_sample_2() {
        let mut memory = BitmaskMemory::new();

        let mut lines = SAMPLE_2.lines();

        // Parse first mask
        memory.parse_line(lines.next().unwrap(), true);
        memory.parse_line(lines.next().unwrap(), true);
        for x in &[26, 27, 58, 59] {
            assert_eq!(memory.memory[x], 100);
        }

        // Parse second mask
        memory.parse_line(lines.next().unwrap(), true);
        memory.parse_line(lines.next().unwrap(), true);
        for x in &[16, 17, 18, 19, 24, 25, 26, 27] {
            assert_eq!(memory.memory[x], 1);
        }

        assert_eq!(memory.get_memory_sum(), 208);
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
