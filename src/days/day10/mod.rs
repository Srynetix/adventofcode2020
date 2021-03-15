//! # Day 10: Adapter Array
//!
//! Patched into the aircraft's data port, you discover weather forecasts of a massive tropical storm. Before you can figure out whether it will impact your vacation plans, however, your device suddenly turns off!
//!
//! Its battery is dead.
//!
//! You'll need to plug it in. There's only one problem: the charging outlet near your seat produces the wrong number of jolts. Always prepared, you make a list of all of the joltage adapters in your bag.
//!
//! Each of your joltage adapters is rated for a specific output joltage (your puzzle input). Any given adapter can take an input 1, 2, or 3 jolts lower than its rating and still produce its rated output joltage.
//!
//! In addition, your device has a built-in joltage adapter rated for 3 jolts higher than the highest-rated adapter in your bag. (If your adapter list were 3, 9, and 6, your device's built-in adapter would be rated for 12 jolts.)
//!
//! Treat the charging outlet near your seat as having an effective joltage rating of 0.
//!
//! Since you have some time to kill, you might as well test all of your adapters. Wouldn't want to get to your resort and realize you can't even charge your device!
//!
//! If you use every adapter in your bag at once, what is the distribution of joltage differences between the charging outlet, the adapters, and your device?
//!
//! For example, suppose that in your bag, you have adapters with the following joltage ratings:
//!
//! ```text
//! 16
//! 10
//! 15
//! 5
//! 1
//! 11
//! 7
//! 19
//! 6
//! 12
//! 4
//! ```
//!
//! With these adapters, your device's built-in joltage adapter would be rated for 19 + 3 = 22 jolts, 3 higher than the highest-rated adapter.
//!
//! Because adapters can only connect to a source 1-3 jolts lower than its rating, in order to use every adapter, you'd need to choose them like this:
//!
//! - The charging outlet has an effective rating of 0 jolts, so the only adapters that could connect to it directly would need to have a joltage rating of 1, 2, or 3 jolts. Of these, only one you have is an adapter rated 1 jolt (difference of 1).
//! - From your 1-jolt rated adapter, the only choice is your 4-jolt rated adapter (difference of 3).
//! - From the 4-jolt rated adapter, the adapters rated 5, 6, or 7 are valid choices. However, in order to not skip any adapters, you have to pick the adapter rated 5 jolts (difference of 1).
//! - Similarly, the next choices would need to be the adapter rated 6 and then the adapter rated 7 (with difference of 1 and 1).
//! - The only adapter that works with the 7-jolt rated adapter is the one rated 10 jolts (difference of 3).
//! - From 10, the choices are 11 or 12; choose 11 (difference of 1) and then 12 (difference of 1).
//! - After 12, only valid adapter has a rating of 15 (difference of 3), then 16 (difference of 1), then 19 (difference of 3).
//! - Finally, your device's built-in adapter is always 3 higher than the highest adapter, so its rating is 22 jolts (always a difference of 3).
//!
//! In this example, when using every adapter, there are 7 differences of 1 jolt and 5 differences of 3 jolts.
//!
//! Here is a larger example:
//!
//! ```text
//! 28
//! 33
//! 18
//! 42
//! 31
//! 14
//! 46
//! 20
//! 48
//! 47
//! 24
//! 23
//! 49
//! 45
//! 19
//! 38
//! 39
//! 11
//! 1
//! 32
//! 25
//! 35
//! 8
//! 17
//! 7
//! 9
//! 4
//! 2
//! 34
//! 10
//! 3
//! ```
//!
//! In this larger example, in a chain that uses all of the adapters, there are 22 differences of 1 jolt and 10 differences of 3 jolts.
//!
//! Find a chain that uses all of your adapters to connect the charging outlet to your device's built-in adapter and count the joltage differences between the charging outlet, the adapters, and your device. What is the number of 1-jolt differences multiplied by the number of 3-jolt differences?
//!
//! # Part Two
//!
//! To completely determine whether you have enough adapters, you'll need to figure out how many different ways they can be arranged. Every arrangement needs to connect the charging outlet to your device. The previous rules about when adapters can successfully connect still apply.
//!
//! The first example above (the one that starts with 16, 10, 15) supports the following arrangements:
//!
//! ```text
//! (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
//! (0), 1, 4, 5, 6, 7, 10, 12, 15, 16, 19, (22)
//! (0), 1, 4, 5, 7, 10, 11, 12, 15, 16, 19, (22)
//! (0), 1, 4, 5, 7, 10, 12, 15, 16, 19, (22)
//! (0), 1, 4, 6, 7, 10, 11, 12, 15, 16, 19, (22)
//! (0), 1, 4, 6, 7, 10, 12, 15, 16, 19, (22)
//! (0), 1, 4, 7, 10, 11, 12, 15, 16, 19, (22)
//! (0), 1, 4, 7, 10, 12, 15, 16, 19, (22)
//! ```
//!
//! (The charging outlet and your device's built-in adapter are shown in parentheses.) Given the adapters from the first example, the total number of arrangements that connect the charging outlet to your device is 8.
//!
//! The second example above (the one that starts with 28, 33, 18) has many arrangements. Here are a few:
//!
//! ```text
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49, (52)
//!
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 49, (52)
//!
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 46, 48, 49, (52)
//!
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 46, 49, (52)
//!
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 47, 48, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 46, 48, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 46, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 47, 48, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 47, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 48, 49, (52)
//! ```
//!
//! In total, this set of adapters can connect the charging outlet to your device in 19208 distinct arrangements.
//!
//! You glance back down at your bag and try to remember why you brought so many adapters; there must be more than a trillion valid ways to arrange them! Surely, there must be an efficient way to count the arrangements.
//!
//! What is the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device?

