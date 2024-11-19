use const_primes::{primes, Primes};

pub fn goldbach_conjecture() -> u64 {
    let mut res = 0;
    for odd in generate_odd_composite_number() {
        if PRIMES_CACHE.iter().take_while(|&&x| x < odd as u32)
            .map(|s| *s as u32 as u64)
            .all(|p| !((odd - p) % 2 == 0 && is_same_after_sqrt((odd - p) / 2))) {

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

/// 生成奇合数
/// 奇合数：奇数且不是素数
pub fn generate_odd_composite_number() -> impl Iterator<Item = u64> {
    let mut cur: u64 = 9;                        // 最小的奇合数
    std::iter::from_fn(move || loop {
        if cur % 2 == 1 && !is_prime(cur) {     // 奇数 且 不是素数(不可能是因数)
            let res = Some(cur);
            cur += 2;                           // 检查下一个奇数
            return res;
        }
        cur += 2
    })
}

const PRIMES_CACHE: Primes<10000> = Primes::new();

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

/// 精度更好的开方匹配
/// odd = primes + 2 * n.pow(2)
/// n = $\left (\sqrt{\frac{odd - primes}{2} } \right ) ^2$
#[inline]
pub fn is_same_after_sqrt(num: u64) -> bool {
    // f64::sqrt(num as f64) % 1_f64 == 0_f64 // TODO： 精度可能不是很高
    let tmp = (num as f64).sqrt() as u64;
    tmp.pow(2) == num
}


/// 求素数
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