//! 基于祖冲之算法的简单加密实现
//!
//! # 国家标准全文:
//! [GB/T 33133.1-2016 信息安全技术 祖冲之序列密码算法 第1部分：算法描述](https://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=8C41A3AEECCA52B5C0011C8010CF0715)
//! [GB/T 33133.2-2021 信息安全技术 祖冲之序列密码算法 第2部分：保密性算法](https://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=5D3CBA3ADEC7989344BD1E63006EF2B3)
//! [GB/T 33133.3-2021 信息安全技术 祖冲之序列密码算法 第3部分：完整性算法](https://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=C6D60AE0A7578E970EF2280ABD49F4F0)
//!
//! # 功能:
//! - 对于输入字符进行 PKCS#7 填充
//! - 使用祖冲之算法对于填充之后的块进行加密
//! - 将加密之后的数据编码为 base64 以方便阅读与传输
//!
//! # 输入:
//!     input: 需要加密的字符串
//!     有关于 密钥生成的 k, iv 默认采用 [0u8;16] 和 [1u8; 16]
//!
//! # 示例:
//! ```rust
//!     let inp = String::from("特朗普");
//!     let result = zuc_encryption::encryption(inp.to_string());
//! ```

use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, BitAnd, BitXor, Mul, Shl, Shr};

use byteorder::WriteBytesExt;
use base64::Engine;

use regs::Regs;

/// zuc 算法所必要的常量矩阵
mod consts;
/// 获取根据 zuc 算法生成的密钥
mod regs;

pub fn encryption(input: String) -> String {
    // Base on requirement
    let k = [0_u8; 16];
    let iv = [1_u8; 16];

    // 根据 zuc 算法要求进行初始化
    let mut regs = Regs::new(k, iv); // CHECK CONSISTENCY
    let mut input_bytes = input.into_bytes();

    // PKCS#7 padding
    // 当使用块大小为32位（4字节）的加密算法时，填充的内容应该是一个值，这个值等于需要填充的字节数。
    let padding_length = 4 - input_bytes.len() % 4;
    input_bytes.append(&mut vec![padding_length as u8; padding_length]); // CHECK CONSISTENCY

    // 根据 祖冲之算法 要求,生成一系列的密钥字
    let mut encrypted_key = vec![];
    for _ in 0..input_bytes.len() / 4 {
        encrypted_key.push(regs.generate().to_be());
    }
    // println!("{:?}", &encrypted_key); // CHECK CONSISTENCY

    // 参考保密性算法实现
    let mut obs: Vec<u8> = vec![];
    for (i, &byte) in input_bytes.iter().enumerate() {
        // NOTE: 每一个中文会被拆分成为四个 bytes
        let key = encrypted_key[i / 4];
        // 對於每一個字節而言，與其對應的加密密鑰進行異或
        let text = byte.bitxor((key >> (8 * (i % 4)) & 0xFF) as u8);
        obs.push(text);
    }

    base64::encode(&obs)
}
