use crate::lexer::KeyWord;
use crate::lexer::Token;

pub struct AST {}

pub struct GlobalDecl {
    pub list: Vec<ValueSepc>,
}

pub struct Ident {
    pub name: String,
}

pub struct ValueSepc {
    pub names: Vec<Ident>,
    pub typ: KeyWord,
}
