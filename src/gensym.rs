use crate::syntax::Ident;

pub struct Gensym(u64);
impl Gensym {
    pub fn new() -> Self {
        Gensym(0)
    }

    pub fn fresh(&mut self) -> Ident {
        let name = format!("${}", self.0);
        self.0 += 1;
        name
    }

    pub fn inc_fresh(&mut self, name: &str) -> Ident {
        let id = self.0;
        self.0 += 1;
        format!("{}.{}", name, id)
    }

    pub fn fresh_with_prefix(&mut self, prefix: &str) -> Ident {
        let name = format!("{}${}", prefix, self.0);
        self.0 += 1;
        name
    }
}
