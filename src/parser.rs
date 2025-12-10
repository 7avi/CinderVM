use crate::bytecode::{Instruction, OpCode, Program};
use anyhow::{Context, Result};
use std::fs;

/// Parser for .cinder files
pub struct Parser;

impl Parser {
    /// Parse a .cinder file and return a Program
    pub fn parse_file(path: &str) -> Result<Program> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Cannot read file: {}", path))?;
        
        Self::parse(&content)
    }

    /// Parse the content of a .cinder file
    pub fn parse(content: &str) -> Result<Program> {
        let mut instructions = Vec::new();
        let mut memory_size = 1024; // Default
        
        for line in content.lines() {
            let line = line.trim();
            
            // Ignore comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Parse special directives
            if line.starts_with(".memory") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 2 {
                    memory_size = parts[1]
                        .parse()
                        .context("Invalid memory size")?;
                }
                continue;
            }
            
            // Parse instructions
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            
            let opcode_str = parts[0].to_uppercase();
            let instruction = match opcode_str.as_str() {
                "PUSH_INT" => {
                    let val = parts.get(1)
                        .ok_or_else(|| anyhow::anyhow!("PUSH_INT requires value"))?
                        .parse()
                        .context("Invalid value for PUSH_INT")?;
                    Instruction::PushInt(val)
                }
                
                "PUSH_REG" => {
                    let reg = parts.get(1)
                        .ok_or_else(|| anyhow::anyhow!("PUSH_REG requires register number"))?
                        .parse()
                        .context("Invalid register")?;
                    Instruction::PushReg(reg)
                }
                
                "POP" => Instruction::Pop,
                "ADD" => Instruction::Add,
                "SUB" => Instruction::Sub,
                "MUL" => Instruction::Mul,
                "DIV" => Instruction::Div,
                "EQ" => Instruction::Eq,
                "LT" => Instruction::Lt,
                "GT" => Instruction::Gt,
                
                "JUMP" => {
                    let target = parts.get(1)
                        .ok_or_else(|| anyhow::anyhow!("JUMP requires target"))?
                        .parse()
                        .context("Invalid target for JUMP")?;
                    Instruction::Jump(target)
                }
                
                "JUMP_IF_ZERO" => {
                    let target = parts.get(1)
                        .ok_or_else(|| anyhow::anyhow!("JUMP_IF_ZERO requires target"))?
                        .parse()
                        .context("Invalid target for JUMP_IF_ZERO")?;
                    Instruction::JumpIfZero(target)
                }
                
                "JUMP_IF_NOT_ZERO" => {
                    let target = parts.get(1)
                        .ok_or_else(|| anyhow::anyhow!("JUMP_IF_NOT_ZERO requires target"))?
                        .parse()
                        .context("Invalid target for JUMP_IF_NOT_ZERO")?;
                    Instruction::JumpIfNotZero(target)
                }
                
                "LOAD" => {
                    let offset = parts.get(1)
                        .ok_or_else(|| anyhow::anyhow!("LOAD requires offset"))?
                        .parse()
                        .context("Invalid offset for LOAD")?;
                    Instruction::Load(offset)
                }
                
                "STORE" => {
                    let offset = parts.get(1)
                        .ok_or_else(|| anyhow::anyhow!("STORE requires offset"))?
                        .parse()
                        .context("Invalid offset for STORE")?;
                    Instruction::Store(offset)
                }
                
                "CALL_NATIVE" => {
                    let id = parts.get(1)
                        .ok_or_else(|| anyhow::anyhow!("CALL_NATIVE requires ID"))?
                        .parse()
                        .context("Invalid ID for CALL_NATIVE")?;
                    Instruction::CallNative(id)
                }
                
                "RETURN" => Instruction::Return,
                "HALT" => Instruction::Halt,
                
                _ => {
                    return Err(anyhow::anyhow!("Unknown instruction: {}", opcode_str));
                }
            };
            
            instructions.push(instruction);
        }
        
        Ok(Program::new(instructions, memory_size))
    }
}

