use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum LdSym {
    Hi(String),
    Lo(String),
}

impl LdSym {
    pub fn hi(name: impl Into<String>) -> Self {
        LdSym::Hi(name.into())
    }
    pub fn lo(name: impl Into<String>) -> Self {
        LdSym::Lo(name.into())
    }
}

impl fmt::Display for LdSym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LdSym::Hi(n) => write!(f, "%hi({n})"),
            LdSym::Lo(n) => write!(f, "%lo({n})"),
        }
    }
}
