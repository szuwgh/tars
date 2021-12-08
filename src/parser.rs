use crate::ast;
use crate::ast::AST;
use crate::lexer::lexer;
use crate::lexer::Aides;
use crate::lexer::DefaultLexer;
use crate::lexer::KeyWord;
use crate::lexer::LexResult;
use crate::lexer::Token;
pub type ParseResult<T> = Result<T, ParseError>;

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
    fn parse(&mut self) -> ParseResult<AST> {
        self.next();
        let mut gro_decl = ast::GlobalDecl { list: Vec::new() };
        loop {
            match self.tok {
                Token::Eof => break,
                Token::KeyWord(KeyWord::Var) => {
                    println!("parse var");
                    if let Ok(value_spec) = self.parse_global_declaration() {
                        println!("{:?}", value_spec);
                        gro_decl.list.push(value_spec);
                    }
                }
                Token::KeyWord(KeyWord::Fn) => {
                    println!("parse fn");
                    if let Ok(value_spec) = self.parse_global_declaration() {
                        println!("{:?}", value_spec);
                        gro_decl.list.push(value_spec);
                    }
                }
                _ => {}
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

    fn parse_function_define(&mut self) {
        self.parse_type().and_then(|t| {
            self.parse_identifier()
                .and_then(|s| Ok(ast::FuncDecl { typ: t, name: s }))
        });
    }

    // variable_decl ::= type {'*'} id { ',' {'*'} id } ';'
    fn parse_var_define(&mut self) -> ParseResult<ast::ValueSepc> {
        if let Ok(t) = self.parse_type() {
            if let Ok(idents) = self.parse_variable_list() {
                if self.expect_token(Token::Aide(Aides::Semicolon)) {
                    return Ok(ast::ValueSepc {
                        names: idents,
                        typ: t,
                    });
                }
            }
        }
        Err(ParseError::Eof)
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
        if let Ok(s) = self.parse_identifier() {
            list.push(ast::Ident { name: s });
        }
        if self.expect_token(Token::Aide(Aides::Comma)) {
            if let Ok(mut i) = self.parse_variable_list() {
                list.append(&mut i);
            }
        }
        Ok(list)
    }

    fn parse_identifier(&mut self) -> ParseResult<String> {
        if let Token::Ident(s) = self.tok.clone() {
            self.next();
            return Ok(s);
        }
        None+
    }

    fn parse_type(&mut self) -> ParseResult<KeyWord> {
        if let Token::KeyWord(k) = self.tok {
            if k.is_type() {
                self.next();
                return Ok(k);
            }
        }
        Err(ParseError::Eof)
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
