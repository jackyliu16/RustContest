/*
从2025年1月1日起，
- 男职工 和 原法定退休年龄为五十五周岁的女职工，
    - 法定退休年龄**每四个月延迟一个月**，分别逐步**延迟至六十三周岁和五十八周岁**；
- 原法定退休年龄为五十周岁的女职工，
    - 法定退休年龄**每二个月延迟一个月**，**逐步延迟至五十五周岁**。
*/
use crate::date::Date;
use crate::rules;
use crate::rules::{CombinedRules, RetirementRules};
use crate::types::PersonnelCategory;

pub fn retire_time(time: &str, tp: &str) -> String {
    // ("1971-04", "原法定退休年龄55周岁女职工", "2026-08,55.33,4"),
    let (year, month) = time.split_once('-').unwrap();
    let types: PersonnelCategory = tp.parse().unwrap();
    let birth_date = Date::new(year.parse().unwrap(), month.parse().unwrap());

    dbg!(&year, &month, &types);

    let retire_rules = CombinedRules { rules: vec![Box::new(rules::Rules20240913), Box::new(rules::Rules1978) ] } ;
    let retire_time = retire_rules.calculate_working_date(&birth_date, &types);

    let original_retire_time = rules::Rules1978.calculate_working_date(&birth_date, &types);
    dbg!(retire_time, original_retire_time);
    let delay_month: usize = (retire_time.unwrap() - (birth_date + original_retire_time.unwrap())).into();
    let retire_age: f32 = (retire_time.unwrap() - birth_date).into();

    dbg!( format_f32(retire_age));
    format!("{},{},{}",
        retire_time.unwrap(),
        format_f32(retire_age),
        delay_month
    )
}

fn format_f32(value: f32) -> String {
    if value.fract() == 0.0 {
        format!("{}", value)
    } else {
        format!("{:.2}", value)
    }
}
