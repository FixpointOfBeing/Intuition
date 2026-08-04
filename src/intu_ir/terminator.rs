use crate::intu_ir::name::Name;
use crate::intu_ir::operand::Operand;
use crate::intu_ir::types::{InstType, TypeRef, Typed, Types};

/// 一个基本块（Basic Block）的终结指令（Terminator）。
///
/// 每个基本块的最后一条指令必须是且只能是一个 Terminator，它决定该
/// 基本块执行完后控制流去往何处（返回、跳转、抛出异常或不可达）。
///
///
/// [`Ret`]、[`Br`]、[`CondBr`]、[`IndirectBr`]、
///
/// 大多数 Terminator 都没有 结果
#[derive(PartialEq, Clone, Debug, Hash)]
pub enum Terminator {
/// `ret` —— 从当前函数返回。
///
/// 语法：
/// ```text
/// ret <type> <value>    // 返回一个值
/// ret void              // 不返回值
/// ```
///
/// 函数的每一条返回路径最终都会归约为一条 `ret`。Rust 中所有正常返回
/// 的函数都会编译出它：
///

/// 注意：`ret` 指令本身的类型是 void（即使它所在函数的返回类型不是
/// void），这也是`Typed` 实现直接返回 `types.void()` 的原因。

    Ret{
        
    return_operand: Option<Operand>,
    },
/// `br` —— 无条件跳转到另一个基本块。
///
/// 语法：
/// ```text
/// br label <dest>
/// ```
///

    Br{
        
    dest: Name,
    },
/// `condbr` —— 根据一个 `i1` 条件跳转到两个基本块之一。
///
/// 语法：
/// ```text
/// br i1 <cond>, label <true_dest>, label <false_dest>
/// ```
///

    CondBr{
        
    condition: Operand,
    true_dest: Name,
    false_dest: Name,
    },
/// `indirectbr` —— 间接跳转：跳到保存在寄存器/内存中的基本块地址。
///
/// 语法：
/// ```text
/// indirectbr ptr <address>, [ label <dest1>, label <dest2>, ... ]

    IndirectBr{
        
    /// 跳转目标所在的基本块地址。
    operand: Operand,
    /// 所有可能跳转到的基本块（供优化器做目标集合分析）。
    possible_dests: Vec<Name>,
    },
/// `unreachable` —— 声明此处代码不可达；一旦执行到即为未定义行为
/// （UB）。
///
/// 语法：
/// ```text
/// unreachable
/// ```
///
/// 它不产生任何跳转，只是告诉优化器「这条路径永远不会被执行」，从而
/// 允许优化器删除死代码、按任意值推导结果。
    Unreachable,
}

impl Typed for Terminator {
    fn get_type(&self, types: &Types) -> TypeRef {
   match self {
    Terminator::Ret { return_operand } => {
        
        types.void()
    }
    Terminator::Br { dest } => {
        
        types.void()
    }
    Terminator::CondBr { condition, true_dest, false_dest } => {
        
        types.void()
    }
    Terminator::IndirectBr { operand, possible_dests } => {
        
        types.void()
    }
    Terminator::Unreachable => {
        
        types.void()
    }
   }
    }
}
