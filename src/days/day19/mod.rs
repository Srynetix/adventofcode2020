//! # Day 19: Monster Messages
//!
//! You land in an airport surrounded by dense forest. As you walk to your high-speed train, the Elves at the Mythical Information Bureau contact you again. They think their satellite has collected an image of a sea monster! Unfortunately, the connection to the satellite is having problems, and many of the messages sent back from the satellite have been corrupted.
//!
//! They sent you a list of the rules valid messages should obey and a list of received messages they've collected so far (your puzzle input).
//!
//! The rules for valid messages (the top part of your puzzle input) are numbered and build upon each other. For example:
//!
//! ```text
//! 0: 1 2
//! 1: "a"
//! 2: 1 3 | 3 1
//! 3: "b"
//! ```
//!
//! Some rules, like 3: "b", simply match a single character (in this case, b).
//!
//! The remaining rules list the sub-rules that must be followed; for example, the rule 0: 1 2 means that to match rule 0, the text being checked must match rule 1, and the text after the part that matched rule 1 must then match rule 2.
//!
//! Some of the rules have multiple lists of sub-rules separated by a pipe (|). This means that at least one list of sub-rules must match. (The ones that match might be different each time the rule is encountered.) For example, the rule 2: 1 3 | 3 1 means that to match rule 2, the text being checked must match rule 1 followed by rule 3 or it must match rule 3 followed by rule 1.
//!
//! Fortunately, there are no loops in the rules, so the list of possible matches will be finite. Since rule 1 matches a and rule 3 matches b, rule 2 matches either ab or ba. Therefore, rule 0 matches aab or aba.
//!
//! Here's a more interesting example:
//!
//! ```text
//! 0: 4 1 5
//! 1: 2 3 | 3 2
//! 2: 4 4 | 5 5
//! 3: 4 5 | 5 4
//! 4: "a"
//! 5: "b"
//! ```
//!
//! Here, because rule 4 matches a and rule 5 matches b, rule 2 matches two letters that are the same (aa or bb), and rule 3 matches two letters that are different (ab or ba).
//!
//! Since rule 1 matches rules 2 and 3 once each in either order, it must match two pairs of letters, one pair with matching letters and one pair with different letters. This leaves eight possibilities: aaab, aaba, bbab, bbba, abaa, abbb, baaa, or babb.
//!
//! Rule 0, therefore, matches a (rule 4), then any of the eight options from rule 1, then b (rule 5): aaaabb, aaabab, abbabb, abbbab, aabaab, aabbbb, abaaab, or ababbb.
//!
//! The received messages (the bottom part of your puzzle input) need to be checked against the rules so you can determine which are valid and which are corrupted. Including the rules and the messages together, this might look like:
//!
//! ```text
//! 0: 4 1 5
//! 1: 2 3 | 3 2
//! 2: 4 4 | 5 5
//! 3: 4 5 | 5 4
//! 4: "a"
//! 5: "b"
//!
//! ababbb
//! bababa
//! abbbab
//! aaabbb
//! aaaabbb
//! ```
//!
//! Your goal is to determine the number of messages that completely match rule 0. In the above example, ababbb and abbbab match, but bababa, aaabbb, and aaaabbb do not, producing the answer 2. The whole message must match all of rule 0; there can't be extra unmatched characters in the message. (For example, aaaabbb might appear to match rule 0 above, but it has an extra unmatched b on the end.)
//!
//! How many messages completely match rule 0?
//!
//! # Part Two
//!
//! As you look over the list of messages, you realize your matching rules aren't quite right. To fix them, completely replace rules 8: 42 and 11: 42 31 with the following:
//!
//! ```text
//! 8: 42 | 42 8
//! 11: 42 31 | 42 11 31
//! ```
//!
//! This small change has a big impact: now, the rules do contain loops, and the list of messages they could hypothetically match is infinite. You'll need to determine how these changes affect which messages are valid.
//!
//! Fortunately, many of the rules are unaffected by this change; it might help to start by looking at which rules always match the same set of values and how those rules (especially rules 42 and 31) are used by the new versions of rules 8 and 11.
//!
//! (Remember, you only need to handle the rules you have; building a solution that could handle any hypothetical combination of rules would be significantly more difficult.)
//!
//! For example:
//!
//! ```text
//! 42: 9 14 | 10 1
//! 9: 14 27 | 1 26
//! 10: 23 14 | 28 1
//! 1: "a"
//! 11: 42 31
//! 5: 1 14 | 15 1
//! 19: 14 1 | 14 14
//! 12: 24 14 | 19 1
//! 16: 15 1 | 14 14
//! 31: 14 17 | 1 13
//! 6: 14 14 | 1 14
//! 2: 1 24 | 14 4
//! 0: 8 11
//! 13: 14 3 | 1 12
//! 15: 1 | 14
//! 17: 14 2 | 1 7
//! 23: 25 1 | 22 14
//! 28: 16 1
//! 4: 1 1
//! 20: 14 14 | 1 15
//! 3: 5 14 | 16 1
//! 27: 1 6 | 14 18
//! 14: "b"
//! 21: 14 1 | 1 14
//! 25: 1 1 | 1 14
//! 22: 14 14
//! 8: 42
//! 26: 14 22 | 1 20
//! 18: 15 15
//! 7: 14 5 | 1 21
//! 24: 14 1
//!
//! abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
//! bbabbbbaabaabba
//! babbbbaabbbbbabbbbbbaabaaabaaa
//! aaabbbbbbaaaabaababaabababbabaaabbababababaaa
//! bbbbbbbaaaabbbbaaabbabaaa
//! bbbababbbbaaaaaaaabbababaaababaabab
//! ababaaaaaabaaab
//! ababaaaaabbbaba
//! baabbaaaabbaaaababbaababb
//! abbbbabbbbaaaababbbbbbaaaababb
//! aaaaabbaabaaaaababaa
//! aaaabbaaaabbaaa
//! aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
//! babaaabbbaaabaababbaabababaaab
//! aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
//! ```
//!
//! Without updating rules 8 and 11, these rules only match three messages: bbabbbbaabaabba, ababaaaaaabaaab, and ababaaaaabbbaba.
//!
//! However, after updating rules 8 and 11, a total of 12 messages match:
//!
//! ```text
//! bbabbbbaabaabba
//! babbbbaabbbbbabbbbbbaabaaabaaa
//! aaabbbbbbaaaabaababaabababbabaaabbababababaaa
//! bbbbbbbaaaabbbbaaabbabaaa
//! bbbababbbbaaaaaaaabbababaaababaabab
//! ababaaaaaabaaab
//! ababaaaaabbbaba
//! baabbaaaabbaaaababbaababb
//! abbbbabbbbaaaababbbbbbaaaababb
//! aaaaabbaabaaaaababaa
//! aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
//! aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
//! ```
//!
//! After updating rules 8 and 11, how many messages completely match rule 0?

