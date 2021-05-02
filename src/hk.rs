//! Utilities for Hong Kong Identity Card

use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref PREFIX_LETTERS: HashMap<&'static str, u32> = {
        let mut map = HashMap::new();
        map.insert("A", 1);
        map.insert("B", 2);
        map.insert("C", 3);
        map.insert("R", 18);
        map.insert("U", 21);
        map.insert("Z", 26);
        map.insert("X", 24);
        map.insert("W", 23);
        map.insert("O", 15);
        map.insert("N", 14);
        map
    };
    static ref PATTERN: Regex = Regex::new(r"^[A-Z]{1,2}[0-9]{6}\(?[0-9A]\)?$").unwrap();
    static ref REMOVAL_PATTERN: Regex = Regex::new(r"[\(|\)]").unwrap();
}

/// Validates the number.
pub fn validate(number: &str) -> bool {
    if !PATTERN.is_match(number) {
        return false;
    }

    let number = REMOVAL_PATTERN
        .replace_all(number, "")
        .trim()
        .to_ascii_uppercase();

    let mut sum: u32;
    let mut card = &number[..];
    if number.len() == 9 {
        let first = match number.chars().nth(0) {
            Some(ch) => ch as u32,
            _ => return false,
        };
        let second = match number.chars().nth(1) {
            Some(ch) => ch as u32,
            _ => return false,
        };
        sum = (first - 55) * 9 + (second - 55) * 8;
        card = &number[1..9];
    } else {
        let first = match number.chars().nth(0) {
            Some(ch) => ch as u32,
            _ => return false,
        };
        sum = 522 + (first - 55) * 8;
    }

    let mid = &card[1..7];
    let end = &card[7..8];

    let mut flag = 7;
    for ch in mid.chars() {
        let i = match ch.to_digit(10) {
            Some(value) => value,
            _ => return false,
        };

        sum = sum + i * flag;
        flag -= 1;
    }

    if end == "A" {
        sum = sum + 10;
    } else {
        let i = match end.chars().nth(0) {
            Some(ch) => match ch.to_digit(10) {
                Some(value) => value,
                _ => return false,
            },
            _ => return false,
        };

        sum = sum + i;
    }
    sum % 11 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_hk() {
        assert_eq!(validate("G123456(A)"), true);
        assert_eq!(validate("G123456(a)"), false);
        assert_eq!(validate("G123456A"), true);
        assert_eq!(validate("L555555(0)"), true);
        assert_eq!(validate("AB987654(3)"), true);
        assert_eq!(validate("C123456(9)"), true);
        assert_eq!(validate("AY987654(A)"), false);
    }
}
