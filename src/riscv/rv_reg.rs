use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Reg {
    /// SPECIAL: Always 0
    ZERO = 0,
    /// Return address
    RA = 1,
    /// SPECIAL: Stack pointer
    SP = 2,
    /// Global pointer
    GP = 3,
    /// Thread pointer
    TP = 4,
    /// Temporary
    T0 = 5,
    /// Temporary
    T1 = 6,
    /// Temporary
    T2 = 7,
    /// Saved (Frame Pointer)
    S0 = 8,
    /// Saved
    S1 = 9,
    /// Function arguments / Return values
    A0 = 10,
    /// Function arguments / Return values
    A1 = 11,
    /// Function arguments
    A2 = 12,
    /// Function arguments
    A3 = 13,
    /// Function arguments
    A4 = 14,
    /// Function arguments
    A5 = 15,
    /// Function arguments
    A6 = 16,
    /// Function arguments
    A7 = 17,
    /// Saved
    S2 = 18,
    /// Saved
    S3 = 19,
    /// Saved
    S4 = 20,
    /// Saved
    S5 = 21,
    /// Saved
    S6 = 22,
    /// Saved
    S7 = 23,
    /// Saved
    S8 = 24,
    /// Saved
    S9 = 25,
    /// Saved
    S10 = 26,
    /// Saved
    S11 = 27,
    /// Temporary
    T3 = 28,
    /// Temporary
    T4 = 29,
    /// Temporary
    T5 = 30,
    /// Temporary
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

impl From<u32> for Reg {
    fn from(reg: u32) -> Self {
        match reg {
            0 => Reg::ZERO,
            1 => Reg::RA,
            2 => Reg::SP,
            3 => Reg::GP,
            4 => Reg::TP,
            5 => Reg::T0,
            6 => Reg::T1,
            7 => Reg::T2,
            8 => Reg::S0,
            9 => Reg::S1,
            10 => Reg::A0,
            11 => Reg::A1,
            12 => Reg::A2,
            13 => Reg::A3,
            14 => Reg::A4,
            15 => Reg::A5,
            16 => Reg::A6,
            17 => Reg::A7,
            18 => Reg::S2,
            19 => Reg::S3,
            20 => Reg::S4,
            21 => Reg::S5,
            22 => Reg::S6,
            23 => Reg::S7,
            24 => Reg::S8,
            25 => Reg::S9,
            26 => Reg::S10,
            27 => Reg::S11,
            28 => Reg::T3,
            29 => Reg::T4,
            30 => Reg::T5,
            31 => Reg::T6,
            _ => unreachable!(),
        }
    }
}