use std::collections::HashMap;

use regex::Regex;
use thiserror::Error;

const INPUT_VALUES: &str = include_str!("input.txt");

/// Part one answer.
pub fn run_ex1() -> usize {
    RuleSystem::from_rules_and_values(INPUT_VALUES).len()
}

/// Part two answer.
pub fn run_ex2() -> usize {
    RuleSystem::from_rules_and_values_alternative(INPUT_VALUES).len()
}

/// Day error.
#[derive(Debug, Error)]
pub enum DayError {
    /// Parse error.
    #[error("Parse error: {0}")]
    ParseError(String),
    /// Invalid rule.
    #[error("Rule is not of type {0}: {1}")]
    InvalidRule(&'static str, String),
    /// Missing rule.
    #[error("Missing rule ID: {0}")]
    MissingRule(usize),
    /// Unexpected.
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

/// Rule type.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum RuleType {
    /// Char match.
    Char(char),
    /// Link to other rules.
    Link(Vec<usize>),
    /// Either match.
    Either(Vec<usize>, Vec<usize>),
}

/// Rule.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rule {
    id: Option<usize>,
    typ: RuleType,
}

impl Rule {
    /// Creates a new rule.
    ///
    /// # Arguments
    ///
    /// * `id` - Rule ID
    /// * `typ` - Rule type
    pub const fn new(id: usize, typ: RuleType) -> Self {
        Self { id: Some(id), typ }
    }

    /// Creates a new anonymous rule.
    ///
    /// # Arguments
    ///
    /// * `typ` - Rule type
    pub const fn new_anonymous(typ: RuleType) -> Self {
        Self { id: None, typ }
    }
}

/// Rule parser.
pub struct RuleParser;

impl RuleParser {
    fn parse_rule_id(rule_id: &str) -> usize {
        rule_id.parse::<usize>().unwrap()
    }

