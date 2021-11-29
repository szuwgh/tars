use std::alloc::{self, Layout};
use std::env;
use std::fs::OpenOptions;
use std::io::{Bytes, Read};
use std::iter::{Iterator, Peekable};
use std::mem;
use std::slice;

enum Token {}

#[repr(u64)]
#[derive(Debug)]
enum Instruction {
    LEA = 1,
    IMM,
    // JMP,
    // CALL,
    // JZ,
    // JNZ,
    // ENT,
    // ADJ,
    // LEV,
    // LI,
    // LC,
    // SI,
    // SC,
    PUSH,
    // OR,
    // XOR,
    // AND,
    // EQ,
    // NE,
    // LT,
    // GT,
    // LE,
    // GE,
    // SHL,
    // SHR,
    ADD,
    // SUB,
    // MUL,
    // DIV,
    // MOD,
    // OPEN,
    // READ,
    // CLOS,
    // PRTF,
    // MALC,
    // MSET,
    // MCMP,
    EXIT,
}

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

fn open(pathname: *mut u8) {}

//虚拟机 模拟计算机
struct VM {
    pc: *mut u64, //程序计数器，它存放的是一个内存地址，该地址中存放着 下一条 要执行的计算机指令
    sp: *mut u64, //指针寄存器，永远指向当前的栈顶。注意的是由于栈是位于高地址并向低地址增长的，所以入栈时 SP 的值减小
    bp: *mut u64, //基址指针。也是用于指向栈的某些位置，在调用函数时会使用到它
    ax: u64,      //通用寄存器，我们的虚拟机中，它用于存放一条指令执行后的结果
    text: *mut u64, //代码段
    old_text: *mut u64, //
    stack: *mut u64, //用于处理函数调用相关的数据，如调用帧（calling frame）或是函数的局部变量等
    data: *mut u64, //数据段 用于存放初始化了的数据，如int i = 10;，就需要存放到数据段中
}

impl VM {
    unsafe fn eval(&mut self) {
        loop {
            self.pc = self.pc.add(1);
            let op = std::mem::transmute(self.pc.read());
            match op {
                Instruction::IMM => {
                    self.pc = self.pc.add(1);
                    self.ax = self.pc.read();
                }
                Instruction::LEA => {
                    println!("op : {:?}", op);
                }
                Instruction::PUSH => {
                    self.sp = self.sp.sub(1);
                    self.sp.write(self.ax);
                }
                Instruction::ADD => {
                    self.ax = self.sp.read() + self.ax;
                    self.sp = self.sp.add(1);
                }
                Instruction::EXIT => {
                    println!("exit({})", self.sp.read());
                }
                _ => {
                    break;
                }
            }
        }
    }
    fn programs(&mut self) {}
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let f = OpenOptions::new().read(true).open(&filename).unwrap();
    // let mut lexer = Lexer::new(f);
    // lexer.parse();
    // let size = 8192; // 256*1024/32
    unsafe {
        let text: *mut u64 = alloc::alloc(Layout::from_size_align_unchecked(
            4096 * mem::size_of::<u64>(),
            mem::size_of::<u64>(),
        )) as *mut u64;
        let stack: *mut u64 = alloc::alloc(Layout::from_size_align_unchecked(
            4096 * mem::size_of::<u64>(),
            mem::size_of::<u64>(),
        )) as *mut u64;
        let data: *mut u64 = alloc::alloc(Layout::from_size_align_unchecked(
            4096 * mem::size_of::<u64>(),
            mem::size_of::<u64>(),
        )) as *mut u64;
        let sp = stack.add(4096);
        let slice: &mut [u64] = slice::from_raw_parts_mut(text, 64);
        let mut i: usize = 1;
        slice[i] = Instruction::IMM as u64;
        i += 1;
        slice[i] = 10;
        i += 1;
        slice[i] = Instruction::PUSH as u64;
        i += 1;
        slice[i] = Instruction::IMM as u64;
        i += 1;
        slice[i] = 20;
        i += 1;
        slice[i] = Instruction::ADD as u64;
        i += 1;
        slice[i] = Instruction::PUSH as u64;
        i += 1;
        slice[i] = Instruction::EXIT as u64;
        //println!("0:{:?}", text);
        //  println!("1:{:?}", text.add(1));
        let mut vm = VM {
            pc: text,
            sp: sp,
            bp: sp,
            ax: 0,
            text: text,
            old_text: text,
            stack: stack,
            data: data,
        };
        vm.eval();
        println!("ax : {}", vm.ax);
    };

    // let vm = VM { pc: &mut text *mut u64 };

    //  println!("{}", filename);
}
