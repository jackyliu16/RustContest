// src/tests.rs
mod calc_time;

#[cfg(test)]
mod tests {
    use super::calc_time::{days_until_months_since_years, days_until_year, get_curr_week_since_this_year, how_many_days_of_this_month, lunar_cal, time_info, zellers_kongruenz};
    use std::time::{Instant, Duration};

    // 定义测试用例和预期结果
    const TEST_CASES: &[(&str, &str)] = &[
        ("2024-11-10", "45,51,79"),
        ("2024-11-18", "47,43,71"),
        ("2024-12-31", "1,0,28"),
        ("2025-01-01", "1,364,27"),
        ("2025-12-31", "1,0,47"),
        ("2020-01-20", "4,346,4"),
        ("2021-02-13", "6,321,352"),
        ("2012-01-22", "3,344,0"),
        ("2013-02-11", "7,323,353"),
        ("2014-02-02", "5,332,381"),
    ];

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_calc_time() {
        let mut total_score = 0.0;
        for (input, expected) in TEST_CASES {
            let start = Instant::now();
            let result = time_info(*input);
            let duration = start.elapsed();

            // 时间超0.2s，判定不合格
            if duration <= Duration::from_millis(200) && result == *expected {
                total_score += 10.0;
            }
        }

        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }

    // generate with AI
    use calc_time::is_leap_year;
    use crate::calc_time;

    #[test]
    fn test_is_leap_year() {
        // 测试常见的闰年
        assert!(is_leap_year(2000)); // 2000 是闰年
        assert!(is_leap_year(2020)); // 2020 是闰年
        assert!(is_leap_year(1600)); // 1600 是闰年

        // 测试常见的非闰年
        assert!(!is_leap_year(1900)); // 1900 不是闰年
        assert!(!is_leap_year(2021)); // 2021 不是闰年
        assert!(!is_leap_year(2100)); // 2100 不是闰年
        assert!(!is_leap_year(2001)); // 2001 不是闰年
    }

    #[test]
    fn test_edge_cases() {
        // 测试负年（如公元前）
        assert!(is_leap_year(-4));   // -4 是闰年（公元前4年）
        assert!(!is_leap_year(-3));  // -3 不是闰年
        assert!(!is_leap_year(-100)); // -100 不是闰年
        assert!(is_leap_year(-400)); // -400 是闰年
    }

    #[test]
    fn test_days_until_year() {
        let test_cases = [
            (1, 0),
            (5, 1461),
            (10, 3287),
            (100, 36159),
            (400, 145731),
            (-1, -365),
            (-4, -1461),
            (-100, -36524),
            (-400, -146097),
            (-2, -730),
        ];

        for (year, expected) in test_cases {
            let result = days_until_year(year);
            assert_eq!(result, expected, "Test failed for year {}: expected {}, got {}", year, expected, result);
        }
    }

    #[test]
    fn tests_how_many_days_in_month() {
        let months = vec![ 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, ];
        let leap_months = vec![ 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31, ];

        for i in 1..=12 {
            assert_eq!(how_many_days_of_this_month(2023, i), months[i - 1]); // 平年
            assert_eq!(how_many_days_of_this_month(2024, i), leap_months[i - 1]); // 闰年
        }
    }

    #[test]
    fn test_days_until_months_since_years() {
        assert_eq!(days_until_months_since_years(2023, 1), 0); // January: 0 days
        assert_eq!(days_until_months_since_years(2023, 2), 31); // January
        assert_eq!(days_until_months_since_years(2023, 3), 59); // January + February (non-leap year)
        assert_eq!(days_until_months_since_years(2024, 3), 60); // January + February (leap year)
        assert_eq!(days_until_months_since_years(2023, 12), 334); // Up to November
        assert_eq!(days_until_months_since_years(2024, 12), 335); // Up to November (leap year)
    }

    #[test]
    fn test_zellers_kongruenz() {
        assert_eq!(zellers_kongruenz(2024, 11, 22), 5); // TODAY - FRIDAY
        assert_eq!(zellers_kongruenz(2024, 11, 21), 4);
        assert_eq!(zellers_kongruenz(2024, 11, 20), 3);
        assert_eq!(zellers_kongruenz(2024, 11, 19), 2);
        assert_eq!(zellers_kongruenz(2024, 11, 18), 1);
        assert_eq!(zellers_kongruenz(2024, 11, 17), 0); // SUNDAY
        assert_eq!(zellers_kongruenz(2024, 1, 1), 1);   // MONDAY
        // Test case for a known date: 2023-11-21 (Tuesday)
        assert_eq!(zellers_kongruenz(2023, 11, 21), 2);
        // // Test for February in a leap year: 2024-02-29 (Thursday)
        assert_eq!(zellers_kongruenz(2024, 2, 29), 4);
        // // Test for the first day of the year: 2023-01-01 (Sunday)
        assert_eq!(zellers_kongruenz(2023, 1, 1), 0);
        // // Test for end-of-year: 2023-12-31 (Sunday)
        assert_eq!(zellers_kongruenz(2023, 12, 31), 0);
        // // Test for a well-known historical date: 1969-07-20 (Sunday)
        assert_eq!(zellers_kongruenz(1969, 7, 20), 0);
    }

