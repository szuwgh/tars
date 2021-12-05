use crate::ast;
use crate::ast::AST;
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

pub trait lexer {
    fn next(&mut self) -> LexResult;
}

struct Parser<L: lexer> {
    lex: L,
    tok: Token,
    //err_log
}

impl<L: lexer> Parser<L> {
    fn parse(&mut self) -> Option<AST> {
        let gro_decl = ast::GlobalDecl { list: Vec::new() };
        loop {
            match self.tok {
                Token::Eof => break,
                _ => self.parse_global_declaration(),
            }
        }
        None
    }

    fn next(&mut self) {
        if let Ok(t) = self.lex.next() {
            self.tok = t
        } else {
            self.tok = Token::Eof;
        }
    }

    fn parse_global_declaration(&mut self) -> Option<ast::ValueSepc> {
        return self.parse_var_define();
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
        if let Ok(Token::Ident(s)) = self.parse_identifier() {
            list.push(ast::Ident { name: s });
        }
        if let Token::Aide(Aides::Comma) = self.tok {
            if let Some(mut i) = self.parse_variable_list() {
                list.append(&mut i);
            }
        }
        Some(list)
    }

    fn parse_identifier(&mut self) -> ParseResult {
        if let Token::Ident(s) = self.tok {
            self.next();
            return Ok(Token::Ident(s));
        }
        Err(ParseError::NoFoundIdent)
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
        let f = OpenOptions::new().read(true).open("./src/a.txt").unwrap();
        let mut lexer = DefaultLexer::new(f);
        loop {
            let m = lexer.lex();
            match m {
                Ok(c) => println!("{:?}", c),
                Err(e) => {
                    println!("{:?}", e);
                    break;
                }
            }
        }
    }
}
