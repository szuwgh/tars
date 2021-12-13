use crate::ast;
use crate::ast::Stmt;
use crate::ast::AST;
use crate::lexer::lexer;
use crate::lexer::Aides;
use crate::lexer::DefaultLexer;
use crate::lexer::KeyWord;
use crate::lexer::LexResult;
use crate::lexer::Operator;
use crate::lexer::Token;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    Eof,
    NoFoundType,
    NoFoundIdent,
    Expect(Token),
}

struct Parser<L: lexer> {
    lex: L,
    tok: Token,
}

impl<L: lexer> Parser<L> {
    pub fn new(l: L) -> Parser<L> {
        Self {
            lex: l,
            tok: Token::Eof,
        }
    }
    fn parse(&mut self) -> ParseResult<AST> {
        self.next();
        let mut gro_decl = ast::GlobalDecl { list: Vec::new() };
        loop {
            match self.tok {
                Token::Eof => break,
                Token::KeyWord(KeyWord::Var) => {
                    println!("parse var");
                    match self.parse_global_declaration() {
                        Ok(value_spec) => {
                            println!("{:?}", value_spec);
                            gro_decl.list.push(value_spec);
                        }
                        Err(e) => println!("{:?}", e),
                    }
                }
                Token::KeyWord(KeyWord::Fn) => {
                    println!("parse fn");
                    match self.parse_function_declaration() {
                        Ok(fn_spec) => {
                            println!("{:?}", fn_spec);
                        }
                        Err(e) => println!("{:?}", e),
                    }
                }
                _ => break,
            }
        }
        Err(ParseError::Eof)
    }

    fn next(&mut self) {
        if let Ok(t) = self.lex.lex() {
            self.tok = t
        } else {
            self.tok = Token::Eof;
        }
    }

    fn parse_global_declaration(&mut self) -> ParseResult<ast::ValueSepc> {
        return self.parse_declaration();
    }

    fn parse_declaration(&mut self) -> ParseResult<ast::ValueSepc> {
        self.next();
        return self.parse_var_define();
    }

    // fn parse_gen_decl<F:>(t: Token, f: F) {}

    fn parse_function_declaration(&mut self) -> ParseResult<ast::FuncDecl> {
        self.next();
        return self.parse_function_define();
    }

    fn parse_stmt_list(&mut self) -> ParseResult<Vec<Box<dyn ast::Stmt>>> {
        let mut list: Vec<Box<dyn ast::Stmt>> = Vec::new();
        while self.tok != Token::Oper(Operator::RightBrace) && self.tok != Token::Eof {
            list.push(self.parse_stmt()?);
        }
        Ok(list)
    }

    fn parse_stmt(&mut self) -> ParseResult<Box<dyn ast::Stmt>> {
        return match self.tok {
            Token::KeyWord(KeyWord::Var) => Ok(Box::new(self.parse_declaration()?)),
            _ => Err(ParseError::NoFoundIdent),
        };
    }

    //function_define ::= type id (param) { func body }
    fn parse_function_define(&mut self) -> ParseResult<ast::FuncDecl> {
        self.parse_type().and_then(|t| {
            self.parse_identifier().and_then(|s| {
                self.expect_token(Token::Oper(Operator::LeftParen))?;
                let mut params: Vec<ast::Param> = Vec::new();
                if !self.match_token(Token::Oper(Operator::RightParen)) {
                    params = self.parse_param_list()?;
                }
                self.expect_token(Token::Oper(Operator::RightParen))?;
                self.expect_token(Token::Oper(Operator::LeftBrace))?;
                let body = self.parse_func_body()?;
                self.expect_token(Token::Oper(Operator::RightBrace))?;
                Ok(ast::FuncDecl {
                    typ: t,
                    fn_name: s,
                    params: params,
                    body: body,
                })
            })
        })
    }

    fn parse_func_body(&mut self) -> ParseResult<ast::FuncBody> {
        Ok(ast::FuncBody {
            list: self.parse_stmt_list()?,
        })
    }

    fn parse_param_list(&mut self) -> ParseResult<Vec<ast::Param>> {
        let mut list: Vec<ast::Param> = Vec::new();
        list.push(self.parse_fn_param()?);
        match self.expect_token(Token::Aide(Aides::Comma)) {
            Ok(()) => list.append(&mut self.parse_param_list()?),
            Err(_) => (),
        }
        Ok(list)
    }

    fn parse_fn_param(&mut self) -> ParseResult<ast::Param> {
        self.parse_type().and_then(|t| {
            self.parse_identifier()
                .and_then(|s| Ok(ast::Param { ident: s, typ: t }))
        })
    }

    // variable_decl ::= type {'*'} id { ',' {'*'} id } ';'
    fn parse_var_define(&mut self) -> ParseResult<ast::ValueSepc> {
        self.parse_type().and_then(|t| {
            self.parse_variable_list().and_then(|idents| {
                self.expect_token(Token::Aide(Aides::Semicolon))?;
                Ok(ast::ValueSepc {
                    names: idents,
                    typ: t,
                })
            })
        })
    }

    fn expect_token(&mut self, t: Token) -> ParseResult<()> {
        if self.tok == t {
            self.next();
            return Ok(());
        }
        Err(ParseError::Expect(t))
    }

    fn match_token(&mut self, t: Token) -> bool {
        if self.tok == t {
            return true;
        }
        false
    }

    fn parse_variable_list(&mut self) -> ParseResult<Vec<ast::Ident>> {
        let mut list: Vec<ast::Ident> = Vec::new();

        list.push(ast::Ident {
            name: self.parse_identifier()?,
        });
        match self.expect_token(Token::Aide(Aides::Comma)) {
            Ok(()) => list.append(&mut self.parse_variable_list()?),
            Err(_) => (),
        }
        Ok(list)
    }

    fn parse_identifier(&mut self) -> ParseResult<String> {
        if let Token::Ident(s) = self.tok.clone() {
            self.next();
            return Ok(s);
        }
        Err(ParseError::NoFoundIdent)
    }

    fn parse_type(&mut self) -> ParseResult<KeyWord> {
        if let Token::KeyWord(k) = self.tok {
            if k.is_type() {
                self.next();
                return Ok(k);
            }
        }
        Err(ParseError::NoFoundType)
    }
}

mod tests {
    use super::*;
    use std::fs::OpenOptions;
    #[test]
    fn test_parser() {
        let s = "
        var int a,c;
        fn int b(int d,int e){
            var int f;
        }
        ";
        let mut lexer = DefaultLexer::new(s.as_bytes());
        let mut parser = Parser::new(lexer);
        parser.parse();
        // loop {
        //     let m = lexer.lex();
        //     match m {
        //         Ok(c) => println!("{:?}", c),
        //         Err(e) => {
        //             println!("{:?}", e);
        //             break;
        //         }
        //     }
        // }
    }
}
