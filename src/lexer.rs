use std::io::{Bytes, Read};
use std::iter::{Iterator, Peekable};

pub type LexResult = Result<Token, LexerError>;

static KEY_WORD: &'static [(&'static str, KeyWord)] = &[
    ("int", KeyWord::Int),
    ("fn", KeyWord::Fn),
    ("return", KeyWord::Return),
];

fn is_keyword(s: &str) -> Option<KeyWord> {
    if let Some((_, k)) = KEY_WORD.iter().find(|(_s, _)| *_s == s) {
        Some(*k)
    } else {
        None
    }
}

#[derive(Debug)]
pub enum LexerError {
    Eof,
    Unterminated(String),
    UnExpected,
}

#[derive(Debug, Clone, Copy)]
pub enum KeyWord {
    Int,   // int
    Float, // float
    Fn,    // fn
    Return,
}

impl KeyWord {
    pub fn is_type(&self) -> bool {
        match self {
            KeyWord::Int => true,
            KeyWord::Float => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Unknown, //
    KeyWord(KeyWord),
    Oper(Operator),
    Aide(Aides),
    Str(String),
    Ident(String),
    Number(isize),
    Eof,
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
    RightArrow,    // ->
    LeftArrow,     // <-
    Question,      // ?
}

pub struct DefaultLexer<R: Read> {
    peeker: Peekable<Bytes<R>>,
    line: u32,
}

impl<R: Read> DefaultLexer<R> {
    pub fn new(r: R) -> DefaultLexer<R> {
        DefaultLexer {
            peeker: r.bytes().peekable(),
            line: 0,
        }
    }

    pub fn lex(&mut self) -> LexResult {
        while let Some(c) = self.next() {
            match c {
                b'\n' | b'\r' | b'\t' => self.skip_line(),
                b' ' => self.skip_space(),
                b'.' => return Ok(Token::Aide(Aides::Dot)),
                b',' => return Ok(Token::Aide(Aides::Comma)),
                b';' => return Ok(Token::Aide(Aides::Semicolon)),
                b':' => return Ok(Token::Aide(Aides::Colon)),
                b'+' => return self.parse_add(),
                b'-' => return self.parse_sub(),
                b'=' => return self.parse_equal(),
                b'"' => return self.parse_string(),
                b'*' => return Ok(Token::Oper(Operator::Star)),
                b'/' => return self.parse_div(),
                b'%' => return Ok(Token::Oper(Operator::Mod)),
                b'&' => return self.parse_and(),
                b'|' => return self.parse_or(),
                b'~' => return Ok(Token::Oper(Operator::BitNot)),
                b'>' => return self.parse_greate(),
                b'<' => return self.parse_less(),
                b'!' => return self.parse_excl(),
                b'?' => return Ok(Token::Oper(Operator::Question)),
                _ => {
                    if c.is_ascii_alphabetic() || c == b'_' {
                        return self.parse_varorkeyword(c);
                    } else if c.is_ascii_digit() {
                        return self.parse_num(c);
                    } else {
                        return Ok(Token::Unknown);
                    }
                }
            };
        }
        Ok(Token::Eof)
        //Err(LexerError::Eof)
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

    fn parse_add(&mut self) -> LexResult {
        match self.peek() {
            Some(c) => match c {
                b'+' => self.take_token(Token::Oper(Operator::Plus)),
                b'=' => self.take_token(Token::Oper(Operator::AddEqual)),
                _ => Ok(Token::Oper(Operator::Add)),
            },
            None => Ok(Token::Oper(Operator::Add)),
        }
    }

    fn parse_sub(&mut self) -> LexResult {
        match self.peek() {
            Some(c) => match c {
                b'-' => self.take_token(Token::Oper(Operator::Minus)),
                b'=' => self.take_token(Token::Oper(Operator::SubEqual)),
                b'>' => self.take_token(Token::Oper(Operator::RightArrow)),
                _ => Ok(Token::Oper(Operator::Sub)),
            },
            None => Ok(Token::Oper(Operator::Sub)),
        }
    }

    fn parse_equal(&mut self) -> LexResult {
        match self.peek() {
            Some(b'=') => self.take_token(Token::Oper(Operator::Equal)),
            _ => Ok(Token::Oper(Operator::Assign)),
        }
    }

    fn parse_div(&mut self) -> LexResult {
        match self.peek() {
            Some(c) => match c {
                b'/' => self.parse_note(),
                b'*' => self.parse_multnote(),
                _ => Ok(Token::Oper(Operator::Div)),
            },
            None => Err(LexerError::UnExpected),
        }
    }

    fn parse_and(&mut self) -> LexResult {
        if let Some(b'&') = self.peek() {
            self.take_token(Token::Oper(Operator::LogicAnd))
        } else {
            Ok(Token::Oper(Operator::BitAnd))
        }
    }

    fn parse_or(&mut self) -> LexResult {
        if let Some(b'|') = self.peek() {
            self.take_token(Token::Oper(Operator::LogicOr))
        } else {
            Ok(Token::Oper(Operator::BitOr))
        }
    }

    fn parse_note(&mut self) -> LexResult {
        self.take();
        while let Some(b'\n') = self.next() {
            break;
        }
        Ok(Token::Aide(Aides::Note))
    }

    fn parse_multnote(&mut self) -> LexResult {
        self.take();
        while let Some(c) = self.next() {
            match c {
                b'*' => {
                    if let Some(b'/') = self.peek() {
                        return self.take_token(Token::Aide(Aides::MultNote));
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

    fn parse_greate(&mut self) -> LexResult {
        match self.peek() {
            Some(c) => match c {
                b'>' => self.take_token(Token::Oper(Operator::BitShiftRight)),
                b'=' => self.take_token(Token::Oper(Operator::GreateEqual)),
                _ => self.take_token(Token::Oper(Operator::Greate)),
            },
            None => self.take_token(Token::Oper(Operator::Greate)),
        }
    }

    fn parse_less(&mut self) -> LexResult {
        match self.peek() {
            Some(c) => match c {
                b'<' => self.take_token(Token::Oper(Operator::BitShiftLeft)),
                b'=' => self.take_token(Token::Oper(Operator::LessEqual)),
                b'-' => self.take_token(Token::Oper(Operator::LeftArrow)),
                _ => self.take_token(Token::Oper(Operator::Less)),
            },
            None => self.take_token(Token::Oper(Operator::Less)),
        }
    }

    fn parse_excl(&mut self) -> LexResult {
        if let Some(b'=') = self.peek() {
            self.take_token(Token::Oper(Operator::NotEqual))
        } else {
            Ok(Token::Oper(Operator::LogicNot))
        }
    }

    fn parse_string(&mut self) -> LexResult {
        let mut s = String::new();
        while let Some(c) = self.next() {
            match c {
                b'"' => return Ok(Token::Str(s)),
                _ => s.push(c as char),
            }
        }
        Err(LexerError::Unterminated(
            "found '\"', no '\"' end ".to_owned(),
        ))
    }

    fn parse_varorkeyword(&mut self, c: u8) -> LexResult {
        let mut s = String::new();
        s.push(c as char);
        while let Some(c) = self.peek() {
            if c.is_ascii_alphabetic() {
                s.push(c as char);
                self.take();
            } else {
                break;
            }
        }
        if let Some(k) = is_keyword(s.as_str()) {
            Ok(Token::KeyWord(k))
        } else {
            Ok(Token::Ident(s))
        }
    }

    fn parse_num(&mut self, c: u8) -> LexResult {
        let mut s = String::new();
        s.push(c as char);
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c as char);
                self.take();
            } else {
                break;
            }
        }
        Ok(Token::Number(s.parse::<isize>().unwrap()))
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

    fn take_token(&mut self, t: Token) -> LexResult {
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
