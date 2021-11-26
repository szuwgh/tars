use std::env;
use std::fs::OpenOptions;
use std::io::{Bytes, Read};
use std::iter::{Iterator, Peekable};

enum Token {}

struct Lexer<R: Read> {
    peeker: Peekable<Bytes<R>>,
}

impl<R: Read> Lexer<R> {
    fn new(r: R) -> Lexer<R> {
        Lexer {
            peeker: r.bytes().peekable(),
        }
    }

    fn parse(&mut self) {
        while let Some(c) = self.next() {
            println!("{}", c as char);
        }
    }

    fn next(&mut self) -> Option<u8> {
        match self.peeker.next() {
            Some(Ok(ch)) => Some(ch),
            _ => None,
        }
    }

    fn peek(&mut self) -> Option<u8> {
        match self.peeker.peek() {
            Some(&Ok(ch)) => Some(ch),
            _ => None,
        }
    }
}

struct Parser {}

impl Parser {
    fn expression(&mut self) {}
}

struct VM {
    pc: u32, //程序计数器，它存放的是一个内存地址，该地址中存放着 下一条 要执行的计算机指令
    text: Vec<u32>,
    old_text: Vec<u32>,
    stack: Vec<u32>,
    data: Vec<u8>,
}

impl VM {
    fn programs(&mut self) {}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = OpenOptions::new().read(true).open(&filename).unwrap();
    let mut lexer = Lexer::new(f);
    lexer.parse();
    let size = 8192; // 256*1024/32
    println!("{}", filename);
}
