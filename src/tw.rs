//! Utilities for Taiwan Identity Card

use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref PREFIX_LETTERS: HashMap<&'static str, u32> = {
        let mut map = HashMap::new();
        map.insert("A", 10);
        map.insert("B", 11);
        map.insert("C", 12);
        map.insert("D", 13);
        map.insert("E", 14);
        map.insert("F", 15);
        map.insert("G", 16);
        map.insert("H", 17);
        map.insert("J", 18);
        map.insert("K", 19);
        map.insert("L", 20);
        map.insert("M", 21);
        map.insert("N", 22);
        map.insert("P", 23);
        map.insert("Q", 24);
        map.insert("R", 25);
        map.insert("S", 26);
        map.insert("T", 27);
        map.insert("U", 28);
        map.insert("V", 29);
        map.insert("X", 30);
        map.insert("Y", 31);
        map.insert("W", 32);
        map.insert("Z", 33);
        map.insert("I", 34);
        map.insert("O", 35);
        map
    };
    static ref PATTERN: Regex = Regex::new(r"^[a-zA-Z][0-9]{9}$").unwrap();
}

/// Validates the number.
pub fn validate(number: &str) -> bool {
    let number = number.trim().to_ascii_uppercase();
    if number.len() == 10 && PATTERN.is_match(&number) {
        let start = &number[0..1];
        let sex = &number[1..2];
        let mid = &number[1..9];
        let end = &number[9..10];

        if sex != "1" && sex != "2" {
            return false;
        }

        let start = match PREFIX_LETTERS.get(start) {
            Some(value) => value,
            _ => return false,
        };

        let mut sum = start / 10 + (start % 10) * 9;
        let mut flag = 8;

        for ch in mid.chars() {
            let i = match ch.to_digit(10) {
                Some(value) => value,
                _ => return false,
            };
            sum = sum + i * flag;
            flag -= 1;
        }

        let end = match end.chars().nth(0) {
            Some(ch) => match ch.to_digit(10) {
                Some(value) => value,
                _ => return false,
            },
            _ => return false,
        };
        let checksum = if sum % 10 == 0 { 0 } else { 10 - sum % 10 };
        checksum == end
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_tw() {
        assert_eq!(validate("A123456789"), true);
        assert_eq!(validate("B142610160"), true);
        assert_eq!(validate("Q155304682"), true);
        assert_eq!(validate("Q155304680"), false);
    }
}
