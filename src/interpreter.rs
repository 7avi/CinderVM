use crate::bytecode::{Instruction, Program};

/// Minimal interpreter for bytecode validation
pub struct Interpreter {
    stack: Vec<i64>,
    memory: Vec<i64>,
    pc: usize,  // Program Counter
    program: Program,
}

#[derive(Debug)]
pub enum InterpreterError {
    StackUnderflow,
    StackOverflow,
    InvalidMemoryAccess(usize),
    InvalidJumpTarget(usize),
    DivisionByZero,
}

impl Interpreter {
    pub fn new(program: Program) -> Self {
        let memory_size = program.memory_size.max(1024); // Minimum 1024 bytes
        Self {
            stack: Vec::new(),
            memory: vec![0; memory_size],
            pc: 0,
            program,
        }
    }

    pub fn execute(&mut self) -> Result<i64, InterpreterError> {
        while self.pc < self.program.instructions.len() {
            let instruction = &self.program.instructions[self.pc];
            
            match instruction {
                Instruction::PushInt(val) => {
                    self.stack.push(*val);
                    self.pc += 1;
                }
                
                Instruction::PushReg(_reg) => {
                    // For simplicity, ignore registers in interpreter
                    // In JIT we will use real registers
                    return Err(InterpreterError::StackUnderflow);
                }
                
                Instruction::Pop => {
                    self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    self.pc += 1;
                }
                
                Instruction::Add => {
                    let b = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    self.stack.push(a + b);
                    self.pc += 1;
                }
                
                Instruction::Sub => {
                    let b = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    self.stack.push(a - b);
                    self.pc += 1;
                }
                
                Instruction::Mul => {
                    let b = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    self.stack.push(a * b);
                    self.pc += 1;
                }
                
                Instruction::Div => {
                    let b = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    if b == 0 {
                        return Err(InterpreterError::DivisionByZero);
                    }
                    self.stack.push(a / b);
                    self.pc += 1;
                }
                
                Instruction::Eq => {
                    let b = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    self.stack.push(if a == b { 1 } else { 0 });
                    self.pc += 1;
                }
                
                Instruction::Lt => {
                    let b = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    self.stack.push(if a < b { 1 } else { 0 });
                    self.pc += 1;
                }
                
                Instruction::Gt => {
                    let b = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    self.stack.push(if a > b { 1 } else { 0 });
                    self.pc += 1;
                }
                
                Instruction::Jump(target) => {
                    if *target >= self.program.instructions.len() {
                        return Err(InterpreterError::InvalidJumpTarget(*target));
                    }
                    self.pc = *target;
                }
                
                Instruction::JumpIfZero(target) => {
                    let val = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    if val == 0 {
                        if *target >= self.program.instructions.len() {
                            return Err(InterpreterError::InvalidJumpTarget(*target));
                        }
                        self.pc = *target;
                    } else {
                        self.pc += 1;
                    }
                }
                
                Instruction::JumpIfNotZero(target) => {
                    let val = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    if val != 0 {
                        if *target >= self.program.instructions.len() {
                            return Err(InterpreterError::InvalidJumpTarget(*target));
                        }
                        self.pc = *target;
                    } else {
                        self.pc += 1;
                    }
                }
                
                Instruction::Load(offset) => {
                    if *offset >= self.memory.len() {
                        return Err(InterpreterError::InvalidMemoryAccess(*offset));
                    }
                    self.stack.push(self.memory[*offset]);
                    self.pc += 1;
                }
                
                Instruction::Store(offset) => {
                    if *offset >= self.memory.len() {
                        return Err(InterpreterError::InvalidMemoryAccess(*offset));
                    }
                    let val = self.stack.pop().ok_or(InterpreterError::StackUnderflow)?;
                    self.memory[*offset] = val;
                    self.pc += 1;
                }
                
                Instruction::CallNative(_id) => {
                    // In interpreter, ignore native calls
                    // In JIT we will implement the whitelist
                    self.pc += 1;
                }
                
                Instruction::Return => {
                    // Return value from stack or 0
                    return Ok(self.stack.pop().unwrap_or(0));
                }
                
                Instruction::Halt => {
                    return Ok(self.stack.pop().unwrap_or(0));
                }
            }
        }
        
        Ok(self.stack.pop().unwrap_or(0))
    }
}

