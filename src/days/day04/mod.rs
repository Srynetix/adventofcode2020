//! # Day 4: Passport Processing
//! You arrive at the airport only to realize that you grabbed your North Pole Credentials instead of your passport. While these documents are extremely similar, North Pole Credentials aren't issued by a country and therefore aren't actually valid documentation for travel in most of the world.
//!
//! It seems like you're not the only one having problems, though; a very long line has formed for the automatic passport scanners, and the delay could upset your travel itinerary.
//!
//! Due to some questionable network security, you realize you might be able to solve both of these problems at the same time.
//!
//! The automatic passport scanners are slow because they're having trouble detecting which passports have all required fields. The expected fields are as follows:
//!
//! ```text
//! byr (Birth Year)
//! iyr (Issue Year)
//! eyr (Expiration Year)
//! hgt (Height)
//! hcl (Hair Color)
//! ecl (Eye Color)
//! pid (Passport ID)
//! cid (Country ID)
//! ```
//!
//! Passport data is validated in batch files (your puzzle input). Each passport is represented as a sequence of key:value pairs separated by spaces or newlines. Passports are separated by blank lines.
//!
//! Here is an example batch file containing four passports:
//!
//! ```text
//! ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
//! byr:1937 iyr:2017 cid:147 hgt:183cm
//!
//! iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
//! hcl:#cfa07d byr:1929
//!
//! hcl:#ae17e1 iyr:2013
//! eyr:2024
//! ecl:brn pid:760753108 byr:1931
//! hgt:179cm
//!
//! hcl:#cfa07d eyr:2025 pid:166559648
//! iyr:2011 ecl:brn hgt:59in
//! ```
//!
//! The first passport is valid - all eight fields are present. The second passport is invalid - it is missing hgt (the Height field).
//!
//! The third passport is interesting; the only missing field is cid, so it looks like data from North Pole Credentials, not a passport at all! Surely, nobody would mind if you made the system temporarily ignore missing cid fields. Treat this "passport" as valid.
//!
//! The fourth passport is missing two fields, cid and byr. Missing cid is fine, but missing any other field is not, so this passport is invalid.
//!
//! According to the above rules, your improved system would report 2 valid passports.
//!
//! Count the number of valid passports - those that have all required fields. Treat cid as optional. In your batch file, how many passports are valid?
//!
//! # Part Two
//! The line is moving more quickly now, but you overhear airport security talking about how passports with invalid data are getting through. Better add some data validation, quick!
//!
//! You can continue to ignore the cid field, but each other field has strict rules about what values are valid for automatic validation:
//!
//! ```text
//! byr (Birth Year) - four digits; at least 1920 and at most 2002.
//! iyr (Issue Year) - four digits; at least 2010 and at most 2020.
//! eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
//! hgt (Height) - a number followed by either cm or in:
//!   If cm, the number must be at least 150 and at most 193.
//!   If in, the number must be at least 59 and at most 76.
//! hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
//! ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
//! pid (Passport ID) - a nine-digit number, including leading zeroes.
//! cid (Country ID) - ignored, missing or not.
//! ```
//!
//! Your job is to count the passports where all required fields are both present and valid according to the above rules. Here are some example values:
//!
//! ```text
//! byr valid:   2002
//! byr invalid: 2003
//!
//! hgt valid:   60in
//! hgt valid:   190cm
//! hgt invalid: 190in
//! hgt invalid: 190
//!
//! hcl valid:   #123abc
//! hcl invalid: #123abz
//! hcl invalid: 123abc
//!
//! ecl valid:   brn
//! ecl invalid: wat
//!
//! pid valid:   000000001
//! pid invalid: 0123456789
//! ```
//!
//! Here are some invalid passports:
//!
//! ```text
//! eyr:1972 cid:100
//! hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
//!
//! iyr:2019
//! hcl:#602927 eyr:1967 hgt:170cm
//! ecl:grn pid:012533040 byr:1946
//!
//! hcl:dab227 iyr:2012
//! ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
//!
//! hgt:59cm ecl:zzz
//! eyr:2038 hcl:74454a iyr:2023
//! pid:3556412378 byr:2007
//! ```
//!
//! Here are some valid passports:
//!
//! ```text
//! pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
//! hcl:#623a2f
//!
//! eyr:2029 ecl:blu cid:129 byr:1989
//! iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
//!
//! hcl:#888785
//! hgt:164cm byr:2001 iyr:2015 cid:88
//! pid:545766238 ecl:hzl
//! eyr:2022
//!
//! iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
//! ```
//!
//! Count the number of valid passports - those that have all required fields and valid values. Continue to treat cid as optional. In your batch file, how many passports are valid?

