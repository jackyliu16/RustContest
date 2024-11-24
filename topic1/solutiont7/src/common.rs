use std::{env, fs};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use serde_json::Value;
use crate::error::ParseError;

// check if region correct
// TODO(OPTIMIZE): use some other ways to remove io operation each time
pub fn region_legitimacy_check<'a>(region: &'a str) -> Result<String, ParseError> {
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let data_path = PathBuf::from(manifest_path).join("region.json");

    let mut file = File::open(data_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data);
    
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
    fn region_check_valid() {
        // 420111 19820325 102 9
        assert_eq!(region_legitimacy_check(&"420111").unwrap(), "湖北省-武汉市-洪山区".to_string());
        assert_eq!(region_legitimacy_check(&"370725").unwrap(), "山东省-潍坊市-昌乐县".to_string());
        assert_eq!(region_legitimacy_check(&"370725").unwrap(), "山东省-潍坊市-昌乐县".to_string());
        assert_eq!(region_legitimacy_check(&"110102").unwrap(), "北京市-市辖区-西城区".to_string());
        assert_eq!(region_legitimacy_check(&"510303").unwrap(), "四川省-自贡市-贡井区".to_string());
        assert_eq!(region_legitimacy_check(&"320106").unwrap(), "江苏省-南京市-鼓楼区".to_string());
        assert_eq!(region_legitimacy_check(&"310104").unwrap(), "上海市-市辖区-徐汇区".to_string());
    }
}