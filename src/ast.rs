use crate::lexer::KeyWord;
use crate::lexer::Token;

pub struct AST {}

pub struct GlobalDecl {
    pub list: Vec<ValueSepc>,
}

#[derive(Debug)]
pub struct FuncDecl {
    pub typ: KeyWord,
    pub fn_name: String,
    pub params: Vec<Param>,
}

#[derive(Debug)]
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
