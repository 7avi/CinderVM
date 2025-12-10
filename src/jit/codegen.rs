use crate::bytecode::{Instruction, Program};
use crate::jit::memory::{ExecutableMemory, MemoryError};
use crate::sandbox::Sandbox;
use anyhow::{Context, Result};

/// JIT compiler for x86-64 machine code generation
pub struct JitCompiler {
    program: Program,
    sandbox: Sandbox,
}

impl JitCompiler {
    pub fn new(program: Program) -> Self {
        Self {
            sandbox: Sandbox::new(&program),
            program,
        }
    }

    /// Compile program to machine code and return executable memory
    pub fn compile(&mut self) -> Result<ExecutableMemory> {
        // Validate program before compilation
        self.sandbox.validate()?;

        // Estimate required code size
        let estimated_size = self.estimate_code_size();
        let mut memory = ExecutableMemory::allocate(estimated_size)
            .context("Cannot allocate executable memory")?;

        // Generate machine code
        let mut offset = 0;
        offset = unsafe { self.emit_prologue(&mut memory, offset)? };
        
        for (idx, instruction) in self.program.instructions.iter().enumerate() {
            offset = self.emit_instruction(&mut memory, offset, instruction, idx)?;
        }
        
        offset = unsafe { self.emit_epilogue(&mut memory, offset)? };

        Ok(memory)
    }

    /// Estimate generated code size
    fn estimate_code_size(&self) -> usize {
        // Conservative estimate: ~20 bytes per instruction
        self.program.instructions.len() * 20 + 100
    }

