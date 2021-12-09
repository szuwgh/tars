use crate::lexer::KeyWord;
use crate::lexer::Token;

pub struct AST {}

pub struct GlobalDecl {
    pub list: Vec<ValueSepc>,
}

pub struct FuncDecl {
    pub typ: KeyWord,
    pub fn_name: String,
    pub param: Vec<Param>,
}

pub struct Param {
    pub ident: String,
    pub typ: KeyWord,
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
