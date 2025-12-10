<<<<<<< HEAD
# CinderVM 🚀

**JIT execution engine and sandbox for custom bytecode, written in Rust**

Developed by **CoreVision (TAVI Development)**

## 📋 Description

CinderVM is a minimal execution engine that demonstrates Rust's capability to handle low-level programming and concurrency complexity, while maintaining a safe interface (safe Rust) for external interaction.

### Key Features:

- ✅ **Custom bytecode** with 15+ instructions
- ✅ **Interpreter** for validation and debugging
- ✅ **JIT compilation** to x86-64 machine code
- ✅ **Sandboxing** with security checks
- ✅ **Controlled FFI** with whitelist for native functions
- ✅ **Complete CLI** with commands for execution, debugging, and disassembly

## 🏗️ Architecture

```
CinderVM/
├── src/
│   ├── main.rs          # Entry point
│   ├── bytecode.rs      # Bytecode instruction definitions
│   ├── interpreter.rs   # Interpreter for validation
│   ├── parser.rs        # Parser for .cinder files
│   ├── jit/
│   │   ├── mod.rs
│   │   ├── codegen.rs   # x86-64 machine code generation
│   │   └── memory.rs    # Executable memory allocation
│   ├── sandbox.rs       # Security validations
│   └── cli.rs           # CLI interface
└── examples/            # Example programs
```

## 🚀 Installation and Usage

### Building

```bash
cargo build --release
```

### CLI Usage

#### Execute with JIT:
```bash
cargo run -- exec examples/simple.cinder
```

#### Execute with interpreter (debug):
```bash
cargo run -- debug examples/simple.cinder
```

#### Disassemble:
```bash
cargo run -- disassemble examples/simple.cinder
```

## 📝 .cinder File Format

`.cinder` files contain bytecode instructions, one per line:

```cinder
# Comment
PUSH_INT 10
PUSH_INT 5
ADD
RETURN
```

### Available Instructions:

#### Operands and Stack:
- `PUSH_INT <value>` - Push an integer value onto the stack
- `POP` - Pop a value from the stack

#### Arithmetic Operations:
- `ADD` - Add two values from the stack
- `SUB` - Subtract two values from the stack
- `MUL` - Multiply two values from the stack
- `DIV` - Divide two values from the stack

#### Logical Operations:
- `EQ` - Check equality (returns 1 or 0)
- `LT` - Check less than (<)
- `GT` - Check greater than (>)

#### Control Flow:
- `JUMP <target>` - Unconditional jump to target instruction
- `JUMP_IF_ZERO <target>` - Jump if value on stack is 0
- `JUMP_IF_NOT_ZERO <target>` - Jump if value on stack is not 0

#### Memory:
- `LOAD <offset>` - Load value from specified offset
- `STORE <offset>` - Store value from stack to offset

#### Calls:
- `CALL_NATIVE <id>` - Call a native function (only if in whitelist)
- `RETURN` - Return value from stack and terminate execution
- `HALT` - Stop execution

#### Special Directive:
- `.memory <size>` - Set allocated memory size

## 🔒 Security

CinderVM implements multiple security layers:

1. **Bytecode validation**: All jumps and memory accesses are validated before execution
2. **Memory sandboxing**: Memory access is limited to allocated region
3. **FFI whitelist**: Only allowed native functions can be called
4. **Unsafe isolation**: All risky operations are isolated in well-defined modules

## 🛠️ Development

### Code Structure:

- **Safe Rust**: External interface and high-level logic
- **Unsafe Rust**: Only in `jit::memory` and `jit::codegen` modules for:
  - Executable memory allocation
  - Machine code writing
  - Compiled code execution

### Extending:

To add new instructions:

1. Add the opcode in `bytecode.rs`
2. Implement execution in `interpreter.rs`
3. Implement code generation in `jit/codegen.rs`
4. Add parser support (`parser.rs`)

## 📚 Examples

See the `examples/` directory for example programs:

- `simple.cinder` - Basic arithmetic operations
- `arithmetic.cinder` - Complex operations test
- `factorial.cinder` - Factorial calculation (simplified)

## ⚠️ Limitations

- Relative jumps are not fully implemented (requires two passes)
- Local variable memory uses function stack (simplified)
- FFI for native functions requires complete function table implementation
- Support only for x86-64 (Windows and Unix)

## 📄 License

This project is an educational demonstrative project.

## 🙏 Contributions

This is a pilot project. For improvements, please open issues or pull requests.
=======
# CinderVM
>>>>>>> b46d50cab6f83a42960efa410bafa050bfa34aba
