/// Lobe - A fast Brainf*ck interpreter

mod interpreter;
mod parser;
mod types;

pub use interpreter::Runtime;
pub use types::Bytecode;

use anyhow::Result;

/// Create a new Runtime from Brainf*ck source code
///
/// # Arguments
///
/// * `src` - Brainf*ck source code
///
/// # Errors
///
/// Returns an error if parsing fails (e.g., unmatched brackets)
pub fn create_runtime(src: &str) -> Result<Runtime> {
    let bytecode = parser::parse(src)?;
    Ok(Runtime::new(bytecode))
}

/// Convenience function: parse and run a Brainf*ck program
///
/// # Arguments
///
/// * `src` - Brainf*ck source code
///
/// # Errors
///
/// Returns an error if parsing or execution fails
pub fn run(src: &str) -> Result<()> {
    let mut runtime = create_runtime(src)?;
    runtime.run()
}

