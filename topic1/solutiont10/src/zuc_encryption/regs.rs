//! 祖冲之算法 内部结构体操作实现
//!
//! # 国家标准全文:
//! [GB/T 33133.1-2016 信息安全技术 祖冲之序列密码算法 第1部分：算法描述](https://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=8C41A3AEECCA52B5C0011C8010CF0715)
//! [GB/T 33133.2-2021 信息安全技术 祖冲之序列密码算法 第2部分：保密性算法](https://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=5D3CBA3ADEC7989344BD1E63006EF2B3)
//! [GB/T 33133.3-2021 信息安全技术 祖冲之序列密码算法 第3部分：完整性算法](https://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=C6D60AE0A7578E970EF2280ABD49F4F0)
//!
//! # 功能:
//! - 根据输入的初始种子密钥和初始向量构建保存 ZUC 记忆单元变量的结构体 Regs
//! - 根据需要调用函数生成对应的加密密钥
//!
//! # 输入:
//!     k: 初始种子密钥
//!     iv: 初始向量
//!
//! # 示例:
//! ```rust
//!     let k = [0_u8; 16];
//!     let iv = [1_u8; 16];
//!     // 初始化结构体
//!     let mut regs = Regs::new(k, iv);
//!
//!     // 获取一个 32 比特密钥
//!     regs.generate()
//! ```
use std::fmt;
use std::fmt::Formatter;
use std::ops::{BitAnd, BitXor, Shr};

use crate::zuc_encryption::consts::{D, S0, S1};
use aint::{u31, u32, BitSplit};

/// 用于存储在算法运行过程中需要临时存储的因子
pub struct Regs {
    /// 线性反馈移位寄存器
    lfsr:   [u31; 17],
    /// 算法所需求的 32 位比特记忆单元 R1
    r1:      u32,
    /// 算法所需求的 32 位比特记忆单元 R2
    r2:      u32,
    /// 比特重组输出的 X
    x:      [u32; 4],
}

impl fmt::Debug for Regs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Regs {{ lfsr: {{ ")?;
        for &value in &self.lfsr {
            write!(f, "{:08x}, ", value)?;
        }
        write!(f, "}}, r : {{");
        write!(f, "{:08x}, ", self.r1)?;
        write!(f, "{:08x}, ", self.r2)?;

        write!(f, "}}, x: {{");
        for &value in &self.x {
            write!(f, "{:08x}, ", value)?;
        }
        write!(f, " }}")
    }
}

impl Regs {
    /// 参照 GB/T 33133.1-2016 传入数据对于保存算法内部信息的结构体进行初始化
    ///
    /// # 输入值:
    ///     k: 初始密钥
    ///     iv: 初始向量
    pub fn new(k: [u8; 16], iv: [u8; 16]) -> Regs {
        let mut s = [u31!(0); 17];
        for idx in 0..=15 { // CHECK CONSISTENCY
            s[idx] =
                u31::from(k[idx]).overflowing_shl(23).0
                    | u31::from(D[idx]).overflowing_shl(8).0
                    | u31::from(iv[idx]);
        }
        let mut regs = Regs { // CHECK CONSISTENCY
            lfsr: s,
            r1: 0,
            r2: 0,
            x: [0; 4]
        };
        // dbg!(&regs);
        // println!("============== 32 LOOP STR ==============");

        for _ in 0..32 {
            // println!(" - LOOP ------------------------------------ ");
            regs.bit_reconstruction(); // CHECK CONSISTENCY
            let w = regs.f();     // CHECK CONSISTENCY
            // dbg!(w);
            regs.lsfr_init(u31::new(w.shr(1)).unwrap());
            // dbg!(&regs);
        }
        regs.generate();

        // println!("============== 32 LOOP END ==============");

        regs
    }

    /// 生成 zuc 算法密钥
    ///
    /// 参照 GB/T 33133.1-2016 进行实现
    pub fn generate(&mut self) -> u32 {
        self.bit_reconstruction();
        let f = self.f();
        let w = f.bitxor(self.x[3]);
        self.lsfr_update();
        w
    }

