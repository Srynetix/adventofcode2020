//! # Day 16: Ticket Translation
//!
//! As you're walking to yet another connecting flight, you realize that one of the legs of your re-routed trip coming up is on a high-speed train. However, the train ticket you were given is in a language you don't understand. You should probably figure out what it says before you get to the train station after the next flight.
//!
//! Unfortunately, you can't actually read the words on the ticket. You can, however, read the numbers, and so you figure out the fields these tickets must have and the valid ranges for values in those fields.
//!
//! You collect the rules for ticket fields, the numbers on your ticket, and the numbers on other nearby tickets for the same train service (via the airport security cameras) together into a single document you can reference (your puzzle input).
//!
//! The rules for ticket fields specify a list of fields that exist somewhere on the ticket and the valid ranges of values for each field. For example, a rule like class: 1-3 or 5-7 means that one of the fields in every ticket is named class and can be any value in the ranges 1-3 or 5-7 (inclusive, such that 3 and 5 are both valid in this field, but 4 is not).
//!
//! Each ticket is represented by a single line of comma-separated values. The values are the numbers on the ticket in the order they appear; every ticket has the same format. For example, consider this ticket:
//!
//! ```text
//! .--------------------------------------------------------.
//! | ????: 101    ?????: 102   ??????????: 103     ???: 104 |
//! |                                                        |
//! | ??: 301  ??: 302             ???????: 303      ??????? |
//! | ??: 401  ??: 402           ???? ????: 403    ????????? |
//! '--------------------------------------------------------'
//! ```
//!
//! Here, ? represents text in a language you don't understand. This ticket might be represented as 101,102,103,104,301,302,303,401,402,403; of course, the actual train tickets you're looking at are much more complicated. In any case, you've extracted just the numbers in such a way that the first number is always the same specific field, the second number is always a different specific field, and so on - you just don't know what each position actually means!
//!
//! Start by determining which tickets are completely invalid; these are tickets that contain values which aren't valid for any field. Ignore your ticket for now.
//!
//! For example, suppose you have the following notes:
//!
//! ```text
//! class: 1-3 or 5-7
//! row: 6-11 or 33-44
//! seat: 13-40 or 45-50
//!
//! your ticket:
//! 7,1,14
//!
//! nearby tickets:
//! 7,3,47
//! 40,4,50
//! 55,2,20
//! 38,6,12
//! ```
//!
//! It doesn't matter which position corresponds to which field; you can identify invalid nearby tickets by considering only whether tickets contain values that are not valid for any field. In this example, the values on the first nearby ticket are all valid for at least one field. This is not true of the other three nearby tickets: the values 4, 55, and 12 are are not valid for any field. Adding together all of the invalid values produces your ticket scanning error rate: 4 + 55 + 12 = 71.
//!
//! Consider the validity of the nearby tickets you scanned. What is your ticket scanning error rate?
//!
//! # Part Two
//!
//! Now that you've identified which tickets contain invalid values, discard those tickets entirely. Use the remaining valid tickets to determine which field is which.
//!
//! Using the valid ranges for each field, determine what order the fields appear on the tickets. The order is consistent between all tickets: if seat is the third field, it is the third field on every ticket, including your ticket.
//!
//! For example, suppose you have the following notes:
//!
//! ```text
//! class: 0-1 or 4-19
//! row: 0-5 or 8-19
//! seat: 0-13 or 16-19
//!
//! your ticket:
//! 11,12,13
//!
//! nearby tickets:
//! 3,9,18
//! 15,1,5
//! 5,14,9
//! ```
//!
//! Based on the nearby tickets in the above example, the first position must be row, the second position must be class, and the third position must be seat; you can conclude that in your ticket, class is 12, row is 11, and seat is 13.
//!
//! Once you work out which field is which, look for the six fields on your ticket that start with the word departure. What do you get if you multiply those six values together?

use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use thiserror::Error;

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> usize {
    InputParser::from(INPUT_VALUES)
        .validate_nearby_tickets()
        .iter()
        .sum()
}

/// Part two answer.
pub fn run_ex2() -> usize {
    let input = InputParser::from(INPUT_VALUES);

    input
        .map_ticket_fields()
        .iter()
        .filter_map(|(&k, &v)| {
            if k.starts_with("departure") {
                Some(input.your_ticket.numbers[v - 1])
            } else {
                None
            }
        })
        .product::<usize>()
}

