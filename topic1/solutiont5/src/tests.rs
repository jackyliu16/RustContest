// src/tests.rs
mod retirement;
mod types;
mod rules;
mod date;

#[cfg(test)]
mod tests {
    use super::retirement::retire_time;
    use super::date::Date;
    use std::time::{Instant, Duration};
    use crate::rules;
    use crate::rules::{CombinedRules, RetirementRules};
    use crate::types::PersonnelCategory;

    // 定义测试用例和预期结果
    const TEST_CASES: &[(&str, &str, &str)] = &[
        ("1971-04", "原法定退休年龄55周岁女职工", "2026-08,55.33,4"),
        ("1965-12", "男职工", "2026-03,60.25,3"),
        ("1965-01", "男职工", "2025-02,60.08,1"),
        ("1995-12", "原法定退休年龄50周岁女职工", "2050-12,55,60"),
        ("1995-12", "男职工", "2058-12,63,36"),
        ("2000-12", "原法定退休年龄55周岁女职工", "2058-12,58,36"),
        ("2000-12", "男职工", "2063-12,63,36"),
        ("1963-12", "男职工", "2023-12,60,0"),
        ("1963-04", "原法定退休年龄55周岁女职工", "2018-04,55,0"),
        ("1964-02", "男职工", "2024-02,60,0"),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_retirement_time() {
        let mut total_score = 0.0;
        for (time, tp, expected) in TEST_CASES {
            let start = Instant::now();
            let result = retire_time(*time, *tp);
            let duration = start.elapsed();

            // 时间超0.2s，判定不合格
            if duration <= Duration::from_millis(200) && result == *expected {
                total_score += 10.0;
            }
        }
        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }

    #[test]
    fn date_add() {
        assert_eq!(
            Date::new(2002, 4) + Date::new(60, 3),
            Date::new(2062, 7)
        );
        assert_eq!(
            Date::new(2002, 4) + Date::new(60, 10),
            Date::new(2063, 2)
        );
        assert_eq!(
            Date::new(2002, 4) + Date::new(0, 13),
            Date::new(2003, 5)
        );
    }
    #[test]
    fn date_sub() {
        assert_eq!(
            Date::new(2062, 7) -
            Date::new(60, 3),
            Date::new(2002, 4)
        );
        assert_eq!(
            Date::new(2063, 2) -
            Date::new(60, 10),
            Date::new(2002, 4)
        );
        assert_eq!(
            Date::new(2003, 5) -
            Date::new(2002, 4),
            Date::new(1, 1),
        );
    }

    #[test]
    fn working_date_curr() {
        let calculator = CombinedRules { rules: vec![ Box::new(rules::Rules1978) ] };
        let date = Date::new(2002, 04);
        assert_eq!(Some(Date::new_abs(60, 0)), calculator.calculate_working_date(&date, &PersonnelCategory::Man));
        assert_eq!(Some(Date::new_abs(55, 0)), calculator.calculate_working_date(&date, &PersonnelCategory::FemaleCadres));
        assert_eq!(Some(Date::new_abs(50, 0)), calculator.calculate_working_date(&date, &PersonnelCategory::FemaleWorkers));
    }

    #[test]
    fn working_date_2024_right() {
        let date = Date::new(2002, 04);
        assert_eq!(Some(Date::new(3, 1)), rules::Rules20240913.calculate_working_date(&date, &PersonnelCategory::Man));
        assert_eq!(Some(Date::new(5, 1)), rules::Rules20240913.calculate_working_date(&date, &PersonnelCategory::FemaleWorkers));
        assert_eq!(Some(Date::new(3, 1)), rules::Rules20240913.calculate_working_date(&date, &PersonnelCategory::FemaleCadres));
    }
    #[test]
    fn working_date_2024_left() {
        let date = Date::new(1960, 04);
        assert_eq!(None, rules::Rules20240913.calculate_working_date(&date, &PersonnelCategory::Man));
        assert_eq!(None, rules::Rules20240913.calculate_working_date(&date, &PersonnelCategory::FemaleWorkers));
        assert_eq!(None, rules::Rules20240913.calculate_working_date(&date, &PersonnelCategory::FemaleCadres));
    }

    #[test]
    fn working_date_2024_mid_man() {
        assert_eq!(Some(Date::new_abs(0, 4)), rules::Rules20240913.calculate_working_date(&Date::new(1966,  1), &PersonnelCategory::Man));
        assert_eq!(Some(Date::new_abs(0, 4)), rules::Rules20240913.calculate_working_date(&Date::new(1966,  2), &PersonnelCategory::Man));
        assert_eq!(Some(Date::new_abs(0, 4)), rules::Rules20240913.calculate_working_date(&Date::new(1966,  3), &PersonnelCategory::Man));
        assert_eq!(Some(Date::new_abs(0, 4)), rules::Rules20240913.calculate_working_date(&Date::new(1966,  4), &PersonnelCategory::Man));
    }
    #[test]
    fn working_date_mix_mid_man() {
        let calculator = CombinedRules { rules: vec![ Box::new(rules::Rules20240913), Box::new(rules::Rules1978) ] };
        assert_eq!(Some(Date::new(2026, 5)), calculator.calculate_working_date(&Date::new(1966,  1), &PersonnelCategory::Man));
        assert_eq!(Some(Date::new(2026, 6)), calculator.calculate_working_date(&Date::new(1966,  2), &PersonnelCategory::Man));
        assert_eq!(Some(Date::new(2026, 7)), calculator.calculate_working_date(&Date::new(1966,  3), &PersonnelCategory::Man));
        assert_eq!(Some(Date::new(2026, 8)), calculator.calculate_working_date(&Date::new(1966,  4), &PersonnelCategory::Man));
    }

    #[test]
    fn test_testcases() {
        let calculator = CombinedRules { rules: vec![ Box::new(rules::Rules20240913), Box::new(rules::Rules1978) ] };
        assert_eq!(calculator.calculate_working_date(&Date::new(1971,04), &PersonnelCategory::FemaleCadres), Some(Date::new(2026,08)));
        assert_eq!(calculator.calculate_working_date(&Date::new(1995,12), &PersonnelCategory::FemaleWorkers), Some(Date::new(2050,12)));
        assert_eq!(calculator.calculate_working_date(&Date::new(2000,12), &PersonnelCategory::FemaleCadres), Some(Date::new(2058,12)));
        assert_eq!(calculator.calculate_working_date(&Date::new(1963,04), &PersonnelCategory::FemaleCadres), Some(Date::new(2018,04)));
        assert_eq!(calculator.calculate_working_date(&Date::new(1995,12), &PersonnelCategory::Man), Some(Date::new(2058,12)));
        assert_eq!(calculator.calculate_working_date(&Date::new(2000,12), &PersonnelCategory::Man), Some(Date::new(2063,12)));
        assert_eq!(calculator.calculate_working_date(&Date::new(1965,12), &PersonnelCategory::Man), Some(Date::new(2026,03)));
        assert_eq!(calculator.calculate_working_date(&Date::new(1963,12), &PersonnelCategory::Man), Some(Date::new(2023,12)));
        assert_eq!(calculator.calculate_working_date(&Date::new(1964,02), &PersonnelCategory::Man), Some(Date::new(2024,02)));
        assert_eq!(calculator.calculate_working_date(&Date::new(1965,01), &PersonnelCategory::Man), Some(Date::new(2025,02)));
    }

    fn test_for_date_convert() {
        let date1 = Date::new(1971, 4);
        let date2 = Date::new(2026, 8);
        let retire_age_1: f32 = (date2 - date1).into();
        // assert_eq!(retire_age_1, 55.33);
    }
}
