use std::io::{Bytes, Read};
use std::iter::{Iterator, Peekable};

type TokenResult = Result<Token, LexerError>;
#[derive(Debug)]
pub enum LexerError {
    Eof,
    Unterminated(String),
    UnExpected,
}

pub enum KeyWord {
    Int, // int
    Fn,  // fn
}

#[derive(Debug)]
pub enum Token {
    Unknown, //
    Oper(Operator),
    Aide(Aides),
    Ident(String),
}

#[derive(Debug)]
pub enum Aides {
    Dot,       // .
    Comma,     // ,
    Semicolon, // ;
    Colon,     // :
    Note,      // //
    MultNote,  // /**/
}

#[derive(Debug)]
pub enum Operator {
    Plus,          // ++
    Add,           // +
    AddEqual,      // +=
    Sub,           // -
    SubEqual,      // -=
    Minus,         // --
    Assign,        // =
    Star,          // *
    Div,           // /
    Mod,           // %
    BitAnd,        // &
    BitOr,         // |
    BitNot,        // ~
    BitShiftRight, // >>
    BitShiftLeft,  // <<
    LogicAnd,      // &&
    LogicOr,       // ||
    LogicNot,      // !
    Equal,         // ==
    NotEqual,      // !=
    Greate,        // >
    GreateEqual,   // >=
    Less,          // <
    LessEqual,     // <=
    Question,      // ?
}

pub struct Lexer<R: Read> {
    peeker: Peekable<Bytes<R>>,
    line: u32,
}

impl<R: Read> Lexer<R> {
    pub fn new(r: R) -> Lexer<R> {
        Lexer {
            peeker: r.bytes().peekable(),
            line: 0,
        }
    }

    pub fn parse(&mut self) -> TokenResult {
        while let Some(c) = self.next() {
            match c {
                b' ' => self.skip_space(),
                b'.' => return self.take_token(Token::Aide(Aides::Dot)),
                b',' => return self.take_token(Token::Aide(Aides::Comma)),
                b';' => return self.take_token(Token::Aide(Aides::Semicolon)),
                b':' => return self.take_token(Token::Aide(Aides::Colon)),
                b'+' => return self.parse_add(),
                b'-' => return self.parse_sub(),
                b'=' => return self.parse_equal(),
                b'*' => return self.take_token(Token::Oper(Operator::Star)),
                b'/' => return self.parse_div(),
                b'%' => return self.take_token(Token::Oper(Operator::Mod)),
                b'&' => return self.take_token(Token::Oper(Operator::BitAnd)),
                b'|' => return self.take_token(Token::Oper(Operator::BitOr)),
                b'|' => return self.take_token(Token::Oper(Operator::BitOr)),
                b'\n' | b'\r' | b'\t' => {
                    self.skip_line();
                }
                _ => return Ok(Token::Unknown),
            };
        }
        Err(LexerError::Eof)
    }
    fn skip_space(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                b' ' => {
                    self.take();
                    continue;
                }
                _ => return,
            }
        }
    }

    fn parse_add(&mut self) -> TokenResult {
        match self.peek() {
            Some(c) => match c {
                b'+' => {
                    self.take();
                    return Ok(Token::Oper(Operator::Plus));
                }
                b'=' => {
                    self.take();
                    return Ok(Token::Oper(Operator::AddEqual));
                }
                _ => Ok(Token::Oper(Operator::Add)),
            },
            None => Ok(Token::Oper(Operator::Add)),
        }
    }

    fn parse_sub(&mut self) -> TokenResult {
        match self.peek() {
            Some(c) => match c {
                b'-' => {
                    self.take();
                    return Ok(Token::Oper(Operator::Minus));
                }
                b'=' => {
                    self.take();
                    return Ok(Token::Oper(Operator::SubEqual));
                }
                _ => Ok(Token::Oper(Operator::Sub)),
            },
            None => Ok(Token::Oper(Operator::Sub)),
        }
    }

    fn parse_equal(&mut self) -> TokenResult {
        match self.peek() {
            Some(b'=') => self.take_token(Token::Oper(Operator::Equal)),
            _ => Ok(Token::Oper(Operator::Assign)),
        }
    }

    fn parse_div(&mut self) -> TokenResult {
        match self.peek() {
            Some(c) => match c {
                b'/' => self.parse_note(),
                b'*' => self.parse_multnote(),
                _ => Ok(Token::Oper(Operator::Div)),
            },
            None => Err(LexerError::UnExpected),
        }
    }

    fn parse_note(&mut self) -> TokenResult {
        self.take();
        while let Some(b'\n') = self.next() {
            break;
        }
        Ok(Token::Aide(Aides::Note))
    }

    fn parse_multnote(&mut self) -> TokenResult {
        self.take();
        while let Some(c) = self.next() {
            match c {
                b'*' => {
                    if let Some(b'/') = self.peek() {
                        self.take();
                        return Ok(Token::Aide(Aides::MultNote));
                    }
                    return Err(LexerError::Unterminated(
                        "found '/*', no '*/' end ".to_owned(),
                    ));
                }
                _ => continue,
            }
        }
        Err(LexerError::Unterminated(
            "found '/*', no '*/' end ".to_owned(),
        ))
    }

    fn skip_line(&mut self) {
        self.line += 1;
        while let Some(c) = self.peek() {
            match c {
                b'\n' | b'\r' | b'\t' => {
                    self.line += 1;
                    self.take();
                    continue;
                }
                _ => return,
            }
        }
    }

    fn take_token(&mut self, t: Token) -> TokenResult {
        self.take();
        Ok(t)
    }

    fn take(&mut self) {
        let _ = self.peeker.next();
    }

    //获取下一个字符
    fn next(&mut self) -> Option<u8> {
        match self.peeker.next() {
            Some(Ok(ch)) => Some(ch),
            _ => None,
        }
    }

    //向前偷看一个字符
    fn peek(&mut self) -> Option<u8> {
        match self.peeker.peek() {
            Some(&Ok(ch)) => Some(ch),
            _ => None,
        }
    }
}

mod tests {
    use super::*;
    use std::fs::OpenOptions;
    #[test]
    fn test_lexer() {
        let f = OpenOptions::new().read(true).open("./src/a.txt").unwrap();
        let mut lexer = Lexer::new(f);
        while let m = lexer.parse() {
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
