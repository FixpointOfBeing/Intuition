/// Many LLVM objects have a `Name`, which is either a string name, or just a
/// sequential numbering (e.g. `%3`).
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Hash)]
pub enum Name {
    Name(String),
    Number(usize),
}

impl Name {
    pub(crate) fn name_or_num(s: String, ctr: &mut usize) -> Self {
        if s.is_empty() {
            let rval = Name::Number(*ctr);
            *ctr += 1;
            rval
        } else {
            Name::Name(s)
        }
    }
}
