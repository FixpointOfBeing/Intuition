use crate::riscv::rv_var::RvVarBasicBlock;
use crate::riscv::rv_var::RvVarLocation;
use crate::riscv::rv64imfd::Label;
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

pub struct RvVarProgram {
    pub functions: Vec<RvVarFunction>,
}

impl RvVarProgram {
    pub fn new() -> Self {
        RvVarProgram { functions: vec![] }
    }

    pub fn append_function(&mut self, function: RvVarFunction) {
        self.functions.push(function);
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
