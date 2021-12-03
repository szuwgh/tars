use crate::ast;
use crate::ast::AST;
use crate::lexer::Aides;
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
    //err_log
}

impl<L: lexer> Parser<L> {
    fn parse(&mut self) -> Option<AST> {
        None
    }

    fn parse_global_declaration() {}
    // variable_decl ::= type {'*'} id { ',' {'*'} id } ';'
    fn parse_var_define(&mut self) -> Option<ast::ValueSepc> {
        if let Ok(Token::KeyWord(t)) = self.parse_type() {
            if let Some(idents) = self.parse_variable_list() {
                Some(ast::ValueSepc {
                    names: idents,
                    typ: t,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_variable_list(&mut self) -> Option<Vec<ast::Ident>> {
        let mut list: Vec<ast::Ident> = Vec::new();
        if let Ok(Token::Ident(s)) = self.parse_identifier() {
            list.push(ast::Ident { name: s });
        }
        if let Ok(Token::Aide(Aides::Comma)) = self.lex.next() {
            if let Some(mut i) = self.parse_variable_list() {
                list.append(&mut i);
            }
        }
        Some(list)
    }

    fn parse_identifier(&mut self) -> ParseResult {
        if let Ok(Token::Ident(s)) = self.lex.next() {
            return Ok(Token::Ident(s));
        }
        Err(ParseError::NoFoundIdent)
    }

    fn parse_type(&mut self) -> ParseResult {
        if let Ok(Token::KeyWord(k)) = self.lex.next() {
            if k.is_type() {
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
