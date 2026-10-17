use std::fmt;
///
/// 汇编器/链接器级别的符号名。
///
/// 用于寻址与调用类伪指令，由 GNU as / 链接器解析：
///   - `call allocate`      （外部函数，走 PLT）
///   - `la t0, free_ptr`    （外部全局变量，走 GOT）
///   - `lla t0, lambda$0`   （本汇编内的本地符号，PC-relative）
///
/// 典型值：runtime 的 `allocate`、`collect`、`print_int`、
/// `free_ptr`、`fromspace_end`、`rootstack_begin`，以及本地函数名。
///
/// 与 [`Label`](crate::riscv::rv64imfd::Label) 的区别：
///   - `Sym` 是符号名，可能是外部符号，也可能是本地的数据/函数符号；
///   - `Label` 只表示编译器在本汇编内定义的控制流标签，仅用于跳转/分支目标。
#[derive(Clone, PartialEq, Debug)]
pub struct Sym {
    pub name: String,
}

impl Sym {
    pub fn new(name: impl Into<String>) -> Self {
        Sym { name: name.into() }
    }
}

impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
