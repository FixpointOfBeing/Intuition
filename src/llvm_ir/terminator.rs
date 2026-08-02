use crate::llvm_ir::constant::ConstantRef;
use crate::llvm_ir::function::{
    CallingConvention, FunctionAttribute, ParameterAttribute,
};
use crate::llvm_ir::instruction::InlineAssembly;
use crate::llvm_ir::name::Name;
use crate::llvm_ir::operand::Operand;
use crate::llvm_ir::types::{LLVMType, TypeRef, Typed, Types};
use either::Either;

/// 一个基本块（Basic Block）的终结指令（Terminator）。
///
/// 每个基本块的最后一条指令必须是且只能是一个 Terminator，它决定该
/// 基本块执行完后控制流去往何处（返回、跳转、抛出异常或不可达）。
///
/// LLVM IR 一共定义了 12 种终结指令，按用途大致分为三类：
///
/// - 普通控制流：[`Ret`]、[`Br`]、[`CondBr`]、[`Switch`]、[`IndirectBr`]、[`Unreachable`]
/// - 带异常展开（unwind）的调用：[`Invoke`]、[`CallBr`]
/// - 异常处理（EH）专用：[`Resume`]、[`CleanupRet`]、[`CatchRet`]、[`CatchSwitch`]
///
/// 大多数 Terminator 都没有 SSA 结果，只有 [`Invoke`]、[`CallBr`] 和
/// [`CatchSwitch`] 会像普通指令一样产生一个 `result`（见
/// [`Terminator::try_get_result`]）。
#[derive(PartialEq, Clone, Debug, Hash)]
pub enum Terminator {
    Ret(Ret),
    Br(Br),
    CondBr(CondBr),
    Switch(Switch),
    IndirectBr(IndirectBr),
    Invoke(Invoke),
    Resume(Resume),
    Unreachable(Unreachable),
    CleanupRet(CleanupRet),
    CatchRet(CatchRet),
    CatchSwitch(CatchSwitch),
    CallBr(CallBr),
}

impl Typed for Terminator {
    fn get_type(&self, types: &Types) -> TypeRef {
        match self {
            Terminator::Ret(t) => types.type_of(t),
            Terminator::Br(t) => types.type_of(t),
            Terminator::CondBr(t) => types.type_of(t),
            Terminator::Switch(t) => types.type_of(t),
            Terminator::IndirectBr(t) => types.type_of(t),
            Terminator::Invoke(t) => types.type_of(t),
            Terminator::Resume(t) => types.type_of(t),
            Terminator::Unreachable(t) => types.type_of(t),
            Terminator::CleanupRet(t) => types.type_of(t),
            Terminator::CatchRet(t) => types.type_of(t),
            Terminator::CatchSwitch(t) => types.type_of(t),
            Terminator::CallBr(t) => types.type_of(t),
        }
    }
}

