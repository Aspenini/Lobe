# Lobe - A Fast Brainfuck Interpreter

Lobe is a high-performance Brainfuck interpreter written in Rust. It features a clean, efficient implementation with dynamic memory management and excellent error handling.

## Installation

### From crates.io (Coming Soon)

```bash
cargo install lobe
```

### From Source

Clone the repository and build:

```bash
git clone https://github.com/Aspenini/Lobe.git
cd Lobe
cargo build --release
```

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
lobe = "0.1.0"
```

Then use in your code:

```rust
use lobe::create_runtime;

fn main() {
    let bf_code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.";
    let mut runtime = create_runtime(bf_code).unwrap();
    runtime.run().unwrap(); // Prints "Hello World!"
}
```

## Features

- **Fast Interpreter**: Optimized bytecode-based execution
- **Dynamic Tape**: Automatically grows as needed (no fixed size limits)
- **Robust Parsing**: Validates brackets and handles comments gracefully
- **Error Handling**: Clear error messages for invalid programs
- **Simple CLI**: Easy-to-use command-line interface
- **Cross-Platform**: Compiles on any platform supported by Rust

## Building

### Prerequisites

- Rust stable (≥ 1.75)
- Cargo

### Build Instructions

```bash
# Build in debug mode
cargo build

# Build optimized release binary
cargo build --release

# The binary will be at target/release/lobe (or target/debug/lobe for debug builds)
```

### Running Tests

```bash
cargo test
```

## Usage

### Basic Usage

Run a Brainfuck program:

```bash
lobe program.bf
```

That's it! No configuration needed.

### Example Brainfuck Program

Create a file `hello.bf`:

```brainfuck
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

Run it:

```bash
lobe hello.bf
# Output: Hello World!
```

## Architecture

### Component Overview

- **`parser.rs`**: Parses BF source, strips non-BF characters, matches brackets
- **`interpreter.rs`**: Core interpreter with dynamic tape and efficient instruction dispatch
- **`types.rs`**: Core data structures (instructions, bytecode)
- **`main.rs`**: CLI interface using Clap
- **`lib.rs`**: Public API for creating and running programs

### Design Decisions

1. **Dynamic Tape**: The tape grows exponentially as needed, starting with 30,000 cells (traditional BF default)
2. **Pointer Wrapping**: Moving left from position 0 wraps to the end of the tape (traditional BF behavior)
3. **Efficient Dispatch**: Direct match on instruction types for fast execution
4. **Bytecode**: Source is parsed once into an optimized bytecode representation

## Supported Brainfuck Features

All standard Brainfuck instructions are supported:

- `>` - Increment data pointer
- `<` - Decrement data pointer  
- `+` - Increment value at data pointer
- `-` - Decrement value at data pointer
- `.` - Output character at data pointer
- `,` - Input character and store at data pointer
- `[` - Jump forward if zero
- `]` - Jump backward if non-zero

The interpreter correctly handles:
- Nested loops
- Deep nesting levels
- Large programs
- Programs requiring large amounts of memory
- Comments (any non-BF characters are ignored)

## Cross-Compilation

Lobe can be cross-compiled to any target platform supported by Rust.

### Setup Cross-Compilation Toolchain

For example, to compile for Linux on ARM64:

```bash
# Install the target
rustup target add aarch64-unknown-linux-gnu

# Build for that target
cargo build --release --target aarch64-unknown-linux-gnu
```

### Supported Targets

Any target supported by the Rust toolchain (`rustup target list`). Common targets:
- `x86_64-unknown-linux-gnu` - Linux x86_64
- `aarch64-unknown-linux-gnu` - Linux ARM64
- `x86_64-pc-windows-msvc` - Windows x86_64
- `x86_64-apple-darwin` - macOS x86_64
- `aarch64-apple-darwin` - macOS ARM64

## Dependencies

- **clap**: Command-line argument parsing
- **anyhow**: Error handling

## Project Structure

```
lobe/
├── Cargo.toml              # Project configuration
├── README.md               # This file
├── src/
│   ├── main.rs            # CLI entry point
│   ├── lib.rs             # Public API
│   ├── parser.rs          # BF source parser
│   ├── interpreter.rs     # Interpreter and runtime
│   └── types.rs           # Core data structures
└── tests/
    └── integration_tests.rs # Integration tests
```

## Contributing

Contributions are welcome! Areas that could use work:

1. Performance optimizations
2. Additional test cases
3. Documentation improvements
4. Benchmarking suite

## License

This project is provided as-is for educational and demonstration purposes.

## Acknowledgments

- Brainfuck language by Urban Müller
- Rust language and ecosystem
