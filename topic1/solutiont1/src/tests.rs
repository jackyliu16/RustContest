// src/tests.rs
mod conjecture;

#[cfg(test)]
mod tests {
    use super::conjecture::goldbach_conjecture;
    use std::time::{Instant, Duration};
    use crate::conjecture;

    // 定义测试用例和预期结果
    // 196702;
    const TEST_CASE: u64 = 11770;

    // 定义一个测试函数来验证每个测试用例
    #[test]
    fn test_conjecture() {
        let start = Instant::now();
        let result = goldbach_conjecture();
        let duration = start.elapsed();

        // 时间超0.5s，判定不合格
        let mut total_score = 0.0;
        if duration <= Duration::from_millis(500) && result == TEST_CASE {
            total_score += 100.0;
        }

        println!("Total score: {:.2}", total_score);
        assert_eq!(100.00, total_score);
    }

    #[test]
    fn poss_factor() {
        let true_vec = vec![ 6, 28, 496, 8128 ];
        let false_vec = vec![ 5, 13, 467, 2455];

        for item in true_vec {
            let factors = conjecture::find_poss_factors(item);
            assert_eq!(factors.iter().fold(0, |acc, &num| acc + num), 2 * item)
        }

        for item in false_vec {
            let factors = conjecture::find_poss_factors(item);
            assert!(factors.iter().all(|&factor| item % factor == 0));
        }
    }

    #[test]
    fn is_odd_composite_number() {
        let true_vec = vec![ 6, 28, 496, 8128 ];
        let false_vec = vec![ 5, 13, 467, 2455];

        for item in true_vec {
            assert!(conjecture::is_odd_composite_number(item), "奇合数检测出错");
        }

        for item in false_vec {
            assert!( ! conjecture::is_odd_composite_number(item), "奇合数检测出错");
        }
    }

    #[test]
    fn is_same() {
        let inp_vec =  vec![21, 42, 64, 26];
        let res_vec = vec![false, false, true, false];

        for idx in 0..inp_vec.len() {
            assert_eq!(conjecture::is_same_after_sqrt(inp_vec[idx]), res_vec[idx]);
        }
    }
}
