use std::fmt;
use crate::riscv::rv_reg::Reg;
use crate::riscv::rv_imm::{I24, I24WithZeroedBits};

/// RISC-V RV64 instruction
#[derive(Debug, Clone, PartialEq)]
pub enum RvInst{
    // R-type
    Add { rd: Reg, rs1: Reg, rs2: Reg },
    Sub { rd: Reg, rs1: Reg, rs2: Reg },
    Sll { rd: Reg, rs1: Reg, rs2: Reg },
    Slt { rd: Reg, rs1: Reg, rs2: Reg },
    Sltu { rd: Reg, rs1: Reg, rs2: Reg },
    Xor { rd: Reg, rs1: Reg, rs2: Reg },
    Srl { rd: Reg, rs1: Reg, rs2: Reg },
    Sra { rd: Reg, rs1: Reg, rs2: Reg },
    Or { rd: Reg, rs1: Reg, rs2: Reg },
    And { rd: Reg, rs1: Reg, rs2: Reg },

    // RV64 R-type W
    Addw { rd: Reg, rs1: Reg, rs2: Reg },
    Subw { rd: Reg, rs1: Reg, rs2: Reg },
    Sllw { rd: Reg, rs1: Reg, rs2: Reg },
    Srlw { rd: Reg, rs1: Reg, rs2: Reg },
    Sraw { rd: Reg, rs1: Reg, rs2: Reg },

    // I-type
    Addi { rd: Reg, rs1: Reg, imm: i16 },
    Slti { rd: Reg, rs1: Reg, imm: i16 },
    Sltiu { rd: Reg, rs1: Reg, imm: i16 },
    Xori { rd: Reg, rs1: Reg, imm: i16 },
    Ori { rd: Reg, rs1: Reg, imm: i16 },
    Andi { rd: Reg, rs1: Reg, imm: i16 },
    Slli { rd: Reg, rs1: Reg, shamt: u8 },
    Srli { rd: Reg, rs1: Reg, shamt: u8 },
    Srai { rd: Reg, rs1: Reg, shamt: u8 },

    // RV64 I-type W
    Addiw { rd: Reg, rs1: Reg, imm: i16 },
    Slliw { rd: Reg, rs1: Reg, shamt: u8 },
    Srliw { rd: Reg, rs1: Reg, shamt: u8 },
    Sraiw { rd: Reg, rs1: Reg, shamt: u8 },

    // Loads (I-type)
    Lb { rd: Reg, rs1: Reg, imm: i16 },
    Lh { rd: Reg, rs1: Reg, imm: i16 },
    Lw { rd: Reg, rs1: Reg, imm: i16 },
    Ld { rd: Reg, rs1: Reg, imm: i16 },
    Lbu { rd: Reg, rs1: Reg, imm: i16 },
    Lhu { rd: Reg, rs1: Reg, imm: i16 },
    Lwu { rd: Reg, rs1: Reg, imm: i16 },

    // Jalr (I-type)
    Jalr { rd: Reg, rs1: Reg, imm: i16 },

    // S-type
    Sb { rs2: Reg, rs1: Reg, imm: i16 },
    Sh { rs2: Reg, rs1: Reg, imm: i16 },
    Sw { rs2: Reg, rs1: Reg, imm: i16 },
    Sd { rs2: Reg, rs1: Reg, imm: i16 },

    // B-type
    Beq { rs1: Reg, rs2: Reg, imm: I24 },
    Bne { rs1: Reg, rs2: Reg, imm: I24 },
    Blt { rs1: Reg, rs2: Reg, imm: I24 },
    Bge { rs1: Reg, rs2: Reg, imm: I24 },
    Bltu { rs1: Reg, rs2: Reg, imm: I24 },
    Bgeu { rs1: Reg, rs2: Reg, imm: I24 },

    // Lui (U-type)
    Lui { rd: Reg, imm: I24WithZeroedBits<12> },

    // Auipc (U-type)
    Auipc { rd: Reg, imm: I24WithZeroedBits<12> },

    // Jal (J-type)
    Jal { rd: Reg, imm: I24 },

    // Fence
    Fence { pred: u8, succ: u8 },
    FenceTso,

    // System instructions
    Ecall,
    Ebreak,

    // Unimplemented/illegal
    Unimp,
}

