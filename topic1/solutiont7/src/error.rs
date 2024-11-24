use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Empty,                      // 输入为空
    InvalidFormat(String),      // 错误的格式 ？
    LengthNotCoincide(String),          // 与数量限制不一致
    RegionNotFound(String),     // 未找到地区
    BirthDayFormat(String), // 生日格式不正确
    CheckSumInvalid(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Empty => write!(f, "empty input"),
            ParseError::InvalidFormat(msg) => write!(f, "invalid format: {}", msg),
            ParseError::LengthNotCoincide(msg) => write!(f, "length not coincide: {}", msg),
            ParseError::RegionNotFound(region) => write!(f, "region not found: {}", region),
            ParseError::BirthDayFormat(birth_day) => write!(f, "birth day format incorrect: {}", birth_day),
            ParseError::CheckSumInvalid(check_sum) => write!(f, "invalid checksum: {}", check_sum),
        }
    }
}

impl std::error::Error for ParseError { }