    fn extract_rule_ids<'a, I>(components: I) -> Vec<usize>
    where
        I: Iterator<Item = &'a str>,
    {
        components.map(Self::parse_rule_id).collect::<Vec<_>>()
    }

    fn try_parse_either(rule_components: &str) -> Result<RuleType, DayError> {
        let pipe_components = rule_components.split('|').collect::<Vec<_>>();
        if pipe_components.len() > 1 {
            let a = Self::extract_rule_ids(pipe_components.get(0).unwrap().split_whitespace());

            let b = Self::extract_rule_ids(pipe_components.get(1).unwrap().split_whitespace());

            Ok(RuleType::Either(a, b))
        } else {
            Err(DayError::InvalidRule("Either", rule_components.to_string()))
        }
    }

    fn try_parse_link(rule_components: &str) -> Result<RuleType, DayError> {
        use std::cmp::Ordering;

        let link_components = rule_components.split_whitespace().collect::<Vec<_>>();
        match link_components.len().cmp(&1) {
            Ordering::Greater => {
                let ids = Self::extract_rule_ids(link_components.into_iter());
                Ok(RuleType::Link(ids))
            }
            Ordering::Equal => {
                if let Ok(v) = link_components[0].parse::<usize>() {
                    Ok(RuleType::Link(vec![v]))
                } else {
                    Err(DayError::InvalidRule("Link", rule_components.to_string()))
                }
            }
            Ordering::Less => Err(DayError::InvalidRule("Link", rule_components.to_string())),
        }
    }

    fn try_parse_char(rule_components: &str) -> Result<RuleType, DayError> {
        // Remove quotes
        let rule_components = rule_components.replace("\"", "");
        let char_component = rule_components.chars().next().ok_or_else(|| {
            DayError::ParseError(format!(
                "Could not parse char from Char rule: {}",
                rule_components
            ))
        })?;
        if char_component.is_ascii_alphabetic() {
            Ok(RuleType::Char(char_component))
        } else {
            Err(DayError::InvalidRule("Char", rule_components))
        }
    }

    /// Parse rule.
    ///
    /// # Arguments
    ///
    /// * `input` - Input
    pub fn parse_rule(value: &str) -> Rule {
        let mut components = value.trim().split(':');
        let rule_id: usize = Self::parse_rule_id(components.next().unwrap().trim());
        let rule_components: &str = components.next().unwrap().trim();
        let rule_type = Self::try_parse_either(rule_components)
            .or_else(|_| Self::try_parse_link(rule_components))
            .or_else(|_| Self::try_parse_char(rule_components))
            .unwrap();

        Rule::new(rule_id, rule_type)
    }
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        RuleParser::parse_rule(value)
    }
}

/// Rule system.
#[derive(Debug, Default)]
pub struct RuleSystem {
    rules: HashMap<usize, Rule>,
}

impl RuleSystem {
    /// Creates new rule system.
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Add rule as string.
    ///
    /// # Arguments
    ///
    /// * `rule_str` - Rule as string
    pub fn add_rule_as_string(&mut self, rule_str: &str) -> &Rule {
        let rule: Rule = rule_str.into();
        let rid = rule.id.unwrap();
        self.rules.insert(rid, rule);
        self.rules.get(&rid).unwrap()
    }

    /// Add multiple rules as string.
    ///
    /// # Arguments
    ///
    /// * `rule_str` - Rule as string
    pub fn add_rules_as_string(&mut self, rules_str: &str) {
        for rule in rules_str.lines() {
            let rule: Rule = rule.into();
            self.rules.insert(rule.id.unwrap(), rule);
        }
    }

    /// Replace rule.
    ///
    /// # Arguments
    ///
    /// * `rule_id` - Rule ID
    /// * `rule_type` - Rule type
    pub fn replace_rule(&mut self, rule_id: usize, rule_type: RuleType) {
        self.rules.insert(rule_id, Rule::new(rule_id, rule_type));
    }

    /// Validate entry.
    ///
    /// # Arguments
    ///
    /// * `entry` - Entry
    /// * `rule_id` - Rule ID
    pub fn validate_entry(&self, entry: &str, rule_id: usize) -> bool {
        let rgx = self.generate_regex_from_rule_id(rule_id, &mut HashMap::new());
        rgx.is_match(entry)
    }

    /// Creates system from rules and values.
    ///
    /// # Arguments
    ///
    /// * `input` - Input
    pub fn from_rules_and_values(input: &str) -> Vec<&str> {
        let (rules, values) = Self::extract_rules_and_values(input);
        let mut inst = Self::new();
        inst.add_rules_as_string(rules);
        inst.filter_values_with_rule(values, 0, &mut HashMap::new())
    }

