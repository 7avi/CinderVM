use crate::bytecode::{Instruction, Program};
use anyhow::Result;

/// Sandbox for validation and securing execution
pub struct Sandbox {
    program: Program,
    allowed_natives: Vec<u32>,
}

impl Sandbox {
    pub fn new(program: &Program) -> Self {
        // Whitelist of allowed native functions
        let allowed_natives = vec![
            0x01, // print_int
            0x02, // print_str
            // Add more functions as needed
        ];

        Self {
            program: program.clone(),
            allowed_natives,
        }
    }

    /// Validate program for security
    pub fn validate(&self) -> Result<()> {
        // Check jumps
        for (idx, instruction) in self.program.instructions.iter().enumerate() {
            match instruction {
                Instruction::Jump(target) |
                Instruction::JumpIfZero(target) |
                Instruction::JumpIfNotZero(target) => {
                    if *target >= self.program.instructions.len() {
                        return Err(anyhow::anyhow!(
                            "Invalid jump at instruction {}: target {} exceeds bounds",
                            idx,
                            target
                        ));
                    }
                }
                
                Instruction::Load(offset) | Instruction::Store(offset) => {
                    if *offset >= self.program.memory_size {
                        return Err(anyhow::anyhow!(
                            "Invalid memory access at instruction {}: offset {} exceeds allocated memory ({})",
                            idx,
                            offset,
                            self.program.memory_size
                        ));
                    }
                }
                
                Instruction::CallNative(id) => {
                    if !self.is_native_allowed(*id) {
                        return Err(anyhow::anyhow!(
                            "Disallowed native call at instruction {}: function {} is not in whitelist",
                            idx,
                            id
                        ));
                    }
                }
                
                _ => {}
            }
        }

        Ok(())
    }

    /// Check if a native function is allowed
    pub fn is_native_allowed(&self, id: u32) -> bool {
        self.allowed_natives.contains(&id)
    }

    /// Add a native function to whitelist
    pub fn allow_native(&mut self, id: u32) {
        if !self.allowed_natives.contains(&id) {
            self.allowed_natives.push(id);
        }
    }
}