    /// 线性移位寄存器初始化模式
    ///
    /// 参照 GB/T 33133.1-2016 进行实现
    fn lsfr_init(&mut self, u: u31) {
        // v = 2^15*S_15 + 2^17*S_13 + 2^21*S_10 + 2^20*S_4 + ((1+2^8)*S_0) mod (2^31 - 1)
        // S_16 = (u + v) mod (2^31 - 1)
        // if S_16 == 0 -> S_16 = 2^31 - 1
        // s[0..15] = s[1..16]
        let mut v: u31 = self.lfsr[0];
        v = mod_add(v, mod_mul(u31::new(1 << 8).unwrap(), self.lfsr[0]));
        v = mod_add(v, mod_mul(u31::new(1 << 20).unwrap(), self.lfsr[4]));
        v = mod_add(v, mod_mul(u31::new(1 << 21).unwrap(), self.lfsr[10]));
        v = mod_add(v, mod_mul(u31::new(1 << 17).unwrap(), self.lfsr[13]));
        v = mod_add(v, mod_mul(u31::new(1 << 15).unwrap(), self.lfsr[15]));
        // dbg!(u, v);

        let mut s16: u31 = mod_add(u31::from(v), u);
        if s16 == u31::new(0).unwrap() {
            s16 = u31!(0x7FFF_FFFF);
        }
        // dbg!(s16);

        for i in 0..15 {
            self.lfsr[i] = self.lfsr[i + 1];
        }
        self.lfsr[15] = s16;
        // dbg!(self.lfsr);
    }

    /// 线性移位寄存器工作模式
    ///
    /// 参照 GB/T 33133.1-2016 进行实现
    fn lsfr_update(&mut self) {
        // S_16 = 2^15*S_15 + 2^17*S_13 + 2^21*S_10 + 2^20*S_4 + ((1+2^8)*S_0) mod (2^31-1)
        // if S_16 == 0 -> S_16 = 2^31 - 1
        // s[0..15] = s[1..16]

        let mut s16 = self.lfsr[0];
        s16 = mod_add(s16, mod_mul(u31::new(1 << 8 ).unwrap(), self.lfsr[0]));
        s16 = mod_add(s16, mod_mul(u31::new(1 << 20).unwrap(), self.lfsr[4]));
        s16 = mod_add(s16, mod_mul(u31::new(1 << 21).unwrap(), self.lfsr[10]));
        s16 = mod_add(s16, mod_mul(u31::new(1 << 17).unwrap(), self.lfsr[13]));
        s16 = mod_add(s16, mod_mul(u31::new(1 << 15).unwrap(), self.lfsr[15]));

        if s16 == u31!(0) {
            s16 = u31!(0x7FFF_FFFF);
        }
        // dbg!(s16);
        for i in 0..15 {
            self.lfsr[i] = self.lfsr[i + 1];
        }
        self.lfsr[15] = s16; // CHECK CONSISTENCY
    }

    /// 比特重组算法
    ///
    /// 参照 GB/T 33133.1-2016 进行实现
    fn bit_reconstruction(&mut self) {
        self.x[0] = u32::from(self.lfsr[14])
            .bitand(0x0000_FFFF)                // 去除高位
            .bitxor(                                // || 合并操作
                u32::from(self.lfsr[15])
                    .overflowing_shl(1).0   // 左侧需要在原先 15 位的基础上添加一位
                    .bitand(0xFFFF_0000)         // 去除低位
            );
        // println!("{:032b}", self.x[0]);
        self.x[1] = u32::from(self.lfsr[11])
            .bitand(0x0000_FFFF)                // 去除高位
            .overflowing_shl(16).0              // 挪移到高位
            .bitxor(
                u32::from(self.lfsr[9])
                    .overflowing_shr(15).0      // 去除低位
            );
        // println!("{:032b}", self.x[1]);
        self.x[2] = u32::from(self.lfsr[7])
            .bitand(0x0000_FFFF)                // 去除高位
            .overflowing_shl(16).0              // 挪移到高位
            .bitxor(
                u32::from(self.lfsr[5])
                    .overflowing_shr(15).0      // 去除低位
            );
        self.x[3] = u32::from(self.lfsr[2])
            .bitand(0x0000_FFFF)                // 去除高位
            .overflowing_shl(16).0              // 挪移到高位
            .bitxor(
                u32::from(self.lfsr[0])
                    .overflowing_shr(15).0      // 去除低位
            );
    }

