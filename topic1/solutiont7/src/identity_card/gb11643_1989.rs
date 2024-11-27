use std::error::Error;
use std::fmt::Formatter;
use std::str::FromStr;
use chrono::Datelike;
use crate::identity_card::common::{how_many_days_of_this_month, region_legitimacy_check};
use crate::identity_card::error::ParseError;
use crate::identity_card::GB11643;
use crate::identity_card::gb11643_1999::GB11643_1999;

pub struct GB11643_1989 {
    region: String,   // 6 [0..5]
    birth_day: String,          // 6 [6..12]
    sex: bool,          // Man True
}

impl GB11643 for GB11643_1989 { }

impl std::fmt::Display for GB11643_1989 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sex = if self.sex { "男" } else { "女" };
        write!(f, "{},{},{}", sex, self.birth_day, self.region)
    }
}

impl FromStr for GB11643_1989 {
    type Err =  ParseError;

    fn from_str<'a>(s: &'a str) -> Result<Self, Self::Err> {
        if s.len() != 15 {
            return Err(ParseError::LengthNotCoincide(s.to_string()))
        };

        if ! (s[..s.len() - 1].chars().all(|c| c.is_numeric())
            && s.chars().next_back().map(|c| c.is_numeric() || c == 'X' || c == 'x').unwrap()) {
            return Err(ParseError::InvalidFormat(s.to_string()))
        }

        let city_name =  region_legitimacy_check(&s[..6])?;
        let birth_day = get_birth_day_with_legitimacy_check(&s[6..12])?;
        let sex = if &s[14..].parse().unwrap() % 2 != 0 { true } else { false };

        Ok(Self {
            region: city_name.to_string(),
            birth_day,
            sex,
        })
    }
}


fn get_birth_day_with_legitimacy_check<'a>(birth: &'a str) -> Result<String, ParseError> {
    if birth.len() != 6 { return Err(ParseError::LengthNotCoincide(birth.to_string())); }
    let (year, month, day): (usize, usize, usize) = (
        birth[..2].parse::<usize>().unwrap() + 1900,
        birth[2..4].parse::<usize>().unwrap(),
        birth[4..6].parse::<usize>().unwrap()
    );

    if  (year < 1900 || year >= (chrono::Utc::now().year() + 1) as usize) ||
        (month == 0 || month > 12) ||
        (day >= how_many_days_of_this_month(year as isize, month)) {
        return Err(ParseError::BirthDayFormat(birth.to_string()))
    }

    Ok(format!("{}年{:02}月{:02}日", year, month, day))
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn birth_day_valid() {
        assert_eq!(get_birth_day_with_legitimacy_check(&"820325"), Ok(String::from("1982年03月25日")));
        assert_eq!(get_birth_day_with_legitimacy_check(&"881105"), Ok(String::from("1988年11月05日")))
    }

    #[test]
    fn birth_day_invalid() {
        assert_eq!(get_birth_day_with_legitimacy_check(&"821325"), Err(ParseError::BirthDayFormat(String::from("821325"))));
        assert_eq!(get_birth_day_with_legitimacy_check(&"880229"), Err(ParseError::BirthDayFormat(String::from("880229"))));
        assert_eq!(get_birth_day_with_legitimacy_check(&"200229"), Err(ParseError::BirthDayFormat(String::from("200229"))));
    }
}