mod ast;
mod lexer;
mod parser;
mod vm;
use lexer::DefaultLexer;
use std::env;
use std::fs::OpenOptions;
extern crate llvm_sys as llvm;

fn open(pathname: *mut u8) {}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let filename = &args[1];
    // let f = OpenOptions::new().read(true).open(&filename).unwrap();
    // let mut lexer = DefaultLexer::new(f);

    // while let Ok(c) = lexer.lex() {
    //     println!("{:?}", c);
    // }
    //  let size = 8192; // 256*1024/32
    // unsafe {
    //     let text: *mut u64 = alloc::alloc(Layout::from_size_align_unchecked(
    //         4096 * mem::size_of::<u64>(),
    //         mem::size_of::<u64>(),
    //     )) as *mut u64;
    //     let stack: *mut u64 = alloc::alloc(Layout::from_size_align_unchecked(
    //         4096 * mem::size_of::<u64>(),
    //         mem::size_of::<u64>(),
    //     )) as *mut u64;
    //     let data: *mut u64 = alloc::alloc(Layout::from_size_align_unchecked(
    //         4096 * mem::size_of::<u64>(),
    //         mem::size_of::<u64>(),
    //     )) as *mut u64;
    //     let sp = stack.add(4096);
    //     let slice: &mut [u64] = slice::from_raw_parts_mut(text, 64);
    //     let mut i: usize = 1;
    //     slice[i] = Instruction::IMM as u64;
    //     i += 1;
    //     slice[i] = 10;
    //     i += 1;
    //     slice[i] = Instruction::PUSH as u64;
    //     i += 1;
    //     slice[i] = Instruction::IMM as u64;
    //     i += 1;
    //     slice[i] = 20;
    //     i += 1;
    //     slice[i] = Instruction::ADD as u64;
    //     i += 1;
    //     slice[i] = Instruction::PUSH as u64;
    //     i += 1;
    //     slice[i] = Instruction::EXIT as u64;
    //     let mut vm = VM {
    //         pc: text,
    //         sp: sp,
    //         bp: sp,
    //         ax: 0,
    //         text: text,
    //         old_text: text,
    //         stack: stack,
    //         data: data,
    //     };
    //     vm.eval();
    //     println!("ax : {}", vm.ax);
    // };
}

mod tests {}