    /// (year, 1, 1) 是周一
    #[test]
    fn test_week_num_situation_1() {
        assert_eq!(get_curr_week_since_this_year(2024, 1, 1), 1);
        assert_eq!(get_curr_week_since_this_year(2024, 1, 2), 1);
        assert_eq!(get_curr_week_since_this_year(2024, 1, 3), 1);
        assert_eq!(get_curr_week_since_this_year(2024, 1, 4), 1);
        assert_eq!(get_curr_week_since_this_year(2024, 1, 5), 1);
        assert_eq!(get_curr_week_since_this_year(2024, 1, 6), 1);
        assert_eq!(get_curr_week_since_this_year(2024, 1, 7), 1);
    }

    /// (year, 1, 1) 是周二到周四
    #[test]
    fn test_week_num_situation_2() {
        assert_eq!(get_curr_week_since_this_year(2024, 1, 1), 1); // MON
        assert_eq!(get_curr_week_since_this_year(2019, 1, 1), 1); // TUE
        assert_eq!(get_curr_week_since_this_year(2025, 1, 1), 1); // WED
        assert_eq!(get_curr_week_since_this_year(2019, 1, 1), 1); // THU
    }

    fn test_week_num_situation_3() {
        assert_eq!(get_curr_week_since_this_year(2236, 1, 1), 52); // FRI
        assert_eq!(get_curr_week_since_this_year(2028, 1, 1), 52); // SAT
        assert_eq!(get_curr_week_since_this_year(2012, 1, 1), 52); // SUN
    }

    fn test_week_num_situation_4() {
        // assert_eq!(get_curr_week_since_this_year(2094, 1, 1), 53); // FRI
        // assert_eq!(get_curr_week_since_this_year(1993, 1, 1), 53); // SAT
        // assert_eq!(get_curr_week_since_this_year(2010, 1, 1), 53); // SUN
    }

    #[test]
    fn test_week_num_in_testcases() {
        assert_eq!(get_curr_week_since_this_year(2024,11,10), 45);
        assert_eq!(get_curr_week_since_this_year(2024,11,18), 47);
        assert_eq!(get_curr_week_since_this_year(2024,12,31), 1);
        assert_eq!(get_curr_week_since_this_year(2025,01,01), 1);
        assert_eq!(get_curr_week_since_this_year(2025,12,31), 1);
        assert_eq!(get_curr_week_since_this_year(2020,01,20), 4);
        assert_eq!(get_curr_week_since_this_year(2021,02,13), 6);
        assert_eq!(get_curr_week_since_this_year(2012,01,22), 3);
        assert_eq!(get_curr_week_since_this_year(2013,02,11), 7);
        assert_eq!(get_curr_week_since_this_year(2014,02,02), 5);
    }


    // #[test]
    // fn test_get_curr_week_since_this_year_firstweek_in_this_year() {
    //     for i in 1..=7 {
    //         assert_eq!(get_curr_week_since_this_year(2024, 1, i), 1);
    //     }
    // }
    //
    // #[test]
    // fn test_get_curr_week_since_this_year_lastweek_in_this_year() {
    //     for i in 25..=31 {
    //         assert_eq!(get_curr_week_since_this_year(2023, 12, i), 52);
    //     }
    // }
    //
    // #[test]
    // fn test_get_curr_week_since_this_year() {
    //     assert_eq!(get_curr_week_since_this_year(1993, 1, 1), 52);
    //     assert_eq!(get_curr_week_since_this_year(2024, 11, 22), 47); // FRIDAY
    //     assert_eq!(get_curr_week_since_this_year(2024, 11, 23), 47);
    //     assert_eq!(get_curr_week_since_this_year(2024, 11, 24), 47);
    //     assert_eq!(get_curr_week_since_this_year(2024, 11, 25), 48); // MONDAY
    //     assert_eq!(get_curr_week_since_this_year(2024, 11, 26), 48);
    //     assert_eq!(get_curr_week_since_this_year(2024, 11, 27), 48);
    //     assert_eq!(get_curr_week_since_this_year(2024, 11, 28), 48);
    //
    //     assert_eq!(get_curr_week_since_this_year(2024, 11, 10), 45);
    //     assert_eq!(get_curr_week_since_this_year(2024, 11, 18), 47);
    //     assert_eq!(get_curr_week_since_this_year(2024, 12, 31), 1);
    //     assert_eq!(get_curr_week_since_this_year(2025, 01, 01), 1);
    //     assert_eq!(get_curr_week_since_this_year(2020, 01, 20), 4);
    //     assert_eq!(get_curr_week_since_this_year(2021, 02, 13), 6);
    //     assert_eq!(get_curr_week_since_this_year(2012, 01, 22), 3);
    //     assert_eq!(get_curr_week_since_this_year(2013, 02, 11), 7);
    //     assert_eq!(get_curr_week_since_this_year(2014, 02, 02), 5);
    // }

    /// DATA FROM https://www.hko.gov.hk/tc/gts/time/conversion.htm
    #[test]
    fn tests_get_lunar_new_year_date() {
        assert_eq!((1901, 2, 19), lunar_cal::get_lunar_new_year_date(1901));
        assert_eq!((1912, 2, 18), lunar_cal::get_lunar_new_year_date(1912));
        assert_eq!((1924, 2, 5), lunar_cal::get_lunar_new_year_date(1924));
        assert_eq!((1949, 1, 29), lunar_cal::get_lunar_new_year_date(1949));
        assert_eq!((1978, 2, 7), lunar_cal::get_lunar_new_year_date(1978));
        assert_eq!((2002, 2, 12), lunar_cal::get_lunar_new_year_date(2002));
        assert_eq!((2035, 2, 8), lunar_cal::get_lunar_new_year_date(2035));
        assert_eq!((2099, 1, 21), lunar_cal::get_lunar_new_year_date(2099));
    }
}
