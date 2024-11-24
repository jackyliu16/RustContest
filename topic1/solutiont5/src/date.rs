use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Date {
    pub year: isize,
    pub month: isize,
}
impl Date {

    /// 用于直接输入年份
    pub fn new(year: isize, month: isize) -> Self {
        Self { year, month }
    }

    /// 用于输入一段数据
    pub fn new_abs(year: isize, month: isize) -> Self {
        Self { year, month }
    }

    pub fn add_month(&self, month: isize) -> Date {
        let mut year = self.year;
        let mut month = self.month + month;

        if month > 12 {
            year += (month - 1) / 12;
            month += (month - 1) % 12 + 1;
        }

        if month > 12 { panic!("Date Error"); }

        Date {
            year,
            month: month
        }
    }

    pub fn to_month(&self) -> usize {
        (self.year * 12 + self.month) as usize - 1
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:02}", self.year, self.month)
    }
}

impl FromStr for Date {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('-');
        let year = iter.next().unwrap().parse::<isize>()?;
        let month = iter.next().unwrap().parse::<isize>()?;
        Ok(Self { year, month })
    }
}

impl std::ops::Add for Date {
    type Output = Date;

    fn add(self, rhs: Self) -> Self::Output {
        if rhs.year < 0 || rhs.month < 0 { panic!("Input Data overflow")}
        let mut year = self.year + rhs.year;
        let mut month = self.month + rhs.month;
        if month > 12 {
            year += (month - 1) / 12;
            month = (month - 1) % 12 + 1;
        }
        Self { year, month }
    }
}

impl std::ops::Sub for Date {
    type Output = Date;

    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.year < 0 || rhs.month < 0 { panic!("Input Data overflow")}
        let mut year = self.year - rhs.year;
        let mut month = self.month - rhs.month;
        while month < 0 {
            year -= 1;
            month += 12;
        }
        Self { year, month }
    }
}
