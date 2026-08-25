use crate::riscv::rv_imm::{I24, I24WithZeroedBits};
use crate::riscv_var::label::Label;
use crate::riscv_var::location::{RvVarLocation, zero};
use std::{fmt, i64};

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

pub fn li(rd: RvVarLocation, imm: i64) -> Vec<RvVarInstr> {
    let mut instrs = Vec::new();

    if -2048 <= imm && imm <= 2047 {
        instrs.push(RvVarInstr::Addi {
            rd: rd,
            rs1: zero(),
            imm: imm as i16,
        });
        return instrs;
    }

    if (i32::MIN as i64) <= imm && imm <= (i32::MAX as i64) {
        let imm32 = imm as i32;
        let hi = ((imm + 0x800) >> 12) & 0xFFFFF;
        let lo = ((imm32 & 0xFFF) ^ 0x800) - 0x800;
        instrs.push(RvVarInstr::Lui {
            rd: rd.clone(),
            imm: I24WithZeroedBits::<12>::from_i32((hi as i32) << 12),
        });
        if lo != 0 {
            instrs.push(RvVarInstr::Addiw {
                rd: rd.clone(),
                rs1: rd.clone(),
                imm: lo as i16,
            });
        }
        return instrs;
    }

    let mut x = imm as u64;
    let mut carry: i64 = 0;
    let mut c0 = ((x & 0xFF) as i64) + carry;
    if c0 > 0x7F {
        c0 -= 0x100;
        carry = 1;
    } else {
        carry = 0;
    }
    x >>= 8;
    let mut chunks: [i16; 3] = [0; 3];
    for chunk in &mut chunks {
        let c = ((x & 0xFFF) as i64) + carry;
        if c > 0x7FF {
            *chunk = (c - 0x1000) as i16;
            carry = 1;
        } else {
            *chunk = c as i16;
            carry = 0;
        }
        x >>= 12;
    }
    let top = ((((x & 0xFFFFF) as i64) + carry) & 0xFFFFF) as i32;

    instrs.push(RvVarInstr::Lui {
        rd: rd.clone(),
        imm: I24WithZeroedBits::<12>::from_i32(top << 12),
    });
    if chunks[2] != 0 {
        instrs.push(RvVarInstr::Addi {
            rd: rd.clone(),
            rs1: rd.clone(),
            imm: chunks[2],
        });
    }
    instrs.push(RvVarInstr::Slli {
        rd: rd.clone(),
        rs1: rd.clone(),
        shamt: 12,
    });
    if chunks[1] != 0 {
        instrs.push(RvVarInstr::Addi {
            rd: rd.clone(),
            rs1: rd.clone(),
            imm: chunks[1],
        });
    }
    instrs.push(RvVarInstr::Slli {
        rd: rd.clone(),
        rs1: rd.clone(),
        shamt: 12,
    });
    if chunks[0] != 0 {
        instrs.push(RvVarInstr::Addi {
            rd: rd.clone(),
            rs1: rd.clone(),
            imm: chunks[0],
        });
    }
    instrs.push(RvVarInstr::Slli {
        rd: rd.clone(),
        rs1: rd.clone(),
        shamt: 8,
    });
    if c0 != 0 {
        instrs.push(RvVarInstr::Addi {
            rd: rd.clone(),
            rs1: rd.clone(),
            imm: c0 as i16,
        });
    }
    instrs
}

pub fn mv(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    RvVarInstr::Addi { rd: rd, rs1: rs, imm: 0 }
}
pub fn seqz(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    RvVarInstr::Sltiu { rd: rd, rs1: rs, imm: 1 }
}

