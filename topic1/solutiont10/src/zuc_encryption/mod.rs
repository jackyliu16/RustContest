use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, BitAnd, BitXor, Mul, Shl, Shr};
// use crate::zuc_encryption::structs::{MyU31, MyU32};

use aint::{u15, u31, Aint, BitSplit};
use crate::zuc_encryption::consts::{D, S0, S1};

mod consts;
mod structs;

const MOD231SUB1: u32 = 0x7FFF_FFFF;

pub fn encryption(input: String) -> String {
    // Base on requirement
    let k = [0u8; 16];
    let iv = [1u8; 16];

    let mut regs = Regs::new(k, iv);
    dbg!(&regs);
    let mut input_bytes = input.into_bytes();

    // PKCS#7 padding
    // 当使用块大小为32位（4字节）的加密算法时，填充的内容应该是一个值，这个值等于需要填充的字节数。
    let padding_length = 4 - input_bytes.len() % 4;
    input_bytes.append(&mut vec![padding_length as u8; padding_length]);

    let mut res = vec![];
    for i in 0..input_bytes.len() {
        let tmp =  regs.generate().to_be();
        res.push(tmp);
    }

    String::new()
}

#[derive(Debug)]
struct Regs {
    lsfr:   [u31; 17],
    r1:      u32,
    r2:      u32,
    x:      [u32; 4],
}

impl Regs {
    fn new(k: [u8; 16], iv: [u8; 16]) -> Regs {
        let mut s = [u31!(0); 17];
        for idx in 0..=15 {
            s[idx] =
                  u31::from(k[idx].overflowing_shl(23).0)
                | u31::from(D[idx].overflowing_shl(8).0)
                | u31::from(iv[idx]);
        }
        let mut regs = Regs {
            lsfr: s,
            r1: 0,
            r2: 0,
            x: [0; 4]
        };
        for _ in 0..32 {
            regs.bit_reconstruction();
            let w = regs.f();
            regs.lsfr_init(u31::try_from(w.shr(1)).unwrap());
        }

        regs
    }
    fn generate(&mut self) -> u32 {
        self.bit_reconstruction();
        let w = self.f().pow(self.x[3]);
        self.lsfr_update();
        w
    }
}

impl Regs { // ======== LSFR（线性反馈移位器） ========
    // v = 2^15*S_15 + 2^17*S_13 + 2^21*S_10 + 2^20*S_4 + ((1+2^8)*S_0) mod (2^31 - 1)
    // S_16 = (u + v) mod (2^31 - 1)
    // if S_16 == 0 -> S_16 = 2^31 - 1
    // s[0..15] = s[1..16]
    fn lsfr_init(&mut self, u: u31) {
        let mut v: u31 = mod_mul(u31::new(1 + 2 << 8).unwrap(), self.lsfr[0]);
        v += u31::new(2 << 15).unwrap() * self.lsfr[15];
        v += u31::new(2 << 17).unwrap() * self.lsfr[13];
        v += u31::new(2 << 21).unwrap() * self.lsfr[10];
        v += u31::new(2 << 20).unwrap() * self.lsfr[4];

        self.lsfr[16] = mod_add(u31::from(v), u);
        if self.lsfr[16] == u31::new(0).unwrap() {
            self.lsfr[16] = u31::new(MOD231SUB1).unwrap();
        }
        for idx in 0..self.lsfr.len() - 2 {
            self.lsfr[idx + 1] = self.lsfr[idx];
        }
    }
    // S_16 = 2^15*S_15 + 2^17*S_13 + 2^21*S_10 + 2^20*S_4 + ((1+2^8)*S_0) mod (2^31-1)
    // if S_16 == 0 -> S_16 = 2^31 - 1
    // s[0..15] = s[1..16]
    fn lsfr_update(&mut self) {
        self.lsfr[16] = mod_mul(u31::new(1 + 2 << 8).unwrap(), self.lsfr[0]);
        self.lsfr[16] += u31::new(2 << 15).unwrap() * self.lsfr[15];
        self.lsfr[16] += u31::new(2 << 17).unwrap() * self.lsfr[13];
        self.lsfr[16] += u31::new(2 << 21).unwrap() * self.lsfr[10];
        self.lsfr[16] += u31::new(2 << 20).unwrap() * self.lsfr[4];
        if self.lsfr[16] == u31!(0) {
            self.lsfr[16] = u31::new(MOD231SUB1).unwrap();
        }
        for idx in 0..self.lsfr.len() - 2 {
            self.lsfr[idx + 1] = self.lsfr[idx];
        }
    }
}

