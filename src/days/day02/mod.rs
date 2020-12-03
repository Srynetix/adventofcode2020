//! # Day 2: Password Philosophy
//!
//! Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.
//!
//! Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.
//!
//! To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.
//!
//! For example, suppose you have the following list:
//!
//! ```text
//! 1-3 a: abcde
//! 1-3 b: cdefg
//! 2-9 c: ccccccccc
//! ```
//!
//! Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.
//!
//! In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.
//!
//! How many passwords are valid according to their policies?
//!
//! # Part Two
//!
//! While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.
//!
//! The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.
//!
//! Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.
//!
//! Given the same example list from above:
//!
//! - `1-3 a: abcde` is valid: position 1 contains a and position 3 does not.
//! - `1-3 b: cdefg` is invalid: neither position 1 nor position 3 contains b.
//! - `2-9 c: ccccccccc` is invalid: both position 2 and position 9 contain c.
//!
//! How many passwords are valid according to the new interpretation of the policies?

use eyre::{eyre, Result};
use once_cell::sync::Lazy;
use regex::Regex;

const INPUT_VALUES: &str = include_str!("input.txt");
static PASSWORD_RGX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<char>\w): (?P<password>\w+)")
        .expect("Bad regex format")
});

/// Part one answer.
pub fn run_ex1() -> usize {
    validate_multiple_passwords_with_count(INPUT_VALUES)
}

/// Part two answer.
pub fn run_ex2() -> usize {
    validate_multiple_passwords_with_position(INPUT_VALUES)
}

/// Validate multiple passwords with count.
///
/// # Arguments
///
/// * `entries` - Input text
pub fn validate_multiple_passwords_with_count(entries: &str) -> usize {
    validate_multiple_passwords_with_fn(entries, validate_password_with_count)
}

/// Validate multiple passwords with character position.
///
/// # Arguments
///
/// * `entries` - Input text
pub fn validate_multiple_passwords_with_position(entries: &str) -> usize {
    validate_multiple_passwords_with_fn(entries, validate_password_with_position)
}

/// Validate multiple passwords using function.
///
/// # Arguments
///
/// * `entries` - Input text
/// * `func` - Function
pub fn validate_multiple_passwords_with_fn<F>(entries: &str, func: F) -> usize
where
    F: Fn(&str) -> Result<bool>,
{
    entries
        .lines()
        .filter_map(|s| func(s).map(|v| v as usize).ok())
        .sum::<usize>()
}

/// Validate password with count.
///
/// # Arguments
///
/// * `entry` - Password
///
/// # Errors
///
/// * Bad password entry
pub fn validate_password_with_count(entry: &str) -> Result<bool> {
    let (min_v, max_v, char_v, password) = parse_password_entry(entry)?;
    let count = password.chars().filter(|c| *c == char_v).count();
    Ok(count <= max_v && count >= min_v)
}

/// Validate password with character position.
///
/// # Arguments
///
/// * `entry` - Password
///
/// # Errors
///
/// * Bad password entry
pub fn validate_password_with_position(entry: &str) -> Result<bool> {
    let (min_v, max_v, char_v, password) = parse_password_entry(entry)?;
    let bytes = password.as_bytes();
    let char_b = char_v as u8;
    let char_min = bytes[min_v - 1];
    let char_max = bytes[max_v - 1];

    Ok(if char_min == char_b && char_max == char_b {
        false
    } else if char_min == char_b {
        true
    } else {
        char_max == char_b
    })
}

/// Parse password entry.
///
/// # Arguments
///
/// * `entry` - Password
///
/// # Errors
///
/// * Bad password entry
pub fn parse_password_entry(entry: &str) -> Result<(usize, usize, char, &str)> {
    if let Some(captures) = PASSWORD_RGX.captures(entry) {
        return Ok((
            captures
                .name("min")
                .ok_or_else(|| eyre!("Missing 'min' value"))?
                .as_str()
                .parse()?,
            captures
                .name("max")
                .ok_or_else(|| eyre!("Missing 'max' value"))?
                .as_str()
                .parse()?,
            captures
                .name("char")
                .ok_or_else(|| eyre!("Missing 'char' value"))?
                .as_str()
                .parse()?,
            captures
                .name("password")
                .ok_or_else(|| eyre!("Missing 'password' value"))?
                .as_str(),
        ));
    }

    Err(eyre!("Entry '{}' does not match regex", entry))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX1_OUTPUT: usize = 418;
    const EX2_OUTPUT: usize = 616;

    #[test]
    fn test_parse_password_entry() {
        assert_eq!(
            parse_password_entry("1-3 c: tototutu").unwrap(),
            (1, 3, 'c', "tototutu")
        );
        assert_eq!(
            parse_password_entry("10-30 z: zzzzzz").unwrap(),
            (10, 30, 'z', "zzzzzz")
        );

        // Errors
        assert!(parse_password_entry("10-30 zz: zzzzzz").is_err());
        assert!(parse_password_entry("10- z: zzzzzz").is_err());
        assert!(parse_password_entry("-1 z = zzzzzz").is_err());
        assert!(parse_password_entry("-1 zzzzzzz").is_err());
    }

    #[test]
    fn test_validate_password_with_count() {
        assert!(validate_password_with_count("1-3 c: ceci").unwrap());
        assert!(!validate_password_with_count("1-3 c: cccc").unwrap());
    }

    #[test]
    fn test_validate_password_with_position() {
        assert!(validate_password_with_position("1-3 c: cabc").unwrap());
        assert!(!validate_password_with_position("1-3 c: cacc").unwrap());
        assert!(validate_password_with_position("1-3 c: aacc").unwrap());
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
