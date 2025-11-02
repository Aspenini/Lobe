use clap::Parser;
use lobe::create_runtime;
use std::fs;
use std::process;

#[derive(Parser)]
#[command(name = "lobe")]
#[command(about = "A fast Brainfuck interpreter")]
struct Cli {
    /// Brainfuck source file to run
    file: String,
}

fn main() {
    let cli = Cli::parse();

    // Read the source file
    let src = match fs::read_to_string(&cli.file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", cli.file, e);
            process::exit(1);
        }
    };

    // Create runtime and run
    let mut runtime = match create_runtime(&src) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = runtime.run() {
        eprintln!("Runtime error: {}", e);
        process::exit(1);
    }
}

