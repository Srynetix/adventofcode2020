//! # Day 7: Handy Haversacks
//!
//! You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.
//!
//! Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!
//!
//! For example, consider the following rules:
//!
//! ```text
//! light red bags contain 1 bright white bag, 2 muted yellow bags.
//! dark orange bags contain 3 bright white bags, 4 muted yellow bags.
//! bright white bags contain 1 shiny gold bag.
//! muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
//! shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
//! dark olive bags contain 3 faded blue bags, 4 dotted black bags.
//! vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
//! faded blue bags contain no other bags.
//! dotted black bags contain no other bags.
//! ```
//!
//! These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.
//!
//! You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)
//!
//! In the above rules, the following options would be available to you:
//!
//! - A bright white bag, which can hold your shiny gold bag directly.
//! - A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
//! - A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
//! - A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
//!
//! So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.
//!
//! How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)
//!
//! # Part two
//!
//! It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!
//!
//! Consider again your shiny gold bag and the rules from the above example:
//!
//! - faded blue bags contain 0 other bags.
//! - dotted black bags contain 0 other bags.
//! - vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
//! - dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
//!
//! So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!
//!
//! Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!
//!
//! Here's another example:
//!
//! ```text
//! shiny gold bags contain 2 dark red bags.
//! dark red bags contain 2 dark orange bags.
//! dark orange bags contain 2 dark yellow bags.
//! dark yellow bags contain 2 dark green bags.
//! dark green bags contain 2 dark blue bags.
//! dark blue bags contain 2 dark violet bags.
//! dark violet bags contain no other bags.
//! ```
//!
//! In this example, a single shiny gold bag must contain 126 other bags.
//!
//! How many individual bags are required inside your single shiny gold bag?

use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

const INPUT_VALUES: &str = include_str!("input.txt");
const INPUT_COLOR_NAME: &str = "shiny gold";

static MAIN_RULE_RGX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<color>\w+ \w+) bags contain (?P<rules>.*)\.$").unwrap());

static SIMPLE_RULE_RGX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<amount>\d+) (?P<color>\w+ \w+) bags?").unwrap());

const NO_OTHER_BAGS_STR: &str = "no other bags";

/// Part one answer.
pub fn run_ex1() -> usize {
    let system = BagSystem::new_from_rules(INPUT_VALUES);
    let color: BagColor = INPUT_COLOR_NAME.into();
    system.find_container_colors_for_color(&color).len()
}

/// Part two answer.
pub fn run_ex2() -> usize {
    let system = BagSystem::new_from_rules(INPUT_VALUES);
    let color: BagColor = INPUT_COLOR_NAME.into();
    system.count_needed_bags_for_color(&color)
}

/// Bag color
#[derive(Debug, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BagColor(String);

impl BagColor {
    /// Create a bag color from a str.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn new(input: &str) -> Self {
        Self(input.to_owned())
    }
}

impl From<&str> for BagColor {
    fn from(input: &str) -> Self {
        Self(input.to_owned())
    }
}

/// Bag relation
#[derive(Debug, Clone)]
pub struct BagRelation {
    color: BagColor,
    amount: usize,
}

impl BagRelation {
    /// Create a bag relation.
    ///
    /// # Arguments
    ///
    /// * `color` - Bag color
    /// * `amount` - Bag amount
    pub const fn new(color: BagColor, amount: usize) -> Self {
        Self { color, amount }
    }
}

/// Bag system
#[derive(Debug)]
pub struct BagSystem(HashMap<BagColor, Vec<BagRelation>>);

impl BagSystem {
    /// Parse rule.
    ///
    /// # Arguments
    ///
    /// * `input` - Input string
    pub fn parse_rule(&mut self, input: &str) {
        let capture = MAIN_RULE_RGX.captures(input.trim()).unwrap();
        let color = capture.name("color").map(|x| x.as_str()).unwrap();
        let rules = capture.name("rules").map(|x| x.as_str()).unwrap();

        let mut relations: Vec<BagRelation> = Vec::new();
        for rule in rules.split(", ") {
            if rule == NO_OTHER_BAGS_STR {
                continue;
            }

            let rule_capture = SIMPLE_RULE_RGX.captures(rule.trim()).unwrap();
            let rule_amount = rule_capture.name("amount").map(|x| x.as_str()).unwrap();
            let rule_color = rule_capture.name("color").map(|x| x.as_str()).unwrap();
            relations.push(BagRelation::new(
                BagColor::new(rule_color),
                rule_amount.parse().unwrap(),
            ));
        }

        self.0.insert(BagColor::new(color), relations);
    }

