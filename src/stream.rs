use crate::parser::parser::Expr;

#[derive(Debug, Clone)]
pub struct Stream {
    pub name: Expr,
    pub content: String
}

impl Stream {
    pub fn new(name: Expr, content: String) -> Self {
        Self {name, content}
    }

    pub fn name(&self) -> &Expr { &self.name }
    pub fn content(&self) -> &String { &self.content }
}
