use regex::Regex;

lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"^[1|5|7][0-9]{6}\(?[0-9A-Z]\)?$").unwrap();
    static ref REMOVAL_PATTERN: Regex = Regex::new(r"[\(|\)]").unwrap();
}

pub fn validate(number: &str) -> bool {
    let number = REMOVAL_PATTERN.replace_all(number, "");
    let number = number.trim().to_ascii_uppercase();
    if number.len() == 8 && PATTERN.is_match(&number) {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_mo() {
        assert_eq!(validate("1123456(A)"), true);
        assert_eq!(validate("7431243(3)"), true);
        assert_eq!(validate("5631279(0)"), true);
        assert_eq!(validate("2000148(3)"), false);
        assert_eq!(validate("5215299A"), true);
    }
}
