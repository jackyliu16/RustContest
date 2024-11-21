use std::ops::Sub;

pub fn time_info(time: &str) -> String {
    todo!()
}

/// 判断是否为闰年
/// 参照公历规范实现： 闰年是指能被4整除的年份，但如果能被100整除而不能被400整除的年份就不是闰年
pub fn is_leap_year(year: isize) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// 生成距离公元元年的天数
pub fn days_until_year(mut year: isize) -> isize {
    let mut days = 0;
    let mut leap_years = 0;
    if year == 0 { year = -1 } // ISO 8601: 其中0000意爲公元前1年
    if year > 0 { // BC
        days = (year - 1) * 365;
        leap_years = (year - 1) / 4 - (year - 1) / 100 + (year - 1) / 400;
    } else { // AC
        days = year * 365;
        leap_years = (year - 1) / 4 - (year - 1) / 100 + (year - 1) / 400;
    }

    days + leap_years
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

pub fn days_until_months_since_years(year: isize, month: usize) -> usize {
    let mut days = 0;
    for m in 1..month {
        days += how_many_days_of_this_month(year, m);
    }
    days
}

/// 蔡勒公式(Zellers Kongruenz)
/// https://zh.wikipedia.org/wiki/%E8%94%A1%E5%8B%92%E5%85%AC%E5%BC%8F
/// w = \left ( y + \left[\frac{y}{4}\right] + \left[\frac{c}{4}\right] - 2c + \left[\frac{26(m+1)}{10}\right] +d -1 \right ) \bmod 7
pub fn zellers_kongruenz(mut year: isize, mut month: usize, mut day: usize) -> usize {
    if year <= 0 { panic!("Incorrect Data: zellers_kongruenz only support BC") }
    let cnt = 0;

    if month < 3 { // 菜勒要求1-3月被诗作为前一年的13-14月
        month += 12;
        year -= 1;
    };

    let (c, y) = (year as usize / 100, year as usize % 100);
    let week_days = (y + y / 4 + c / 4+ 26 * (month + 1) / 10 + day).sub(2 * c + 1);

    (week_days as usize % 7 + 7) % 7
}
