use std::fmt::format;
use const_primes::Primes;

/// 求对角线上素数的比例第一次低于某个比例时，螺旋数阵的边长是多少，此时有多少个素数？（比例不会小于 7%)
/// 对角线上素数的比例第一次低于 threshold / 100 时返回当前边长与素数
///
/// # 输入值:
///     threshold: 以百分比形式输入的比率, 当对角线上 素数数量/总对角点数量 小于此比例时返回
///
/// # 返回值:
///     String: format!("{边长},{对角线上素数的个数}");
pub fn min_edge_prime_num(threshold: u32) -> String {
    // 1, (2), _3_, 4, _5_, 6, _7_, 8, _9_, (10), 11, 12, _13_, 14, 15, 16, _17_
    // 18, 19, 20, _21_, 22, 23, 24, 25, (26), 27, 28, 29, 30, _31_, 32, ...
    // NOTE: 自起始点前一个点起，每间隔(2 * turn)则到达下一个边角。

    let mut turn = 1;
    let mut primes_cnt = 0;

    loop {
        primes_cnt += how_many_prime_edges_in_this_turn(turn);

        if primes_cnt as f64 / (get_edge(turn) as f64) < threshold as f64 / 100_f64 {
            return format!("{},{primes_cnt}", 1 + 2 * turn);
        }

        turn += 1;
    }
}

/// 返回当前轮次中存在多少个 素数对角点
fn how_many_prime_edges_in_this_turn(this_turn: u32) -> u32 {
    let last_turn = this_turn - 1;
    let last_edge_len = 1 + 2 * last_turn;
    let mut cnt: u64 = last_edge_len as u64 * last_edge_len as u64; // 上一圈的最后一个点的位置
    let mut prime_cnt = 0;

    for _ in 0..3 { // 没有必要计算右下角
        cnt += 2 * this_turn as u64;
        // 此处自己实现的 Miller-Rabin 素性测试算法性能不够, 无法在规定时间内完成计算
        // 因此采用 const-primes 提供的高性能 Miller-Rabin 素性测试算法
        if const_primes::is_prime(cnt as u64) {
            prime_cnt += 1;
        }
    }

    prime_cnt
}

/// 获取到某一轮次为止的对角点数量
fn get_edge(turn: u32) -> u32 {
    match turn {
        0 => 1,
        _ => 1 + 4 * turn
    }
}


/*
   ================================================== 
   # 下面的部分与最终版本的实现无关, 仅作为曾经的实现而留存 #
   ================================================== 
*/


/// 计算直到当前 cnt 数为止, 存在多少个符合标准的 对角点
/// 
/// # 输入:
///     螺旋矩阵中对应点的数字
/// 
/// # Example:
///     如果想要计算到 520 为止(包含520) 有多少个 对角点
///     使用下面的函数, 会根据上一轮次的对角点数量 + 当前轮次的对角点数量
///     计算得到当前 cnt 对应的对角点数量
/// 
/// ```rust
/// how_many_edge(cnt, turn);
/// ```
#[inline(always)]
#[deprecated] // 本题没有必要针对于每一个对角点进行判断
fn how_many_edge(cnt: u32, turn: u32) -> u32 {
    if cnt <= 9 || turn <= 1{ panic!("Not support cnt <= 9"); }
    let last_turn = turn - 1;
    let remain_edge = last_turn * 4;
    let this_len = 1 + 2 * turn;
    let remain_len = 1 + 2 * last_turn;
    let len_this = cnt - remain_len * remain_len;
    let this_edge = len_this / (this_len - 1);
    remain_edge + this_edge
}

// mod tests {
//     use super::how_many_edge;
//     use super::*;
//
//     #[test]
//     fn test_prime_in_edge() {
//         assert_eq!(how_many_prime_edge_in_this_turn(1), 3); // 3 5 7
//         assert_eq!(how_many_prime_edge_in_this_turn(2), 2); // 13 17
//         assert_eq!(how_many_prime_edge_in_this_turn(3), 3); // 31 37 43
//     }
//
//     // #[test]
//     // fn test_how_many_edge() {
//     //     assert_eq!(4, how_many_edge(10, 2));
//     //     assert_eq!(4, how_many_edge(11, 2));
//     //     assert_eq!(4, how_many_edge(12, 2));
//     //
//     //     assert_eq!(5, how_many_edge(13, 2));
//     //     assert_eq!(5, how_many_edge(14, 2));
//     //     assert_eq!(5, how_many_edge(15, 2));
//     //     assert_eq!(5, how_many_edge(16, 2));
//     //
//     //     assert_eq!(6, how_many_edge(17, 2));
//     //     assert_eq!(6, how_many_edge(18, 2));
//     //     assert_eq!(6, how_many_edge(19, 2));
//     //     assert_eq!(6, how_many_edge(20, 2));
//     //
//     //     assert_eq!(7, how_many_edge(21, 2));
//     //     assert_eq!(7, how_many_edge(22, 2));
//     //     assert_eq!(7, how_many_edge(23, 2));
//     //     assert_eq!(7, how_many_edge(24, 2));
//     //     assert_eq!(8, how_many_edge(25, 2));
//     //
//     //     assert_eq!(8, how_many_edge(26, 3));
//     //     assert_eq!(8, how_many_edge(27, 3));
//     //     assert_eq!(8, how_many_edge(28, 3));
//     //     assert_eq!(8, how_many_edge(29, 3));
//     //     assert_eq!(8, how_many_edge(30, 3));
//     //
//     //     assert_eq!(9, how_many_edge(31, 3));
//     //     assert_eq!(9, how_many_edge(32, 3));
//     //     assert_eq!(9, how_many_edge(33, 3));
//     //     assert_eq!(9, how_many_edge(34, 3));
//     //     assert_eq!(9, how_many_edge(35, 3));
//     // }
// }
