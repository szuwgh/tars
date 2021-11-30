use std::io::{Bytes, Read};
use std::iter::{Iterator, Peekable};

pub enum KeyWord {}

#[derive(Debug)]
pub enum Token {
    Unknown, //
    Oper(Operator),
}

#[derive(Debug)]
pub enum Operator {
    Plus,     //++
    Add,      //+
    AddEqual, //+=
    Sub,      //-
    SubEqual, //-=
    Minus,    //--
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

    fn parse_add(&mut self) -> Option<Token> {
        match self.peek() {
            Some(c) => match c {
                b'+' => {
                    self.take();
                    return Some(Token::Oper(Operator::Plus));
                }
                b'=' => {
                    self.take();
                    return Some(Token::AddEqual);
                }
                _ => Some(Token::Add),
            },
            None => Some(Token::Add),
        }
    }

    fn parse_sub(&mut self) -> Option<Token> {
        match self.peek() {
            Some(c) => match c {
                b'-' => {
                    self.take();
                    return Some(Token::Minus);
                }
                b'=' => {
                    self.take();
                    return Some(Token::SubEqual);
                }
                _ => Some(Token::Sub),
            },
            None => Some(Token::Sub),
        }
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

    pub fn parse(&mut self) -> Option<Token> {
        while let Some(c) = self.next() {
            match c {
                b'+' => return self.parse_add(),
                b' ' => self.skip_space(),
                b'-' => return self.parse_sub(),
                b'\n' | b'\r' | b'\t' => {
                    self.line += 1;
                }
                _ => return Some(Token::Unknown),
            };
        }
        return None;

        // Some(c) => {
        //     println!("{}", c as char);
        // }
        // None => {}

        // println!("{}", c as char);
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

mod tests {}
