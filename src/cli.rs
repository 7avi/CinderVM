use crate::interpreter::Interpreter;
use crate::jit::JitCompiler;
use crate::parser::Parser;
use anyhow::{Context, Result};
use clap::{Parser as ClapParser, Subcommand};

#[derive(ClapParser)]
#[command(name = "cinder")]
#[command(about = "CinderVM - JIT execution engine and sandbox")]
pub struct CinderCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compile and run program using JIT
    Exec {
        /// .cinder file to execute
        file: String,
    },
    
    /// Run program using interpreter (for debugging)
    Debug {
        /// .cinder file to execute
        file: String,
    },
    
    /// Display generated machine code
    Disassemble {
        /// .cinder file to disassemble
        file: String,
    },
}

impl CinderCli {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            Commands::Exec { file } => {
                self.execute_jit(file)
            }
            
            Commands::Debug { file } => {
                self.execute_interpreter(file)
            }
            
            Commands::Disassemble { file } => {
                self.disassemble(file)
            }
        }
    }

    fn execute_jit(&self, file: &str) -> Result<()> {
        println!("🔧 JIT compilation for: {}", file);
        
        let program = Parser::parse_file(file)
            .with_context(|| format!("Error parsing file: {}", file))?;
        
        let mut compiler = JitCompiler::new(program);
        let memory = compiler.compile()
            .context("Error during JIT compilation")?;
        
        println!("✅ Compilation successful!");
        println!("🚀 Executing native code...");
        
        // Execute compiled code
        unsafe {
            type NativeFunction = unsafe extern "C" fn() -> i64;
            let func: NativeFunction = memory.as_function();
            let result = func();
            println!("📊 Result: {}", result);
        }
        
        Ok(())
    }

    fn execute_interpreter(&self, file: &str) -> Result<()> {
        println!("🐛 Debug execution (interpreter) for: {}", file);
        
        let program = Parser::parse_file(file)
            .with_context(|| format!("Error parsing file: {}", file))?;
        
        let mut interpreter = Interpreter::new(program);
        let result = interpreter.execute()
            .map_err(|e| anyhow::anyhow!("Execution error: {:?}", e))?;
        
        println!("📊 Result: {}", result);
        Ok(())
    }

    fn disassemble(&self, file: &str) -> Result<()> {
        println!("📖 Disassembly for: {}", file);
        
        let program = Parser::parse_file(file)
            .with_context(|| format!("Error parsing file: {}", file))?;
        
        println!("\n📋 Bytecode instructions:");
        for (idx, instruction) in program.instructions.iter().enumerate() {
            println!("  {:04}: {:?}", idx, instruction);
        }
        
        println!("\n🔧 Generating machine code...");
        let mut compiler = JitCompiler::new(program);
        let memory = compiler.compile()
            .context("Error during JIT compilation")?;
        
        println!("\n💾 Generated machine code ({} bytes):", memory.size());
        unsafe {
            let ptr = memory.as_ptr();
            for i in 0..memory.size().min(256) {
                if i % 16 == 0 {
                    print!("\n  {:04X}: ", i);
                }
                print!("{:02X} ", *ptr.add(i));
            }
            println!();
        }
        
        Ok(())
    }
}

