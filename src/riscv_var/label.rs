use std::fmt;
#[derive(Clone, PartialEq, Debug)]
pub struct Label {
    pub name: String,
}

impl Label {
    pub fn new(name: String) -> Self {
        Label { name }
    }
}

impl fmt::Display for Label {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
