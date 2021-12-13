use crate::lexer::KeyWord;
use crate::lexer::Token;
use std::fmt;
use std::fmt::Debug;

pub struct AST {}

pub struct GlobalDecl {
    pub list: Vec<ValueSepc>,
}

#[derive(Debug)]
pub struct FuncDecl {
    pub typ: KeyWord,
    pub fn_name: String,
    pub params: Vec<Param>,
    pub body: FuncBody,
}

pub struct FuncBody {
    pub list: Vec<Box<dyn Stmt>>,
}

impl Debug for FuncBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
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

pub trait Stmt {
    // fn test(&self);
}

#[derive(Debug)]
pub struct ValueSepc {
    pub names: Vec<Ident>,
    pub typ: KeyWord,
}

impl Stmt for ValueSepc {
    //fn test(&self) {}
}
