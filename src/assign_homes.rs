use crate::syntax::Ident;

#[derive(Debug, Clone, PartialEq)]
pub enum RvReg {
    Zero,
    Ra,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RvArg {
    Reg(RvReg),
    Var(Ident),
    Imm(i64),
    Mem(RvReg, i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RvInstr {
    Add(RvArg, RvArg, RvArg),
    Sub(RvArg, RvArg, RvArg),
    Mul(RvArg, RvArg, RvArg),
    Div(RvArg, RvArg, RvArg),
    And(RvArg, RvArg, RvArg),
    Or(RvArg, RvArg, RvArg),
    Xor(RvArg, RvArg, RvArg),
    Slt(RvArg, RvArg, RvArg),

    Seqz(RvArg, RvArg),
    Snez(RvArg, RvArg),

    Mv(RvArg, RvArg),
    Li(RvArg, i64),
    Call(Ident),
    Tail(Ident),
    Ret,

    Load(RvArg, RvArg, i64), // rd, base, offset
    Store(RvArg, RvArg, i64),

    Beqz(RvArg, Ident),
    Bnez(RvArg, Ident),
    J(Ident),
    Jalr(RvArg),
    Label(Ident),
}
