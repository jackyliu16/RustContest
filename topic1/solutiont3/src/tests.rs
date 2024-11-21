// src/tests.rs
mod calc_time;

#[cfg(test)]
mod tests {
    use super::calc_time::time_info;
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
}
