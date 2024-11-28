/*
从2025年1月1日起，
- 男职工 和 原法定退休年龄为五十五周岁的女职工，
    - 法定退休年龄**每四个月延迟一个月**，分别逐步**延迟至六十三周岁和五十八周岁**；
- 原法定退休年龄为五十周岁的女职工，
    - 法定退休年龄**每二个月延迟一个月**，**逐步延迟至五十五周岁**。
*/
use crate::retirement::date::Date;
use crate::retirement::rules::{CombinedRules, RetirementRules};
use crate::retirement::types::PersonnelCategory;

mod date;
mod types;
mod rules;

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

// #[cfg(test)]
// mod tests {
//     use crate::retirement::date::Date;
//     use crate::retirement::rules;
//     use crate::retirement::rules::{CombinedRules, RetirementRules};
//     use crate::retirement::types::PersonnelCategory;
//
//     #[deprecated]
//     fn working_date_curr() {
//         let calculator = CombinedRules { rules: vec![ Box::new(rules::Rules1978) ] };
//         let date = Date::new(2002, 04);
//         assert_eq!(Some(Date::new_abs(60, 0)), calculator.calculate_working_date(&date, &PersonnelCategory::Man));
//         assert_eq!(Some(Date::new_abs(55, 0)), calculator.calculate_working_date(&date, &PersonnelCategory::FemaleCadres));
//         assert_eq!(Some(Date::new_abs(50, 0)), calculator.calculate_working_date(&date, &PersonnelCategory::FemaleWorkers));
//     }
//
//     #[deprecated]
//     fn working_date_2024_right() {
//         let date = Date::new(2002, 04);
//         assert_eq!(Some(Date::new(3, 1)), rules::Rules20240913.calculate_working_date(&date, &PersonnelCategory::Man));
//         assert_eq!(Some(Date::new(5, 1)), rules::Rules20240913.calculate_working_date(&date, &PersonnelCategory::FemaleWorkers));
//         assert_eq!(Some(Date::new(3, 1)), rules::Rules20240913.calculate_working_date(&date, &PersonnelCategory::FemaleCadres));
//     }
//     #[test]
//     fn working_date_2024_left() {
//         let date = Date::new(1960, 04);
//         assert_eq!(None, rules::Rules20240913.calculate_working_date(&date, &PersonnelCategory::Man));
//         assert_eq!(None, rules::Rules20240913.calculate_working_date(&date, &PersonnelCategory::FemaleWorkers));
//         assert_eq!(None, rules::Rules20240913.calculate_working_date(&date, &PersonnelCategory::FemaleCadres));
//     }
//
//     #[test]
//     fn working_date_2024_mid_man() {
//         assert_eq!(Some(Date::new_abs(0, 4)), rules::Rules20240913.calculate_working_date(&Date::new(1966,  1), &PersonnelCategory::Man));
//         assert_eq!(Some(Date::new_abs(0, 4)), rules::Rules20240913.calculate_working_date(&Date::new(1966,  2), &PersonnelCategory::Man));
//         assert_eq!(Some(Date::new_abs(0, 4)), rules::Rules20240913.calculate_working_date(&Date::new(1966,  3), &PersonnelCategory::Man));
//         assert_eq!(Some(Date::new_abs(0, 4)), rules::Rules20240913.calculate_working_date(&Date::new(1966,  4), &PersonnelCategory::Man));
//     }
//     #[test]
//     fn working_date_mix_mid_man() {
//         let calculator = CombinedRules { rules: vec![ Box::new(rules::Rules20240913), Box::new(rules::Rules1978) ] };
//         assert_eq!(Some(Date::new(2026, 5)), calculator.calculate_working_date(&Date::new(1966,  1), &PersonnelCategory::Man));
//         assert_eq!(Some(Date::new(2026, 6)), calculator.calculate_working_date(&Date::new(1966,  2), &PersonnelCategory::Man));
//         assert_eq!(Some(Date::new(2026, 7)), calculator.calculate_working_date(&Date::new(1966,  3), &PersonnelCategory::Man));
//         assert_eq!(Some(Date::new(2026, 8)), calculator.calculate_working_date(&Date::new(1966,  4), &PersonnelCategory::Man));
//     }
//
//     #[test]
//     fn test_testcases() {
//         let calculator = CombinedRules { rules: vec![ Box::new(rules::Rules20240913), Box::new(rules::Rules1978) ] };
//         assert_eq!(calculator.calculate_working_date(&Date::new(1971,04), &PersonnelCategory::FemaleCadres), Some(Date::new(2026,08)));
//         assert_eq!(calculator.calculate_working_date(&Date::new(1995,12), &PersonnelCategory::FemaleWorkers), Some(Date::new(2050,12)));
//         assert_eq!(calculator.calculate_working_date(&Date::new(2000,12), &PersonnelCategory::FemaleCadres), Some(Date::new(2058,12)));
//         assert_eq!(calculator.calculate_working_date(&Date::new(1963,04), &PersonnelCategory::FemaleCadres), Some(Date::new(2018,04)));
//         assert_eq!(calculator.calculate_working_date(&Date::new(1995,12), &PersonnelCategory::Man), Some(Date::new(2058,12)));
//         assert_eq!(calculator.calculate_working_date(&Date::new(2000,12), &PersonnelCategory::Man), Some(Date::new(2063,12)));
//         assert_eq!(calculator.calculate_working_date(&Date::new(1965,12), &PersonnelCategory::Man), Some(Date::new(2026,03)));
//         assert_eq!(calculator.calculate_working_date(&Date::new(1963,12), &PersonnelCategory::Man), Some(Date::new(2023,12)));
//         assert_eq!(calculator.calculate_working_date(&Date::new(1964,02), &PersonnelCategory::Man), Some(Date::new(2024,02)));
//         assert_eq!(calculator.calculate_working_date(&Date::new(1965,01), &PersonnelCategory::Man), Some(Date::new(2025,02)));
//     }
//
//     fn test_for_date_convert() {
//         let date1 = Date::new(1971, 4);
//         let date2 = Date::new(2026, 8);
//         let retire_age_1: f32 = (date2 - date1).into();
//         // assert_eq!(retire_age_1, 55.33);
//     }
// }