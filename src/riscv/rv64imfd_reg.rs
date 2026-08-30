use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum IReg {
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

impl fmt::Display for IReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            IReg::ZERO => "zero",
            IReg::RA => "ra",
            IReg::SP => "sp",
            IReg::GP => "gp",
            IReg::TP => "tp",
            IReg::T0 => "t0",
            IReg::T1 => "t1",
            IReg::T2 => "t2",
            IReg::S0 => "s0",
            IReg::S1 => "s1",
            IReg::A0 => "a0",
            IReg::A1 => "a1",
            IReg::A2 => "a2",
            IReg::A3 => "a3",
            IReg::A4 => "a4",
            IReg::A5 => "a5",
            IReg::A6 => "a6",
            IReg::A7 => "a7",
            IReg::S2 => "s2",
            IReg::S3 => "s3",
            IReg::S4 => "s4",
            IReg::S5 => "s5",
            IReg::S6 => "s6",
            IReg::S7 => "s7",
            IReg::S8 => "s8",
            IReg::S9 => "s9",
            IReg::S10 => "s10",
            IReg::S11 => "s11",
            IReg::T3 => "t3",
            IReg::T4 => "t4",
            IReg::T5 => "t5",
            IReg::T6 => "t6",
        };
        f.write_str(name)
    }
}

impl IReg {
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


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FReg {
    /// 临时寄存器
    /// f0
    FT0 = 0,

    /// 临时寄存器
    /// f1
    FT1 = 1,

    /// 临时寄存器
    /// f2
    FT2 = 2,

    /// 临时寄存器
    /// f3
    FT3 = 3,

    /// 临时寄存器
    /// f4
    FT4 = 4,

    /// 临时寄存器
    /// f5
    FT5 = 5,

    /// 临时寄存器
    /// f6
    FT6 = 6,

    /// 临时寄存器
    /// f7
    FT7 = 7,

    /// 保存寄存器
    /// f8
    FS0 = 8,

    /// 保存寄存器
    /// f9
    FS1 = 9,

    /// 函数参数 / 返回值
    /// f10
    FA0 = 10,

    /// 函数参数 / 返回值
    /// f11
    FA1 = 11,

    /// 函数参数
    /// f12
    FA2 = 12,

    /// 函数参数
    /// f13
    FA3 = 13,

    /// 函数参数
    /// f14
    FA4 = 14,

    /// 函数参数
    /// f15
    FA5 = 15,

    /// 函数参数
    /// f16
    FA6 = 16,

    /// 函数参数
    /// f17
    FA7 = 17,

    /// 保存寄存器
    /// f18
    FS2 = 18,

    /// 保存寄存器
    /// f19
    FS3 = 19,

    /// 保存寄存器
    /// f20
    FS4 = 20,

    /// 保存寄存器
    /// f21
    FS5 = 21,

    /// 保存寄存器
    /// f22
    FS6 = 22,

    /// 保存寄存器
    /// f23
    FS7 = 23,

    /// 保存寄存器
    /// f24
    FS8 = 24,

    /// 保存寄存器
    /// f25
    FS9 = 25,

    /// 保存寄存器
    /// f26
    FS10 = 26,

    /// 保存寄存器
    /// f27
    FS11 = 27,

    /// 临时寄存器
    /// f28
    FT8 = 28,

    /// 临时寄存器
    /// f29
    FT9 = 29,

    /// 临时寄存器
    /// f30
    FT10 = 30,

    /// 临时寄存器
    /// f31
    FT11 = 31,
}

impl fmt::Display for FReg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            FReg::FT0 => "ft0",
            FReg::FT1 => "ft1",
            FReg::FT2 => "ft2",
            FReg::FT3 => "ft3",
            FReg::FT4 => "ft4",
            FReg::FT5 => "ft5",
            FReg::FT6 => "ft6",
            FReg::FT7 => "ft7",
            FReg::FS0 => "fs0",
            FReg::FS1 => "fs1",
            FReg::FA0 => "fa0",
            FReg::FA1 => "fa1",
            FReg::FA2 => "fa2",
            FReg::FA3 => "fa3",
            FReg::FA4 => "fa4",
            FReg::FA5 => "fa5",
            FReg::FA6 => "fa6",
            FReg::FA7 => "fa7",
            FReg::FS2 => "fs2",
            FReg::FS3 => "fs3",
            FReg::FS4 => "fs4",
            FReg::FS5 => "fs5",
            FReg::FS6 => "fs6",
            FReg::FS7 => "fs7",
            FReg::FS8 => "fs8",
            FReg::FS9 => "fs9",
            FReg::FS10 => "fs10",
            FReg::FS11 => "fs11",
            FReg::FT8 => "ft8",
            FReg::FT9 => "ft9",
            FReg::FT10 => "ft10",
            FReg::FT11 => "ft11",
        };
        f.write_str(name)
    }
}

