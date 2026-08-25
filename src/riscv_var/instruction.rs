use crate::riscv::rv_imm::{I24, I24WithZeroedBits};
use crate::riscv_var::label::Label;
use crate::riscv_var::location::{zero, RvVarLocation};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum RvVarInstr {
    Add { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    Sub { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    Sll { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    Slt { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    Sltu { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    Xor { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    Srl { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    Sra { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    Or { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    And { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },

    Addw { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    Subw { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    Sllw { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    Srlw { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    Sraw { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },

    Addi { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Slti { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Sltiu { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Xori { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Ori { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Andi { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Slli { rd: RvVarLocation, rs1: RvVarLocation, shamt: u8 },
    Srli { rd: RvVarLocation, rs1: RvVarLocation, shamt: u8 },
    Srai { rd: RvVarLocation, rs1: RvVarLocation, shamt: u8 },

    Addiw { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Slliw { rd: RvVarLocation, rs1: RvVarLocation, shamt: u8 },
    Srliw { rd: RvVarLocation, rs1: RvVarLocation, shamt: u8 },
    Sraiw { rd: RvVarLocation, rs1: RvVarLocation, shamt: u8 },

    Lb { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Lh { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Lw { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Ld { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Lbu { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Lhu { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Lwu { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },

    Jalr { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },

    Sb { rs2: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Sh { rs2: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Sw { rs2: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    Sd { rs2: RvVarLocation, rs1: RvVarLocation, imm: i16 },

    Beq { rs1: RvVarLocation, rs2: RvVarLocation, label: Label },
    Bne { rs1: RvVarLocation, rs2: RvVarLocation, label: Label },
    Blt { rs1: RvVarLocation, rs2: RvVarLocation, label: Label },
    Bge { rs1: RvVarLocation, rs2: RvVarLocation, label: Label },
    Bltu { rs1: RvVarLocation, rs2: RvVarLocation, label: Label },
    Bgeu { rs1: RvVarLocation, rs2: RvVarLocation, label: Label },

    Lui { rd: RvVarLocation, imm: I24WithZeroedBits<12> },

    Auipc { rd: RvVarLocation, imm: I24WithZeroedBits<12> },

    Jal { rd: RvVarLocation, imm: I24 },

    Fence { pred: u8, succ: u8 },
    FenceTso,

    Ecall,
    Ebreak,

    Unimp,
}

pub fn seqz(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    RvVarInstr::Sltiu { rd: rd, rs1: rs, imm: 1 }
}

pub fn snez(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    RvVarInstr::Sltu { rd: rd, rs1: zero(), rs2:rs }
}

impl fmt::Display for RvVarInstr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add { rd, rs1, rs2 } => {
                write!(f, "add {rd}, {rs1}, {rs2}")
            },
            Self::Sub { rd, rs1, rs2 } => {
                write!(f, "sub {rd}, {rs1}, {rs2}")
            },
            Self::Sll { rd, rs1, rs2 } => {
                write!(f, "sll {rd}, {rs1}, {rs2}")
            },
            Self::Slt { rd, rs1, rs2 } => {
                write!(f, "slt {rd}, {rs1}, {rs2}")
            },
            Self::Sltu { rd, rs1, rs2 } => {
                write!(f, "sltu {rd}, {rs1}, {rs2}")
            },
            Self::Xor { rd, rs1, rs2 } => {
                write!(f, "xor {rd}, {rs1}, {rs2}")
            },
            Self::Srl { rd, rs1, rs2 } => {
                write!(f, "srl {rd}, {rs1}, {rs2}")
            },
            Self::Sra { rd, rs1, rs2 } => {
                write!(f, "sra {rd}, {rs1}, {rs2}")
            },
            Self::Or { rd, rs1, rs2 } => {
                write!(f, "or {rd}, {rs1}, {rs2}")
            },
            Self::And { rd, rs1, rs2 } => {
                write!(f, "and {rd}, {rs1}, {rs2}")
            },

            Self::Addw { rd, rs1, rs2 } => {
                write!(f, "addw {rd}, {rs1}, {rs2}")
            },
            Self::Subw { rd, rs1, rs2 } => {
                write!(f, "subw {rd}, {rs1}, {rs2}")
            },
            Self::Sllw { rd, rs1, rs2 } => {
                write!(f, "sllw {rd}, {rs1}, {rs2}")
            },
            Self::Srlw { rd, rs1, rs2 } => {
                write!(f, "srlw {rd}, {rs1}, {rs2}")
            },
            Self::Sraw { rd, rs1, rs2 } => {
                write!(f, "sraw {rd}, {rs1}, {rs2}")
            },

            Self::Addi { rd, rs1, imm } => {
                write!(f, "addi {rd}, {rs1}, {imm}")
            },
            Self::Slti { rd, rs1, imm } => {
                write!(f, "slti {rd}, {rs1}, {imm}")
            },
            Self::Sltiu { rd, rs1, imm } => {
                write!(f, "sltiu {rd}, {rs1}, {imm}")
            },
            Self::Xori { rd, rs1, imm } => {
                write!(f, "xori {rd}, {rs1}, {imm}")
            },
            Self::Ori { rd, rs1, imm } => {
                write!(f, "ori {rd}, {rs1}, {imm}")
            },
            Self::Andi { rd, rs1, imm } => {
                write!(f, "andi {rd}, {rs1}, {imm}")
            },
            Self::Slli { rd, rs1, shamt } => {
                write!(f, "slli {rd}, {rs1}, {shamt}")
            },
            Self::Srli { rd, rs1, shamt } => {
                write!(f, "srli {rd}, {rs1}, {shamt}")
            },
            Self::Srai { rd, rs1, shamt } => {
                write!(f, "srai {rd}, {rs1}, {shamt}")
            },

            Self::Addiw { rd, rs1, imm } => {
                write!(f, "addiw {rd}, {rs1}, {imm}")
            },
            Self::Slliw { rd, rs1, shamt } => {
                write!(f, "slliw {rd}, {rs1}, {shamt}")
            },
            Self::Srliw { rd, rs1, shamt } => {
                write!(f, "srliw {rd}, {rs1}, {shamt}")
            },
            Self::Sraiw { rd, rs1, shamt } => {
                write!(f, "sraiw {rd}, {rs1}, {shamt}")
            },

            Self::Lb { rd, rs1, imm } => {
                write!(f, "lb {rd}, {imm}({rs1})")
            },
            Self::Lh { rd, rs1, imm } => {
                write!(f, "lh {rd}, {imm}({rs1})")
            },
            Self::Lw { rd, rs1, imm } => {
                write!(f, "lw {rd}, {imm}({rs1})")
            },
            Self::Ld { rd, rs1, imm } => {
                write!(f, "ld {rd}, {imm}({rs1})")
            },
            Self::Lbu { rd, rs1, imm } => {
                write!(f, "lbu {rd}, {imm}({rs1})")
            },
            Self::Lhu { rd, rs1, imm } => {
                write!(f, "lhu {rd}, {imm}({rs1})")
            },
            Self::Lwu { rd, rs1, imm } => {
                write!(f, "lwu {rd}, {imm}({rs1})")
            },

            Self::Jalr { rd, rs1, imm } => {
                write!(f, "jalr {rd}, {imm}({rs1})")
            },

            Self::Sb { rs2, rs1, imm } => {
                write!(f, "sb {rs2}, {imm}({rs1})")
            },
            Self::Sh { rs2, rs1, imm } => {
                write!(f, "sh {rs2}, {imm}({rs1})")
            },
            Self::Sw { rs2, rs1, imm } => {
                write!(f, "sw {rs2}, {imm}({rs1})")
            },
            Self::Sd { rs2, rs1, imm } => {
                write!(f, "sd {rs2}, {imm}({rs1})")
            },

            Self::Beq { rs1, rs2, label } => {
                write!(f, "beq {rs1}, {rs2}, {label}")
            },
            Self::Bne { rs1, rs2, label } => {
                write!(f, "bne {rs1}, {rs2}, {label}")
            },
            Self::Blt { rs1, rs2, label } => {
                write!(f, "blt {rs1}, {rs2}, {label}")
            },
            Self::Bge { rs1, rs2, label } => {
                write!(f, "bge {rs1}, {rs2}, {label}")
            },
            Self::Bltu { rs1, rs2, label } => {
                write!(f, "bltu {rs1}, {rs2}, {label}")
            },
            Self::Bgeu { rs1, rs2, label } => {
                write!(f, "bgeu {rs1}, {rs2}, {label}")
            },

            Self::Lui { rd, imm } => {
                write!(f, "lui {rd}, 0x{:x}", imm.to_i32() >> 12)
            },

            Self::Auipc { rd, imm } => {
                write!(f, "auipc {rd}, 0x{:x}", imm.to_i32() >> 12)
            },

            Self::Jal { rd, imm } => write!(f, "jal {rd}, {imm}"),

            Self::Fence { pred, succ } => {
                write!(f, "fence {pred}, {succ}")
            },
            Self::FenceTso => write!(f, "fence.tso"),

            Self::Ecall => write!(f, "ecall"),
            Self::Ebreak => write!(f, "ebreak"),

            Self::Unimp => write!(f, "unimp"),
        }
    }
}
