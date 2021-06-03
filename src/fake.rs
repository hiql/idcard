//! Utilities for generating fake ID numbers

use crate::{get_check_code, get_weights_sum, region, string_to_integer_array, Error, Gender};
use chrono::prelude::*;
use chrono::{Datelike, Duration, Local, NaiveDate};
use rand::{thread_rng, Rng};

/// Generates a new fake ID number.
pub fn new(
    region: &str,
    year: i32,
    month: i32,
    date: i32,
    gender: Gender,
) -> Result<String, Error> {
    if region.len() != 6 {
        return Err(Error::GenerateFakeIDError(
            "The length of region code must be 6 digits".to_string(),
        ));
    }

    let mut rng = thread_rng();
    let mut seq = rng.gen_range(0..999);

    if gender == Gender::Male && seq % 2 == 0 {
        seq += 1;
    }
    if gender == Gender::Female && seq % 2 == 1 {
        seq += 1;
    }

    let birthdate_str = format!("{}{:0>2}{:0>2}", year, month, date);
    let birth_date = NaiveDate::parse_from_str(&birthdate_str, "%Y%m%d");
    if birth_date.is_err() {
        return Err(Error::GenerateFakeIDError(
            "Invalid date of birth".to_string(),
        ));
    }

    let seg17 = format!("{}{}{:0>3}", region, birthdate_str, seq);

    let iarr = match string_to_integer_array(&seg17) {
        Ok(value) => value,
        _ => return Err(Error::GenerateFakeIDError("Invalid characters".to_string())),
    };

    let weight = get_weights_sum(&iarr);
    if let Some(code) = get_check_code(weight) {
        Ok(seg17 + code)
    } else {
        return Err(Error::GenerateFakeIDError("Invalid check code".to_string()));
    }
}

/// Options which can be used to configure how a fake ID number is generated.
#[derive(Debug, Default, Clone)]
pub struct FakeOptions {
    region: Option<String>,
    min_year: Option<i32>,
    max_year: Option<i32>,
    gender: Option<Gender>,
}

impl FakeOptions {
    /// Creates a blank new set of options ready for configuration.
    pub fn new() -> Self {
        FakeOptions::default()
    }

    /// Sets the minimum year(min_year <= max_year <= current).
    pub fn min_year(mut self, year: i32) -> Self {
        self.min_year = Some(year);
        self
    }

    /// Sets the maximum year(min_year <= max_year <= current).
    pub fn max_year(mut self, year: i32) -> Self {
        self.max_year = Some(year);
        self
    }

    /// Sets the region code, the length must be 2..6.
    pub fn region(mut self, code: &str) -> Self {
        self.region = Some(code.to_owned());
        self
    }

    /// Sets the gender.
    pub fn gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);
        self
    }
}

/// Generates a random fake ID number.
pub fn rand() -> Result<String, Error> {
    let option = FakeOptions::new();
    rand_with_opts(&option)
}

/// Generates a random fake ID number using the given options.
pub fn rand_with_opts(opts: &FakeOptions) -> Result<String, Error> {
    let region_code = if let Some(reg) = &opts.region {
        match region::rand_code_starts_with(&reg) {
            Some(code) => code,
            _ => {
                return Err(Error::GenerateFakeIDError(
                    "Invalid region code".to_string(),
                ))
            }
        }
    } else {
        region::rand_code()
    };

    let mut rng = thread_rng();
    let now = Local::now();

    if let Some(value) = opts.max_year {
        if value > now.year() {
            return Err(Error::GenerateFakeIDError(format!(
                "Max year must be less than or equal to {}",
                now.year()
            )));
        }
    }

    if let Some(value) = opts.min_year {
        if value > now.year() {
            return Err(Error::GenerateFakeIDError(format!(
                "Min year must be less than or equal to {}",
                now.year()
            )));
        }
    }

    if opts.min_year.is_some() && opts.max_year.is_some() {
        let min = opts.min_year.unwrap();
        let max = opts.max_year.unwrap();
        if max < min {
            return Err(Error::GenerateFakeIDError(
                "Max year must be greater than or equal to min year".to_string(),
            ));
        }
    }

    let min_age = if let Some(y) = opts.max_year {
        now.year() - y
    } else {
        0
    };
    let max_age = if let Some(y) = opts.min_year {
        now.year() - y
    } else {
        100
    };

    let age = if max_age == min_age {
        max_age
    } else {
        rng.gen_range(min_age..=max_age)
    };

    let days = rng.gen_range(1..365);
    let dt = Local.ymd(now.year(), 1, 1);
    let birth = dt - Duration::days((age * 365 - days) as i64);
    let gender = if let Some(value) = &opts.gender {
        match value {
            Gender::Male => Gender::Male,
            Gender::Female => Gender::Female,
        }
    } else {
        let flag = rng.gen_range(0..10);
        if flag % 2 == 0 {
            Gender::Male
        } else {
            Gender::Female
        }
    };
    new(
        &region_code,
        birth.year(),
        birth.month() as i32,
        birth.day() as i32,
        gender,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Identity;

    #[test]
    fn generate_fake_id() {
        let f = new("654325", 2018, 2, 28, Gender::Male).unwrap();
        let id = Identity::new(&f);
        println!("{}", id.to_json_string(true));
        assert_eq!(id.is_valid(), true);

        let f = new("310104", 2020, 2, 29, Gender::Female).unwrap();
        let id = Identity::new(&f);
        println!("{}", id.to_json_string(true));
        assert_eq!(id.is_valid(), true);

        for i in 1..=10 {
            let num = rand().unwrap();
            println!("{}: {}", i, num);
        }

        let f = new("2300", 1970, 2, 28, Gender::Male);
        assert_eq!(f.is_err(), true);

        let f = new("230000", 1970, 2, 29, Gender::Male);
        assert_eq!(f.is_err(), true);
    }

    #[test]
    fn generate_fake_id_with_options() {
        let opts = FakeOptions::new()
            .region("3301")
            .min_year(1990)
            .max_year(2000)
            .gender(Gender::Female);

        for i in 1..=5 {
            let num = rand_with_opts(&opts).unwrap();
            println!("{}: {:}", i, num);
        }

        let opts = opts
            .clone()
            .region("11")
            .max_year(1990)
            .gender(Gender::Male);

        for i in 1..=5 {
            let num = rand_with_opts(&opts).unwrap();
            println!("{}: {:}", i, num);
        }
    }
}
