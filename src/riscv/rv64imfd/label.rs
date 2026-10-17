use std::fmt;

/// 汇编内部的控制流标签。
///
/// 由编译器在生成的汇编文件中自行定义，以 `label:` 形式出现，
/// 只作为跳转与分支指令的目标：
///   - `j .Lthen`
///   - `beq t0, zero, .Lelse`
///   - `tail block0`
///
/// 典型值：`.Lthen`、`.Lelse`、`block0`、函数入口名等。
///
/// 与 [`Sym`](crate::riscv::rv64imfd::Sym) 的区别：
///   - `Label` 一定由当前汇编定义，不参与跨目标文件的链接解析；
///   - `Sym` 是交给汇编器/链接器解析的符号名，可以指向外部符号
#[derive(Clone, PartialEq, Debug)]
pub struct Label {
    pub name: String,
}

impl Label {
    pub fn new(name: impl Into<String>) -> Self {
        Label { name: name.into() }
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