impl Regs { // ======== BR（比特重组） ========
    fn bit_reconstruction(&mut self) {
        self.x[0] = u32::from(self.lsfr[15].bitand(u31!(0x0000_FFFF)).add(self.lsfr[14].bitand(u31!(0x7FFF_0000))));
        self.x[1] = u32::from(self.lsfr[11].bitand(u31!(0x0000_FFFF)).add(self.lsfr[9].bitand(u31!(0x7FFF_0000))));
        self.x[2] = u32::from(self.lsfr[7].bitand(u31!(0x0000_FFFF)).add(self.lsfr[5].bitand(u31!(0x7FFF_0000))));
        self.x[3] = u32::from(self.lsfr[2].bitand(u31!(0x0000_FFFF)).add(self.lsfr[0].bitand(u31!(0x7FFF_0000))));
    }
}

impl Regs { // ======== F 非线性函数 ========
    fn f(&mut self) -> u32 {
        // w = (x_0 (+) R_1) mod 2^31
        // let w1 = mod_add(u31::try_from(self.r1).unwrap(), u31::try_from(self.x[1]).unwrap());
        let w = u32::from(self.x[0]).bitxor(self.r1) % MOD231SUB1;
        let w1: Aint<u32, 31> = Aint::try_from(self.r1.wrapping_add(self.x[1])).unwrap();
        let w2 = u31::try_from(self.r2.bitxor(self.x[2])).unwrap();

        let x: (u16, u16) = u32::from(w1.overflowing_shl(16).0 | w2.overflowing_shr(16).0).bit_split();
        let (x0, x1): (u8, u8) = x.0.bit_split();
        let (x2, x3): (u8, u8) = x.1.bit_split();
        self.r1 = (S0[x0 as usize] as u32) << 24 | (S1[x1 as usize] as u32) << 16 | (S0[x2 as usize] as u32) << 8 | S1[x3 as usize] as u32;

        let x: (u16, u16) = u32::from(w2.overflowing_shl(16).0 | w1.overflowing_shr(16).0).bit_split();
        let (x0, x1): (u8, u8) = x.0.bit_split();
        let (x2, x3): (u8, u8) = x.1.bit_split();
        self.r2 = (S0[x0 as usize] as u32) << 24 | (S1[x1 as usize] as u32) << 16 | (S0[x2 as usize] as u32) << 8 | S1[x3 as usize] as u32;
        w
    }
}

/// return (a + b) mod (2^31 - 1)
fn mod_add(a: u31, b: u31) -> u31 {
    a.wrapping_add(b).bitand(u31!(0x7FFF_FFFF)).wrapping_add(a.wrapping_add(b).shr(31))
}

/// return (a * b) mod (2^31 - 1)
fn mod_mul(a: u31, b: u31) -> u31 {
    a.wrapping_mul(b)
}

fn l1(bits: u32) -> u32 {
    bits ^ bits.rotate_left(2) ^ bits.rotate_left(10) ^ bits.rotate_left(18) ^ bits.rotate_left(24)
}

fn l2(bits: u32) -> u32 {
    bits ^ bits.rotate_left(8) ^ bits.rotate_left(14) ^ bits.rotate_left(22) ^ bits.rotate_left(30)
}

#[cfg(test)]
mod tests {
    use aint::u31;
    use crate::zuc_encryption::{mod_mul, mod_add};
    const MOD231SUB1: u32 = 0x7FFF_FFFF;

    #[test]
    fn test_mod_mul_typical() {
        assert_eq!(mod_mul(u31!(5), u31!(3)), u31!(15));
        assert_eq!(mod_mul(u31!(0x7FFF_FFFF), u31!(0x0000_0001)), u31!(0x0000_0000));
        assert_eq!(mod_mul(u31!(0x7FFF_FFFF), u31!(0x0000_0002)), u31!(0x0000_0000));
    }

    #[test]
    fn test_mod_mul_zero() {
        assert_eq!(mod_mul(u31!(0), u31!(100)), u31!(0));
        assert_eq!(mod_mul(u31!(100), u31!(0)), u31!(0));
    }

    fn test_mod_add_typical() {
        // dbg!(0x7FFF_FFFF, u31::MAX, u32::MAX); // 2147483647 2147483647 4294967295
        assert_eq!(mod_add(u31!(245), u31!(134)), u31!(379));
        assert_eq!(mod_add(u31!(0x7FFF_FFFF), u31!(0x0)), u31!(0x7FFF_FFFF));
        assert_eq!(mod_add(u31!(0x7FFF_FFFF), u31!(0x1)), u31!(0x0));
        assert_eq!(mod_add(u31!(0x7FFF_FFFF), u31!(0x2)), u31!(0x1));
    }

    #[test]
    fn test_mod_add_zero() {
        assert_eq!(mod_add(u31!(0), u31!(100)), u31!(100));
        assert_eq!(mod_add(u31!(100), u31!(0)), u31!(100));
        assert_eq!(mod_add(u31!(0), u31!(0)), u31!(0));
    }
}