    /// 非线性函数 f
    ///
    /// 参照 GB/T 33133.1-2016 进行实现
    fn f(&mut self) -> u32 {
        // w = (x_0 (+) R_1) mod 2^31
        let w = u32::from(self.x[0]).bitxor(self.r1).wrapping_add(self.r2);
        let w1: u32 = self.r1.wrapping_add(self.x[1]);
        let w2: u32 = self.r2.bitxor(self.x[2]);
        // dbg!(w, w1, w2);

        // R_1 = S[L_1(W_1L || W_2H)]
        // R_2 = S[L_2(W_2L || W_1H)]
        let combine1 = l1(w1.overflowing_shl(16).0 | w2.overflowing_shr(16).0);
        let combine2 = l2(w2.overflowing_shl(16).0 | w1.overflowing_shr(16).0);
        // dbg!(combine1, combine2);

        let (xa, xb): (u16, u16) = combine1.bit_split(); // 对于输入数据而言，将其拆分成为 u8
        let (x0, x1): (u8, u8) = xa.bit_split();
        let (x2, x3): (u8, u8) = xb.bit_split();

        self.r1 = // 经过 S0..4 盒的映射之后得到最终结果并合并到一起
            (S0[x0 as usize] as u32) << 24 |
                (S1[x1 as usize] as u32) << 16 |
                (S0[x2 as usize] as u32) << 8 |
                (S1[x3 as usize] as u32);
        // dbg!(self.r1);

        let (xa, xb): (u16, u16) = combine2.bit_split();
        let (x0, x1): (u8, u8) = xa.bit_split();
        let (x2, x3): (u8, u8) = xb.bit_split();

        self.r2 =
            (S0[x0 as usize] as u32) << 24 |
                (S1[x1 as usize] as u32) << 16 |
                (S0[x2 as usize] as u32) << 8  |
                (S1[x3 as usize] as u32);
        // dbg!(self.r2);

        w
    }

}

/// return (a + b) mod (2^31 - 1)
fn mod_add(a: u31, b: u31) -> u31 {
    u31::new(((u64::from(a) + u64::from(b)) % 2147483647u64) as u32).unwrap()
}

/// return (a * b) mod (2^31 - 1)
pub fn mod_mul(a: u31, b: u31) -> u31 {
    // NOTE: 这几种操作结果对应的结果是不一致的
    // a.overflowing_mul(b)
    // a.wrapping_mul(b)
    // = (u64::from(a) * u64::from(b)) & 0x7FFF_FFFF
    u31::new(((u64::from(a) * u64::from(b)) % 0x7FFF_FFFF) as u32).unwrap()
}

/// L1 32 比特线性转换实现
///
/// 参照 GB/T 33133.1-2016 进行实现
fn l1(bits: u32) -> u32 {
    bits ^ bits.rotate_left(2) ^ bits.rotate_left(10) ^ bits.rotate_left(18) ^ bits.rotate_left(24)
}


/// L2 32 比特线性转换实现
///
/// 参照 GB/T 33133.1-2016 进行实现
fn l2(bits: u32) -> u32 {
    bits ^ bits.rotate_left(8) ^ bits.rotate_left(14) ^ bits.rotate_left(22) ^ bits.rotate_left(30)
}

// #[cfg(test)]
// mod tests {
//     use aint::u31;
//     use crate::zuc_encryption::{mod_mul, mod_add};
//     const MOD231SUB1: u32 = 0x7FFF_FFFF;
//
//     #[test]
//     fn test_mod_mul_typical() {
//         assert_eq!(mod_mul(u31!(5), u31!(3)), u31!(15));
//         assert_eq!(mod_mul(u31!(0x7FFF_FFFF), u31!(0x0000_0001)), u31!(0x0000_0000));
//         assert_eq!(mod_mul(u31!(0x7FFF_FFFF), u31!(0x0000_0002)), u31!(0x0000_0000));
//     }
//
//     #[test]
//     fn test_mod_mul_zero() {
//         assert_eq!(mod_mul(u31!(0), u31!(100)), u31!(0));
//         assert_eq!(mod_mul(u31!(100), u31!(0)), u31!(0));
//     }
//
//     fn test_mod_add_typical() {
//         // dbg!(0x7FFF_FFFF, u31::MAX, u32::MAX); // 2147483647 2147483647 4294967295
//         assert_eq!(mod_add(u31!(245), u31!(134)), u31!(379));
//         assert_eq!(mod_add(u31!(0x7FFF_FFFF), u31!(0x0)), u31!(0x7FFF_FFFF));
//         assert_eq!(mod_add(u31!(0x7FFF_FFFF), u31!(0x1)), u31!(0x0));
//         assert_eq!(mod_add(u31!(0x7FFF_FFFF), u31!(0x2)), u31!(0x1));
//     }
//
//     #[test]
//     fn test_mod_add_zero() {
//         assert_eq!(mod_add(u31!(0), u31!(100)), u31!(100));
//         assert_eq!(mod_add(u31!(100), u31!(0)), u31!(100));
//         assert_eq!(mod_add(u31!(0), u31!(0)), u31!(0));
//     }
// }
