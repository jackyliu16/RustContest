use std::collections::HashMap;
use std::env;
use std::fs;
use std::error::Error;
use std::fmt::Formatter;
use std::str::FromStr;
use chrono::Datelike;
use serde_json::{json, Value};
use crate::error::ParseError;

pub struct GB11643_1989 {
    region: String,   // 6 [0..5]
    birth_day: String,          // 6 [6..12]
    sex: bool,          // Man True
}

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

        dbg!(sex);
        Ok(Self {
            region: city_name.to_string(),
            birth_day,
            sex,
        })
    }
}

// check if region correct
// TODO(OPTIMIZE): use some other ways to remove io operation each time
fn region_legitimacy_check<'a>(region: &'a str) -> Result<String, ParseError> {
    let path = env::current_dir()
        .expect("Cannot access current working directory")
        .join("topic1")
        .join("solutiont7")
        .join("region.json");
    dbg!(&path);

    let data = fs::read_to_string(&path).unwrap();
    let json_data: Value = serde_json::from_str(&data).unwrap();
    let map: HashMap<String, String> = serde_json::from_value(json_data).unwrap();

    if map.contains_key(region) {
        let (y, m, d) = (
            &region[0..2],
            &region[2..4],
            &region[4..6],
        );

        let res: Vec<String> = vec![
            get_value_from_map(format!("{}0000", y), &map)?,
            get_value_from_map(format!("{}{}00", y, m), &map)?,
            get_value_from_map(format!("{}{}{}", y, m, d), &map)?,
        ];

        Ok(res.join("-"))
    } else {
        Err(ParseError::RegionNotFound(region.to_string()))
    }
}

fn get_value_from_map(id: String, map: &HashMap<String, String>) -> Result<String, ParseError> {
    if let Some(v) = map.get(&id) {
        Ok(v.clone())
    } else {
        Err(ParseError::RegionNotFound(id))
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

/// 判断是否为闰年
/// 参照公历规范实现： 闰年是指能被4整除的年份，但如果能被100整除而不能被400整除的年份就不是闰年
pub fn is_leap_year(year: isize) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}


pub fn how_many_days_of_this_month(year: isize, month: usize) -> usize {
    if month == 0 { panic!("Incorrect Format") }
    match month {
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 if ! is_leap_year(year) => 28,
        _ => 31,
    }
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

    #[test]
    fn region_check_valid() {
        // 420111 19820325 102 9
        assert_eq!(region_legitimacy_check(&"420111").unwrap(),"湖北省-武汉市-洪山区".to_string());
        assert_eq!(region_legitimacy_check(&"370725").unwrap(),"山东省-潍坊市-昌乐县".to_string());
        assert_eq!(region_legitimacy_check(&"370725").unwrap(),"山东省-潍坊市-昌乐县".to_string());
        assert_eq!(region_legitimacy_check(&"110102").unwrap(),"北京市-市辖区-西城区".to_string());
        assert_eq!(region_legitimacy_check(&"510303").unwrap(),"四川省-自贡市-贡井区".to_string());
        assert_eq!(region_legitimacy_check(&"320106").unwrap(),"江苏省-南京市-鼓楼区".to_string());
        assert_eq!(region_legitimacy_check(&"310104").unwrap(),"上海市-市辖区-徐汇区".to_string());
    }
}