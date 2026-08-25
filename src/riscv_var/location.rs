use crate::riscv::rv_reg::Reg;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum RvVarLocation {
    Var(String),
    Reg(Reg),
}

#[inline]
pub fn var(s: String) -> RvVarLocation {
    RvVarLocation::Var(s)
}

#[inline]
pub fn x(n: u8) -> RvVarLocation {
    RvVarLocation::Reg(Reg::from_u8(n))
}

#[inline]
pub fn x0() -> RvVarLocation {
    x(0)
}

#[inline]
pub fn zero() -> RvVarLocation {
    RvVarLocation::Reg(Reg::zero())
}

#[inline]
pub fn ra() -> RvVarLocation {
    RvVarLocation::Reg(Reg::ra())
}

#[inline]
pub fn sp() -> RvVarLocation {
    RvVarLocation::Reg(Reg::sp())
}

#[inline]
pub fn gp() -> RvVarLocation {
    RvVarLocation::Reg(Reg::gp())
}

#[inline]
pub fn tp() -> RvVarLocation {
    RvVarLocation::Reg(Reg::tp())
}

#[inline]
pub fn t0() -> RvVarLocation {
    RvVarLocation::Reg(Reg::t0())
}

#[inline]
pub fn t1() -> RvVarLocation {
    RvVarLocation::Reg(Reg::t1())
}

#[inline]
pub fn t2() -> RvVarLocation {
    RvVarLocation::Reg(Reg::t2())
}

#[inline]
pub fn s0() -> RvVarLocation {
    RvVarLocation::Reg(Reg::s0())
}

#[inline]
pub fn fp() -> RvVarLocation {
    RvVarLocation::Reg(Reg::fp())
}

#[inline]
pub fn s1() -> RvVarLocation {
    RvVarLocation::Reg(Reg::s1())
}

#[inline]
pub fn a0() -> RvVarLocation {
    RvVarLocation::Reg(Reg::a0())
}

#[inline]
pub fn a1() -> RvVarLocation {
    RvVarLocation::Reg(Reg::a1())
}

#[inline]
pub fn a2() -> RvVarLocation {
    RvVarLocation::Reg(Reg::a2())
}

#[inline]
pub fn a3() -> RvVarLocation {
    RvVarLocation::Reg(Reg::a3())
}

#[inline]
pub fn a4() -> RvVarLocation {
    RvVarLocation::Reg(Reg::a4())
}

#[inline]
pub fn a5() -> RvVarLocation {
    RvVarLocation::Reg(Reg::a5())
}

#[inline]
pub fn a6() -> RvVarLocation {
    RvVarLocation::Reg(Reg::a6())
}

#[inline]
pub fn a7() -> RvVarLocation {
    RvVarLocation::Reg(Reg::a7())
}

#[inline]
pub fn s2() -> RvVarLocation {
    RvVarLocation::Reg(Reg::s2())
}

#[inline]
pub fn s3() -> RvVarLocation {
    RvVarLocation::Reg(Reg::s3())
}

#[inline]
pub fn s4() -> RvVarLocation {
    RvVarLocation::Reg(Reg::s4())
}

#[inline]
pub fn s5() -> RvVarLocation {
    RvVarLocation::Reg(Reg::s5())
}

#[inline]
pub fn s6() -> RvVarLocation {
    RvVarLocation::Reg(Reg::s6())
}

#[inline]
pub fn s7() -> RvVarLocation {
    RvVarLocation::Reg(Reg::s7())
}

#[inline]
pub fn s8() -> RvVarLocation {
    RvVarLocation::Reg(Reg::s8())
}

#[inline]
pub fn s9() -> RvVarLocation {
    RvVarLocation::Reg(Reg::s9())
}

#[inline]
pub fn s10() -> RvVarLocation {
    RvVarLocation::Reg(Reg::s10())
}

#[inline]
pub fn s11() -> RvVarLocation {
    RvVarLocation::Reg(Reg::s11())
}

#[inline]
pub fn t3() -> RvVarLocation {
    RvVarLocation::Reg(Reg::t3())
}

#[inline]
pub fn t4() -> RvVarLocation {
    RvVarLocation::Reg(Reg::t4())
}

#[inline]
pub fn t5() -> RvVarLocation {
    RvVarLocation::Reg(Reg::t5())
}

#[inline]
pub fn t6() -> RvVarLocation {
    RvVarLocation::Reg(Reg::t6())
}

impl fmt::Display for RvVarLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RvVarLocation::Var(name) => write!(f, "{}", name),
            RvVarLocation::Reg(reg) => write!(f, "{}", reg),
        }
    }
}
