//! Lobe - A fast Brainfuck interpreter

mod interpreter;
mod parser;
mod types;

pub use interpreter::Runtime;
pub use types::Bytecode;

use anyhow::Result;

/// Create a new Runtime from Brainfuck source code
///
/// # Arguments
///
/// * `src` - Brainfuck source code
///
/// # Errors
///
/// Returns an error if parsing fails (e.g., unmatched brackets)
pub fn create_runtime(src: &str) -> Result<Runtime> {
    let bytecode = parser::parse(src)?;
    Ok(Runtime::new(bytecode))
}

/// Convenience function: parse and run a Brainfuck program
///
/// # Arguments
///
/// * `src` - Brainfuck source code
///
/// # Errors
///
/// Returns an error if parsing or execution fails
pub fn run(src: &str) -> Result<()> {
    let mut runtime = create_runtime(src)?;
    runtime.run()
}

