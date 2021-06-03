//!  Chinese Indentity Card Utilities
//!
//! This package provides utilities to validate ID number, to extract detailed
//! ID information, to upgrade ID number from 15-digit to 18-digit, to generate
//! fake ID number, and some related functions for HongKong/Macau/Taiwan ID.
//!
//! # Examples
//!
//! ```
//! use idcard::{Identity, fake, Gender};
//!
//! let id = Identity::new("632123820927051");
//!
//! // Determines whether the ID number is valid.
//! assert_eq!(id.is_valid(), true);
//!
//! // Gets properties
//! let number = id.number();
//! let gender = id.gender();
//! let age = id.age();
//! let birth_date = id.birth_date();
//! let region = id.region();
//! // and so on...
//!
//! // Converts the value to a JSON string.
//! println!("{}", id.to_json_string(true));
//!
//! // Upgrades an ID number from 15-digit to 18-digit.
//! let id18 = idcard::upgrade("310112850409522").unwrap();
//! assert_eq!(&id18, "310112198504095227");
//!
//! // Validates an ID number.
//! assert_eq!(idcard::validate("230127197908177456"), true);
//!
//! // Generates a random fake ID number using the given options.
//! let opts = fake::FakeOptions::new()
//!     .region("3301")
//!     .min_year(1990)
//!     .max_year(2000)
//!     .gender(Gender::Female);
//! match fake::rand_with_opts(&opts) {
//!     Ok(num) => println!("{}", num),
//!     Err(e) => println!("{}", e),
//! }
//! ```
//! For more information ,please read the API documentation.
//!

#[macro_use]
extern crate lazy_static;

use chrono::{Datelike, Local, NaiveDate};
use std::collections::HashMap;
use std::fmt;

pub mod fake;
pub mod hk;
pub mod mo;
pub mod region;
pub mod tw;

const ID_V1_LEN: usize = 15;
const ID_V2_LEN: usize = 18;

static CHINESE_ZODIAC: [&'static str; 12] = [
    "猪", "鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗",
];

static CELESTIAL_STEM: [&'static str; 10] =
    ["癸", "甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "任"];

static TERRESTRIAL_BRANCH: [&'static str; 12] = [
    "亥", "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌",
];

lazy_static! {
    static ref PROVINCE_CODE_NAME: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("11", "北京");
        map.insert("12", "天津");
        map.insert("13", "河北");
        map.insert("14", "山西");
        map.insert("15", "内蒙古");
        map.insert("21", "辽宁");
        map.insert("22", "吉林");
        map.insert("23", "黑龙江");
        map.insert("31", "上海");
        map.insert("32", "江苏");
        map.insert("33", "浙江");
        map.insert("34", "安徽");
        map.insert("35", "福建");
        map.insert("36", "江西");
        map.insert("37", "山东");
        map.insert("41", "河南");
        map.insert("42", "湖北");
        map.insert("43", "湖南");
        map.insert("44", "广东");
        map.insert("45", "广西");
        map.insert("46", "海南");
        map.insert("50", "重庆");
        map.insert("51", "四川");
        map.insert("52", "贵州");
        map.insert("53", "云南");
        map.insert("54", "西藏");
        map.insert("61", "陕西");
        map.insert("62", "甘肃");
        map.insert("63", "青海");
        map.insert("64", "宁夏");
        map.insert("65", "新疆");
        map.insert("71", "台湾");
        map.insert("81", "香港");
        map.insert("82", "澳门");
        map.insert("83", "台湾");
        map.insert("91", "国外");
        map
    };
}

/// Custom error type.
#[derive(Debug)]
pub enum Error {
    InvalidNumber,
    UpgradeError,
    GenerateFakeIDError(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidNumber => write!(f, "Invalid Number"),
            Error::UpgradeError => write!(f, "Upgrade Failed"),
            Error::GenerateFakeIDError(msg) => write!(f, "Generate Fake ID Error: {}", msg),
        }
    }
}

