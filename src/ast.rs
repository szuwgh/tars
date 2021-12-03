use crate::lexer::KeyWord;
use crate::lexer::Token;

pub struct AST {}
pub struct Ident {
    pub name: String,
}

pub struct ValueSepc {
    pub names: Vec<Ident>,
    pub typ: KeyWord,
}
