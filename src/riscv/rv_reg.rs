use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Reg {
    /// SPECIAL: Always 0
    /// x0
    ZERO = 0,

    /// Return address
    /// x1
    RA = 1,

    /// SPECIAL: Stack pointer
    /// x2
    SP = 2,

    /// Global pointer
    /// x3
    GP = 3,

    /// Thread pointer
    /// x4
    TP = 4,

    /// Temporary
    /// x5
    T0 = 5,

    /// Temporary
    /// x6
    T1 = 6,

    /// Temporary
    /// x7
    T2 = 7,

    /// Saved (Frame Pointer)
    /// x8
    S0 = 8,

    /// Saved
    /// x9
    S1 = 9,

    /// Function arguments / Return values
    /// x10
    A0 = 10,

    /// Function arguments / Return values
    /// x11
    A1 = 11,

    /// Function arguments
    /// x12
    A2 = 12,

    /// Function arguments
    /// x13
    A3 = 13,

    /// Function arguments
    /// x14
    A4 = 14,

    /// Function arguments
    /// x15
    A5 = 15,

    /// Function arguments
    /// x16
    A6 = 16,

    /// Function arguments
    /// x17
    A7 = 17,

    /// Saved
    /// x18
    S2 = 18,

    /// Saved
    /// x19
    S3 = 19,

    /// Saved
    /// x20
    S4 = 20,

    /// Saved
    /// x21
    S5 = 21,

    /// Saved
    /// x22
    S6 = 22,

    /// Saved
    /// x23
    S7 = 23,

    /// Saved
    /// x24
    S8 = 24,

    /// Saved
    /// x25
    S9 = 25,

    /// Saved
    /// x26
    S10 = 26,

    /// Saved
    /// x27
    S11 = 27,

    /// Temporary
    /// x28
    T3 = 28,

    /// Temporary
    /// x29
    T4 = 29,

    /// Temporary
    /// x30
    T5 = 30,

    /// Temporary
    /// x31
    T6 = 31,
}

impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Reg::ZERO => "zero",
            Reg::RA => "ra",
            Reg::SP => "sp",
            Reg::GP => "gp",
            Reg::TP => "tp",
            Reg::T0 => "t0",
            Reg::T1 => "t1",
            Reg::T2 => "t2",
            Reg::S0 => "s0",
            Reg::S1 => "s1",
            Reg::A0 => "a0",
            Reg::A1 => "a1",
            Reg::A2 => "a2",
            Reg::A3 => "a3",
            Reg::A4 => "a4",
            Reg::A5 => "a5",
            Reg::A6 => "a6",
            Reg::A7 => "a7",
            Reg::S2 => "s2",
            Reg::S3 => "s3",
            Reg::S4 => "s4",
            Reg::S5 => "s5",
            Reg::S6 => "s6",
            Reg::S7 => "s7",
            Reg::S8 => "s8",
            Reg::S9 => "s9",
            Reg::S10 => "s10",
            Reg::S11 => "s11",
            Reg::T3 => "t3",
            Reg::T4 => "t4",
            Reg::T5 => "t5",
            Reg::T6 => "t6",
        };
        f.write_str(name)
    }
}

impl Reg {
    pub fn from_u8(num: u8) -> Self {
        match num {
            0 => Self::ZERO,
            1 => Self::RA,
            2 => Self::SP,
            3 => Self::GP,
            4 => Self::TP,
            5 => Self::T0,
            6 => Self::T1,
            7 => Self::T2,
            8 => Self::S0,
            9 => Self::S1,
            10 => Self::A0,
            11 => Self::A1,
            12 => Self::A2,
            13 => Self::A3,
            14 => Self::A4,
            15 => Self::A5,
            16 => Self::A6,
            17 => Self::A7,
            18 => Self::S2,
            19 => Self::S3,
            20 => Self::S4,
            21 => Self::S5,
            22 => Self::S6,
            23 => Self::S7,
            24 => Self::S8,
            25 => Self::S9,
            26 => Self::S10,
            27 => Self::S11,
            28 => Self::T3,
            29 => Self::T4,
            30 => Self::T5,
            31 => Self::T6,
            _ => unreachable!(),
        }
    }
    #[inline]
    pub fn num(self) -> u8 {
        self as u8
    }
    #[inline]
    pub fn zero() -> Self {
        Self::ZERO
    }
    #[inline]
    pub fn ra() -> Self {
        Self::RA
    }
    #[inline]
    pub fn sp() -> Self {
        Self::SP
    }
    #[inline]
    pub fn gp() -> Self {
        Self::GP
    }
    #[inline]
    pub fn tp() -> Self {
        Self::TP
    }
    #[inline]
    pub fn t0() -> Self {
        Self::T0
    }
    #[inline]
    pub fn t1() -> Self {
        Self::T1
    }
    #[inline]
    pub fn t2() -> Self {
        Self::T2
    }
    #[inline]
    pub fn s0() -> Self {
        Self::S0
    }
    #[inline]
    pub fn fp() -> Self {
        Self::S0
    }
    #[inline]
    pub fn s1() -> Self {
        Self::S1
    }
    #[inline]
    pub fn a0() -> Self {
        Self::A0
    }
    #[inline]
    pub fn a1() -> Self {
        Self::A1
    }
    #[inline]
    pub fn a2() -> Self {
        Self::A2
    }
    #[inline]
    pub fn a3() -> Self {
        Self::A3
    }
    #[inline]
    pub fn a4() -> Self {
        Self::A4
    }
    #[inline]
    pub fn a5() -> Self {
        Self::A5
    }
    #[inline]
    pub fn a6() -> Self {
        Self::A6
    }
    #[inline]
    pub fn a7() -> Self {
        Self::A7
    }
    #[inline]
    pub fn s2() -> Self {
        Self::S2
    }
    #[inline]
    pub fn s3() -> Self {
        Self::S3
    }
    #[inline]
    pub fn s4() -> Self {
        Self::S4
    }
    #[inline]
    pub fn s5() -> Self {
        Self::S5
    }
    #[inline]
    pub fn s6() -> Self {
        Self::S6
    }
    #[inline]
    pub fn s7() -> Self {
        Self::S7
    }
    #[inline]
    pub fn s8() -> Self {
        Self::S8
    }
    #[inline]
    pub fn s9() -> Self {
        Self::S9
    }
    #[inline]
    pub fn s10() -> Self {
        Self::S10
    }
    #[inline]
    pub fn s11() -> Self {
        Self::S11
    }
    #[inline]
    pub fn t3() -> Self {
        Self::T3
    }
    #[inline]
    pub fn t4() -> Self {
        Self::T4
    }
    #[inline]
    pub fn t5() -> Self {
        Self::T5
    }
    #[inline]
    pub fn t6() -> Self {
        Self::T6
    }
}
