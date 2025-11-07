# Lobe

A fast, cross-platform Brainfuck interpreter written in Rust

## Install

```bash
cargo install lobe
```

Or from source:

```bash
git clone https://github.com/Aspenini/Lobe.git
cd Lobe
cargo build --release
```

## Use

**CLI:**
```bash
lobe program.bf
```

**Library:**
```bash
cargo add lobe
```

```rust
use lobe::create_runtime;

let mut runtime = create_runtime("++++.").unwrap();
runtime.run().unwrap();
```

## Features

- Bytecode-based execution
- Fixed 30,000 cell tape (original Brainfuck specification)
- Pointer wrapping at tape boundaries
- Cross-platform

## License

MIT
