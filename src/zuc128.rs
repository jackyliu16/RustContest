//! ZUC-128 Algorithms

use crate::consts::{D, S0, S1};
use std::mem;

/// (a + b) mod (2^32)
#[inline(always)]
fn add(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

/// rotate left
#[inline(always)]
fn rol(x: u32, n: u32) -> u32 {
    x.rotate_left(n)
}

/// L1 linear transform
#[inline(always)]
fn l1(x: u32) -> u32 {
    x ^ rol(x, 2) ^ rol(x, 10) ^ rol(x, 18) ^ rol(x, 24)
}

/// L2 linear transform
#[inline(always)]
fn l2(x: u32) -> u32 {
    x ^ rol(x, 8) ^ rol(x, 14) ^ rol(x, 22) ^ rol(x, 30)
}

/// S box transform
#[inline(always)]
fn sbox(x: u32) -> u32 {
    let x = x.to_be_bytes();
    let y = [
        S0[x[0] as usize],
        S1[x[1] as usize],
        S0[x[2] as usize],
        S1[x[3] as usize],
    ];
    u32::from_be_bytes(y)
}

/// (a * b) mod (2^31 - 1)
#[inline(always)]
fn mul_m31(a: u32, b: u32) -> u32 {
    ((u64::from(a) * u64::from(b)) % ((1 << 31) - 1)) as u32
}

/// (a + b) mod (2^31 - 1)
#[inline(always)]
fn add_m31(a: u32, b: u32) -> u32 {
    let c = add(a, b);
    (c & 0x7FFF_FFFF) + (c >> 31)
}

/// ZUC128 keystream generator
/// ([GB/T 33133.1-2016](https://openstd.samr.gov.cn/bzgk/gb/newGbInfo?hcno=8C41A3AEECCA52B5C0011C8010CF0715))
#[derive(Debug, Clone)]
pub struct ZUC128 {
    /// LFSR registers (31-bit words x16)
    s: [u32; 16],

    /// R1 state unit (32 bits)
    r1: u32,

    /// R2 state unit (32 bits)
    r2: u32,

    /// X buffer
    x: [u32; 4],
}

impl ZUC128 {
    /// Zero-initialized
    #[allow(unsafe_code)]
    fn zeroed() -> Self {
        unsafe { mem::zeroed() }
    }

    /// Creates a ZUC128 keystream generator
    #[must_use]
    pub fn new(key: &[u8; 16], iv: &[u8; 16]) -> Self {
        let mut zuc = Self::zeroed();

        for i in 0..16 {
            let k_i = u32::from(key[i]);
            let d_i = u32::from(D[i]);
            let iv_i = u32::from(iv[i]);
            zuc.s[i] = (k_i << 23) | (d_i << 8) | iv_i;
        }

        for _ in 0..32 {
            zuc.bit_reconstruction();
            let w = zuc.f();
            zuc.lfsr_with_initialization_mode(w >> 1);
        }
        zuc.generate();

        zuc
    }

    /// `BitReconstruction` function
    fn bit_reconstruction(&mut self) {
        let Self { s, x, .. } = self;
        x[0] = ((s[15] & 0x7FFF_8000) << 1) | (s[14] & 0xFFFF);
        x[1] = ((s[11] & 0xFFFF) << 16) | (s[9] >> 15);
        x[2] = ((s[7] & 0xFFFF) << 16) | (s[5] >> 15);
        x[3] = ((s[2] & 0xFFFF) << 16) | (s[0] >> 15);
    }

    /// F non-linear function
    fn f(&mut self) -> u32 {
        let Self { x, r1, r2, .. } = self;

        let w = add(x[0] ^ (*r1), *r2);
        let w1 = add(*r1, x[1]);
        let w2 = (*r2) ^ x[2];
        *r1 = sbox(l1((w1 << 16) | (w2 >> 16)));
        *r2 = sbox(l2((w2 << 16) | (w1 >> 16)));

        w
    }

    /// `LFSRWithInitialisationMode` function
    fn lfsr_with_initialization_mode(&mut self, u: u32) {
        let Self { s, .. } = self;
        let v = {
            let v1 = mul_m31(1 << 15, s[15]);
            let v2 = mul_m31(1 << 17, s[13]);
            let v3 = mul_m31(1 << 21, s[10]);
            let v4 = mul_m31(1 << 20, s[4]);
            let v5 = mul_m31((1 << 8) + 1, s[0]);
            add_m31(v1, add_m31(v2, add_m31(v3, add_m31(v4, v5))))
        };
        let mut s16 = add_m31(v, u);
        if s16 == 0 {
            s16 = (1 << 31) - 1;
        }
        for i in 0..15 {
            s[i] = s[i + 1];
        }
        s[15] = s16;
    }

    /// `LFSRWithWorkMode` function
    fn lfsr_with_work_mode(&mut self) {
        let Self { s, .. } = self;
        let v = {
            let v1 = mul_m31(1 << 15, s[15]);
            let v2 = mul_m31(1 << 17, s[13]);
            let v3 = mul_m31(1 << 21, s[10]);
            let v4 = mul_m31(1 << 20, s[4]);
            let v5 = mul_m31((1 << 8) + 1, s[0]);
            add_m31(v1, add_m31(v2, add_m31(v3, add_m31(v4, v5))))
        };
        let mut s16 = v;
        if s16 == 0 {
            s16 = (1 << 31) - 1;
        }
        for i in 0..15 {
            s[i] = s[i + 1];
        }
        s[15] = s16;
    }

    /// Generates the next 32-bit word in ZUC128 keystream
    pub fn generate(&mut self) -> u32 {
        self.bit_reconstruction();
        let z = self.f() ^ self.x[3];
        self.lfsr_with_work_mode();
        z
    }
}

impl Iterator for ZUC128 {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.generate())
    }
}

