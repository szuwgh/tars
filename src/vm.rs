use std::alloc::{self, Layout};
use std::mem;
use std::slice;

#[repr(u64)]
#[derive(Debug)]
enum Instruction {
    Lea = 1,
    Imm,
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
    Push,
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
    Add,
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
    Exit,
}
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
                Instruction::Imm => {
                    self.pc = self.pc.add(1);
                    self.ax = self.pc.read();
                }
                Instruction::Lea => {
                    println!("op : {:?}", op);
                }
                Instruction::Push => {
                    self.sp = self.sp.sub(1);
                    self.sp.write(self.ax);
                }
                Instruction::Add => {
                    self.ax = self.sp.read() + self.ax;
                    self.sp = self.sp.add(1);
                }
                Instruction::Exit => {
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

struct Pointer<'a, T> {
    data: *mut T,
    slice: &'a mut [T],
}

impl<'a, T> Pointer<'a, T> {
    unsafe fn new() -> Pointer<'a, T> {
        let data: *mut T = alloc::alloc(Layout::from_size_align_unchecked(
            4096 * mem::size_of::<T>(),
            mem::size_of::<T>(),
        )) as *mut T;
        Self {
            data: data,
            slice: slice::from_raw_parts_mut(data, 4096),
        }
    }
}
