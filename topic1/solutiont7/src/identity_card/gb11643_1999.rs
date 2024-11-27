use std::fmt::Formatter;
use std::str::FromStr;
use chrono::Datelike;
use crate::identity_card::common::{how_many_days_of_this_month, region_legitimacy_check};
use crate::identity_card::error::ParseError;
use crate::identity_card::GB11643;

pub struct GB11643_1999 {
    region: String,
    birth_day: String,
    sex: bool,
}

impl GB11643 for GB11643_1999 { }

impl std::fmt::Display for GB11643_1999 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sex = if self.sex { "男" } else { "女" };
        write!(f, "{},{},{}", sex, self.birth_day, self.region)
    }
}


impl FromStr for GB11643_1999 {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 18 {
            return Err(ParseError::LengthNotCoincide(s.to_string()));
        }

        if ! (s[..s.len() - 1].chars().all(|c| c.is_numeric())
            && s.chars().next_back().map(|c| c.is_numeric() || c == 'X' || c == 'x').unwrap()) {
            return Err(ParseError::InvalidFormat(s.to_string()))
        }

        let city_name =  region_legitimacy_check(&s[..6])?;
        let birth_day = get_birth_day_with_legitimacy_check(&s[6..14])?;
        let sex = if &s[16..17].parse().unwrap() % 2 != 0 { true } else { false };

        if ! validate_checksum(&s) {
           return Err(ParseError::CheckSumInvalid(s.to_string()))
        }

        Ok(Self {
            region: city_name,
            birth_day,
            sex
        })
    }
}

fn get_birth_day_with_legitimacy_check<'a>(birth: &'a str) -> Result<String, ParseError> {
    if birth.len() != 8 { return Err(ParseError::LengthNotCoincide(birth.to_string())); }
    let (year, month, day): (usize, usize, usize) = (
        birth[..4].parse::<usize>().unwrap(),
        birth[4..6].parse::<usize>().unwrap(),
        birth[6..].parse::<usize>().unwrap(),
    );

    if  (year < 1900 || year >= (chrono::Utc::now().year() + 1) as usize) ||
        (month == 0 || month > 12) ||
        (day >= how_many_days_of_this_month(year as isize, month)) {
        return Err(ParseError::BirthDayFormat(birth.to_string()))
    }

    Ok(format!("{}年{:02}月{:02}日", year, month, day))
}

const GB11643_1999_WEIGHTING_FACTOR: &[i32] = &[7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
const GB11643_1999_CHECKSUM: &[char] = &['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];

fn validate_checksum(id: &str) -> bool {
    if id.len() == 15 {
        return true;
    }

    let mut sum = 0;
    for (idx, c) in id.chars().take(18 - 1).enumerate() {
        if let Some(x) = c.to_digit(10) {
            sum += GB11643_1999_WEIGHTING_FACTOR[idx] * x as i32;
        } else {
            return false;
        }
    }

    let checksum = GB11643_1999_CHECKSUM[(sum % 11) as usize];

    id.chars()
        .next_back()
        .unwrap()
        .to_ascii_uppercase()
        == checksum
}

#[cfg(test)]
mod tests { // DATA FROM https://www.cnblogs.com/linus-tan/p/7111797.html
    use super::*;

    #[test]
    fn birth_day_valid() {
        assert_eq!(get_birth_day_with_legitimacy_check(&"19820325"), Ok(String::from("1982年03月25日")));
        assert_eq!(get_birth_day_with_legitimacy_check(&"19881105"), Ok(String::from("1988年11月05日")))
    }
    
    #[test]
    fn birth_day_invalid() {
        assert_eq!(get_birth_day_with_legitimacy_check(&"19821325"), Err(ParseError::BirthDayFormat(String::from("19821325"))));
        assert_eq!(get_birth_day_with_legitimacy_check(&"19880229"), Err(ParseError::BirthDayFormat(String::from("19880229"))));
        assert_eq!(get_birth_day_with_legitimacy_check(&"19200229"), Err(ParseError::BirthDayFormat(String::from("19200229"))));
    }

    #[test]
    fn id_checksum_valid() {
        let vec = vec![
            "371522199402189127",
            "120111199207178301",
            "230713198012022856",
            "45128119860426100X",
            "652701198205147107",
            "33010619930228635X",
            "431023198504075666",
        ];
        for x in vec {
           assert!(validate_checksum(x));
        }
    }
    #[test]
    fn id_checksum_invalid() {
        let vec = vec![
            "371522199402189122",
            "120111199207178308",
            "23071319801202285x",
            "451281198604261009",
            "652701198205147101",
            "330106199302286353",
            "431023198504075664",
        ];
        for x in vec {
            assert!(! validate_checksum(x));
        }
    }
}