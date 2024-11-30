use std::ops::{Add, AddAssign, BitAnd, Shl, Shr};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MyU31(u32);


impl Add<MyU31> for MyU31 {
    type Output = MyU31;

    fn add(self, rhs: MyU31) -> Self::Output {
        MyU31(self.0 + rhs.0)
    }
}
impl Add<MyU31> for &mut u32 {
    type Output = ();

    fn add(self, rhs: MyU31) -> Self::Output {
        self + rhs
    }
}

impl Add<MyU31> for &mut MyU31 {
    type Output = MyU31;

    fn add(self, rhs: MyU31) -> Self::Output {
        self + rhs
    }
}
impl AddAssign for MyU31 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl BitAnd<MyU31> for MyU31 {
    type Output = MyU31;

    fn bitand(self, rhs: MyU31) -> Self::Output {
        self & rhs
    }
}


impl Shl<i32> for MyU31 {
    type Output = MyU31;

    fn shl(self, rhs: i32) -> Self::Output {
        self << rhs
    }
}
impl Shr<MyU31> for MyU31 {
    type Output = MyU31;

    fn shr(self, rhs: MyU31) -> Self::Output {
        self >> rhs
    }
}
impl From<MyU31> for u128 {
    fn from(value: MyU31) -> Self {
        (u128::from(value.0) << 96) | 0xFFFF_FFFF_FFFF_FFFF
    }
}

impl From<u32> for MyU31 {
    fn from(value: u32) -> Self {
        MyU31(value)
    }
}
impl From<u128> for MyU31 {
    fn from(value: u128) -> Self {
        MyU31::from((value & 0x7FFFFFFF_FFFFFFFF) as u32)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct MyU32(u32);

impl From<u32> for MyU32 {
    fn from(value: u32) -> Self {
        MyU32(value)
    }
}

impl MyU32 {
    pub(crate) fn combine_u31(a: MyU31, b: MyU31) -> MyU32 {
        MyU32::from(a << 16 & b)
    }
}
impl From<MyU31> for MyU32 {
    fn from(value: MyU31) -> Self {
        MyU32::from(value.0)
    }
}

impl Add<MyU32> for MyU32 {
    type Output = MyU32;

    fn add(self, rhs: MyU32) -> Self::Output {
        MyU32(self.0 + rhs.0)
    }
}

impl BitAnd<MyU32> for MyU32 {
    type Output = MyU32;

    fn bitand(self, rhs: MyU32) -> Self::Output {
        self & rhs
    }
}

