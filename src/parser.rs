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
    NoSemicolon,
    NoLeftBrace,
    NoLeftParen,
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
                    if let Ok(value_spec) = self.parse_global_declaration() {
                        println!("{:?}", value_spec);
                        gro_decl.list.push(value_spec);
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
                if !self.expect_token(Token::Oper(Operator::LeftParen)) {
                    return Err(ParseError::NoLeftBrace);
                }

                Ok(ast::FuncDecl { typ: t, name: s })
            })
        })
    }

    fn parse_param_list(&mut self) -> ParseResult<Vec<ast::Param>> {
        let mut list: Vec<ast::Param> = Vec::new();
        match self.parse_fn_param() {
            Ok(s) => list.push(ast::Ident { name: s }),
            Err(e) => return Err(e),
        }
        if self.expect_token(Token::Aide(Aides::Comma)) {
            match self.parse_param_list() {
                Ok(mut i) => list.append(&mut i),
                Err(e) => return Err(e),
            }
        }
        Ok(list)
    }

    fn parse_fn_param(&mut self) -> ParseResult<ast::Param> {}

    // variable_decl ::= type {'*'} id { ',' {'*'} id } ';'
    fn parse_var_define(&mut self) -> ParseResult<ast::ValueSepc> {
        self.parse_type().and_then(|t| {
            self.parse_variable_list().and_then(|idents| {
                if self.expect_token(Token::Aide(Aides::Semicolon)) {
                    Ok(ast::ValueSepc {
                        names: idents,
                        typ: t,
                    })
                } else {
                    Err(ParseError::NoSemicolon)
                }
            })
        })
    }

    fn expect_token(&mut self, t: Token) -> bool {
        if self.tok == t {
            self.next();
            return true;
        }
        false
    }

    fn parse_variable_list(&mut self) -> ParseResult<Vec<ast::Ident>> {
        let mut list: Vec<ast::Ident> = Vec::new();
        match self.parse_identifier() {
            Ok(s) => list.push(ast::Ident { name: s }),
            Err(e) => return Err(e),
        }
        if self.expect_token(Token::Aide(Aides::Comma)) {
            match self.parse_variable_list() {
                Ok(mut i) => list.append(&mut i),
                Err(e) => return Err(e),
            }
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