pub fn snez(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    RvVarInstr::Sltu { rd: rd, rs1: zero(), rs2: rs }
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
                write!(
                    f,
                    "lui {rd}, 0x{:x}",
                    (imm.to_i32() >> 12) & 0xFFFFF
                )
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::riscv_var::location::var;

    fn sext(v: i64, bits: u32) -> i64 {
        (v << (64 - bits)) >> (64 - bits)
    }

    fn lookup(
        env: &[(RvVarLocation, i64)],
        loc: &RvVarLocation,
    ) -> i64 {
        env.iter().rev().find(|(l, _)| l == loc).unwrap().1
    }

    fn store(
        env: &mut Vec<(RvVarLocation, i64)>,
        loc: &RvVarLocation,
        v: i64,
    ) {
        if let Some(pair) = env.iter_mut().find(|(l, _)| l == loc) {
            pair.1 = v;
        } else {
            env.push((loc.clone(), v));
        }
    }

    fn exec(instrs: &[RvVarInstr], dest: &RvVarLocation) -> i64 {
        let mut env = vec![(zero(), 0)];
        for instr in instrs {
            match instr {
                RvVarInstr::Addi { rd, rs1, imm } => {
                    let v = lookup(&env, rs1)
                        .wrapping_add(sext(*imm as i64, 12));
                    store(&mut env, rd, v);
                },
                RvVarInstr::Addiw { rd, rs1, imm } => {
                    let v = lookup(&env, rs1)
                        .wrapping_add(sext(*imm as i64, 12));
                    store(&mut env, rd, sext(v, 32));
                },
                RvVarInstr::Slli { rd, rs1, shamt } => {
                    let v = lookup(&env, rs1) << shamt;
                    store(&mut env, rd, v);
                },
                RvVarInstr::Lui { rd, imm } => {
                    let v =
                        sext((imm.to_i32() >> 12) as i64, 20) << 12;
                    store(&mut env, rd, v);
                },
                other => panic!(
                    "unexpected instruction in li/mv: {other:?}"
                ),
            }
        }
        lookup(&env, dest)
    }

    #[test]
    fn li_loads_64bit_signed_immediates() {
        let values: Vec<i64> = vec![
            0,
            1,
            -1,
            2,
            42,
            -42,
            2047,
            2048,
            -2048,
            -2049,
            4095,
            4096,
            -4096,
            0x7FF,
            0x800,
            0x7FFFFF,
            0x7FFFFF00,
            0x7FFFF800,
            0x12345678,
            -0x12345678,
            i32::MAX as i64,
            i32::MIN as i64,
            i32::MAX as i64 + 1,
            i32::MIN as i64 - 1,
            0xFFFFF800,
            0x123456789abcdef0,
            -0x123456789abcdef0,
            0x0000FFFF0000FFFF,
            1 << 31,
            1 << 32,
            1 << 44,
            (1 << 44) - 1,
            (1 << 44) + 1,
            1 << 48,
            -(1 << 48),
            1 << 52,
            -(1 << 52),
            -0x7fffffffffffffff,
            0x7FFFFFFFFFFF,
            i64::MAX,
            i64::MIN,
            i64::MIN + 1,
            i64::MAX - 1,
        ];
        for imm in values {
            let dest = var("rd".to_string());
            let instrs = li(dest.clone(), imm);
            assert_eq!(exec(&instrs, &dest), imm, "li rd, {imm:#x}");
        }
    }

    #[test]
    fn li_instructions_are_valid_12bit_fields() {
        let values: Vec<i64> = vec![
            i64::MAX,
            i64::MIN,
            i64::MIN + 1,
            0x123456789abcdef0,
            -0x7fffffffffffffff,
            1 << 63,
        ];
        for imm in values {
            let instrs = li(var("rd".to_string()), imm);
            for instr in &instrs {
                match instr {
                    RvVarInstr::Addi { imm, .. }
                    | RvVarInstr::Addiw { imm, .. } => {
                        assert!(
                            (-2048..=2047).contains(&(*imm as i32)),
                            "imm out of 12-bit range: {imm} in {instrs:?}"
                        );
                    },
                    _ => {},
                }
            }
        }
    }

    #[test]
    fn mv_copies_value() {
        let src = var("src".to_string());
        let dest = var("dest".to_string());
        let instr = mv(dest.clone(), src.clone());
        let mut env = vec![(src.clone(), 42)];
        match &instr {
            RvVarInstr::Addi { rd, rs1, imm } => {
                let v = lookup(&env, rs1)
                    .wrapping_add(sext(*imm as i64, 12));
                store(&mut env, rd, v);
            },
            other => {
                panic!("unexpected instruction in mv: {other:?}")
            },
        }
        assert_eq!(lookup(&env, &dest), 42);
    }
}
