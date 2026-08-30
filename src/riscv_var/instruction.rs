use crate::riscv::rv64imfd_imm::I24WithZeroedBits;
use crate::riscv_var::label::Label;
use crate::riscv_var::location::{
    RvVarFLocation, RvVarLocation, zero,
};
use std::{fmt, i64};

/// 带变量操作数的 RISC-V 指令(RV64IMFD)
#[derive(Debug, Clone, PartialEq)]
pub enum RvVarInstr {
    // R-type
    /// 有符号加法：rd = rs1 + rs2
    Add { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 有符号减法：rd = rs1 - rs2
    Sub { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 逻辑左移：rd = rs1 << (rs2 & 0x3F)
    Sll { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 有符号小于比较：rs1 < rs2 时 rd = 1，否则 rd = 0
    Slt { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 无符号小于比较：rs1 < rs2（无符号）时 rd = 1，否则 rd = 0
    Sltu { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 按位异或：rd = rs1 ^ rs2
    Xor { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 逻辑右移：rd = rs1 >> (rs2 & 0x3F)，高位补 0
    Srl { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 算术右移：rd = rs1 >> (rs2 & 0x3F)，高位补符号位
    Sra { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 按位或：rd = rs1 | rs2
    Or { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 按位与：rd = rs1 & rs2
    And { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },

    // R-type（M 扩展）
    /// 有符号乘法（取低 64 位）：rd = (rs1 *s rs2)[63:0]
    Mul { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 有符号乘法（取高 64 位）：rd = (rs1 *s rs2) >> 64
    Mulh { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 无符号乘法（取高 64 位）：rd = (rs1 *u rs2) >> 64
    Mulhu {
        rd: RvVarLocation,
        rs1: RvVarLocation,
        rs2: RvVarLocation,
    },
    /// 有符号乘无符号（取高 64 位）：rd = (rs1 *s rs2_u) >> 64
    Mulhsu {
        rd: RvVarLocation,
        rs1: RvVarLocation,
        rs2: RvVarLocation,
    },
    /// 有符号除法（向零截断）：rd = rs1 /s rs2；除零时 rd = -1，溢出（MIN / -1）时 rd = MIN
    Div { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 无符号除法：rd = rs1 /u rs2；除零时 rd = 2^64 - 1
    Divu { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 有符号取余：rd = rs1 %s rs2；除零时 rd = rs1，溢出（MIN % -1）时 rd = 0
    Rem { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 无符号取余：rd = rs1 %u rs2；除零时 rd = rs1
    Remu { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },

    // RV64 R-type W
    /// 32 位有符号加法，结果按 32 位符号扩展：rd = sext32(rs1 + rs2)
    Addw { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 32 位有符号减法，结果按 32 位符号扩展：rd = sext32(rs1 - rs2)
    Subw { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 32 位逻辑左移（移位数取低 5 位），结果按 32 位符号扩展
    Sllw { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 32 位逻辑右移（移位数取低 5 位），结果按 32 位符号扩展
    Srlw { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 32 位算术右移（移位数取低 5 位），结果按 32 位符号扩展
    Sraw { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },

    // RV64 R-type W（M 扩展）
    /// 32 位有符号乘法（取低 32 位），结果按 32 位符号扩展
    Mulw { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 32 位有符号除法（向零截断），结果按 32 位符号扩展；除零时 rd = -1，溢出时 rd = sext32(INT32_MIN)
    Divw { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 32 位无符号除法，结果按 32 位符号扩展；除零时 rd = 2^32 - 1
    Divuw {
        rd: RvVarLocation,
        rs1: RvVarLocation,
        rs2: RvVarLocation,
    },
    /// 32 位有符号取余，结果按 32 位符号扩展；除零时 rd = rs1，溢出时 rd = 0
    Remw { rd: RvVarLocation, rs1: RvVarLocation, rs2: RvVarLocation },
    /// 32 位无符号取余，结果按 32 位符号扩展；除零时 rd = rs1
    Remuw {
        rd: RvVarLocation,
        rs1: RvVarLocation,
        rs2: RvVarLocation,
    },

    // I-type
    /// 加立即数（12 位符号扩展）：rd = rs1 + sext(imm)
    Addi { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 有符号小于立即数比较：rs1 < sext(imm) 时 rd = 1，否则 rd = 0
    Slti { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 无符号小于立即数比较：rs1 < sext(imm)（无符号）时 rd = 1，否则 rd = 0
    Sltiu { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 与立即数按位异或：rd = rs1 ^ sext(imm)
    Xori { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 与立即数按位或：rd = rs1 | sext(imm)
    Ori { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 与立即数按位与：rd = rs1 & sext(imm)
    Andi { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 立即数逻辑左移：rd = rs1 << shamt
    Slli { rd: RvVarLocation, rs1: RvVarLocation, shamt: u8 },
    /// 立即数逻辑右移：rd = rs1 >> shamt，高位补 0
    Srli { rd: RvVarLocation, rs1: RvVarLocation, shamt: u8 },
    /// 立即数算术右移：rd = rs1 >> shamt，高位补符号位
    Srai { rd: RvVarLocation, rs1: RvVarLocation, shamt: u8 },

    // RV64 I-type W
    /// 32 位加立即数（12 位符号扩展），结果按 32 位符号扩展：rd = sext32(rs1 + sext(imm))
    Addiw { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 32 位立即数逻辑左移，结果按 32 位符号扩展
    Slliw { rd: RvVarLocation, rs1: RvVarLocation, shamt: u8 },
    /// 32 位立即数逻辑右移，结果按 32 位符号扩展
    Srliw { rd: RvVarLocation, rs1: RvVarLocation, shamt: u8 },
    /// 32 位立即数算术右移，结果按 32 位符号扩展
    Sraiw { rd: RvVarLocation, rs1: RvVarLocation, shamt: u8 },

    // Loads (I-type)
    /// 从内存加载字节并符号扩展：rd = sext8(Mem[rs1 + sext(imm)])
    Lb { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 从内存加载半字并符号扩展：rd = sext16(Mem[rs1 + sext(imm)])
    Lh { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 从内存加载字并符号扩展：rd = sext32(Mem[rs1 + sext(imm)])
    Lw { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 从内存加载双字：rd = Mem[rs1 + sext(imm)]
    Ld { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 从内存加载无符号字节并零扩展：rd = zeroext8(Mem[rs1 + sext(imm)])
    Lbu { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 从内存加载无符号半字并零扩展：rd = zeroext16(Mem[rs1 + sext(imm)])
    Lhu { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 从内存加载无符号字并零扩展：rd = zeroext32(Mem[rs1 + sext(imm)])
    Lwu { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },

    // Jalr (I-type)
    /// 寄存器间接跳转并链接：rd = PC + 4，PC = (rs1 + sext(imm)) & ~1
    Jalr { rd: RvVarLocation, rs1: RvVarLocation, imm: i16 },

    // S-type
    /// 存字节：Mem[rs1 + sext(imm)] = rs2 & 0xFF
    Sb { rs2: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 存半字：Mem[rs1 + sext(imm)] = rs2 & 0xFFFF
    Sh { rs2: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 存字：Mem[rs1 + sext(imm)] = rs2 & 0xFFFFFFFF
    Sw { rs2: RvVarLocation, rs1: RvVarLocation, imm: i16 },
    /// 存双字：Mem[rs1 + sext(imm)] = rs2
    Sd { rs2: RvVarLocation, rs1: RvVarLocation, imm: i16 },

    // B-type
    /// 相等则跳转：rs1 == rs2 时跳转到 label
    Beq { rs1: RvVarLocation, rs2: RvVarLocation, label: Label },
    /// 不等则跳转：rs1 != rs2 时跳转到 label
    Bne { rs1: RvVarLocation, rs2: RvVarLocation, label: Label },
    /// 有符号小于则跳转：rs1 < rs2 时跳转到 label
    Blt { rs1: RvVarLocation, rs2: RvVarLocation, label: Label },
    /// 有符号大于等于则跳转：rs1 >= rs2 时跳转到 label
    Bge { rs1: RvVarLocation, rs2: RvVarLocation, label: Label },
    /// 无符号小于则跳转：rs1 < rs2（无符号）时跳转到 label
    Bltu { rs1: RvVarLocation, rs2: RvVarLocation, label: Label },
    /// 无符号大于等于则跳转：rs1 >= rs2（无符号）时跳转到 label
    Bgeu { rs1: RvVarLocation, rs2: RvVarLocation, label: Label },

    // Lui (U-type)
    /// 加载高位立即数：rd = sext(imm << 12)，低 12 位为 0
    Lui { rd: RvVarLocation, imm: I24WithZeroedBits<12> },

    // Auipc (U-type)
    /// PC 加高位立即数：rd = PC + sext(imm << 12)
    Auipc { rd: RvVarLocation, imm: I24WithZeroedBits<12> },

    // Jal (J-type)
    /// 直接跳转并链接：rd = PC + 4，PC = PC + sext(imm)
    Jal { rd: RvVarLocation, label: Label },

    /// 带变量操作数的 RISC-V 浮点指令（F/D 扩展）
    // 浮点算术（带舍入模式）
    /// 单精度浮点加法：rd = rs1 + rs2（按 rm 舍入）
    FaddS {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rm: Rm,
    },
    /// 双精度浮点加法：rd = rs1 + rs2（按 rm 舍入）
    FaddD {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rm: Rm,
    },
    /// 单精度浮点减法：rd = rs1 - rs2（按 rm 舍入）
    FsubS {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rm: Rm,
    },
    /// 双精度浮点减法：rd = rs1 - rs2（按 rm 舍入）
    FsubD {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rm: Rm,
    },
    /// 单精度浮点乘法：rd = rs1 × rs2（按 rm 舍入）
    FmulS {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rm: Rm,
    },
    /// 双精度浮点乘法：rd = rs1 × rs2（按 rm 舍入）
    FmulD {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rm: Rm,
    },
    /// 单精度浮点除法：rd = rs1 / rs2（按 rm 舍入）
    FdivS {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rm: Rm,
    },
    /// 双精度浮点除法：rd = rs1 / rs2（按 rm 舍入）
    FdivD {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rm: Rm,
    },
    /// 单精度浮点平方根：rd = √rs1（按 rm 舍入）
    FsqrtS { rd: RvVarFLocation, rs1: RvVarFLocation, rm: Rm },
    /// 双精度浮点平方根：rd = √rs1（按 rm 舍入）
    FsqrtD { rd: RvVarFLocation, rs1: RvVarFLocation, rm: Rm },

    // 符号注入/拷贝（无舍入）
    /// 单精度拷贝符号位：rd = rs1 的绝对值与 rs2 的符号位组合
    FsgnjS {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },
    /// 双精度拷贝符号位：rd = rs1 的绝对值与 rs2 的符号位组合
    FsgnjD {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },
    /// 单精度拷贝相反符号位：rd = rs1 的绝对值与 ~rs2 的符号位组合
    FsgnjnS {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },
    /// 双精度拷贝相反符号位：rd = rs1 的绝对值与 ~rs2 的符号位组合
    FsgnjnD {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },
    /// 单精度符号位异或：rd = rs1 的绝对值与 (rs1 符号位 xor rs2 符号位) 组合
    FsgnjxS {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },
    /// 双精度符号位异或：rd = rs1 的绝对值与 (rs1 符号位 xor rs2 符号位) 组合
    FsgnjxD {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },

    // 取最小值/最大值（无舍入，遵循 IEEE-754 2019 语义）
    /// 单精度取最小值：rd = min(rs1, rs2)
    FminS {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },
    /// 双精度取最小值：rd = min(rs1, rs2)
    FminD {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },
    /// 单精度取最大值：rd = max(rs1, rs2)
    FmaxS {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },
    /// 双精度取最大值：rd = max(rs1, rs2)
    FmaxD {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },

    // 浮点比较（结果写入整数寄存器）
    /// 单精度浮点相等比较：rs1 == rs2 时 rd = 1，否则 rd = 0
    FeqS {
        rd: RvVarLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },
    /// 双精度浮点相等比较：rs1 == rs2 时 rd = 1，否则 rd = 0
    FeqD {
        rd: RvVarLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },
    /// 单精度浮点小于比较：rs1 < rs2 时 rd = 1，否则 rd = 0
    FltS {
        rd: RvVarLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },
    /// 双精度浮点小于比较：rs1 < rs2 时 rd = 1，否则 rd = 0
    FltD {
        rd: RvVarLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },
    /// 单精度浮点小于等于比较：rs1 <= rs2 时 rd = 1，否则 rd = 0
    FleS {
        rd: RvVarLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },
    /// 双精度浮点小于等于比较：rs1 <= rs2 时 rd = 1，否则 rd = 0
    FleD {
        rd: RvVarLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
    },

    // 寄存器移动（整数 ↔ 浮点，位模式不变）
    /// 将浮点寄存器的位模式按 32 位符号扩展移到整数寄存器：rd = sext32(bits(rs1))
    FmvXW { rd: RvVarLocation, rs1: RvVarFLocation },
    /// 将整数寄存器的低 32 位位模式移到浮点寄存器：rd = bits(rs1)[31:0]
    FmvWX { rd: RvVarFLocation, rs1: RvVarLocation },
    /// 将浮点寄存器的位模式（64 位）移到整数寄存器：rd = bits(rs1)
    FmvXD { rd: RvVarLocation, rs1: RvVarFLocation },
    /// 将整数寄存器的位模式（64 位）移到浮点寄存器：rd = bits(rs1)
    FmvDX { rd: RvVarFLocation, rs1: RvVarLocation },

    // 类型转换（整数 → 浮点，无舍入）
    /// 有符号整数转单精度浮点：rd = (float)rs1
    FcvtSW { rd: RvVarFLocation, rs1: RvVarLocation },
    /// 无符号整数转单精度浮点：rd = (float)rs1
    FcvtSWu { rd: RvVarFLocation, rs1: RvVarLocation },
    /// 有符号整数转双精度浮点：rd = (double)rs1
    FcvtDW { rd: RvVarFLocation, rs1: RvVarLocation },
    /// 无符号整数转双精度浮点：rd = (double)rs1
    FcvtDWu { rd: RvVarFLocation, rs1: RvVarLocation },

    // 类型转换（浮点 ↔ 浮点，无舍入）
    /// 单精度浮点转双精度浮点：rd = (double)rs1
    FcvtSD { rd: RvVarFLocation, rs1: RvVarFLocation },
    /// 双精度浮点转单精度浮点：rd = (float)rs1
    FcvtDS { rd: RvVarFLocation, rs1: RvVarFLocation },

    // 类型转换（浮点 → 整数，带舍入模式）
    /// 单精度浮点转有符号整数（按 rm 舍入，向零截断等价于 rtz）
    FcvtWS { rd: RvVarLocation, rs1: RvVarFLocation, rm: Rm },
    /// 单精度浮点转无符号整数（按 rm 舍入）
    FcvtWuS { rd: RvVarLocation, rs1: RvVarFLocation, rm: Rm },
    /// 双精度浮点转有符号整数（按 rm 舍入）
    FcvtWD { rd: RvVarLocation, rs1: RvVarFLocation, rm: Rm },
    /// 双精度浮点转无符号整数（按 rm 舍入）
    FcvtWuD { rd: RvVarLocation, rs1: RvVarFLocation, rm: Rm },

    // 融合乘加（R4-type，带舍入模式）
    /// 单精度融合乘加：rd = rs1×rs2 + rs3（按 rm 舍入）
    FmaddS {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rs3: RvVarFLocation,
        rm: Rm,
    },
    /// 单精度融合乘减：rd = rs1×rs2 - rs3（按 rm 舍入）
    FmsubS {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rs3: RvVarFLocation,
        rm: Rm,
    },
    /// 单精度融合负乘加：rd = -(rs1×rs2) + rs3（按 rm 舍入）
    FnmsubS {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rs3: RvVarFLocation,
        rm: Rm,
    },
    /// 单精度融合负乘减：rd = -(rs1×rs2) - rs3（按 rm 舍入）
    FnmaddS {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rs3: RvVarFLocation,
        rm: Rm,
    },
    /// 双精度融合乘加：rd = rs1×rs2 + rs3（按 rm 舍入）
    FmaddD {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rs3: RvVarFLocation,
        rm: Rm,
    },
    /// 双精度融合乘减：rd = rs1×rs2 - rs3（按 rm 舍入）
    FmsubD {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rs3: RvVarFLocation,
        rm: Rm,
    },
    /// 双精度融合负乘加：rd = -(rs1×rs2) + rs3（按 rm 舍入）
    FnmsubD {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rs3: RvVarFLocation,
        rm: Rm,
    },
    /// 双精度融合负乘减：rd = -(rs1×rs2) - rs3（按 rm 舍入）
    FnmaddD {
        rd: RvVarFLocation,
        rs1: RvVarFLocation,
        rs2: RvVarFLocation,
        rs3: RvVarFLocation,
        rm: Rm,
    },

    // 浮点分类（结果写入整数寄存器）
    /// 单精度浮点分类：rd = 指示 rs1 类型的位掩码
    FclassS { rd: RvVarLocation, rs1: RvVarFLocation },
    /// 双精度浮点分类：rd = 指示 rs1 类型的位掩码
    FclassD { rd: RvVarLocation, rs1: RvVarFLocation },

    // RV64F/D 64 位整数转换（带舍入模式）
    /// 单精度浮点转有符号 64 位整数（按 rm 舍入）
    FcvtLS { rd: RvVarLocation, rs1: RvVarFLocation, rm: Rm },
    /// 单精度浮点转无符号 64 位整数（按 rm 舍入）
    FcvtLuS { rd: RvVarLocation, rs1: RvVarFLocation, rm: Rm },
    /// 有符号 64 位整数转单精度浮点（按 rm 舍入）
    FcvtSL { rd: RvVarFLocation, rs1: RvVarLocation, rm: Rm },
    /// 无符号 64 位整数转单精度浮点（按 rm 舍入）
    FcvtSLu { rd: RvVarFLocation, rs1: RvVarLocation, rm: Rm },
    /// 双精度浮点转有符号 64 位整数（按 rm 舍入）
    FcvtLD { rd: RvVarLocation, rs1: RvVarFLocation, rm: Rm },
    /// 双精度浮点转无符号 64 位整数（按 rm 舍入）
    FcvtLuD { rd: RvVarLocation, rs1: RvVarFLocation, rm: Rm },
    /// 有符号 64 位整数转双精度浮点（按 rm 舍入）
    FcvtDL { rd: RvVarFLocation, rs1: RvVarLocation, rm: Rm },
    /// 无符号 64 位整数转双精度浮点（按 rm 舍入）
    FcvtDLu { rd: RvVarFLocation, rs1: RvVarLocation, rm: Rm },

    // 浮点访存
    /// 加载单精度浮点：rd = Mem[rs1 + sext(imm)]
    Flw { rd: RvVarFLocation, rs1: RvVarLocation, imm: i16 },
    /// 加载双精度浮点：rd = Mem[rs1 + sext(imm)]
    Fld { rd: RvVarFLocation, rs1: RvVarLocation, imm: i16 },
    /// 存单精度浮点：Mem[rs1 + sext(imm)] = rs2
    Fsw { rs2: RvVarFLocation, rs1: RvVarLocation, imm: i16 },
    /// 存双精度浮点：Mem[rs1 + sext(imm)] = rs2
    Fsd { rs2: RvVarFLocation, rs1: RvVarLocation, imm: i16 },
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

pub fn not(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    RvVarInstr::Xori { rd, rs1: rs, imm: -1 }
}

pub fn nop() -> RvVarInstr {
    todo!()
}

pub fn neg(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    todo!()
}

pub fn negw(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    todo!()
}

pub fn sltz(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    todo!()
}

pub fn sgtz(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    todo!()
}

pub fn seqz(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    RvVarInstr::Sltiu { rd: rd, rs1: rs, imm: 1 }
}

pub fn snez(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    RvVarInstr::Sltu { rd: rd, rs1: zero(), rs2: rs }
}

pub fn beqz(rd: RvVarLocation, offset: Label) -> RvVarInstr {
    todo!()
}

pub fn bnez(rd: RvVarLocation, offset: Label) -> RvVarInstr {
    todo!()
}

pub fn blez(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    todo!()
}

pub fn bgez(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    todo!()
}

pub fn bltz(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    todo!()
}

pub fn bgtz(rd: RvVarLocation, rs: RvVarLocation) -> RvVarInstr {
    todo!()
}

pub fn j(offset: Label) -> RvVarInstr {
    todo!()
}

pub fn jr(rs: RvVarLocation) -> RvVarInstr {
    todo!()
}

pub fn ret() -> RvVarInstr {
    todo!()
}

pub fn tail(offset: Label) -> RvVarInstr {
    todo!()
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

            Self::Mul { rd, rs1, rs2 } => {
                write!(f, "mul {rd}, {rs1}, {rs2}")
            },
            Self::Mulh { rd, rs1, rs2 } => {
                write!(f, "mulh {rd}, {rs1}, {rs2}")
            },
            Self::Mulhu { rd, rs1, rs2 } => {
                write!(f, "mulhu {rd}, {rs1}, {rs2}")
            },
            Self::Mulhsu { rd, rs1, rs2 } => {
                write!(f, "mulhsu {rd}, {rs1}, {rs2}")
            },
            Self::Div { rd, rs1, rs2 } => {
                write!(f, "div {rd}, {rs1}, {rs2}")
            },
            Self::Divu { rd, rs1, rs2 } => {
                write!(f, "divu {rd}, {rs1}, {rs2}")
            },
            Self::Rem { rd, rs1, rs2 } => {
                write!(f, "rem {rd}, {rs1}, {rs2}")
            },
            Self::Remu { rd, rs1, rs2 } => {
                write!(f, "remu {rd}, {rs1}, {rs2}")
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

            Self::Mulw { rd, rs1, rs2 } => {
                write!(f, "mulw {rd}, {rs1}, {rs2}")
            },
            Self::Divw { rd, rs1, rs2 } => {
                write!(f, "divw {rd}, {rs1}, {rs2}")
            },
            Self::Divuw { rd, rs1, rs2 } => {
                write!(f, "divuw {rd}, {rs1}, {rs2}")
            },
            Self::Remw { rd, rs1, rs2 } => {
                write!(f, "remw {rd}, {rs1}, {rs2}")
            },
            Self::Remuw { rd, rs1, rs2 } => {
                write!(f, "remuw {rd}, {rs1}, {rs2}")
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

            Self::Jal { rd, label } => write!(f, "jal {rd}, {label}"),

            Self::FaddS { rd, rs1, rs2, rm } => {
                write!(f, "fadd.s {rd}, {rs1}, {rs2}, {rm}")
            },
            Self::FaddD { rd, rs1, rs2, rm } => {
                write!(f, "fadd.d {rd}, {rs1}, {rs2}, {rm}")
            },
            Self::FsubS { rd, rs1, rs2, rm } => {
                write!(f, "fsub.s {rd}, {rs1}, {rs2}, {rm}")
            },
            Self::FsubD { rd, rs1, rs2, rm } => {
                write!(f, "fsub.d {rd}, {rs1}, {rs2}, {rm}")
            },
            Self::FmulS { rd, rs1, rs2, rm } => {
                write!(f, "fmul.s {rd}, {rs1}, {rs2}, {rm}")
            },
            Self::FmulD { rd, rs1, rs2, rm } => {
                write!(f, "fmul.d {rd}, {rs1}, {rs2}, {rm}")
            },
            Self::FdivS { rd, rs1, rs2, rm } => {
                write!(f, "fdiv.s {rd}, {rs1}, {rs2}, {rm}")
            },
            Self::FdivD { rd, rs1, rs2, rm } => {
                write!(f, "fdiv.d {rd}, {rs1}, {rs2}, {rm}")
            },
            Self::FsqrtS { rd, rs1, rm } => {
                write!(f, "fsqrt.s {rd}, {rs1}, {rm}")
            },
            Self::FsqrtD { rd, rs1, rm } => {
                write!(f, "fsqrt.d {rd}, {rs1}, {rm}")
            },

            Self::FsgnjS { rd, rs1, rs2 } => {
                write!(f, "fsgnj.s {rd}, {rs1}, {rs2}")
            },
            Self::FsgnjD { rd, rs1, rs2 } => {
                write!(f, "fsgnj.d {rd}, {rs1}, {rs2}")
            },
            Self::FsgnjnS { rd, rs1, rs2 } => {
                write!(f, "fsgnjn.s {rd}, {rs1}, {rs2}")
            },
            Self::FsgnjnD { rd, rs1, rs2 } => {
                write!(f, "fsgnjn.d {rd}, {rs1}, {rs2}")
            },
            Self::FsgnjxS { rd, rs1, rs2 } => {
                write!(f, "fsgnjx.s {rd}, {rs1}, {rs2}")
            },
            Self::FsgnjxD { rd, rs1, rs2 } => {
                write!(f, "fsgnjx.d {rd}, {rs1}, {rs2}")
            },
            Self::FminS { rd, rs1, rs2 } => {
                write!(f, "fmin.s {rd}, {rs1}, {rs2}")
            },
            Self::FminD { rd, rs1, rs2 } => {
                write!(f, "fmin.d {rd}, {rs1}, {rs2}")
            },
            Self::FmaxS { rd, rs1, rs2 } => {
                write!(f, "fmax.s {rd}, {rs1}, {rs2}")
            },
            Self::FmaxD { rd, rs1, rs2 } => {
                write!(f, "fmax.d {rd}, {rs1}, {rs2}")
            },

            Self::FeqS { rd, rs1, rs2 } => {
                write!(f, "feq.s {rd}, {rs1}, {rs2}")
            },
            Self::FeqD { rd, rs1, rs2 } => {
                write!(f, "feq.d {rd}, {rs1}, {rs2}")
            },
            Self::FltS { rd, rs1, rs2 } => {
                write!(f, "flt.s {rd}, {rs1}, {rs2}")
            },
            Self::FltD { rd, rs1, rs2 } => {
                write!(f, "flt.d {rd}, {rs1}, {rs2}")
            },
            Self::FleS { rd, rs1, rs2 } => {
                write!(f, "fle.s {rd}, {rs1}, {rs2}")
            },
            Self::FleD { rd, rs1, rs2 } => {
                write!(f, "fle.d {rd}, {rs1}, {rs2}")
            },

            Self::FmvXW { rd, rs1 } => {
                write!(f, "fmv.x.w {rd}, {rs1}")
            },
            Self::FmvWX { rd, rs1 } => {
                write!(f, "fmv.w.x {rd}, {rs1}")
            },
            Self::FmvXD { rd, rs1 } => {
                write!(f, "fmv.x.d {rd}, {rs1}")
            },
            Self::FmvDX { rd, rs1 } => {
                write!(f, "fmv.d.x {rd}, {rs1}")
            },

            Self::FcvtSW { rd, rs1 } => {
                write!(f, "fcvt.s.w {rd}, {rs1}")
            },
            Self::FcvtSWu { rd, rs1 } => {
                write!(f, "fcvt.s.wu {rd}, {rs1}")
            },
            Self::FcvtDW { rd, rs1 } => {
                write!(f, "fcvt.d.w {rd}, {rs1}")
            },
            Self::FcvtDWu { rd, rs1 } => {
                write!(f, "fcvt.d.wu {rd}, {rs1}")
            },
            Self::FcvtSD { rd, rs1 } => {
                write!(f, "fcvt.s.d {rd}, {rs1}")
            },
            Self::FcvtDS { rd, rs1 } => {
                write!(f, "fcvt.d.s {rd}, {rs1}")
            },
            Self::FcvtWS { rd, rs1, rm } => {
                write!(f, "fcvt.w.s {rd}, {rs1}, {rm}")
            },
            Self::FcvtWuS { rd, rs1, rm } => {
                write!(f, "fcvt.wu.s {rd}, {rs1}, {rm}")
            },
            Self::FcvtWD { rd, rs1, rm } => {
                write!(f, "fcvt.w.d {rd}, {rs1}, {rm}")
            },
            Self::FcvtWuD { rd, rs1, rm } => {
                write!(f, "fcvt.wu.d {rd}, {rs1}, {rm}")
            },

            Self::FmaddS { rd, rs1, rs2, rs3, rm } => {
                write!(f, "fmadd.s {rd}, {rs1}, {rs2}, {rs3}, {rm}")
            },
            Self::FmsubS { rd, rs1, rs2, rs3, rm } => {
                write!(f, "fmsub.s {rd}, {rs1}, {rs2}, {rs3}, {rm}")
            },
            Self::FnmsubS { rd, rs1, rs2, rs3, rm } => {
                write!(f, "fnmsub.s {rd}, {rs1}, {rs2}, {rs3}, {rm}")
            },
            Self::FnmaddS { rd, rs1, rs2, rs3, rm } => {
                write!(f, "fnmadd.s {rd}, {rs1}, {rs2}, {rs3}, {rm}")
            },
            Self::FmaddD { rd, rs1, rs2, rs3, rm } => {
                write!(f, "fmadd.d {rd}, {rs1}, {rs2}, {rs3}, {rm}")
            },
            Self::FmsubD { rd, rs1, rs2, rs3, rm } => {
                write!(f, "fmsub.d {rd}, {rs1}, {rs2}, {rs3}, {rm}")
            },
            Self::FnmsubD { rd, rs1, rs2, rs3, rm } => {
                write!(f, "fnmsub.d {rd}, {rs1}, {rs2}, {rs3}, {rm}")
            },
            Self::FnmaddD { rd, rs1, rs2, rs3, rm } => {
                write!(f, "fnmadd.d {rd}, {rs1}, {rs2}, {rs3}, {rm}")
            },
            Self::FclassS { rd, rs1 } => {
                write!(f, "fclass.s {rd}, {rs1}")
            },
            Self::FclassD { rd, rs1 } => {
                write!(f, "fclass.d {rd}, {rs1}")
            },
            Self::FcvtLS { rd, rs1, rm } => {
                write!(f, "fcvt.l.s {rd}, {rs1}, {rm}")
            },
            Self::FcvtLuS { rd, rs1, rm } => {
                write!(f, "fcvt.lu.s {rd}, {rs1}, {rm}")
            },
            Self::FcvtSL { rd, rs1, rm } => {
                write!(f, "fcvt.s.l {rd}, {rs1}, {rm}")
            },
            Self::FcvtSLu { rd, rs1, rm } => {
                write!(f, "fcvt.s.lu {rd}, {rs1}, {rm}")
            },
            Self::FcvtLD { rd, rs1, rm } => {
                write!(f, "fcvt.l.d {rd}, {rs1}, {rm}")
            },
            Self::FcvtLuD { rd, rs1, rm } => {
                write!(f, "fcvt.lu.d {rd}, {rs1}, {rm}")
            },
            Self::FcvtDL { rd, rs1, rm } => {
                write!(f, "fcvt.d.l {rd}, {rs1}, {rm}")
            },
            Self::FcvtDLu { rd, rs1, rm } => {
                write!(f, "fcvt.d.lu {rd}, {rs1}, {rm}")
            },

            Self::Flw { rd, rs1, imm } => {
                write!(f, "flw {rd}, {imm}({rs1})")
            },
            Self::Fld { rd, rs1, imm } => {
                write!(f, "fld {rd}, {imm}({rs1})")
            },
            Self::Fsw { rs2, rs1, imm } => {
                write!(f, "fsw {rs2}, {imm}({rs1})")
            },
            Self::Fsd { rs2, rs1, imm } => {
                write!(f, "fsd {rs2}, {imm}({rs1})")
            },
        }
    }
}

/// 浮点舍入模式
#[derive(Debug, Clone, PartialEq)]
pub enum Rm {
    /// 就近舍入，平局取偶（round to nearest, ties to even）
    Rne,
    /// 向零舍入（round towards zero）
    Rtz,
    /// 向下舍入，朝 -∞（round down）
    Rdn,
    /// 向上舍入，朝 +∞（round up）
    Rup,
    /// 就近舍入，平局取绝对值大者（round to nearest, ties to max magnitude）
    Rmm,
    /// 动态舍入模式，由 frm 寄存器决定
    Dyn,
}

impl fmt::Display for Rm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Rm::Rne => "rne",
            Rm::Rtz => "rtz",
            Rm::Rdn => "rdn",
            Rm::Rup => "rup",
            Rm::Rmm => "rmm",
            Rm::Dyn => "dyn",
        };
        f.write_str(name)
    }
}

/// 伪指令：单精度浮点拷贝 rd = rs，展开为 fsgnj.s rd, rs, rs
pub fn fmv_s(rd: RvVarFLocation, rs: RvVarFLocation) -> RvVarInstr {
    RvVarInstr::FsgnjS { rd: rd, rs1: rs.clone(), rs2: rs }
}

/// 伪指令：双精度浮点拷贝 rd = rs，展开为 fsgnj.d rd, rs, rs
pub fn fmv_d(rd: RvVarFLocation, rs: RvVarFLocation) -> RvVarInstr {
    RvVarInstr::FsgnjD { rd: rd, rs1: rs.clone(), rs2: rs }
}

/// 伪指令：单精度取负 rd = -rs，展开为 fsgnjn.s rd, rs, rs
pub fn fneg_s(rd: RvVarFLocation, rs: RvVarFLocation) -> RvVarInstr {
    RvVarInstr::FsgnjnS { rd: rd, rs1: rs.clone(), rs2: rs }
}

/// 伪指令：双精度取负 rd = -rs，展开为 fsgnjn.d rd, rs, rs
pub fn fneg_d(rd: RvVarFLocation, rs: RvVarFLocation) -> RvVarInstr {
    RvVarInstr::FsgnjnD { rd: rd, rs1: rs.clone(), rs2: rs }
}

/// 伪指令：单精度取绝对值 rd = |rs|，展开为 fsgnjx.s rd, rs, rs
pub fn fabs_s(rd: RvVarFLocation, rs: RvVarFLocation) -> RvVarInstr {
    RvVarInstr::FsgnjxS { rd: rd, rs1: rs.clone(), rs2: rs }
}

/// 伪指令：双精度取绝对值 rd = |rs|，展开为 fsgnjx.d rd, rs, rs
pub fn fabs_d(rd: RvVarFLocation, rs: RvVarFLocation) -> RvVarInstr {
    RvVarInstr::FsgnjxD { rd: rd, rs1: rs.clone(), rs2: rs }
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
