use std::fmt;

/// signed integers that stores 24-bit numbers
#[derive(Clone, PartialEq, Debug, Hash)]
pub struct I24([u8; 3]);

/// unsigned integers that stores 24-bit numbers
#[derive(Clone, PartialEq, Debug, Hash)]
pub struct U24([u8; 3]);

// signed integers that stores 24-bit numbers with the lowest LOW_ZEROED_BITS bits zeroed
#[derive(Clone, PartialEq, Debug, Hash)]
pub struct I24WithZeroedBits<const LOW_ZEROED_BITS: u8>([u8; 3]);

impl fmt::Display for I24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_i32(), f)
    }
}

impl fmt::Display for U24 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_u32(), f)
    }
}

impl<const LOW_ZEROED_BITS: u8> fmt::Display
    for I24WithZeroedBits<LOW_ZEROED_BITS>
{
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_i32(), f)
    }
}

impl I24 {
    pub const fn from_i32(v: i32) -> Self {
        let b = v.to_le_bytes();
        Self([b[0], b[1], b[2]])
    }

    pub const fn to_i32(&self) -> i32 {
        let [a, b, c] = self.0;
        i32::from_ne_bytes([a, b, c, 0]) << u8::BITS >> u8::BITS
    }
}

impl U24 {
    pub const fn from_u32(v: u32) -> Self {
        let b = v.to_le_bytes();
        Self([b[0], b[1], b[2]])
    }

    pub const fn to_u32(&self) -> u32 {
        let [a, b, c] = self.0;
        u32::from_ne_bytes([a, b, c, 0])
    }
}

impl<const LOW_ZEROED_BITS: u8> I24WithZeroedBits<LOW_ZEROED_BITS> {
    pub const fn from_i32(v_original: i32) -> Self {
        let v = v_original >> LOW_ZEROED_BITS;
        let b = v.to_le_bytes();
        Self([b[0], b[1], b[2]])
    }

    pub const fn to_i32(&self) -> i32 {
        let [a, b, c] = self.0;
        // Sign-extend and shift back
        ((((i32::from_le_bytes([a, b, c, 0]) << u8::BITS >> u8::BITS)
            << LOW_ZEROED_BITS) as u32)
            & (u32::MAX << LOW_ZEROED_BITS)) as i32
    }
}
