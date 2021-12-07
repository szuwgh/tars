use crate::ast;
use crate::ast::AST;
use crate::lexer::lexer;
use crate::lexer::Aides;
use crate::lexer::DefaultLexer;
use crate::lexer::KeyWord;
use crate::lexer::LexResult;
use crate::lexer::Token;
pub type ParseResult = Result<Token, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    Eof,
    NoFoundType,
    NoFoundIdent,
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
    fn parse(&mut self) -> Option<AST> {
        self.next();
        let mut gro_decl = ast::GlobalDecl { list: Vec::new() };
        loop {
            match self.tok {
                Token::Eof => break,
                _ => {
                    println!("parse");
                    if let Some(value_spec) = self.parse_global_declaration() {
                        println!("{:?}", value_spec);
                        gro_decl.list.push(value_spec);
                    }
                }
            }
        }
        None
    }

    fn next(&mut self) {
        if let Ok(t) = self.lex.lex() {
            self.tok = t
        } else {
            self.tok = Token::Eof;
        }
    }

    fn parse_global_declaration(&mut self) -> Option<ast::ValueSepc> {
        return self.parse_var_define();
    }

    fn parse_function_declaration(&mut self) -> Option<ast::FuncDecl> {
        None
    }

    // variable_decl ::= type {'*'} id { ',' {'*'} id } ';'
    fn parse_var_define(&mut self) -> Option<ast::ValueSepc> {
        if let Ok(Token::KeyWord(t)) = self.parse_type() {
            if let Some(idents) = self.parse_variable_list() {
                if self.expect_token(Token::Aide(Aides::Semicolon)) {
                    return Some(ast::ValueSepc {
                        names: idents,
                        typ: t,
                    });
                }
            }
        }
        None
    }

    fn expect_token(&mut self, t: Token) -> bool {
        if self.tok == t {
            self.next();
            return true;
        }
        false
    }

    fn parse_variable_list(&mut self) -> Option<Vec<ast::Ident>> {
        let mut list: Vec<ast::Ident> = Vec::new();
        if let Some(s) = self.parse_identifier() {
            list.push(ast::Ident { name: s });
        }
        if self.expect_token(Token::Aide(Aides::Comma)) {
            if let Some(mut i) = self.parse_variable_list() {
                list.append(&mut i);
            }
        }
        Some(list)
    }

    fn parse_identifier(&mut self) -> Option<String> {
        if let Token::Ident(s) = self.tok.clone() {
            self.next();
            return Some(s);
        }
        None
    }

    fn parse_type(&mut self) -> ParseResult {
        if let Token::KeyWord(k) = self.tok {
            if k.is_type() {
                self.next();
                return Ok(Token::KeyWord(k));
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
        int a,b;";
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
