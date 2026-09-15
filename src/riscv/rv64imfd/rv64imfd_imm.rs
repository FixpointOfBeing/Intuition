use std::fmt;

use bitvec::prelude::*;

/// 12 位有符号立即数（I/S 型）
#[derive(Clone, PartialEq, Debug, Hash)]
pub struct Imm12(BitArr!(for 12, in u8));

/// 13 位有符号立即数，最低 1 位补 0（B 型）
#[derive(Clone, PartialEq, Debug, Hash)]
pub struct Imm13LowZeroBits1(BitArr!(for 13, in u8));

/// 21 位有符号立即数，最低 1 位补 0（J 型）
#[derive(Clone, PartialEq, Debug, Hash)]
pub struct Imm21LowZeroBits1(BitArr!(for 21, in u8));

/// 32 位有符号立即数，最低 12 位补 0（U 型）
#[derive(Clone, PartialEq, Debug, Hash)]
pub struct Imm32LowZeroBits12(BitArr!(for 32, in u8));

/// 6 位无符号移位量（slli/srli/srai）
#[derive(Clone, PartialEq, Debug, Hash)]
pub struct Shamt6(BitArr!(for 6, in u8));

/// 5 位无符号移位量（slliw/srliw/sraiw）
#[derive(Clone, PartialEq, Debug, Hash)]
pub struct Shamt5(BitArr!(for 5, in u8));

impl fmt::Display for Imm12 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_i16(), f)
    }
}

impl fmt::Display for Imm13LowZeroBits1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_i16(), f)
    }
}

impl fmt::Display for Imm21LowZeroBits1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_i32(), f)
    }
}

impl fmt::Display for Imm32LowZeroBits12 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_i32(), f)
    }
}

impl fmt::Display for Shamt6 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_u8(), f)
    }
}

impl fmt::Display for Shamt5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_u8(), f)
    }
}

impl Imm12 {
    pub fn from_i16(v: i16) -> Self {
        let mut bits: BitArr!(for 12, in u8) = Default::default();
        bits.store_le((v as u16) & 0x0FFF);
        Self(bits)
    }

    pub fn to_i16(&self) -> i16 {
        let raw = self.0.load_le::<u16>();
        ((raw << 4) as i16) >> 4
    }
}

impl Imm13LowZeroBits1 {
    pub fn from_i16(offset: i16) -> Self {
        let mut bits: BitArr!(for 13, in u8) = Default::default();
        bits.store_le(((offset >> 1) as u16) & 0x1FFF);
        Self(bits)
    }

    pub fn to_i16(&self) -> i16 {
        let raw = self.0.load_le::<u16>();
        (((raw << 3) as i16) >> 3) << 1
    }
}

impl Imm21LowZeroBits1 {
    pub fn from_i32(offset: i32) -> Self {
        let mut bits: BitArr!(for 21, in u8) = Default::default();
        bits.store_le(((offset >> 1) as u32) & 0x1F_FFFF);
        Self(bits)
    }

    pub fn to_i32(&self) -> i32 {
        let raw = self.0.load_le::<u32>();
        (((raw << 11) as i32) >> 11) << 1
    }
}

impl Imm32LowZeroBits12 {
    pub fn from_i32(v: i32) -> Self {
        let mut bits: BitArr!(for 32, in u8) = Default::default();
        bits.store_le(((v >> 12) as u32) & 0xF_FFFF);
        Self(bits)
    }

    pub fn to_i32(&self) -> i32 {
        let raw = self.0.load_le::<u32>();
        (((raw << 12) as i32) >> 12) << 12
    }
}

impl Shamt6 {
    pub fn from_u8(v: u8) -> Self {
        let mut bits: BitArr!(for 6, in u8) = Default::default();
        bits.store_le(v & 0x3F);
        Self(bits)
    }

    pub fn to_u8(&self) -> u8 {
        self.0.load_le::<u8>()
    }
}

impl Shamt5 {
    pub fn from_u8(v: u8) -> Self {
        let mut bits: BitArr!(for 5, in u8) = Default::default();
        bits.store_le(v & 0x1F);
        Self(bits)
    }

    pub fn to_u8(&self) -> u8 {
        self.0.load_le::<u8>()
    }
}
