use const_primes::Primes;

/// 求对角线上素数的比例第一次低于某个比例时，螺旋数阵的边长是多少，此时有多少个素数？（比例不会小于 7%)
pub fn min_edge_prime_num(threshold: u32) -> String {
    // 1, (2), _3_, 4, _5_, 6, _7_, 8, _9_, (10), 11, 12, _13_, 14, 15, 16, _17_
    // 18, 19, 20, _21_, 22, 23, 24, 25, (26), 27, 28, 29, 30, _31_, 32, ...
    // 在距离中心一圈以外的地方，每隔 (边长 - 1)^2 则到达下一圈的起始点
    // 自起始点前一个点起，每间隔(边长 - 2)则到达下一个边角。

    if threshold == 1 {
        return String::from("3,1");
    }

    let mut cnt: u64 = 10;
    let mut primes_cnt = 3;
    let mut edge_len: u64= 5; // 1 + 2 turn_indicator
    let mut turn = 2;
    let mut edge_num = 4;

    // println!("{cnt}, {turn}, {primes_cnt}, {}", how_many_edge(cnt, turn));
    // println!("{} > {}", primes_cnt as f64 / (how_many_edge(cnt, turn) as f64), threshold as f64 / 100_f64);
    // 如果尚未满足要求
    println!("{:.4} > {}", primes_cnt as f64 / (how_many_edge(cnt, turn) as f64 + 1_f64), threshold as f64 / 100_f64);

    loop {
        cnt += 1;
        if cnt == edge_len * edge_len {
            // 终止条件
            if primes_cnt as f64 / (how_many_edge(cnt, turn) as f64 + 1_f64) < threshold as f64 / 100_f64 {
               return format!("{edge_len},{primes_cnt}");
            }
            // println!("cnt{cnt}, turn{turn}, edge{edge_len}, prime{primes_cnt}, {}", how_many_edge(cnt, turn));
            turn += 1;
            edge_len += 2;
        }

        if how_many_edge(cnt, turn) != edge_num {
            edge_num = how_many_edge(cnt, turn);
            if const_primes::is_prime(cnt) {
               primes_cnt += 1;
            }
        }
    }

    format!("{edge_len},{primes_cnt}")
}

// const PRIMES_CACHE: Primes<100000> = Primes::new();
// /// 求素数
// /// 当值小于 10000 时直接查表获取 cnt 是否是素数
// /// 否则采用遍历法求解
// fn is_prime(num: u64) -> bool {
//     match PRIMES_CACHE.is_prime(num as u32) {
//         Some(true) => true,
//         Some(false) => false,
//         None => {
//             if num <= 1 {
//                 return false;
//             }
//             for i in 2..=((num as f64).sqrt() as u64) {
//                 if num % i == 0 {
//                     return false;
//                 }
//             }
//             true
//         }
//     }
// }

#[inline(always)]
/// 计算直到当前 cnt 位置一共存在多少个对角点
fn how_many_edge(cnt: u64, turn: u32) -> u32 {
    if cnt <= 9 || turn <= 1{ panic!("Not support cnt <= 9"); }
    let last_turn = turn - 1;
    let remain_edge = last_turn * 4;
    let this_len = 1 + 2 * turn;
    let remain_len = 1 + 2 * last_turn;
    let len_this = cnt - remain_len as u64 * remain_len as u64;
    let this_edge = len_this / (this_len - 1) as u64;
    remain_edge + this_edge as u32
}

// mod tests {
//     use super::how_many_edge;
//     #[test]
//     fn test_how_many_edge() {
//         assert_eq!(4, how_many_edge(10, 2));
//         assert_eq!(4, how_many_edge(11, 2));
//         assert_eq!(4, how_many_edge(12, 2));
//
//         assert_eq!(5, how_many_edge(13, 2));
//         assert_eq!(5, how_many_edge(14, 2));
//         assert_eq!(5, how_many_edge(15, 2));
//         assert_eq!(5, how_many_edge(16, 2));
//
//         assert_eq!(6, how_many_edge(17, 2));
//         assert_eq!(6, how_many_edge(18, 2));
//         assert_eq!(6, how_many_edge(19, 2));
//         assert_eq!(6, how_many_edge(20, 2));
//
//         assert_eq!(7, how_many_edge(21, 2));
//         assert_eq!(7, how_many_edge(22, 2));
//         assert_eq!(7, how_many_edge(23, 2));
//         assert_eq!(7, how_many_edge(24, 2));
//         assert_eq!(8, how_many_edge(25, 2));
//
//         assert_eq!(8, how_many_edge(26, 3));
//         assert_eq!(8, how_many_edge(27, 3));
//         assert_eq!(8, how_many_edge(28, 3));
//         assert_eq!(8, how_many_edge(29, 3));
//         assert_eq!(8, how_many_edge(30, 3));
//
//         assert_eq!(9, how_many_edge(31, 3));
//         assert_eq!(9, how_many_edge(32, 3));
//         assert_eq!(9, how_many_edge(33, 3));
//         assert_eq!(9, how_many_edge(34, 3));
//         assert_eq!(9, how_many_edge(35, 3));
//     }
// }
