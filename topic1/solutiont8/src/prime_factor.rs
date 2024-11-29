use num::abs;
use rand::{thread_rng, Rng};
use std::cmp::max;
use std::panic::resume_unwind;

pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut max_factor = 0_u128;
    fac(number, &mut max_factor);
    return max_factor;
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// 快速乘
fn bmul(a: u128, b: u128, m: u128) -> u128 {
    // (a % m * b % m) % m
    let mut a = a % m;
    let mut b = b % m;

    let mut res = 0_u128;
    let mut a = a;

    while b > 0 {
        // 对于 b 的每一位
        if b & 1 == 1 {
            // 如果最后一位是 1，则在累积上加上 a
            res = (res + a) % m;
        }
        a = (a + a) % m; // 向左进位（类比书写进位）
        b >>= 1; // b 最后一位计算结束，继续下一位
    }
    res
}

/// 快速幂
fn qpow(mut x: u128, mut p: u128, m: u128) -> u128 {
    let mut ans = 1;
    while p != 0 {
        if p % 2 == 1 {
            // 奇数
            ans = bmul(ans, x, m);
        }
        x = bmul(x, x, m);
        p >>= 1;
    }
    ans
}

fn fac(x: u128, max_factor: &mut u128) {
    if x <= *max_factor || x < 2 {
        return;
    }
    if miller_rabin(x) {
        // Miller Rabin 算法 判断是否为素数，如果是就可以直接返回了
        *max_factor = std::cmp::max(*max_factor, x);
        return;
    }
    let mut p = x;
    while p >= x {
        p = pollard_rho(x);
    }
    let mut x = x;
    while x % p == 0 {
        x /= p;
    }
    fac(x, max_factor);
    fac(p, max_factor);
}

fn miller_rabin(p: u128) -> bool {
    if p < 2 || p % 2 == 0 {
        return false;
    }
    if p == 2 || p == 3 {
        return true;
    }

    let mut d: u128 = p - 1;
    let mut r: u128 = 0;

    while d % 2 == 0 {
        // 将d处理为奇数
        d /= 2;
        r += 1;
    }

    let mut rng = rand::thread_rng();
    for k in 0..10 {
        let a = rng.gen_range(2..p - 1);
        let mut x = qpow(a, d, p);
        if x == 1 || x == p - 1 {
            continue;
        }
        for i in 0..r - 1 {
            x = bmul(x, x, p);
            if x == p - 1 {
                break;
            }
        }
        if x != p - 1 {
            return false;
        }
    }
    true
}

fn pollard_rho(x: u128) -> u128 {
    let mut s = 0;
    let mut t = 0;
    let mut rng = rand::thread_rng();
    let c = rng.gen_range(1..x);

    let mut step = 0;
    let mut goal = 1;
    let mut val = 1;

    loop {
        for _ in 0..goal {
            t = (bmul(t, t, x) + c) % x;
            val = bmul(val, t.abs_diff(s), x); // t - s
            step += 1;
            if step % 127 == 0 {
                let d = gcd(val, x);
                if d > 1 {
                    return d;
                }
            }
        }
        let d = gcd(val, x);
        if d > 1 {
            return d;
        }
        goal *= 2;
        s = t;
        val = 1
    }
}
