//! Utilities for Taiwan Identity Card

use crate::Gender;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref PREFIX_LETTERS: HashMap<&'static str, (u32, &'static str)> = {
        let mut map = HashMap::new();
        map.insert("A", (10,  "台北市"));
        map.insert("B", (11,  "台中市"));
        map.insert("C", (12,  "基隆市"));
        map.insert("D", (13,  "台南市"));
        map.insert("E", (14,  "高雄市"));
        map.insert("F", (15,  "新北市"));
        map.insert("G", (16,  "宜兰县"));
        map.insert("H", (17,  "桃园市"));
        map.insert("J", (18,  "新竹县"));
        map.insert("K", (19,  "苗栗县"));
        map.insert("L", (20,  "台中县")); // obsoleted
        map.insert("M", (21,  "南投县"));
        map.insert("N", (22,  "彰化县"));
        map.insert("P", (23,  "云林县"));
        map.insert("Q", (24,  "嘉义县"));
        map.insert("R", (25,  "台南县")); // obsoleted
        map.insert("S", (26,  "高雄县")); // obsoleted
        map.insert("T", (27,  "屏东县"));
        map.insert("U", (28,  "花莲县"));
        map.insert("V", (29,  "台东县"));
        map.insert("X", (30,  "澎湖县"));
        map.insert("Y", (31,  "阳明山管理局")); // obsoleted
        map.insert("W", (32,  "金门县"));
        map.insert("Z", (33,  "连江县"));
        map.insert("I", (34,  "嘉义市"));
        map.insert("O", (35,  "新竹市"));
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
        let end = &number[9..];

        if sex != "1" && sex != "2" {
            return false;
        }

        let start = match PREFIX_LETTERS.get(start) {
            Some(value) => value,
            _ => return false,
        };

        let mut sum = start.0 / 10 + (start.0 % 10) * 9;
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

/// Returns the gender.
pub fn gender(number: &str) -> Option<Gender> {
    if !validate(number) {
        return None;
    }

    if let Some(sex) = number.chars().nth(1) {
        if sex == '1' {
            Some(Gender::Male)
        } else if sex == '2' {
            Some(Gender::Female)
        } else {
            None
        }
    } else {
        None
    }
}

/// Returns the place by the initial letter
pub fn region(number: &str) -> Option<&str> {
    if !validate(number) {
        return None;
    }

    let code = &number[0..1];
    if !code.is_empty() {
        if let Some((_, name)) = PREFIX_LETTERS.get(code) {
            Some(*name)
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate() {
        assert_eq!(validate("A123456789"), true);
        assert_eq!(validate("B142610160"), true);
        assert_eq!(validate("Q155304682"), true);
        assert_eq!(validate("Q155304680"), false);
    }

    #[test]
    fn test_get_region() {
        let r = region("B142610160");
        assert_eq!(r, Some("台中市"));
        let r = region("0142610160");
        assert_eq!(r, None);
        let r = region("Q155304680");
        assert_eq!(r, None);
    }

    #[test]
    fn test_get_gender() {
        let g = gender("Q155304682");
        assert_eq!(g, Some(super::super::Gender::Male));
        let g = gender("A225376624");
        assert_eq!(g, Some(super::super::Gender::Female));
        let g = gender("Q155304680");
        assert_eq!(g, None);
    }
}
