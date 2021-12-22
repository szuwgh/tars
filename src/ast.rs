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
    pub fn_name: Ident,
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
    pub ident: Ident,
    pub typ: KeyWord,
}

#[derive(Debug)]
pub struct Ident {
    pub name: String,
}

#[derive(Debug)]
pub struct AssignStmt {
    pub x: ExprNode,
    pub op: Token,
    pub y: ExprNode,
}

pub trait Stmt {
    fn stmt_node(&self);
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

#[derive(Debug)]
pub enum StmtNode {
    ValueSepc(ValueSepc),
    AssignStmt(AssignStmt),
}

impl Stmt for StmtNode {
    fn stmt_node(&self) {
        println!("ValueSepc");
    }
}

pub trait Expr {}

#[derive(Debug)]
pub enum ExprNode {
    IdentExpr(Ident),
    UnaryExpr(UnaryExpr),
    BinaryExpr(BinaryExpr),
}

impl Expr for ExprNode {}

#[derive(Debug)]
pub struct BinaryExpr {
    pub x: Box<ExprNode>,
    pub op: Token,
    pub y: Box<ExprNode>,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub op: Token,
    pub x: Box<ExprNode>,
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
