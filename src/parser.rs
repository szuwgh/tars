use crate::ast;
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
    //err_log
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
                    if let Ok(fn_spec) = self.parse_global_declaration() {
                        println!("{:?}", fn_spec);
                        gro_decl.list.push(fn_spec);
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
        self.next();
        return self.parse_var_define();
    }

    fn parse_function_declaration(&mut self) -> ParseResult<ast::FuncDecl> {
        self.next();
        Err(ParseError::Eof)
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
                self.expect_token(Token::Oper(Operator::RightBrace))?;
                Ok(ast::FuncDecl {
                    typ: t,
                    fn_name: s,
                    params: params,
                })
            })
        })
    }

    //  let params: Vec<ast::Param>;
    //     if !self.match_token(Token::Oper(Operator::RightParen)) {
    //         self.parse_param_list()
    //             .and_then(|params| {
    //                 if !self.expect_token(Token::Oper(Operator::RightParen){
    //                     return Err(ParseError::NoRightParen);
    //                 }
    //                  Ok(ast::FuncDecl {
    //         typ: t,
    //         fn_name: s,
    //         params: params,
    //     })
    // }

    fn parse_func_body() {}

    fn parse_param_list(&mut self) -> ParseResult<Vec<ast::Param>> {
        let mut list: Vec<ast::Param> = Vec::new();
        list.push(self.parse_fn_param()?);
        self.expect_token(Token::Aide(Aides::Comma))?;
        list.append(&mut self.parse_param_list()?);
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
        self.expect_token(Token::Aide(Aides::Comma)).and_then(|()| {
            list.append(&mut self.parse_variable_list()?);
            Ok(())
        });
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
        var int a,c;";
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
