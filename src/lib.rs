#[macro_use]
extern crate lazy_static;

use chrono::{Datelike, NaiveDate};
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fmt;

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

#[derive(Debug)]
pub enum Error {
    InvalidNumber,
    UpgradeError,
    GenerateFakeIDError,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidNumber => write!(f, "Invalid Number"),
            Error::UpgradeError => write!(f, "Upgrade Failed"),
            Error::GenerateFakeIDError => write!(f, "Generate Fake ID Error"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Identity {
    number: String,
    valid: bool,
}

impl Identity {
    pub fn new(number: &str) -> Identity {
        let mut id = Identity {
            number: number.trim().to_ascii_uppercase().to_owned(),
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

    pub fn number(&self) -> String {
        self.number.clone()
    }

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

    pub fn date(&self) -> Option<i32> {
        if !self.is_valid() {
            return None;
        }

        if let Ok(date) = self.number[12..14].parse::<i32>() {
            Some(date)
        } else {
            None
        }
    }

    pub fn age(&self) -> Option<u32> {
        if !self.is_valid() {
            return None;
        }

        if let Ok(year) = self.number[6..10].parse::<u32>() {
            let current = chrono::Local::now().year() as u32;
            Some(current - year)
        } else {
            None
        }
    }

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

    pub fn province(&self) -> Option<String> {
        if !self.is_valid() {
            return None;
        }

        let code = &self.number[0..2];
        match PROVINCE_CODE_NAME.get(code) {
            Some(name) => Some(name.to_string()),
            None => None,
        }
    }

    pub fn region(&self) -> Option<String> {
        if !self.is_valid() {
            return None;
        }

        region::query(&self.number[0..6])
    }

    pub fn constellation(&self) -> Option<String> {
        if !self.is_valid() {
            return None;
        }

        let month = match self.month() {
            Some(value) => value,
            None => return None,
        };
        let day = match self.date() {
            Some(value) => value,
            None => return None,
        };

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
        Some(result.to_owned())
    }

    pub fn chinese_era(&self) -> Option<String> {
        if !self.is_valid() {
            return None;
        }

        let year = match self.year() {
            Some(value) => value,
            None => return None,
        };

        let i = (year - 3) % 10;
        let j = (year - 3) % 12;
        let era = format!(
            "{}{}",
            CELESTIAL_STEM[i as usize], TERRESTRIAL_BRANCH[j as usize]
        );
        Some(era)
    }

    pub fn chinese_zodiac(&self) -> Option<String> {
        if !self.is_valid() {
            return None;
        }

        let year = match self.year() {
            Some(value) => value,
            None => return None,
        };

        let end = 3;
        let idx = (year - end) % 12;
        let zod = CHINESE_ZODIAC[idx as usize];
        Some(zod.to_owned())
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }

    pub fn is_empty(&self) -> bool {
        self.number.is_empty()
    }

    pub fn len(&self) -> usize {
        self.number.len()
    }
}

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
        if let Some(code) = get_check_code_18(weight) {
            idv2.push_str(&code);
            Ok(idv2)
        } else {
            return Err(Error::UpgradeError);
        }
    } else {
        Err(Error::UpgradeError)
    }
}

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
        if let Some(code) = get_check_code_18(sum17) {
            if code == code18.to_uppercase() {
                return true;
            }
        }
    }
    false
}

pub fn new_fake(
    region: &str,
    year: i32,
    month: i32,
    date: i32,
    gender: Gender,
) -> Result<String, Error> {
    let mut rng = thread_rng();
    let mut seq = rng.gen_range(0..999);

    if gender == Gender::Male && seq % 2 == 0 {
        seq += 1;
    }
    if gender == Gender::Female && seq % 2 == 1 {
        seq += 1;
    }

    let seg17 = format!("{}{}{:0>2}{:0>2}{:0>3}", region, year, month, date, seq);

    let iarr = match string_to_integer_array(&seg17) {
        Ok(value) => value,
        _ => return Err(Error::GenerateFakeIDError),
    };

    let weight = get_weights_sum(&iarr);
    if let Some(code) = get_check_code_18(weight) {
        Ok(seg17 + &code)
    } else {
        return Err(Error::GenerateFakeIDError);
    }
}

fn is_digital(s: &str) -> bool {
    if s.is_empty() {
        false
    } else {
        s.chars().all(char::is_numeric)
    }
}

fn get_check_code_18(sum: u32) -> Option<String> {
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
    Some(code.to_string())
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
        assert_eq!(id.number(), "632123198209270518".to_string());

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
        print_details(&id);
    }

    #[test]
    fn query_region() {
        let name = region::query("511702").unwrap();
        assert_eq!(&name, "四川省达州市通川区");
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

    #[test]
    fn generate_fake_id() {
        let f = new_fake("654325", 2018, 2, 28, Gender::Male).unwrap();
        let id = Identity::new(&f);
        print_details(&id);
        assert_eq!(id.is_valid(), true);

        let f = new_fake("310104", 2020, 2, 29, Gender::Female).unwrap();
        let id = Identity::new(&f);
        print_details(&id);
        assert_eq!(id.is_valid(), true);

        let f = new_fake("230000", 1970, 2, 29, Gender::Male).unwrap();
        let id = Identity::new(&f);
        print_details(&id);
        assert_eq!(id.is_valid(), false);
    }

    fn print_details(id: &Identity) {
        let detail = vec![
            format!("Number: {:?}", id.number()),
            format!("Age: {:?}", id.age()),
            format!("Year: {:?}", id.year()),
            format!("Month: {:?}", id.month()),
            format!("Date: {:?}", id.date()),
            format!("BirthDate: {:?}", id.birth_date()),
            format!("ChineseEra: {:?}", id.chinese_era()),
            format!("ChineseZodiac: {:?}", id.chinese_zodiac()),
            format!("Constellation: {:?}", id.constellation()),
            format!("Gender: {:?}", id.gender()),
            format!("Province: {:?}", id.province()),
            format!("IsValid: {:?}", id.is_valid()),
            format!("IsEmpty: {:?}", id.is_empty()),
            format!("NumberLength: {:?}", id.len()),
            format!("Region: {:?}", id.region()),
        ];
        println!("#--------- Summary ---------#\n{}\n", detail.join("\n"));
    }
}
