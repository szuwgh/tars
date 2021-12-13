use crate::lexer::KeyWord;
use crate::lexer::Token;
use std::fmt;
use std::fmt::Debug;

pub struct AST {}

pub struct GlobalDecl {
    pub list: Vec<ValueSepc>,
}

#[derive(Debug)]
pub struct FuncDecl<T: Stmt + Debug> {
    pub typ: KeyWord,
    pub fn_name: String,
    pub params: Vec<Param>,
    pub body: FuncBody<T>,
}

pub struct FuncBody<T: Stmt + Debug> {
    pub list: Vec<T>,
}

impl<T: Stmt + Debug> Debug for FuncBody<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.list.iter() {
            x.fmt(f)?;
        }
        Ok(())
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

pub enum StmtNode {
    ValueSepc(ValueSepc),
}

impl Stmt for StmtNode {
    fn stmt_node(&self) {
        println!("ValueSepc");
    }
}

impl Debug for StmtNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            StmtNode::ValueSepc(v) => v.fmt(f),
            _ => Err(fmt::Error {}),
        };
    }
}

pub trait Stmt {
    fn stmt_node(&self);
}

#[derive(Debug)]
pub struct ValueSepc {
    pub names: Vec<Ident>,
    pub typ: KeyWord,
}

impl Stmt for ValueSepc {
    fn stmt_node(&self) {
        println!("ValueSepc");
    }
}