    /// Creates system from rules and values, alternative mode.
    ///
    /// # Arguments
    ///
    /// * `input` - Input
    pub fn from_rules_and_values_alternative(input: &str) -> Vec<&str> {
        let (rules, values) = Self::extract_rules_and_values(input);
        let mut inst = Self::new();
        inst.add_rules_as_string(rules);

        // Replace rules
        inst.replace_rule(8, RuleType::Either(vec![42], vec![42, 8]));
        inst.replace_rule(11, RuleType::Either(vec![42, 31], vec![42, 11, 31]));

        // Precompute some cache values
        let mut cache = HashMap::new();
        let rule_42 = inst.generate_regex_inner_str_from_rule_id(42, &mut cache);
        let rule_31 = inst.generate_regex_inner_str_from_rule_id(31, &mut cache);

        // Match same quantity of 42 and 31, unroll until 4 (working magic value)
        let unroll_11 = (1..=4)
            .map(|i| format!("({}{{{}}}{}{{{}}})", rule_42, i, rule_31, i))
            .collect::<Vec<_>>()
            .join("|");

        // Overwrite cache
        cache.insert(8, format!("{}+", rule_42));
        cache.insert(11, format!("({})", unroll_11));

        inst.filter_values_with_rule(values, 0, &mut cache)
    }