    /// Parse multiple rules.
    ///
    /// # Arguments
    ///
    /// * `entries` - Input string
    pub fn parse_rules(&mut self, entries: &str) {
        for entry in entries.lines() {
            self.parse_rule(entry);
        }
    }

    /// Create new bag system.
    pub fn new_from_rules(rules: &str) -> Self {
        let mut instance = Self(HashMap::new());

        instance.parse_rules(rules);
        instance
    }

    /// Get direct links for a known bag color.
    ///
    /// # Arguments
    ///
    /// * `color` - Known color
    pub fn get_direct_links_for_color(&self, color: &BagColor) -> Vec<BagColor> {
        self.0
            .iter()
            .filter_map(|(k, v)| {
                if k == color {
                    None
                } else {
                    let values: Vec<BagColor> = v
                        .iter()
                        .filter_map(|r| {
                            if r.color == *color {
                                Some(k.clone())
                            } else {
                                None
                            }
                        })
                        .collect();
                    Some(values)
                }
            })
            .flatten()
            .collect()
    }

    /// Find container bag colors for a target color.
    ///
    /// # Arguments
    ///
    /// * `color` - Known color
    pub fn find_container_colors_for_color(&self, color: &BagColor) -> Vec<BagColor> {
        let mut colors_to_scan: Vec<BagColor> = self.get_direct_links_for_color(color);
        let mut seen_colors: Vec<BagColor> = vec![];

        while let Some(scanned_color) = colors_to_scan.pop() {
            if seen_colors.contains(&scanned_color) {
                continue;
            }

            seen_colors.push(scanned_color.clone());
            let linked_colors = self.get_direct_links_for_color(&scanned_color);
            for linked_color in linked_colors {
                if seen_colors.contains(&linked_color)
                    || colors_to_scan.contains(&linked_color)
                    || &linked_color == color
                {
                    continue;
                }

                colors_to_scan.push(linked_color.clone());
            }
        }

        seen_colors
    }

    /// Count needed bags for a target color.
    ///
    /// # Arguments
    ///
    /// * `color` - Known color
    pub fn count_needed_bags_for_color(&self, color: &BagColor) -> usize {
        self.count_inner_bags_for_color(color) - 1
    }

    /// Count inner bags for a target color.
    ///
    /// # Arguments
    ///
    /// * `color` - Known color
    pub fn count_inner_bags_for_color(&self, inner_color: &BagColor) -> usize {
        let relations = self.0.get(inner_color).unwrap();
        let sum = relations
            .iter()
            .map(|x| x.amount * self.count_inner_bags_for_color(&x.color))
            .sum::<usize>()
            + 1;
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 378;
    const EX2_OUTPUT: usize = 27526;

    const EXAMPLE_FIXTURE_EX1: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
    dark orange bags contain 3 bright white bags, 4 muted yellow bags.
    bright white bags contain 1 shiny gold bag.
    muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
    shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    dark olive bags contain 3 faded blue bags, 4 dotted black bags.
    vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
    faded blue bags contain no other bags.
    dotted black bags contain no other bags."#;

    const EXAMPLE_FIXTURE_EX2: &str = r#"shiny gold bags contain 2 dark red bags.
    dark red bags contain 2 dark orange bags.
    dark orange bags contain 2 dark yellow bags.
    dark yellow bags contain 2 dark green bags.
    dark green bags contain 2 dark blue bags.
    dark blue bags contain 2 dark violet bags.
    dark violet bags contain no other bags."#;

    #[test]
    fn test_parse_rules() {
        BagSystem::new_from_rules(EXAMPLE_FIXTURE_EX1);
    }

    #[test]
    fn test_find_container_bag_colors() {
        let system = BagSystem::new_from_rules(EXAMPLE_FIXTURE_EX1);
        let color: BagColor = "shiny gold".into();
        let mut colors = system.find_container_colors_for_color(&color);
        colors.sort();

        let target: Vec<BagColor> =
            vec!["bright white", "dark orange", "light red", "muted yellow"]
                .into_iter()
                .map(Into::into)
                .collect();
        assert_eq!(colors, target);
    }

    #[test]
    fn test_count_needed_bags_for_color() {
        let system = BagSystem::new_from_rules(EXAMPLE_FIXTURE_EX2);
        let color: BagColor = "shiny gold".into();
        assert_eq!(system.count_needed_bags_for_color(&color), 126);
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
