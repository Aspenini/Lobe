use crate::types::{Bytecode, Instr};
use anyhow::Result;
use std::io::{self, Read, Write};

/// Fixed tape size per original Brainfuck specification
const TAPE_SIZE: usize = 30000;

/// Runtime state for executing Brainfuck programs
pub struct Runtime {
    bytecode: Bytecode,
    tape: Vec<u8>,
    dp: usize,
    ip: usize, // instruction pointer
}

impl Runtime {
    /// Create a new runtime with the given bytecode
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            bytecode,
            tape: vec![0u8; TAPE_SIZE], // Fixed-size tape per original spec
            dp: 0,
            ip: 0,
        }
    }

    /// Execute the program following original Brainfuck rules:
    /// - Fixed 30,000 cell tape
    /// - Pointer wraps around at both ends
    /// - 8-bit cell values with wrapping
    pub fn run(&mut self) -> Result<()> {
        while self.ip < self.bytecode.instrs.len() {
            match &self.bytecode.instrs[self.ip] {
                Instr::IncrementPtr => {
                    // Wrap around: if at end, go to beginning
                    self.dp = (self.dp + 1) % TAPE_SIZE;
                    self.ip += 1;
                }
                Instr::DecrementPtr => {
                    // Wrap around: if at beginning, go to end
                    self.dp = if self.dp == 0 {
                        TAPE_SIZE - 1
                    } else {
                        self.dp - 1
                    };
                    self.ip += 1;
                }
                Instr::Increment => {
                    self.tape[self.dp] = self.tape[self.dp].wrapping_add(1);
                    self.ip += 1;
                }
                Instr::Decrement => {
                    self.tape[self.dp] = self.tape[self.dp].wrapping_sub(1);
                    self.ip += 1;
                }
                Instr::Output => {
                    print!("{}", self.tape[self.dp] as char);
                    io::stdout().flush()?;
                    self.ip += 1;
                }
                Instr::Input => {
                    let mut buf = [0u8; 1];
                    match io::stdin().read_exact(&mut buf) {
                        Ok(_) => self.tape[self.dp] = buf[0],
                        Err(_) => self.tape[self.dp] = 0, // EOF behavior: set to 0
                    }
                    self.ip += 1;
                }
                Instr::JumpIfZero(target) => {
                    if self.tape[self.dp] == 0 {
                        self.ip = *target + 1;
                    } else {
                        self.ip += 1;
                    }
                }
                Instr::JumpIfNonZero(target) => {
                    if self.tape[self.dp] != 0 {
                        self.ip = *target;
                    } else {
                        self.ip += 1;
                    }
                }
            }
        }

        Ok(())
    }
}