impl Terminator {
    pub fn try_get_result(&self) -> Option<&Name> {
        match self {
            Terminator::Ret(_) => None,
            Terminator::Br(_) => None,
            Terminator::CondBr(_) => None,
            Terminator::Switch(_) => None,
            Terminator::IndirectBr(_) => None,
            Terminator::Invoke(t) => Some(&t.result),
            Terminator::Resume(_) => None,
            Terminator::Unreachable(_) => None,
            Terminator::CleanupRet(_) => None,
            Terminator::CatchRet(_) => None,
            Terminator::CatchSwitch(t) => Some(&t.result),
            Terminator::CallBr(t) => Some(&t.result),
        }
    }
}

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
/// ```rust,ignore
/// fn foo(x: i32) -> i32 { x + 1 }   // -> ret i32 %add
/// fn bar() {}                       // -> ret void
/// ```
///
/// 注意：`ret` 指令本身的类型是 void（即使它所在函数的返回类型不是
/// void），这也是下面 `Typed` 实现直接返回 `types.void()` 的原因。
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Ret {
    /// 要返回的值；`None` 对应 `ret void`。
    pub return_operand: Option<Operand>,
}
impl Typed for Ret {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

/// `br` —— 无条件跳转到另一个基本块。
///
/// 语法：
/// ```text
/// br label <dest>
/// ```
///
/// Rust 中的循环回边、`if`/`else` 的汇合点、以及所有不需要判断条件
/// 的跳转都会编译成 `br`。例如：
///
/// ```rust,ignore
/// fn count_down(n: i32) -> i32 {
///     let mut i = n;
///     while i > 0 {
///         i -= 1;              // 循环体末尾编译为: br label %loop（回边）
///     }
///     i                        // 退出循环后: br label %exit（汇合）
/// }
/// ```
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Br {
    pub dest: Name,
}

impl Typed for Br {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

/// `condbr` —— 根据一个 `i1` 条件跳转到两个基本块之一。
///
/// 语法：
/// ```text
/// br i1 <cond>, label <true_dest>, label <false_dest>
/// ```
///
/// Rust 中的 `if` / `if let`、以及尚未被优化合并成 `switch` 的分支型
/// `match`，都会编译成 `condbr`：
///
/// ```rust,ignore
/// fn sign(x: i32) -> i32 {
///     if x > 0 { 1 } else { 0 }
///     // icmp sgt i32 %x, 0
///     // br i1 %cmp, label %then, label %else
/// }
/// ```
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CondBr {
    pub condition: Operand,
    pub true_dest: Name,
    pub false_dest: Name,
}

impl From<CondBr> for Terminator {
    fn from(term: CondBr) -> Terminator {
        Terminator::CondBr(term)
    }
}

impl Typed for CondBr {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

/// `switch` —— 按一个整数操作数的取值，在多个目标基本块之间跳转
/// （跳转表 jump table）。
///
/// 语法：
/// ```text
/// switch <intty> <value>, label <default_dest> [
///     <intty> <val1>, label <dest1>
///     <intty> <val2>, label <dest2>
///     ...
/// ]
/// ```
///
/// 语义上等价于一系列 `icmp` + `condbr`，但跳转表形式便于后端生成
/// 高效的查表跳转。Rust 中 `match` 一个整数且分支较多时，优化后 LLVM
/// 会把分支链合并成 `switch`：
///
/// ```rust,ignore
/// fn classify(x: i32) -> i32 {
///     match x {
///         0 => 100,
///         1 => 200,
///         2 => 300,
///         _ => 0,      // 对应 default_dest
///     }
/// }
/// ```
///
/// 注意：`-C opt-level=0` 时上面的 match 会被编译成一串 `condbr`，
/// `switch` 需要经过优化（跳转表合并）才会出现。
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Switch {
    /// 作为跳转依据的整数值。
    pub operand: Operand,
    /// 各个 case 的取值与对应的目标基本块（值必须互不相同）。
    pub dests: Vec<(ConstantRef, Name)>,
    /// 没有命中任何 case 时跳转的默认基本块。
    pub default_dest: Name,
}

impl Typed for Switch {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

/// `indirectbr` —— 间接跳转：跳到保存在寄存器/内存中的基本块地址。
///
/// 语法：
/// ```text
/// indirectbr ptr <address>, [ label <dest1>, label <dest2>, ... ]
/// ```
///
/// 后面的 label 列表列出所有可能的目标，供优化器做跳转目标集合分析。
/// 它对应 C 的「计算 goto」（GCC 扩展 `goto *ptr`），常被解释器主循环
/// 用来做指令分派。
///
/// Rust 没有计算 goto，因此正常编译不会产生 `indirectbr`；它一般来自
/// C/C++ 代码或手工编写的 IR。
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct IndirectBr {
    /// 跳转目标所在的基本块地址。
    pub operand: Operand,
    /// 所有可能跳转到的基本块（供优化器做目标集合分析）。
    pub possible_dests: Vec<Name>,
}

impl Typed for IndirectBr {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

/// `invoke` —— 带异常路径的函数调用。
///
/// 语法：
/// ```text
/// %result = invoke <retty> @func(<args>)
///     to label <return_label> unwind label <exception_label>
/// ```
///
/// 与普通 `call` 的区别：被调函数可能「展开」（unwind，例如 Rust 的
/// panic）。调用结束后控制流有两个去向：
///
/// - 正常返回（被调函数以 `ret` 结束）→ 跳到 `return_label`
/// - 异常展开（异常传播到这里）→ 跳到 `exception_label`（必须是
///   landing pad，通常以 `landingpad` / `catchswitch` / `cleanuppad`
///   指令开头）
///
/// 在 `-C panic=unwind` 下，rustc 把所有「可能 panic 的调用」编译成
/// `invoke`，landing pad 里运行 drop 清理并调用 panic 处理逻辑。例如
/// 索引访问可能越界 panic：
///
/// ```rust,ignore
/// fn first(v: &Vec<i32>) -> i32 {
///     v[0]
///     // invoke void ...panic_bounds_check(...)
///     //     to label %bb1 unwind label %lpad
/// }
/// ```
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Invoke {
    pub function: Either<InlineAssembly, Operand>,
    pub function_ty: TypeRef,
    pub arguments: Vec<(Operand, Vec<ParameterAttribute>)>,
    pub return_attributes: Vec<ParameterAttribute>,
    /// 存放调用结果（被调函数以 `ret` 正常返回时）的 SSA 变量名。
    pub result: Name,
    /// 正常返回后继续执行的基本块：被调函数以 `ret` 返回时，控制流跳到这里。
    pub return_label: Name,
    /// 异常路径的目标基本块：被调函数以 `resume` 等方式展开异常时，
    /// 控制流跳到这里（该基本块必须是 landing pad）。
    pub exception_label: Name,
    pub function_attributes: Vec<FunctionAttribute>,
    pub calling_convention: CallingConvention,
}

impl Typed for Invoke {
    fn get_type(&self, _types: &Types) -> TypeRef {
        match self.function_ty.as_ref() {
            LLVMType::FuncType { result_type, .. } => {
                result_type.clone()
            },
            ty => panic!(
                "Expected Invoke.function_ty to be a FuncType, got {:?}",
                ty
            ),
        }
    }
}

/// `resume` —— landing pad 完成清理后，继续向外传播异常。
///
/// 语法：
/// ```text
/// resume <type> <value>
/// ```
///
/// 当一个 landing pad 只负责清理（运行 drop、释放资源）而不处理异常
/// 时，清理完毕后必须用 `resume` 把异常继续抛给外层（调用者）。
/// 操作数通常是 `landingpad` / `cleanuppad` 指令产生的结果。
///
/// Rust 的 panic 展开路径就是这种模式：先运行沿途各局部变量的 drop，
/// 再用 `resume` 继续展开；`catch_unwind` 的闭包内再次 panic 时，内部
/// landing pad 清理后也会走到 `resume`：
///
/// ```rust,ignore
/// use std::panic;
/// fn foo() {
///     let _ = panic::catch_unwind(|| panic!("unwound"));
///     // 闭包 panic 后: landingpad -> 运行清理 -> resume 继续抛出，
///     // 最后由 catch_unwind 所在的 landing pad 接住。
/// }
/// ```
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Resume {
    pub operand: Operand,
}

impl Typed for Resume {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

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
///
/// Rust 中以下代码会编译出 `unreachable`：
///
/// ```rust,ignore
/// fn always_panics() -> i32 {
///     panic!("boom");          // call panic 之后紧跟 unreachable
/// }
///
/// fn dead_branch(x: bool) -> i32 {
///     if x { 1 } else { unreachable!() }  // else 分支: call + unreachable
/// }
///
/// fn diverges() -> ! {
///     loop {}                  // 无限循环之后的所有路径都是 unreachable
/// }
/// ```
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct Unreachable {
}

impl From<Unreachable> for Terminator {
    fn from(term: Unreachable) -> Terminator {
        Terminator::Unreachable(term)
    }
}

impl Typed for Unreachable {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

/// `cleanupret` —— 终止一个 cleanup pad（由 `cleanuppad` 开启），完成
/// 清理后继续异常展开。
///
/// 语法：
/// ```text
/// cleanupret from <cleanup_pad> unwind to caller
/// cleanupret from <cleanup_pad> unwind label <dest>
/// ```
///
/// 与 `resume` 的关系：`resume` 属于「旧」的 landingpad 异常模型，而
/// `cleanupret` 属于「新」的 funclet 异常模型（LLVM 的 Windows EH）。
/// cleanup pad 只负责清理（运行 drop、释放资源），不处理异常；清理完
/// 用 `cleanupret` 把异常交给 `unwind` 目标，或继续抛给调用者。
///
/// Rust 以 `-C panic=unwind` 在 MSVC 目标上编译时，panic 展开路径上的
/// drop 清理就是通过 cleanuppad/cleanupret 实现的：
///
/// ```rust,ignore
/// struct Guard;
/// impl Drop for Guard { fn drop(&mut self) {} }
///
/// fn foo() {
///     let _g = Guard;
///     panic!("unwind");
///     // 展开时: cleanuppad -> drop _g -> cleanupret unwind to caller
/// }
/// ```
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CleanupRet {
    /// 本 cleanup pad 的入口值（`cleanuppad` 指令的结果）。
    pub cleanup_pad: Operand,
    /// 清理完成后异常的去向：`Some(dest)` 对应 `unwind label <dest>`，
    /// `None` 对应 `unwind to caller`（继续抛给调用者）。
    pub unwind_dest: Option<Name>,
}

impl Typed for CleanupRet {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

/// `catchret` —— 终止一个 catch pad（由 `catchpad` 开启），表示异常已
/// 被处理，控制流正常继续。
///
/// 语法：
/// ```text
/// catchret from <catch_pad> to label <successor>
/// ```
///
/// 当某个 catch handler 完整执行完（没有重新抛出异常）时，用它跳回
/// 正常控制流；此后该异常视为已被处理，不再向上传播。
///
/// Rust 的 `catch_unwind` 在 MSVC 目标上会用到 catchswitch / catchpad /
/// catchret：闭包被包在 catch pad 里，正常返回后由 `catchret` 跳回
/// 后续代码：
///
/// ```rust,ignore
/// use std::panic;
/// fn foo() {
///     let r = panic::catch_unwind(|| 42);
///     // 闭包正常返回后: catchret from %catchpad to label %cont
/// }
/// ```
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CatchRet {
    pub catch_pad: Operand,
    pub successor: Name,
}

impl Typed for CatchRet {
    fn get_type(&self, types: &Types) -> TypeRef {
        types.void()
    }
}

/// `catchswitch` —— 异常分派：根据传入的异常跳转到对应的 catch
/// handler（或 unwind 目标）。
///
/// 语法：
/// ```text
/// %result = catchswitch within <parent_pad> [
///     label <handler1>, label <handler2>, ...
/// ] unwind to caller
/// %result = catchswitch within <parent_pad> [
///     label <handler1>, label <handler2>, ...
/// ] unwind label <dest>
/// ```
///
/// `catchswitch` 本身是一个 pad（异常处理入口）。它把到达的异常分派给
/// 列表中的 catch pad（每个 handler 都是一条 `catchpad` 指令）；如果
/// 异常不匹配任何 handler，则交给 `unwind` 目标，或继续抛给调用者。
/// `parent_pad` 是它的父 funclet，最外层时为 `none`。
///
/// `invoke` 的异常目标也可以是 `catchswitch`（`invoke ... unwind label
/// %catchswitch`），表示异常先进 catchswitch 分派。Rust 的
/// `catch_unwind` 在 MSVC 目标上编译时，异常入口就是一个 catchswitch：
///
/// ```rust,ignore
/// use std::panic;
/// fn foo() {
///     let _ = panic::catch_unwind(|| panic!("boom"));
///     // invoke ... unwind label %catchswitch
///     // %catchswitch: catchswitch within none [label %catchpad] unwind to caller
/// }
/// ```
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CatchSwitch {
    /// 父 funclet（`catchswitch` 或 `cleanuppad` 的结果）；最外层为 `none`。
    pub parent_pad: Operand,
    /// 候选的 catch handler（每个都是一条 `catchpad` 指令所在的基本块）。
    pub catch_handlers: Vec<Name>,
    /// 异常不匹配任何 handler 时的去向：`None` 对应 `unwind to caller`。
    pub default_unwind_dest: Option<Name>,
    /// 存放本 catchswitch pad 结果的 SSA 变量名。
    pub result: Name,
}

impl Typed for CatchSwitch {
    fn get_type(&self, _types: &Types) -> TypeRef {
        unimplemented!("Typed for CatchSwitch")
    }
}

/// `callbr` —— 带额外跳转目标的调用（主要用于内联汇编的 `goto`）。
///
/// 语法：
/// ```text
/// %result = callbr <retty> @func(<args>)
///     to label <return_label> [label <dest1>, label <dest2>, ...]
/// ```
///
/// 除了正常返回时跳到 `return_label` 之外，被调用的内联汇编可以直接
/// 把控制流转到后面的任意一个 label（对应 GCC 风格内联汇编的 "goto"
/// 功能 / C 的 `asm goto`）。与 `invoke` 不同，这些额外目标不是异常
/// 路径，而是正常的控制流。
///
/// Rust 的 `asm!` 不支持 label 操作数，因此 Rust 编译不出 `callbr`；
/// 它主要来自 C 代码的 `asm goto`、部分 LLVM 内建函数或手工 IR。
///
/// 注意：本仓库的 [`CallBr::other_labels`] 目前用 `()` 占位，尚未实现
/// 这些额外跳转目标。
#[derive(PartialEq, Clone, Debug, Hash)]
pub struct CallBr {
    pub function: Either<InlineAssembly, Operand>,
    pub arguments: Vec<(Operand, Vec<ParameterAttribute>)>,
    pub return_attributes: Vec<ParameterAttribute>,
    /// 存放调用结果（被调函数以 `ret` 正常返回时）的 SSA 变量名。
    pub result: Name,
    /// 正常返回（fallthrough）后继续执行的基本块。
    pub return_label: Name,
    /// 额外的跳转目标：内联汇编可以通过 `goto` 把控制流直接跳到这些
    /// 基本块（对应 LLVM 的 indirect labels）。目前以 `()` 占位，尚未实现。
    pub other_labels: (),
    pub function_attributes: Vec<FunctionAttribute>,
    pub calling_convention: CallingConvention,
}

impl Typed for CallBr {
    fn get_type(&self, types: &Types) -> TypeRef {
        match types.type_of(&self.function).as_ref() {
            LLVMType::FuncType { result_type, .. } => {
                result_type.clone()
            },
            ty => panic!(
                "Expected the function argument of a CallBr to have type FuncType; got {:?}",
                ty
            ),
        }
    }
}