use std::collections::HashMap;

use eyre::Result;
use once_cell::sync::Lazy;
use regex::Regex;

const INPUT_VALUES: &str = include_str!("input.txt");
const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const VALID_EYE_COLOR: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
static HEIGHT_RGX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<amount>\d+)(?P<unit>cm|in)$").expect("Bad regex format"));
static COLOR_RGX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^#(?P<color>[0-9a-f]{6})$").expect("Bad regex format"));
static PID_RGX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?P<num>\d{9})$").expect("Bad regex format"));

/// Part one answer.
pub fn run_ex1() -> usize {
    PassportValidator::parse_entries(INPUT_VALUES)
        .expect("Bad input format")
        .iter()
        .filter(|x| x.is_valid())
        .count()
}

/// Part two answer.
pub fn run_ex2() -> usize {
    PassportValidator::parse_entries(INPUT_VALUES)
        .expect("Bad input format")
        .iter()
        .filter(|x| x.is_valid_full())
        .count()
}

/// Passport validator.
pub struct PassportValidator(HashMap<String, String>);

impl PassportValidator {
    /// Parse passport entry.
    ///
    /// # Arguments
    ///
    /// * `entry` - Passport entry
    ///
    /// # Errors
    ///
    /// * Entry parse error
    pub fn parse_entry(entry: &str) -> Result<Self> {
        Ok(Self(
            entry
                .split_whitespace()
                .map(|e| {
                    let mut spl = e.split_terminator(':');
                    let key = spl.next().expect("missing key").to_owned();
                    let value = spl.next().expect("missing value").to_owned();
                    (key, value)
                })
                .collect(),
        ))
    }

    /// Validate passport field.
    ///
    /// # Arguments
    ///
    /// * `key` - Field key
    /// * `value` - Field value
    ///
    /// # Errors
    ///
    /// * Parse errors
    pub fn try_validate_field(key: &str, value: &str) -> Result<bool> {
        match key {
            "byr" => {
                // At least 1920 at most 2002
                let value: usize = value.parse()?;
                Ok(value >= 1920 && value <= 2002)
            }
            "iyr" => {
                // At least 2010 at most 2020
                let value: usize = value.parse()?;
                Ok(value >= 2010 && value <= 2020)
            }
            "eyr" => {
                // At least 2020 at most 2030
                let value: usize = value.parse()?;
                Ok(value >= 2020 && value <= 2030)
            }
            "hgt" => {
                if let Some(v) = HEIGHT_RGX.captures(value) {
                    let amount = v
                        .name("amount")
                        .and_then(|x| x.as_str().parse::<usize>().ok())
                        .expect("should work");
                    let unit = v.name("unit").map(|x| x.as_str()).expect("should exist");

                    match unit {
                        // At least 150 at most 193
                        "cm" => Ok(amount >= 150 && amount <= 193),
                        // At least 59 at most 76
                        "in" => Ok(amount >= 59 && amount <= 76),
                        _ => unreachable!(),
                    }
                } else {
                    Ok(false)
                }
            }
            "hcl" => Ok(COLOR_RGX.is_match(value)),
            "ecl" => Ok(VALID_EYE_COLOR.contains(&value)),
            "pid" => Ok(PID_RGX.is_match(value)),
            _ => Ok(false),
        }
    }

    /// Check if passport is valid.
    pub fn is_valid(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .filter(|&x| self.0.contains_key(*x))
            .count()
            == REQUIRED_FIELDS.len()
    }

    /// Check if passport is valid.
    pub fn is_valid_full(&self) -> bool {
        REQUIRED_FIELDS
            .iter()
            .filter(|&x| {
                self.0
                    .get(*x)
                    .map_or(false, |v| Self::try_validate_field(x, v).unwrap_or(false))
            })
            .count()
            == REQUIRED_FIELDS.len()
    }

    /// Parse multiple passport entries.
    ///
    /// # Arguments
    ///
    /// * `entries` - Passport entries
    ///
    /// # Errors
    ///
    /// * Entry parse error
    pub fn parse_entries(entries: &str) -> Result<Vec<Self>> {
        entries.split("\n\n").map(Self::parse_entry).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 213;
    const EX2_OUTPUT: usize = 147;

    #[test]
    fn test_run_ex1() {
        assert_eq!(run_ex1(), EX1_OUTPUT);
    }

    #[test]
    fn test_run_ex2() {
        assert_eq!(run_ex2(), EX2_OUTPUT);
    }
}
