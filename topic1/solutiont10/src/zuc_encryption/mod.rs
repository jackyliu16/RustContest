use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Mul, Shl};
use crate::zuc_encryption::structs::{MyU31, MyU32};

mod consts;
mod structs;

const MOD231SUB1: u32 = 0x7FFF_FFFF;

pub fn encryption(input: String) -> String {
    todo!()
}

#[derive(Debug)]
struct Regs {
    lsfr:   [MyU31; 17],
    r1:      MyU32,
    r2:      MyU32,
    x:      [MyU32; 4],
}

impl Mul<MyU31> for i32 {
    type Output = MyU31;

    fn mul(self, rhs: MyU31) -> Self::Output {
        self * rhs
    }
}

impl Regs { // ======== LSFR（线性反馈移位器） ========
    // v = 2^15*S_15 + 2^17*S_13 + 2^21*S_10 + 2^20*S_4 + ((1+2^8)*S_0) mod (2^31 - 1)
    // S_16 = (u + v) mod (2^31 - 1)
    // if S_16 == 0 -> S_16 = 2^31 - 1
    // s[0..15] = s[1..16]
    fn lsfr_init(&mut self, u: MyU31) {
        let mut v: MyU31 = mod_mul(MyU31::from((1 + 2 << 8) as u32), self.lsfr[0]);
        v += (2 << 15) * self.lsfr[15];
        v += (2 << 17) * self.lsfr[13];
        v += (2 << 21) * self.lsfr[10];
        v += (2 << 20) * self.lsfr[4];

        self.lsfr[16] = mod_add(MyU31::from(v), u);
        if self.lsfr[16] == MyU31::from(0_u32) {
            self.lsfr[16] = MyU31::from(MOD231SUB1);
        }
        for idx in 0..self.lsfr.len() - 2 {
            self.lsfr[idx + 1] = self.lsfr[idx];
        }
    }
    // S_16 = 2^15*S_15 + 2^17*S_13 + 2^21*S_10 + 2^20*S_4 + ((1+2^8)*S_0) mod (2^31-1)
    // if S_16 == 0 -> S_16 = 2^31 - 1
    // s[0..15] = s[1..16]
    fn lsfr_update(&mut self) {
        self.lsfr[16] = mod_mul(MyU31::from((1 + 2 << 8) as u32), self.lsfr[0]);
        self.lsfr[16] += (2 << 15) * self.lsfr[15];
        self.lsfr[16] += (2 << 17) * self.lsfr[13];
        self.lsfr[16] += (2 << 21) * self.lsfr[10];
        self.lsfr[16] += (2 << 20) * self.lsfr[4];
        if self.lsfr[16] == MyU31::from(0_u32) {
            self.lsfr[16] = MyU31::from(MOD231SUB1);
        }
        for idx in 0..self.lsfr.len() - 2 {
            self.lsfr[idx + 1] = self.lsfr[idx];
        }
    }
}

impl Regs { // ======== BR（比特重组） ========
    fn bit_reconstruction(&mut self) {
    }
}

/// return (a + b) mod (2^31 - 1)
fn mod_add(a: MyU31, b: MyU31) -> MyU31 {
    let c = a + b;
    (c & MOD231SUB1.into()) + (c >> 31_u32.into())
}

/// return (a * b) mod (2^31 - 1)
fn mod_mul(a: MyU31, b: MyU31) -> MyU31 {
    bmul(a.into(), b.into(), MOD231SUB1.into()).into()
}

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

#[cfg(test)]
mod tests {
    use crate::zuc_encryption::structs::{MyU31, MyU32};

    #[test]
    fn test_combine_u32() {
        // 测试用例 1: a 的前 16 位为 0x1234，b 的后 16 位为 0x5678
        let a = MyU31::from(0x1234_0000_u32);
        let b = MyU31::from(0x5678_u32);
        assert_eq!(MyU32::combine_u31(a, b), MyU32::from(0x1234_5678));

        // // 测试用例 2: a 的前 16 位为 0xFFFF，b 的后 16 位为 0x0
        // let a = MyU31::from(0xFFFF_0000_u32);
        // let b = MyU31::from(0x0_u32);
        // assert_eq!(MyU32::combine_u31(a, b), MyU32::from(0xFFFF_0000));
        //
        // // 测试用例 3: a 的前 16 位为 0x0，b 的后 16 位为 0xFFFF
        // let a = MyU31::from(0x0_u32);
        // let b = MyU31::from(0xFFFF_u32);
        // assert_eq!(MyU32::combine_u31(a, b), MyU32::from(0x0000_FFFF));
        //
        // // 测试用例 4: a 和 b 都是随机值
        // let a = MyU31::from(0x89AB_CDEF_u32);
        // let b = MyU31::from(0x1234_5678_u32);
        // assert_eq!(MyU32::combine_u31(a, b), MyU32::from(0x89AB_CDEF & 0xFFFF_0000 | 0x0000_5678));
    }

}