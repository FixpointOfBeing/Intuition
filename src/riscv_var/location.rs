use crate::riscv::rv64imfd_reg::{FReg, IReg};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum RvVarLocation {
    Var(String),
    IReg(IReg),
}

#[derive(Clone, Debug, PartialEq)]
pub enum RvVarFLocation {
    Var(String),
    FReg(FReg),
}

#[inline]
pub fn fvar(s: String) -> RvVarFLocation {
    RvVarFLocation::Var(s)
}

#[inline]
pub fn var(s: String) -> RvVarLocation {
    RvVarLocation::Var(s)
}

#[inline]
pub fn x(n: u8) -> RvVarLocation {
    RvVarLocation::IReg(IReg::from_u8(n))
}

#[inline]
pub fn x0() -> RvVarLocation {
    x(0)
}

#[inline]
pub fn zero() -> RvVarLocation {
    RvVarLocation::IReg(IReg::zero())
}

#[inline]
pub fn ra() -> RvVarLocation {
    RvVarLocation::IReg(IReg::ra())
}

#[inline]
pub fn sp() -> RvVarLocation {
    RvVarLocation::IReg(IReg::sp())
}

#[inline]
pub fn gp() -> RvVarLocation {
    RvVarLocation::IReg(IReg::gp())
}

#[inline]
pub fn tp() -> RvVarLocation {
    RvVarLocation::IReg(IReg::tp())
}

#[inline]
pub fn t0() -> RvVarLocation {
    RvVarLocation::IReg(IReg::t0())
}

#[inline]
pub fn t1() -> RvVarLocation {
    RvVarLocation::IReg(IReg::t1())
}

#[inline]
pub fn t2() -> RvVarLocation {
    RvVarLocation::IReg(IReg::t2())
}

#[inline]
pub fn s0() -> RvVarLocation {
    RvVarLocation::IReg(IReg::s0())
}

#[inline]
pub fn fp() -> RvVarLocation {
    RvVarLocation::IReg(IReg::fp())
}

#[inline]
pub fn s1() -> RvVarLocation {
    RvVarLocation::IReg(IReg::s1())
}

#[inline]
pub fn a0() -> RvVarLocation {
    RvVarLocation::IReg(IReg::a0())
}

#[inline]
pub fn a1() -> RvVarLocation {
    RvVarLocation::IReg(IReg::a1())
}

#[inline]
pub fn a2() -> RvVarLocation {
    RvVarLocation::IReg(IReg::a2())
}

#[inline]
pub fn a3() -> RvVarLocation {
    RvVarLocation::IReg(IReg::a3())
}

#[inline]
pub fn a4() -> RvVarLocation {
    RvVarLocation::IReg(IReg::a4())
}

#[inline]
pub fn a5() -> RvVarLocation {
    RvVarLocation::IReg(IReg::a5())
}

#[inline]
pub fn a6() -> RvVarLocation {
    RvVarLocation::IReg(IReg::a6())
}

#[inline]
pub fn a7() -> RvVarLocation {
    RvVarLocation::IReg(IReg::a7())
}

#[inline]
pub fn s2() -> RvVarLocation {
    RvVarLocation::IReg(IReg::s2())
}

#[inline]
pub fn s3() -> RvVarLocation {
    RvVarLocation::IReg(IReg::s3())
}

#[inline]
pub fn s4() -> RvVarLocation {
    RvVarLocation::IReg(IReg::s4())
}

#[inline]
pub fn s5() -> RvVarLocation {
    RvVarLocation::IReg(IReg::s5())
}

#[inline]
pub fn s6() -> RvVarLocation {
    RvVarLocation::IReg(IReg::s6())
}

#[inline]
pub fn s7() -> RvVarLocation {
    RvVarLocation::IReg(IReg::s7())
}

#[inline]
pub fn s8() -> RvVarLocation {
    RvVarLocation::IReg(IReg::s8())
}

#[inline]
pub fn s9() -> RvVarLocation {
    RvVarLocation::IReg(IReg::s9())
}

#[inline]
pub fn s10() -> RvVarLocation {
    RvVarLocation::IReg(IReg::s10())
}

#[inline]
pub fn s11() -> RvVarLocation {
    RvVarLocation::IReg(IReg::s11())
}

#[inline]
pub fn t3() -> RvVarLocation {
    RvVarLocation::IReg(IReg::t3())
}

#[inline]
pub fn t4() -> RvVarLocation {
    RvVarLocation::IReg(IReg::t4())
}

#[inline]
pub fn t5() -> RvVarLocation {
    RvVarLocation::IReg(IReg::t5())
}

#[inline]
pub fn t6() -> RvVarLocation {
    RvVarLocation::IReg(IReg::t6())
}

impl fmt::Display for RvVarLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RvVarLocation::Var(name) => write!(f, "{}", name),
            RvVarLocation::IReg(reg) => write!(f, "{}", reg),
        }
    }
}

impl fmt::Display for RvVarFLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RvVarFLocation::Var(name) => write!(f, "{}", name),
            RvVarFLocation::FReg(reg) => write!(f, "{}", reg),
        }
    }
}
