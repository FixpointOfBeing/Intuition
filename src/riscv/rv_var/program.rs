use crate::riscv::rv_var::RvVarBasicBlock;
use crate::riscv::rv_var::RvVarLocation;
use crate::riscv::rv64imfd::Label;
use crate::syntax::Type;
use std::fmt;

pub struct RvVarFunction {
    pub fun_name: String,           // 函数名
    pub params: Vec<RvVarLocation>, // 参数（变量位置）
    pub blocks: Vec<RvVarBasicBlock>,
    pub entry: Label, // 入口块标签
}

impl RvVarFunction {
    pub fn append_basic_block(&mut self, block: RvVarBasicBlock) {
        self.blocks.push(block);
    }
}

impl fmt::Display for RvVarFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.fun_name)?;
        for (i, block) in self.blocks.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{block}")?;
        }
        Ok(())
    }
}

pub struct RvVarGlobal {
    pub name: String, // 符号名
    pub ty: Type,     // 决定 ld 还是 fld
}

pub struct RvVarProgram {
    pub functions: Vec<RvVarFunction>,
    pub globals: Vec<RvVarGlobal>,
}

impl RvVarProgram {
    pub fn new() -> Self {
        RvVarProgram { functions: vec![], globals: vec![] }
    }

    pub fn append_function(&mut self, function: RvVarFunction) {
        self.functions.push(function);
    }

    pub fn append_global(&mut self, global: RvVarGlobal) {
        self.globals.push(global);
    }
}

impl fmt::Display for RvVarProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, function) in self.functions.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{function}")?;
        }
        Ok(())
    }
}
