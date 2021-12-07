use crate::lexer::KeyWord;
use crate::lexer::Token;

pub struct AST {}

pub struct GlobalDecl {
    pub list: Vec<ValueSepc>,
}

#[derive(Debug)]
pub struct Ident {
    pub name: String,
}

#[derive(Debug)]
pub struct ValueSepc {
    pub names: Vec<Ident>,
    pub typ: KeyWord,
}
