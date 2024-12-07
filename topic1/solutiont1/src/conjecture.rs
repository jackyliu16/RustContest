/// 获取前两个不满足 Goldbach 猜想的数之和
///
/// 如果该和存在, 则返回数之和, 否则返回 0
pub fn goldbach_conjecture() -> u64 {
    let mut res = 0;
    // 由 迭代器 中获取奇合数
    // 这样可以根据需要往后自动拓展
    for odd in generate_odd_composite_number() {
        if PRIMES_CACHE.iter().take_while(|&&x| x < odd as u32)    // 所有可能的素数
            .map(|s| *s as u32 as u64)
            .all(|p| !((odd - p) % 2 == 0 && is_same_after_sqrt((odd - p) / 2))) {         // (odd - p) / 2 是完全平方数的情况下返回

            // 遍历全部但是还没找到 -> 确实不满足
            if res == 0 { // 第一个
                res += odd;
            } else {
                res += odd;
                return res;
            }
        }

    };
    0
}

/// 判断 num 是否是素数
#[inline(always)]
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u64) {
        if num % i == 0 {
            return false;
        }
    }
    true
}


/// 生成并返回 奇合数 的迭代器
pub fn generate_odd_composite_number() -> impl Iterator<Item = u64> {
    let mut cur: u64 = 9;                        // 最小的奇合数
    std::iter::from_fn(move || loop {
        if cur % 2 == 1 && !is_prime(cur) {      // 奇数 且 不是素数
            let res = Some(cur);
            cur += 2;                            // 检查下一个奇数
            return res;
        }
        cur += 2
    })
}

/// 精度更好的开方匹配
/// odd = primes + 2 * n.pow(2)
/// n = $\left (\sqrt{\frac{odd - primes}{2} } \right ) ^2$
#[inline]
pub fn is_same_after_sqrt(num: u64) -> bool {
    // f64::sqrt(num as f64) % 1_f64 == 0_f64 // TODO： 精度可能不是很高
    let tmp = (num as f64).sqrt() as u64;
    tmp.pow(2) == num
}

/*
    下面的文件是历史实现版本所引用的, 与当前实现无关
 */

/// 计算一个数能否由一个素数与其平方的两倍计算而成
/// n: 奇合数
#[deprecated]
pub fn goldbach_falsification(odd: u32) -> bool {
    for prime in PRIMES_CACHE.iter().take_while(|&&x| x <= (odd / 2) as u32) {
        if is_same_after_sqrt(((odd - prime) / 2) as u64) {
            return true;
        }
    }
    false
}

/// 计算某个数是不是奇合数
/// 计算公式为所有正因子的总和相当于两倍的该数字
#[deprecated]
pub fn is_odd_composite_number(n: u32) -> bool {
    find_poss_factors(n).iter().sum::<u32>() == 2 * n
}

/// 正因子计算
#[deprecated]
pub fn find_poss_factors(n: u32) -> Vec<u32> {
    let mut factors = Vec::new();
    let limit = (n as f32).sqrt() as u32;

    for i in 1..=limit {
        if n % i == 0 {
            factors.push(i);
            if i != n / i {
                factors.push(n / i); // 添加配对的因子
            }
        }
    }
    factors
}