impl fmt::Display for RvInst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add { rd, rs1, rs2 } => write!(f, "add {rd}, {rs1}, {rs2}"),
            Self::Sub { rd, rs1, rs2 } => write!(f, "sub {rd}, {rs1}, {rs2}"),
            Self::Sll { rd, rs1, rs2 } => write!(f, "sll {rd}, {rs1}, {rs2}"),
            Self::Slt { rd, rs1, rs2 } => write!(f, "slt {rd}, {rs1}, {rs2}"),
            Self::Sltu { rd, rs1, rs2 } => write!(f, "sltu {rd}, {rs1}, {rs2}"),
            Self::Xor { rd, rs1, rs2 } => write!(f, "xor {rd}, {rs1}, {rs2}"),
            Self::Srl { rd, rs1, rs2 } => write!(f, "srl {rd}, {rs1}, {rs2}"),
            Self::Sra { rd, rs1, rs2 } => write!(f, "sra {rd}, {rs1}, {rs2}"),
            Self::Or { rd, rs1, rs2 } => write!(f, "or {rd}, {rs1}, {rs2}"),
            Self::And { rd, rs1, rs2 } => write!(f, "and {rd}, {rs1}, {rs2}"),

            Self::Addw { rd, rs1, rs2 } => write!(f, "addw {rd}, {rs1}, {rs2}"),
            Self::Subw { rd, rs1, rs2 } => write!(f, "subw {rd}, {rs1}, {rs2}"),
            Self::Sllw { rd, rs1, rs2 } => write!(f, "sllw {rd}, {rs1}, {rs2}"),
            Self::Srlw { rd, rs1, rs2 } => write!(f, "srlw {rd}, {rs1}, {rs2}"),
            Self::Sraw { rd, rs1, rs2 } => write!(f, "sraw {rd}, {rs1}, {rs2}"),

            Self::Addi { rd, rs1, imm } => write!(f, "addi {rd}, {rs1}, {imm}"),
            Self::Slti { rd, rs1, imm } => write!(f, "slti {rd}, {rs1}, {imm}"),
            Self::Sltiu { rd, rs1, imm } => write!(f, "sltiu {rd}, {rs1}, {imm}"),
            Self::Xori { rd, rs1, imm } => write!(f, "xori {rd}, {rs1}, {imm}"),
            Self::Ori { rd, rs1, imm } => write!(f, "ori {rd}, {rs1}, {imm}"),
            Self::Andi { rd, rs1, imm } => write!(f, "andi {rd}, {rs1}, {imm}"),
            Self::Slli { rd, rs1, shamt } => write!(f, "slli {rd}, {rs1}, {shamt}"),
            Self::Srli { rd, rs1, shamt } => write!(f, "srli {rd}, {rs1}, {shamt}"),
            Self::Srai { rd, rs1, shamt } => write!(f, "srai {rd}, {rs1}, {shamt}"),

            Self::Addiw { rd, rs1, imm } => write!(f, "addiw {rd}, {rs1}, {imm}"),
            Self::Slliw { rd, rs1, shamt } => write!(f, "slliw {rd}, {rs1}, {shamt}"),
            Self::Srliw { rd, rs1, shamt } => write!(f, "srliw {rd}, {rs1}, {shamt}"),
            Self::Sraiw { rd, rs1, shamt } => write!(f, "sraiw {rd}, {rs1}, {shamt}"),

            Self::Lb { rd, rs1, imm } => write!(f, "lb {rd}, {imm}({rs1})"),
            Self::Lh { rd, rs1, imm } => write!(f, "lh {rd}, {imm}({rs1})"),
            Self::Lw { rd, rs1, imm } => write!(f, "lw {rd}, {imm}({rs1})"),
            Self::Ld { rd, rs1, imm } => write!(f, "ld {rd}, {imm}({rs1})"),
            Self::Lbu { rd, rs1, imm } => write!(f, "lbu {rd}, {imm}({rs1})"),
            Self::Lhu { rd, rs1, imm } => write!(f, "lhu {rd}, {imm}({rs1})"),
            Self::Lwu { rd, rs1, imm } => write!(f, "lwu {rd}, {imm}({rs1})"),

            Self::Jalr { rd, rs1, imm } => write!(f, "jalr {rd}, {imm}({rs1})"),

            Self::Sb { rs2, rs1, imm } => write!(f, "sb {rs2}, {imm}({rs1})"),
            Self::Sh { rs2, rs1, imm } => write!(f, "sh {rs2}, {imm}({rs1})"),
            Self::Sw { rs2, rs1, imm } => write!(f, "sw {rs2}, {imm}({rs1})"),
            Self::Sd { rs2, rs1, imm } => write!(f, "sd {rs2}, {imm}({rs1})"),

            Self::Beq { rs1, rs2, imm } => write!(f, "beq {rs1}, {rs2}, {imm}"),
            Self::Bne { rs1, rs2, imm } => write!(f, "bne {rs1}, {rs2}, {imm}"),
            Self::Blt { rs1, rs2, imm } => write!(f, "blt {rs1}, {rs2}, {imm}"),
            Self::Bge { rs1, rs2, imm } => write!(f, "bge {rs1}, {rs2}, {imm}"),
            Self::Bltu { rs1, rs2, imm } => write!(f, "bltu {rs1}, {rs2}, {imm}"),
            Self::Bgeu { rs1, rs2, imm } => write!(f, "bgeu {rs1}, {rs2}, {imm}"),

            Self::Lui { rd, imm } => write!(f, "lui {rd}, 0x{:x}", imm.to_i32() >> 12),

            Self::Auipc { rd, imm } => write!(f, "auipc {rd}, 0x{:x}", imm.to_i32() >> 12),

            Self::Jal { rd, imm } => write!(f, "jal {rd}, {imm}"),

            Self::Fence { pred, succ } => write!(f, "fence {pred}, {succ}"),
            Self::FenceTso => write!(f, "fence.tso"),

            Self::Ecall => write!(f, "ecall"),
            Self::Ebreak => write!(f, "ebreak"),

            Self::Unimp => write!(f, "unimp"),
        }
    }
}