impl FReg {
    pub fn from_u8(num: u8) -> Self {
        match num {
            0 => Self::FT0,
            1 => Self::FT1,
            2 => Self::FT2,
            3 => Self::FT3,
            4 => Self::FT4,
            5 => Self::FT5,
            6 => Self::FT6,
            7 => Self::FT7,
            8 => Self::FS0,
            9 => Self::FS1,
            10 => Self::FA0,
            11 => Self::FA1,
            12 => Self::FA2,
            13 => Self::FA3,
            14 => Self::FA4,
            15 => Self::FA5,
            16 => Self::FA6,
            17 => Self::FA7,
            18 => Self::FS2,
            19 => Self::FS3,
            20 => Self::FS4,
            21 => Self::FS5,
            22 => Self::FS6,
            23 => Self::FS7,
            24 => Self::FS8,
            25 => Self::FS9,
            26 => Self::FS10,
            27 => Self::FS11,
            28 => Self::FT8,
            29 => Self::FT9,
            30 => Self::FT10,
            31 => Self::FT11,
            _ => unreachable!(),
        }
    }
    #[inline]
    pub fn num(self) -> u8 {
        self as u8
    }
    #[inline]
    pub fn ft0() -> Self {
        Self::FT0
    }
    #[inline]
    pub fn ft1() -> Self {
        Self::FT1
    }
    #[inline]
    pub fn ft2() -> Self {
        Self::FT2
    }
    #[inline]
    pub fn ft3() -> Self {
        Self::FT3
    }
    #[inline]
    pub fn ft4() -> Self {
        Self::FT4
    }
    #[inline]
    pub fn ft5() -> Self {
        Self::FT5
    }
    #[inline]
    pub fn ft6() -> Self {
        Self::FT6
    }
    #[inline]
    pub fn ft7() -> Self {
        Self::FT7
    }
    #[inline]
    pub fn fs0() -> Self {
        Self::FS0
    }
    #[inline]
    pub fn fs1() -> Self {
        Self::FS1
    }
    #[inline]
    pub fn fa0() -> Self {
        Self::FA0
    }
    #[inline]
    pub fn fa1() -> Self {
        Self::FA1
    }
    #[inline]
    pub fn fa2() -> Self {
        Self::FA2
    }
    #[inline]
    pub fn fa3() -> Self {
        Self::FA3
    }
    #[inline]
    pub fn fa4() -> Self {
        Self::FA4
    }
    #[inline]
    pub fn fa5() -> Self {
        Self::FA5
    }
    #[inline]
    pub fn fa6() -> Self {
        Self::FA6
    }
    #[inline]
    pub fn fa7() -> Self {
        Self::FA7
    }
    #[inline]
    pub fn fs2() -> Self {
        Self::FS2
    }
    #[inline]
    pub fn fs3() -> Self {
        Self::FS3
    }
    #[inline]
    pub fn fs4() -> Self {
        Self::FS4
    }
    #[inline]
    pub fn fs5() -> Self {
        Self::FS5
    }
    #[inline]
    pub fn fs6() -> Self {
        Self::FS6
    }
    #[inline]
    pub fn fs7() -> Self {
        Self::FS7
    }
    #[inline]
    pub fn fs8() -> Self {
        Self::FS8
    }
    #[inline]
    pub fn fs9() -> Self {
        Self::FS9
    }
    #[inline]
    pub fn fs10() -> Self {
        Self::FS10
    }
    #[inline]
    pub fn fs11() -> Self {
        Self::FS11
    }
    #[inline]
    pub fn ft8() -> Self {
        Self::FT8
    }
    #[inline]
    pub fn ft9() -> Self {
        Self::FT9
    }
    #[inline]
    pub fn ft10() -> Self {
        Self::FT10
    }
    #[inline]
    pub fn ft11() -> Self {
        Self::FT11
    }
}