#[cfg(test)]
mod tests {
    use super::ZUC128;

    struct Example {
        k: [u8; 16],
        iv: [u8; 16],
        expected: [[u32; 8]; 3],
    }

    static EXAMPLE1: Example = Example {
        k: [0; 16],
        iv: [0; 16],
        expected: [
            [
                0x7c37_ba6b,
                0xb136_7f6c,
                0x1e42_6568,
                0xdd0b_f9c2,
                0x3512_bf50,
                0xa092_0453,
                0x286d_afe5,
                0x7f08_e141,
            ],
            [
                0xfe11_8d6a,
                0xd452_2c3a,
                0xe955_463d,
                0x4c2b_e8f9,
                0xc7ee_7f13,
                0x0c0f_a817,
                0x27be_de74,
                0x3d38_3d04,
            ],
            [
                0x7a70_e141,
                0x9a74_e229,
                0x071e_62e2,
                0xc82e_c4b3,
                0xdde6_3da7,
                0xb9dd_6a41,
                0x0180_82da,
                0x13d6_d780,
            ],
        ],
    };

    static EXAMPLE2: Example = Example {
        k: [0xff; 16],
        iv: [0xff; 16],
        expected: [
            [
                0x3fc8_1ce8,
                0xc2d1_41d1,
                0x4bd0_8879,
                0x4227_1346,
                0xaa13_1b11,
                0x09d7_706c,
                0x668b_56df,
                0x13f5_6dbf,
            ],
            [
                0x27ea_6106,
                0x82c8_f4b6,
                0x0b14_d499,
                0x9187_2523,
                0x251e_7804,
                0xcaac_5d66,
                0x0657_cfa0,
                0x0c0f_e353,
            ],
            [
                0x181f_6dbf,
                0x04a2_1879,
                0xf24c_93c6,
                0x773b_4aaa,
                0xd94e_9228,
                0x91d8_8fba,
                0x7096_398b,
                0x10f1_eecf,
            ],
        ],
    };

    static EXAMPLE3: Example = Example {
        k: [
            0x3d, 0x4c, 0x4b, 0xe9, 0x6a, 0x82, 0xfd, 0xae, //
            0xb5, 0x8f, 0x64, 0x1d, 0xb1, 0x7b, 0x45, 0x5b, //
        ],
        iv: [
            0x84, 0x31, 0x9a, 0xa8, 0xde, 0x69, 0x15, 0xca, //
            0x1f, 0x6b, 0xda, 0x6b, 0xfb, 0xd8, 0xc7, 0x66, //
        ],
        expected: [
            [
                0xf534_2a57,
                0x6e20_ef69,
                0x5d6a_8f32,
                0x0ce1_21b4,
                0x129d_8b39,
                0x2d7c_dce1,
                0x3ead_461d,
                0x3d4a_a9e7,
            ],
            [
                0x7a95_1cff,
                0x40b9_2b65,
                0x0a37_4ea7,
                0x8174_b6d5,
                0xab7c_f688,
                0xc159_8aa6,
                0x14f1_c272,
                0x71db_1828,
            ],
            [
                0xe3b6_a9e7,
                0x5503_49fe,
                0xaf31_e6ee,
                0x385a_2e0c,
                0x3cec_1a4a,
                0x9053_cc0e,
                0x3279_c419,
                0x2589_37da,
            ],
        ],
    };

    #[test]
    fn examples() {
        for Example { k, iv, expected } in [&EXAMPLE1, &EXAMPLE2, &EXAMPLE3] {
            let mut zuc = ZUC128::new(k, iv);

            assert_eq!(zuc.x, expected[0][..4]);
            assert_eq!(zuc.r1, expected[0][4]);
            assert_eq!(zuc.r2, expected[0][5]);
            assert_eq!(zuc.s[15], expected[0][7]);

            let z1 = zuc.generate();

            assert_eq!(zuc.x, expected[1][..4]);
            assert_eq!(zuc.r1, expected[1][4]);
            assert_eq!(zuc.r2, expected[1][5]);
            assert_eq!(z1, expected[1][6]);
            assert_eq!(zuc.s[15], expected[1][7]);

            let z2 = zuc.generate();

            assert_eq!(zuc.x, expected[2][..4]);
            assert_eq!(zuc.r1, expected[2][4]);
            assert_eq!(zuc.r2, expected[2][5]);
            assert_eq!(z2, expected[2][6]);
            assert_eq!(zuc.s[15], expected[2][7]);
        }
    }
}
