/// Bytecode instruction definitions for CinderVM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    // Operands and stack
    PushInt = 0x01,
    PushReg = 0x02,
    Pop = 0x03,
    
    // Arithmetic operations
    Add = 0x10,
    Sub = 0x11,
    Mul = 0x12,
    Div = 0x13,
    
    // Logical operations
    Eq = 0x20,
    Lt = 0x21,
    Gt = 0x22,
    
    // Control flow
    Jump = 0x30,
    JumpIfZero = 0x31,
    JumpIfNotZero = 0x32,
    
    // Memory
    Load = 0x40,
    Store = 0x41,
    
    // Calls and return
    CallNative = 0x50,
    Return = 0x51,
    
    // Halt
    Halt = 0xFF,
}

impl OpCode {
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(OpCode::PushInt),
            0x02 => Some(OpCode::PushReg),
            0x03 => Some(OpCode::Pop),
            0x10 => Some(OpCode::Add),
            0x11 => Some(OpCode::Sub),
            0x12 => Some(OpCode::Mul),
            0x13 => Some(OpCode::Div),
            0x20 => Some(OpCode::Eq),
            0x21 => Some(OpCode::Lt),
            0x22 => Some(OpCode::Gt),
            0x30 => Some(OpCode::Jump),
            0x31 => Some(OpCode::JumpIfZero),
            0x32 => Some(OpCode::JumpIfNotZero),
            0x40 => Some(OpCode::Load),
            0x41 => Some(OpCode::Store),
            0x50 => Some(OpCode::CallNative),
            0x51 => Some(OpCode::Return),
            0xFF => Some(OpCode::Halt),
            _ => None,
        }
    }
}

/// Complete instruction representation
#[derive(Debug, Clone)]
pub enum Instruction {
    // Operands
    PushInt(i64),
    PushReg(u8),
    Pop,
    
    // Arithmetic (no operands, works on stack)
    Add,
    Sub,
    Mul,
    Div,
    
    // Logical
    Eq,
    Lt,
    Gt,
    
    // Control flow
    Jump(usize),
    JumpIfZero(usize),
    JumpIfNotZero(usize),
    
    // Memory
    Load(usize),  // memory offset
    Store(usize),
    
    // Calls
    CallNative(u32),  // native function ID
    Return,
    
    Halt,
}

/// Complete program representation
#[derive(Debug, Clone)]
pub struct Program {
    pub instructions: Vec<Instruction>,
    pub memory_size: usize,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>, memory_size: usize) -> Self {
        Self {
            instructions,
            memory_size,
        }
    }
}