    fn filter_values_with_rule<'a>(
        &self,
        values: &'a str,
        rule_id: usize,
        cache: &mut HashMap<usize, String>,
    ) -> Vec<&'a str> {
        let rule_0 = self.generate_regex_from_rule_id(rule_id, cache);
        values
            .split('\n')
            .filter(|&l| rule_0.is_match(l))
            .collect::<Vec<_>>()
    }

    fn extract_rules_and_values(input: &str) -> (&str, &str) {
        let mut components = input.split("\n\n");
        let rules = components.next().unwrap();
        let values = components.next().unwrap();

        (rules, values)
    }

    fn get_rule(&self, rule_id: usize) -> &Rule {
        self.rules.get(&rule_id).unwrap()
    }

    fn generate_regex_str(&self, rule: &Rule, regex_cache: &mut HashMap<usize, String>) -> String {
        if let Some(rid) = rule.id {
            if let Some(v) = regex_cache.get(&rid) {
                return v.clone();
            }
        }

        let r = match &rule.typ {
            RuleType::Char(c) => c.to_string(),
            RuleType::Link(l) => {
                let lst: String = l
                    .iter()
                    .map(|i| self.generate_regex_str(self.get_rule(*i), regex_cache))
                    .collect::<Vec<_>>()
                    .join("");
                format!("({})", lst)
            }
            RuleType::Either(la, lb) => {
                let rule_a = Rule::new_anonymous(RuleType::Link(la.clone()));
                let rule_b = Rule::new_anonymous(RuleType::Link(lb.clone()));
                let regex_a = self.generate_regex_str(&rule_a, regex_cache);
                let regex_b = self.generate_regex_str(&rule_b, regex_cache);
                format!("({}|{})", regex_a, regex_b)
            }
        };

        if let Some(rid) = rule.id {
            regex_cache.insert(rid, r.clone());
        }

        r
    }

    fn generate_regex_str_from_rule_id(
        &self,
        rule_id: usize,
        cache: &mut HashMap<usize, String>,
    ) -> String {
        format!(
            "^{}$",
            self.generate_regex_inner_str_from_rule_id(rule_id, cache)
        )
    }

    fn generate_regex_inner_str_from_rule_id(
        &self,
        rule_id: usize,
        cache: &mut HashMap<usize, String>,
    ) -> String {
        let rule = self.get_rule(rule_id).clone();
        self.generate_regex_str(&rule, cache)
    }

    fn generate_regex_from_rule_id(
        &self,
        rule_id: usize,
        cache: &mut HashMap<usize, String>,
    ) -> Regex {
        let rgx_str = self.generate_regex_str_from_rule_id(rule_id, cache);
        Regex::new(&rgx_str).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 118;
    const EX2_OUTPUT: usize = 246;

    #[test]
    fn test_rule_parse() {
        assert_eq!(
            Rule::from("0: 4 1 5"),
            Rule::new(0, RuleType::Link(vec![4, 1, 5]))
        );
        assert_eq!(
            Rule::from("1: 2 3 | 3 2"),
            Rule::new(1, RuleType::Either(vec![2, 3], vec![3, 2]))
        );
        assert_eq!(
            Rule::from("2: 4 4 | 5 5"),
            Rule::new(2, RuleType::Either(vec![4, 4], vec![5, 5]))
        );
        assert_eq!(
            Rule::from("3: 4 5 | 5 4"),
            Rule::new(3, RuleType::Either(vec![4, 5], vec![5, 4]))
        );
        assert_eq!(Rule::from("4: \"a\""), Rule::new(4, RuleType::Char('a')));
        assert_eq!(Rule::from("5: \"b\""), Rule::new(5, RuleType::Char('b')));
    }

    #[test]
    fn test_generate_regex_str() {
        fn generate_regex_str(system: &mut RuleSystem, rule_id: usize) -> String {
            system.generate_regex_str_from_rule_id(rule_id, &mut HashMap::new())
        }

        let mut system = RuleSystem::new();
        let sample = indoc::indoc! {r#"
            0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: "a"
            5: "b"
        "#};
        system.add_rules_as_string(sample);

        assert_eq!(generate_regex_str(&mut system, 5), "^b$");
        assert_eq!(generate_regex_str(&mut system, 4), "^a$");
        assert_eq!(generate_regex_str(&mut system, 3), "^((ab)|(ba))$");
        assert_eq!(generate_regex_str(&mut system, 2), "^((aa)|(bb))$");
        assert_eq!(
            generate_regex_str(&mut system, 1),
            "^((((aa)|(bb))((ab)|(ba)))|(((ab)|(ba))((aa)|(bb))))$"
        );
        assert_eq!(
            generate_regex_str(&mut system, 0),
            "^(a((((aa)|(bb))((ab)|(ba)))|(((ab)|(ba))((aa)|(bb))))b)$"
        )
    }

    #[test]
    fn test_match_str() {
        let mut system = RuleSystem::new();
        let sample = indoc::indoc! {r#"
            0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: "a"
            5: "b"
        "#};
        system.add_rules_as_string(sample);

        assert!(system.validate_entry("ababbb", 0));
        assert!(system.validate_entry("abbbab", 0));
        assert!(!system.validate_entry("bababa", 0));
        assert!(!system.validate_entry("aaabbb", 0));
        assert!(!system.validate_entry("aaabbbb", 0));
    }

    #[test]
    fn test_sample() {
        let sample = indoc::indoc! {r#"
            0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: "a"
            5: "b"

            ababbb
            bababa
            abbbab
            aaabbb
            aaaabbb
        "#};

        assert_eq!(RuleSystem::from_rules_and_values(sample).len(), 2);
    }

    #[test]
    fn test_sample_2() {
        let sample = indoc::indoc! {r#"
            42: 9 14 | 10 1
            9: 14 27 | 1 26
            10: 23 14 | 28 1
            1: "a"
            11: 42 31
            5: 1 14 | 15 1
            19: 14 1 | 14 14
            12: 24 14 | 19 1
            16: 15 1 | 14 14
            31: 14 17 | 1 13
            6: 14 14 | 1 14
            2: 1 24 | 14 4
            0: 8 11
            13: 14 3 | 1 12
            15: 1 | 14
            17: 14 2 | 1 7
            23: 25 1 | 22 14
            28: 16 1
            4: 1 1
            20: 14 14 | 1 15
            3: 5 14 | 16 1
            27: 1 6 | 14 18
            14: "b"
            21: 14 1 | 1 14
            25: 1 1 | 1 14
            22: 14 14
            8: 42
            26: 14 22 | 1 20
            18: 15 15
            7: 14 5 | 1 21
            24: 14 1

            abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            bbabbbbaabaabba
            babbbbaabbbbbabbbbbbaabaaabaaa
            aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            bbbbbbbaaaabbbbaaabbabaaa
            bbbababbbbaaaaaaaabbababaaababaabab
            ababaaaaaabaaab
            ababaaaaabbbaba
            baabbaaaabbaaaababbaababb
            abbbbabbbbaaaababbbbbbaaaababb
            aaaaabbaabaaaaababaa
            aaaabbaaaabbaaa
            aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            babaaabbbaaabaababbaabababaaab
            aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
        "#};

        assert_eq!(
            RuleSystem::from_rules_and_values_alternative(sample).len(),
            12
        )
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