    /// Emit function prologue (stack setup, etc.)
    unsafe fn emit_prologue(&self, memory: &mut ExecutableMemory, offset: usize) -> Result<usize> {
        let mut code = Vec::new();
        
        // push rbp
        code.push(0x55);
        // mov rbp, rsp
        code.extend_from_slice(&[0x48, 0x89, 0xE5]);
        
        // Allocate space for local stack (16 bytes for alignment)
        // sub rsp, 16
        code.extend_from_slice(&[0x48, 0x83, 0xEC, 0x10]);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    /// Emit function epilogue (cleanup, return)
    unsafe fn emit_epilogue(&self, memory: &mut ExecutableMemory, offset: usize) -> Result<usize> {
        let mut code = Vec::new();
        
        // Return value is in RAX (already set by instructions)
        // mov rsp, rbp
        code.extend_from_slice(&[0x48, 0x89, 0xEC]);
        // pop rbp
        code.push(0x5D);
        // ret
        code.push(0xC3);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    /// Emit code for an instruction
    fn emit_instruction(
        &self,
        memory: &mut ExecutableMemory,
        offset: usize,
        instruction: &Instruction,
        pc: usize,
    ) -> Result<usize> {
        unsafe {
            match instruction {
                Instruction::PushInt(val) => self.emit_push_int(memory, offset, *val),
                
                Instruction::Add => self.emit_add(memory, offset),
                Instruction::Sub => self.emit_sub(memory, offset),
                Instruction::Mul => self.emit_mul(memory, offset),
                Instruction::Div => self.emit_div(memory, offset),
                
                Instruction::Eq => self.emit_eq(memory, offset),
                Instruction::Lt => self.emit_lt(memory, offset),
                Instruction::Gt => self.emit_gt(memory, offset),
                
                Instruction::Jump(target) => {
                    self.emit_jump(memory, offset, *target, pc)
                }
                
                Instruction::JumpIfZero(target) => {
                    self.emit_jump_if_zero(memory, offset, *target, pc)
                }
                
                Instruction::JumpIfNotZero(target) => {
                    self.emit_jump_if_not_zero(memory, offset, *target, pc)
                }
                
                Instruction::Load(mem_offset) => {
                    self.emit_load(memory, offset, *mem_offset)
                }
                
                Instruction::Store(mem_offset) => {
                    self.emit_store(memory, offset, *mem_offset)
                }
                
                Instruction::CallNative(id) => {
                    self.emit_call_native(memory, offset, *id)
                }
                
                Instruction::Return => self.emit_return(memory, offset),
                Instruction::Halt => self.emit_halt(memory, offset),
                
                _ => Ok(offset), // Unimplemented instructions yet
            }
        }
    }

    // Implementations for each instruction type
    unsafe fn emit_push_int(&self, memory: &mut ExecutableMemory, offset: usize, val: i64) -> Result<usize> {
        let mut code = Vec::new();
        
        // push val (8 bytes)
        // mov rax, val
        code.extend_from_slice(&[0x48, 0xB8]);
        code.extend_from_slice(&val.to_le_bytes());
        // push rax
        code.push(0x50);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_add(&self, memory: &mut ExecutableMemory, offset: usize) -> Result<usize> {
        let mut code = Vec::new();
        
        // pop rbx (second operand)
        code.extend_from_slice(&[0x5B]);
        // pop rax (first operand)
        code.extend_from_slice(&[0x58]);
        // add rax, rbx
        code.extend_from_slice(&[0x48, 0x01, 0xD8]);
        // push rax (result)
        code.push(0x50);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_sub(&self, memory: &mut ExecutableMemory, offset: usize) -> Result<usize> {
        let mut code = Vec::new();
        
        // pop rbx
        code.extend_from_slice(&[0x5B]);
        // pop rax
        code.extend_from_slice(&[0x58]);
        // sub rax, rbx
        code.extend_from_slice(&[0x48, 0x29, 0xD8]);
        // push rax
        code.push(0x50);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_mul(&self, memory: &mut ExecutableMemory, offset: usize) -> Result<usize> {
        let mut code = Vec::new();
        
        // pop rbx
        code.extend_from_slice(&[0x5B]);
        // pop rax
        code.extend_from_slice(&[0x58]);
        // imul rax, rbx
        code.extend_from_slice(&[0x48, 0x0F, 0xAF, 0xC3]);
        // push rax
        code.push(0x50);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_div(&self, memory: &mut ExecutableMemory, offset: usize) -> Result<usize> {
        let mut code = Vec::new();
        
        // pop rbx (divisor)
        code.extend_from_slice(&[0x5B]);
        // pop rax (dividend)
        code.extend_from_slice(&[0x58]);
        // cdq (extend rax to rdx:rax for signed division)
        code.extend_from_slice(&[0x48, 0x99]);
        // idiv rbx
        code.extend_from_slice(&[0x48, 0xF7, 0xFB]);
        // push rax (quotient)
        code.push(0x50);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_eq(&self, memory: &mut ExecutableMemory, offset: usize) -> Result<usize> {
        let mut code = Vec::new();
        
        // pop rbx
        code.extend_from_slice(&[0x5B]);
        // pop rax
        code.extend_from_slice(&[0x58]);
        // cmp rax, rbx
        code.extend_from_slice(&[0x48, 0x39, 0xD8]);
        // sete al
        code.extend_from_slice(&[0x0F, 0x94, 0xC0]);
        // movzx rax, al
        code.extend_from_slice(&[0x48, 0x0F, 0xB6, 0xC0]);
        // push rax
        code.push(0x50);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_lt(&self, memory: &mut ExecutableMemory, offset: usize) -> Result<usize> {
        let mut code = Vec::new();
        
        // pop rbx
        code.extend_from_slice(&[0x5B]);
        // pop rax
        code.extend_from_slice(&[0x58]);
        // cmp rax, rbx
        code.extend_from_slice(&[0x48, 0x39, 0xD8]);
        // setl al
        code.extend_from_slice(&[0x0F, 0x9C, 0xC0]);
        // movzx rax, al
        code.extend_from_slice(&[0x48, 0x0F, 0xB6, 0xC0]);
        // push rax
        code.push(0x50);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_gt(&self, memory: &mut ExecutableMemory, offset: usize) -> Result<usize> {
        let mut code = Vec::new();
        
        // pop rbx
        code.extend_from_slice(&[0x5B]);
        // pop rax
        code.extend_from_slice(&[0x58]);
        // cmp rax, rbx
        code.extend_from_slice(&[0x48, 0x39, 0xD8]);
        // setg al
        code.extend_from_slice(&[0x0F, 0x9F, 0xC0]);
        // movzx rax, al
        code.extend_from_slice(&[0x48, 0x0F, 0xB6, 0xC0]);
        // push rax
        code.push(0x50);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_jump(
        &self,
        memory: &mut ExecutableMemory,
        offset: usize,
        target: usize,
        current_pc: usize,
    ) -> Result<usize> {
        // For simplicity, use relative jump
        // In complete implementation, we should calculate correct offset
        let mut code = Vec::new();
        
        // jmp [relative offset]
        code.push(0xE9);
        // Placeholder for offset (will be calculated in two passes)
        code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        memory.write(offset, &code)?;
        // Note: In complete implementation, we should do two passes
        // to calculate correct offsets
        Ok(offset + code.len())
    }

    unsafe fn emit_jump_if_zero(
        &self,
        memory: &mut ExecutableMemory,
        offset: usize,
        target: usize,
        current_pc: usize,
    ) -> Result<usize> {
        let mut code = Vec::new();
        
        // pop rax
        code.extend_from_slice(&[0x58]);
        // test rax, rax
        code.extend_from_slice(&[0x48, 0x85, 0xC0]);
        // jz [offset]
        code.extend_from_slice(&[0x0F, 0x84]);
        code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_jump_if_not_zero(
        &self,
        memory: &mut ExecutableMemory,
        offset: usize,
        target: usize,
        current_pc: usize,
    ) -> Result<usize> {
        let mut code = Vec::new();
        
        // pop rax
        code.extend_from_slice(&[0x58]);
        // test rax, rax
        code.extend_from_slice(&[0x48, 0x85, 0xC0]);
        // jnz [offset]
        code.extend_from_slice(&[0x0F, 0x85]);
        code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_load(
        &self,
        memory: &mut ExecutableMemory,
        offset: usize,
        mem_offset: usize,
    ) -> Result<usize> {
        // Verify offset is within safe bounds
        if mem_offset >= self.program.memory_size {
            return Err(anyhow::anyhow!("Invalid memory access: offset {}", mem_offset));
        }

        let mut code = Vec::new();
        
        // mov rax, [rbp - offset] (use local stack as memory)
        // For simplicity, use a fixed memory area
        // In complete implementation, we should allocate separate memory
        code.extend_from_slice(&[0x48, 0x8B, 0x85]);
        code.extend_from_slice(&(mem_offset as i32).to_le_bytes());
        // push rax
        code.push(0x50);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_store(
        &self,
        memory: &mut ExecutableMemory,
        offset: usize,
        mem_offset: usize,
    ) -> Result<usize> {
        if mem_offset >= self.program.memory_size {
            return Err(anyhow::anyhow!("Invalid memory access: offset {}", mem_offset));
        }

        let mut code = Vec::new();
        
        // pop rax
        code.extend_from_slice(&[0x58]);
        // mov [rbp - offset], rax
        code.extend_from_slice(&[0x48, 0x89, 0x85]);
        code.extend_from_slice(&(mem_offset as i32).to_le_bytes());
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_call_native(
        &self,
        memory: &mut ExecutableMemory,
        offset: usize,
        id: u32,
    ) -> Result<usize> {
        // Verify function is in whitelist
        if !self.sandbox.is_native_allowed(id) {
            return Err(anyhow::anyhow!("Disallowed native call: {}", id));
        }

        // Placeholder - in complete implementation, we should have
        // a native function table
        let mut code = Vec::new();
        
        // call [function]
        code.push(0xE8);
        code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_return(&self, memory: &mut ExecutableMemory, offset: usize) -> Result<usize> {
        let mut code = Vec::new();
        
        // pop rax (return value)
        code.extend_from_slice(&[0x58]);
        // mov rsp, rbp
        code.extend_from_slice(&[0x48, 0x89, 0xEC]);
        // pop rbp
        code.push(0x5D);
        // ret
        code.push(0xC3);
        
        memory.write(offset, &code)?;
        Ok(offset + code.len())
    }

    unsafe fn emit_halt(&self, memory: &mut ExecutableMemory, offset: usize) -> Result<usize> {
        // Halt is similar to return
        self.emit_return(memory, offset)
    }
}