use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> usize {
    let (diff1, diff3) = JoltAnalyzer::get_1x3_jolt_differences(
        &JoltAnalyzer::from_input(INPUT_VALUES).determine_jolt_chain(),
    );

    diff1 * diff3
}

/// Part two answer.
pub fn run_ex2() -> usize {
    JoltAnalyzer::from_input(INPUT_VALUES).count_adapter_permutations()
}

/// Jolt analyzer
pub struct JoltAnalyzer {
    data: Vec<usize>,
}

impl JoltAnalyzer {
    /// Creates analyzer from input string.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn from_input(input: &str) -> Self {
        let mut data: Vec<usize> = input
            .lines()
            .filter_map(|x| x.trim().parse().ok())
            .sorted()
            .collect();

        // Add charging outlet: 0
        data.insert(0, 0);
        // Add device builtin adapter
        data.push(Self::get_builtin_adapter_jolts(&data));

        Self { data }
    }

    /// Get device builtin adapter jolts.
    pub fn get_builtin_adapter_jolts(data: &[usize]) -> usize {
        data.iter().max().unwrap() + 3
    }

    /// Determine jolt chain.
    pub fn determine_jolt_chain(&self) -> Vec<(usize, usize)> {
        let mut remaining: VecDeque<usize> = self.data.clone().into();
        let mut available = self.data.clone();
        let mut chain = vec![];

        while let Some(num) = remaining.pop_front() {
            if let Some((diff, adapter, idx)) =
                Self::find_smallest_compatible_adapter(&available, num)
            {
                chain.push((diff, adapter));
                available.remove(idx);
            }
        }

        chain
    }

    /// Count adapter permutations.
    pub fn count_adapter_permutations(&self) -> usize {
        let max_value = self.data.last().unwrap();
        let data_set: HashSet<usize> = self.data.iter().copied().collect();
        let mut memory: HashMap<usize, usize> = HashMap::new();
        Self::count_inner_adapter_permutations(&data_set, 0, *max_value, &mut memory)
    }

    /// Recursively count inner adapter permutations.
    ///
    /// # Arguments
    ///
    /// * `data` - Adapter data set
    /// * `number` - Base number to search
    /// * `max_value` - Maximum value to found
    /// * `memory` - Memory to write permutations for better performance
    pub fn count_inner_adapter_permutations(
        data: &HashSet<usize>,
        number: usize,
        max_value: usize,
        memory: &mut HashMap<usize, usize>,
    ) -> usize {
        memory.get(&number).cloned().map_or_else(
            || {
                let memory_value = {
                    if number == max_value {
                        1
                    } else {
                        (number + 1..=number + 3)
                            .filter_map(|i| {
                                data.get(&i).map(|i| {
                                    Self::count_inner_adapter_permutations(
                                        data, *i, max_value, memory,
                                    )
                                })
                            })
                            .sum()
                    }
                };

                memory.insert(number, memory_value);
                memory_value
            },
            |v| v,
        )
    }

    /// Get 1-jolt differences and 3-jolt differences from chain.
    ///
    /// # Arguments
    ///
    /// * `chain` - Adapter chain
    pub fn get_1x3_jolt_differences(chain: &[(usize, usize)]) -> (usize, usize) {
        let mut diff1 = 0;
        let mut diff3 = 0;

        for c in chain.iter().map(|c| c.0) {
            match c {
                1 => diff1 += 1,
                3 => diff3 += 1,
                _ => (),
            }
        }

        (diff1, diff3)
    }

    /// Find smallest compatible adapter.
    ///
    /// # Arguments
    ///
    /// * `remaining` - Remaining adapters
    /// * `target` - Jolt target
    pub fn find_smallest_compatible_adapter(
        remaining: &[usize],
        target: usize,
    ) -> Option<(usize, usize, usize)> {
        let mut compatible: Vec<(usize, usize, usize)> = remaining
            .iter()
            .enumerate()
            .filter_map(|(idx, x)| {
                let diff = x.wrapping_sub(target);
                if diff > 0 && diff <= 3 {
                    Some((diff, *x, idx))
                } else {
                    None
                }
            })
            .collect();

        compatible.sort_unstable();
        compatible.first().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 1820;
    const EX2_OUTPUT: usize = 3_454_189_699_072;

    const SAMPLE1: &str = r#"16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4"#;

    const SAMPLE2: &str = r#"28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3"#;

    #[test]
    fn test_builtin_adapter_jolts() {
        let data: Vec<usize> = SAMPLE1
            .lines()
            .filter_map(|n| n.trim().parse().ok())
            .collect();
        assert_eq!(JoltAnalyzer::get_builtin_adapter_jolts(&data), 22)
    }

    #[test]
    fn test_determine_jolt_chain() {
        assert_eq!(
            JoltAnalyzer::from_input(SAMPLE1).determine_jolt_chain(),
            vec![
                (1, 1),
                (3, 4),
                (1, 5),
                (1, 6),
                (1, 7),
                (3, 10),
                (1, 11),
                (1, 12),
                (3, 15),
                (1, 16),
                (3, 19),
                (3, 22)
            ]
        )
    }

    #[test]
    fn test_get_1x3_jolt_differences() {
        let chain = JoltAnalyzer::from_input(SAMPLE1).determine_jolt_chain();
        assert_eq!(JoltAnalyzer::get_1x3_jolt_differences(&chain), (7, 5));
    }

    #[test]
    fn test_get_1x3_jolt_differences_larger() {
        let chain = JoltAnalyzer::from_input(SAMPLE2).determine_jolt_chain();
        assert_eq!(JoltAnalyzer::get_1x3_jolt_differences(&chain), (22, 10));
    }

    #[test]
    fn test_count_adapter_permutations() {
        assert_eq!(
            JoltAnalyzer::from_input(SAMPLE1).count_adapter_permutations(),
            8
        );
    }

    #[test]
    fn test_count_adapter_permutations_larger() {
        assert_eq!(
            JoltAnalyzer::from_input(SAMPLE2).count_adapter_permutations(),
            19208
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