/// The type of demographic genders
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Gender {
    Male,
    Female,
}

/// An object representation of the Chinese ID.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identity {
    number: String,
    valid: bool,
}

impl Identity {
    /// Creates an identity object from given number.
    pub fn new(number: &str) -> Self {
        let mut id = Identity {
            number: number.trim().to_ascii_uppercase(),
            valid: false,
        };
        if id.number.len() == ID_V1_LEN {
            match upgrade(&id.number) {
                Ok(value) => {
                    id.number = value;
                    id.valid = true;
                }
                _ => id.valid = false,
            }
        } else if id.number.len() == ID_V2_LEN {
            id.valid = if validate_v2(&id.number) { true } else { false }
        } else {
            id.valid = false;
        }
        id
    }

    /// Returns the ID number.
    pub fn number(&self) -> &str {
        &self.number
    }

    /// Returns the formatted date of birth(yyyy-mm-dd).
    pub fn birth_date(&self) -> Option<String> {
        if !self.is_valid() {
            return None;
        }

        let birth = &self.number[6..14];
        let year = &birth[0..4];
        let month = &birth[4..6];
        let date = &birth[6..8];
        Some(format!("{}-{}-{}", year, month, date))
    }

    /// Returns the year of birth.
    pub fn year(&self) -> Option<i32> {
        if !self.is_valid() {
            return None;
        }

        if let Ok(year) = self.number[6..10].parse::<i32>() {
            Some(year)
        } else {
            None
        }
    }

    /// Returns the month of birth.
    pub fn month(&self) -> Option<i32> {
        if !self.is_valid() {
            return None;
        }

        if let Ok(month) = self.number[10..12].parse::<i32>() {
            Some(month)
        } else {
            None
        }
    }

    /// Returns the day in the month of the birth.
    pub fn day(&self) -> Option<i32> {
        if !self.is_valid() {
            return None;
        }

        if let Ok(day) = self.number[12..14].parse::<i32>() {
            Some(day)
        } else {
            None
        }
    }

    /// Calculates the current age based on the computer's local date,
    /// if the birth year is less than the local's, it returns `None`.
    pub fn age(&self) -> Option<u32> {
        if !self.is_valid() {
            return None;
        }

        if let Ok(year) = self.number[6..10].parse::<u32>() {
            let current = Local::now().year() as u32;
            if current < year {
                return None;
            }
            Some(current - year)
        } else {
            None
        }
    }

    /// Calculates the age based on the given year, if the given year is less
    /// than the birth year, it returns `None`.
    pub fn age_in_year(&self, year: u32) -> Option<u32> {
        if !self.is_valid() {
            return None;
        }

        if let Ok(value) = self.number[6..10].parse::<u32>() {
            if year < value {
                return None;
            }
            Some(year - value)
        } else {
            None
        }
    }

    /// Returns the gender.
    pub fn gender(&self) -> Option<Gender> {
        if !self.is_valid() {
            return None;
        }

        if let Ok(code) = self.number[16..17].parse::<i32>() {
            if code % 2 != 0 {
                Some(Gender::Male)
            } else {
                Some(Gender::Female)
            }
        } else {
            None
        }
    }

    /// Returns the province name based on the first 2 digits of the number
    pub fn province(&self) -> Option<&str> {
        if !self.is_valid() {
            return None;
        }

        let code = &self.number[0..2];
        match PROVINCE_CODE_NAME.get(code) {
            Some(name) => Some(*name),
            None => None,
        }
    }

    /// Returns the region name based on the first 6 digits of the number
    pub fn region(&self) -> Option<&str> {
        if !self.is_valid() {
            return None;
        }

        region::query(&self.number[0..6])
    }

    /// Returns the region code(the first 6 digits)
    pub fn region_code(&self) -> Option<&str> {
        if !self.is_valid() {
            return None;
        }

        Some(&self.number[0..6])
    }