/// Day error.
#[derive(Debug, Error)]
pub enum DayError {
    /// Rule parse error
    #[error("Rule parse error: {0}")]
    RuleParseError(String),
    /// Input parse error
    #[error("Input parse error: {0}")]
    InputParseError(String),
    /// Ticket parse error
    #[error("Ticket parse error: {0}")]
    TicketParseError(String),
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Ticket rule.
#[derive(Debug, PartialEq, Eq)]
pub struct TicketRule {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

impl TicketRule {
    /// Validate number.
    ///
    /// # Arguments
    ///
    /// * `number` - Number
    pub fn validate_number(&self, number: usize) -> bool {
        self.ranges.iter().filter(|x| x.contains(&number)).count() > 0
    }
}

impl From<&str> for TicketRule {
    fn from(value: &str) -> Self {
        let mut split_iter = value.trim().split(':');
        let (name, ranges_rule) = (
            split_iter.next().unwrap().to_string(),
            split_iter.next().unwrap().to_string(),
        );

        let ranges: Vec<RangeInclusive<usize>> = ranges_rule
            .trim()
            .split("or")
            .map(|r| {
                let vec: Vec<_> = r
                    .trim()
                    .split('-')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();

                let first = vec.get(0).copied().unwrap();
                let second = vec.get(1).copied().unwrap();

                RangeInclusive::new(first, second)
            })
            .collect();

        Self { name, ranges }
    }
}

/// Ticket.
#[derive(Debug, PartialEq, Eq)]
pub struct Ticket {
    numbers: Vec<usize>,
}

impl From<&str> for Ticket {
    fn from(value: &str) -> Self {
        let numbers: Vec<_> = value
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        Self { numbers }
    }
}

/// Input parser.
#[derive(Debug, PartialEq, Eq)]
pub struct InputParser {
    rules: Vec<TicketRule>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl InputParser {
    /// Validate number using rules.
    ///
    /// # Arguments
    ///
    /// * `number` - Number
    pub fn validate_number(&self, number: usize) -> bool {
        self.rules
            .iter()
            .filter(|&r| r.validate_number(number))
            .count()
            > 0
    }

    /// Validate ticket using rules.
    /// Returns numbers that are not valid (for all rules).
    ///
    /// # Arguments
    ///
    /// * `ticket` - Ticket
    pub fn validate_ticket(&self, ticket: &Ticket) -> Vec<usize> {
        ticket
            .numbers
            .iter()
            .filter(|&&n| !self.validate_number(n))
            .copied()
            .collect()
    }

    /// Validate nearby tickets.
    /// Returns numbers that are not valid (for all rules), for all tickets.
    pub fn validate_nearby_tickets(&self) -> Vec<usize> {
        self.nearby_tickets
            .iter()
            .flat_map(|t| self.validate_ticket(t))
            .collect()
    }

    /// Map ticket fields with position.
    pub fn map_ticket_fields(&self) -> HashMap<&str, usize> {
        // Filter nearby tickets
        let remaining_tickets: Vec<_> = self
            .nearby_tickets
            .iter()
            .filter(|t| self.validate_ticket(t).is_empty())
            .collect();

        let mut rules_affectation: HashMap<&str, usize> =
            self.rules.iter().map(|x| (&*x.name, usize::MAX)).collect();
        let mut remaining_rules: Vec<&str> = self.rules.iter().map(|x| &*x.name).collect();
        let mut invalid_positions: HashMap<&str, HashSet<usize>> = self
            .rules
            .iter()
            .map(|x| (&*x.name, HashSet::new()))
            .collect();
        let mut positions: HashMap<&str, Vec<usize>> =
            self.rules.iter().map(|x| (&*x.name, vec![])).collect();

        // Accumulate potential positions
        for t in &remaining_tickets {
            for r in &self.rules {
                for (idx, n) in t.numbers.iter().enumerate() {
                    if r.validate_number(*n) {
                        positions.get_mut(&*r.name).unwrap().push(idx);
                    } else {
                        invalid_positions.get_mut(&*r.name).unwrap().insert(idx);
                    }
                }
            }
        }

        while !remaining_rules.is_empty() {
            // Remove invalid positions from positions
            for (rn, counter) in &mut positions {
                let new_counter: Vec<_> = counter
                    .iter()
                    .filter(|x| invalid_positions.get(rn).map_or(true, |v| !v.contains(x)))
                    .copied()
                    .collect();

                *counter = new_counter;
            }

            // Get the most seen value per rule
            let mut rules_to_remove = vec![];
            for (idx, rn) in remaining_rules.iter().enumerate() {
                let counter = positions.get(rn).unwrap();
                let count_map = Self::count_occurences_in_vec(counter);

                // Simple case, only one number available in counter
                if count_map.len() == 1 {
                    let position = *count_map.keys().next().unwrap();
                    rules_affectation.insert(rn, position);
                    rules_to_remove.push(idx);

                    // Add as invalid positions for other rules
                    for r in remaining_rules.iter().filter(|&&o_rn| o_rn != *rn) {
                        invalid_positions.get_mut(r).unwrap().insert(position);
                    }
                }
            }

            // Remove already found rules
            while let Some(r_idx) = rules_to_remove.pop() {
                remaining_rules.remove(r_idx);
            }
        }

        // Increment affectations to match needed output
        for pos in rules_affectation.values_mut() {
            *pos += 1;
        }

        rules_affectation
    }

    fn count_occurences_in_vec(v: &[usize]) -> HashMap<usize, usize> {
        let mut m = HashMap::new();
        for n in v {
            *m.entry(*n).or_default() += 1;
        }
        m
    }
}

impl From<&str> for InputParser {
    fn from(value: &str) -> Self {
        let groups: Vec<&str> = value.trim().split("\n\n").collect();
        let rules_section = groups.get(0).cloned().unwrap();
        let your_ticket_section = groups.get(1).cloned().unwrap();
        let nearby_tickets_section = groups.get(2).cloned().unwrap();

        let rules: Vec<_> = rules_section.trim().lines().map(TicketRule::from).collect();
        let your_ticket = Ticket::from(your_ticket_section.trim().lines().nth(1).unwrap());
        let nearby_tickets: Vec<_> = nearby_tickets_section
            .trim()
            .lines()
            .skip(1)
            .map(Ticket::from)
            .collect();

        Self {
            rules,
            your_ticket,
            nearby_tickets,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 22_000;
    const EX2_OUTPUT: usize = 410_460_648_673;

    const SAMPLE: &str = indoc::indoc! {"
        class: 1-3 or 5-7
        row: 6-11 or 33-44
        seat: 13-40 or 45-50

        your ticket:
        7,1,14

        nearby tickets:
        7,3,47
        40,4,50
        55,2,20
        38,6,12
    "};

    const SAMPLE_2: &str = indoc::indoc! {"
        class: 0-1 or 4-19
        row: 0-5 or 8-19
        seat: 0-13 or 16-19

        your ticket:
        11,12,13

        nearby tickets:
        3,9,18
        15,1,5
        5,14,9
    "};

    #[test]
    fn test_ticket_rule_parse() {
        assert_eq!(
            TicketRule::from("class: 1-3 or 5-7"),
            TicketRule {
                name: "class".into(),
                ranges: vec![
                    RangeInclusive::<usize>::new(1, 3),
                    RangeInclusive::<usize>::new(5, 7)
                ]
            }
        );
    }

    #[test]
    fn test_ticket_parse() {
        assert_eq!(
            Ticket::from("7,1,14"),
            Ticket {
                numbers: vec![7, 1, 14]
            }
        );
    }

    #[test]
    fn test_input_parse() {
        assert_eq!(
            InputParser::from(SAMPLE),
            InputParser {
                rules: vec![
                    TicketRule {
                        name: "class".into(),
                        ranges: vec![
                            RangeInclusive::<usize>::new(1, 3),
                            RangeInclusive::<usize>::new(5, 7),
                        ]
                    },
                    TicketRule {
                        name: "row".into(),
                        ranges: vec![
                            RangeInclusive::<usize>::new(6, 11),
                            RangeInclusive::<usize>::new(33, 44),
                        ]
                    },
                    TicketRule {
                        name: "seat".into(),
                        ranges: vec![
                            RangeInclusive::<usize>::new(13, 40),
                            RangeInclusive::<usize>::new(45, 50),
                        ]
                    }
                ],
                your_ticket: Ticket {
                    numbers: vec![7, 1, 14]
                },
                nearby_tickets: vec![
                    Ticket {
                        numbers: vec![7, 3, 47]
                    },
                    Ticket {
                        numbers: vec![40, 4, 50]
                    },
                    Ticket {
                        numbers: vec![55, 2, 20]
                    },
                    Ticket {
                        numbers: vec![38, 6, 12]
                    }
                ]
            }
        );
    }

    #[test]
    fn test_validate_nearby_tickets() {
        let parser = InputParser::from(SAMPLE);
        assert_eq!(parser.validate_nearby_tickets(), vec![4, 55, 12]);
        assert_eq!(parser.validate_nearby_tickets().iter().sum::<usize>(), 71);
    }

    #[test]
    fn test_map_ticket_fields() {
        let parser = InputParser::from(SAMPLE_2);
        let res: HashMap<&str, usize> = maplit::hashmap! {
            "class" => 2,
            "row" => 1,
            "seat" => 3
        };

        assert_eq!(parser.map_ticket_fields(), res);
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