    /// Returns the constellation by the date of birth.
    pub fn constellation(&self) -> Option<&str> {
        if !self.is_valid() {
            return None;
        }

        let month = match self.month() {
            Some(value) => value,
            None => return None,
        };
        let day = match self.day() {
            Some(value) => value,
            None => return None,
        };

        constellation(month as u32, day as u32)
    }

    /// Returns the Chinese Era by the year of birth.
    pub fn chinese_era(&self) -> Option<String> {
        if !self.is_valid() {
            return None;
        }

        let year = match self.year() {
            Some(value) => value,
            None => return None,
        };

        chinese_era(year as u32)
    }

    /// Returns the Chinese Zodiac animal by the year of birth.
    pub fn chinese_zodiac(&self) -> Option<&str> {
        if !self.is_valid() {
            return None;
        }

        let year = match self.year() {
            Some(value) => value,
            None => return None,
        };

        chinese_zodiac(year as u32)
    }

    /// Checks if the number is valid.
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Checks if the number is empty.
    pub fn is_empty(&self) -> bool {
        self.number.is_empty()
    }

    /// Returns the length of the number.
    pub fn len(&self) -> usize {
        self.number.len()
    }

    /// Converts the value to a JSON string.
    pub fn to_json_string(&self, pretty: bool) -> String {
        let indent = if pretty { "    " } else { "" };
        let space = if pretty { " " } else { "" };
        let props = if self.is_valid() {
            vec![
                format!(r#"{}"number":{}{:?}"#, indent, space, self.number()),
                format!(
                    r#"{}"gender":{}"{:?}""#,
                    indent,
                    space,
                    self.gender().unwrap()
                ),
                format!(
                    r#"{}"birthDate":{}{:?}"#,
                    indent,
                    space,
                    self.birth_date().unwrap()
                ),
                format!(r#"{}"year":{}{:?}"#, indent, space, self.year().unwrap()),
                format!(r#"{}"month":{}{:?}"#, indent, space, self.month().unwrap()),
                format!(r#"{}"day":{}{:?}"#, indent, space, self.day().unwrap()),
                format!(r#"{}"age":{}{:?}"#, indent, space, self.age().unwrap()),
                format!(
                    r#"{}"province":{}{:?}"#,
                    indent,
                    space,
                    self.province().unwrap()
                ),
                format!(
                    r#"{}"region":{}{:?}"#,
                    indent,
                    space,
                    self.region().unwrap()
                ),
                format!(
                    r#"{}"regionCode":{}{:?}"#,
                    indent,
                    space,
                    self.region_code().unwrap()
                ),
                format!(
                    r#"{}"chineseEra":{}{:?}"#,
                    indent,
                    space,
                    self.chinese_era().unwrap()
                ),
                format!(
                    r#"{}"chineseZodiac":{}{:?}"#,
                    indent,
                    space,
                    self.chinese_zodiac().unwrap()
                ),
                format!(
                    r#"{}"constellation":{}{:?}"#,
                    indent,
                    space,
                    self.constellation().unwrap()
                ),
                format!(r#"{}"isValid":{}{:?}"#, indent, space, self.is_valid()),
            ]
        } else {
            vec![
                format!(r#"{}"number":{}{:?}"#, indent, space, self.number()),
                format!(r#"{}"isValid":{}{:?}"#, indent, space, self.is_valid()),
            ]
        };

        if pretty {
            let s = props.join(",\n");
            format!("{{\n{}\n}}", s)
        } else {
            let s = props.join(",");
            format!("{{{}}}", s)
        }
    }
}

/// Returns the Chinese Zodiac animal by the given year, the given year
/// should not be less than 1000.
pub fn chinese_zodiac(year: u32) -> Option<&'static str> {
    if year < 1000 {
        return None;
    }
    let end = 3;
    let idx = (year - end) % 12;
    let zod = CHINESE_ZODIAC[idx as usize];
    Some(zod)
}

/// Returns the Chinese Era by the given year, the given year
/// should not be less than 1000.
pub fn chinese_era(year: u32) -> Option<String> {
    if year < 1000 {
        return None;
    }
    let i = (year - 3) % 10;
    let j = (year - 3) % 12;
    let era = format!(
        "{}{}",
        CELESTIAL_STEM[i as usize], TERRESTRIAL_BRANCH[j as usize]
    );
    Some(era)
}

/// Returns the constellation by the given month and day.
pub fn constellation(month: u32, day: u32) -> Option<&'static str> {
    let result = if (month == 1 && day >= 20) || (month == 2 && day <= 18) {
        "水瓶座"
    } else if (month == 2 && day >= 19) || (month == 3 && day <= 20) {
        "双鱼座"
    } else if (month == 3 && day > 20) || (month == 4 && day <= 19) {
        "白羊座"
    } else if (month == 4 && day >= 20) || (month == 5 && day <= 20) {
        "金牛座"
    } else if (month == 5 && day >= 21) || (month == 6 && day <= 21) {
        "双子座"
    } else if (month == 6 && day > 21) || (month == 7 && day <= 22) {
        "巨蟹座"
    } else if (month == 7 && day > 22) || (month == 8 && day <= 22) {
        "狮子座"
    } else if (month == 8 && day >= 23) || (month == 9 && day <= 22) {
        "处女座"
    } else if (month == 9 && day >= 23) || (month == 10 && day <= 23) {
        "天秤座"
    } else if (month == 10 && day > 23) || (month == 11 && day <= 22) {
        "天蝎座"
    } else if (month == 11 && day > 22) || (month == 12 && day <= 21) {
        "射手座"
    } else if (month == 12 && day > 21) || (month == 1 && day <= 19) {
        "魔羯座"
    } else {
        return None;
    };
    Some(result)
}

/// Upgrades a Chinese ID number from 15-digit to 18-digit.
pub fn upgrade(number: &str) -> Result<String, Error> {
    let number = number.trim().to_ascii_uppercase();
    if number.len() == ID_V1_LEN && is_digital(&number) {
        let mut idv2 = String::new();

        let birthday = "19".to_owned() + &number[6..12];
        let birth_date = NaiveDate::parse_from_str(&birthday, "%Y%m%d");

        let cal = match birth_date {
            Ok(value) => value,
            _ => return Err(Error::UpgradeError),
        };

        idv2.push_str(&number[0..6]);
        idv2.push_str(&cal.year().to_string());
        idv2.push_str(&number[8..]);

        let iarr = match string_to_integer_array(&idv2) {
            Ok(value) => value,
            _ => return Err(Error::UpgradeError),
        };

        let weight = get_weights_sum(&iarr);
        if let Some(code) = get_check_code(weight) {
            idv2.push_str(code);
            Ok(idv2)
        } else {
            return Err(Error::UpgradeError);
        }
    } else {
        Err(Error::InvalidNumber)
    }
}

/// Validates a Chinese ID number(only supports 15/18-digit).
pub fn validate(number: &str) -> bool {
    let number = number.trim().to_ascii_uppercase();
    if number.len() == ID_V1_LEN {
        validate_v1(&number)
    } else if number.len() == ID_V2_LEN {
        validate_v2(&number)
    } else {
        false
    }
}

fn validate_v1(number: &str) -> bool {
    if number.len() == ID_V1_LEN && is_digital(number) {
        let code = &number[0..2];
        if !PROVINCE_CODE_NAME.contains_key(code) {
            return false;
        }

        let birthday = "19".to_owned() + &number[6..12];
        let birth_date = NaiveDate::parse_from_str(&birthday, "%Y%m%d");
        birth_date.is_ok()
    } else {
        false
    }
}

fn validate_v2(number: &str) -> bool {
    if number.len() != ID_V2_LEN {
        return false;
    }

    let birth_date = NaiveDate::parse_from_str(&number[6..14], "%Y%m%d");
    if !birth_date.is_ok() {
        return false;
    }

    let code17 = &number[0..17];
    let code18 = &number[17..18];
    if is_digital(code17) {
        let iarr = match string_to_integer_array(code17) {
            Ok(value) => value,
            _ => return false,
        };

        let sum17 = get_weights_sum(&iarr);
        if let Some(code) = get_check_code(sum17) {
            if code == code18.to_uppercase() {
                return true;
            }
        }
    }
    false
}

fn is_digital(s: &str) -> bool {
    if s.is_empty() {
        false
    } else {
        s.chars().all(char::is_numeric)
    }
}

fn get_check_code(sum: u32) -> Option<&'static str> {
    let code = match sum % 11 {
        10 => "2",
        9 => "3",
        8 => "4",
        7 => "5",
        6 => "6",
        5 => "7",
        4 => "8",
        3 => "9",
        2 => "X",
        1 => "0",
        0 => "1",
        _ => return None,
    };
    Some(code)
}

fn string_to_integer_array(s: &str) -> Result<Vec<u32>, Error> {
    let mut v: Vec<u32> = Vec::new();
    for ch in s.chars() {
        match ch.to_digit(10) {
            Some(i) => v.push(i as u32),
            None => return Err(Error::InvalidNumber),
        }
    }
    Ok(v)
}

fn get_weights_sum(arr: &[u32]) -> u32 {
    let weights = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    let mut sum = 0;
    if weights.len() == arr.len() {
        for i in 0..arr.len() {
            for j in 0..weights.len() {
                if i == j {
                    sum = sum + arr[i] * weights[j];
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upgrade_v1_to_v2() {
        let id = Identity::new("632123820927051");
        assert_eq!(id.is_valid(), true);
        assert_eq!(id.number(), "632123198209270518");

        let id = upgrade("310112850409522").unwrap();
        assert_eq!(&id, "310112198504095227");
    }

    #[test]
    fn validate_v1_and_v2() {
        assert_eq!(validate("511702800222130"), true);
        assert_eq!(validate("230127197908177456"), true);
    }

    #[test]
    fn show_details() {
        let id = Identity::new("511702800222130");
        println!("{}", id.to_json_string(true));

        let id = Identity::new("51170280022213X");
        println!("{}", id.to_json_string(false));
    }

    #[test]
    fn calc_age() {
        let id = Identity::new("511702800222130");
        assert_eq!(id.age(), Some(41));
        assert_eq!(id.age_in_year(2020), Some(40));
        assert_eq!(id.age_in_year(1980), Some(0));
        assert_eq!(id.age_in_year(1900), None);
    }

    #[test]
    fn test_some_utilities() {
        assert_eq!(chinese_zodiac(1000), Some("鼠"));
        assert_eq!(chinese_zodiac(1900), Some("鼠"));
        assert_eq!(chinese_zodiac(2021), Some("牛"));

        assert_eq!(chinese_era(1000), Some("庚子".to_string()));
        assert_eq!(chinese_era(1900), Some("庚子".to_string()));
        assert_eq!(chinese_era(2021), Some("辛丑".to_string()));

        assert_eq!(constellation(10, 25), Some("天蝎座"));
        assert_eq!(constellation(2, 29), Some("双鱼座"));
        assert_eq!(constellation(0, 32), None);
    }

    #[test]
    fn query_region() {
        let name = region::query("511702").unwrap();
        assert_eq!(name, "四川省达州市通川区");
    }

    #[test]
    fn compare() {
        let a = Identity::new("632123820927051");
        let b = Identity::new("632123198209270518");
        assert_eq!(a == b, true);

        let a = Identity::new("21021119810503545X");
        let b = Identity::new("21021119810503545x");
        assert_eq!(a == b, true);

        let a = Identity::new("330421197402080974");
        let b = Identity::new("130133197909136078");
        assert_eq!(a != b, true);
    }
}
