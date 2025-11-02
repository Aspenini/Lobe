use crate::types::{Bytecode, Instr};
use anyhow::Result;
use std::io::{self, Read, Write};

/// Runtime state for executing Brainf*ck programs
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
            tape: vec![0u8; 30000], // Start with traditional 30k cells
            dp: 0,
            ip: 0,
        }
    }

    /// Ensure the tape is large enough for the current data pointer
    fn ensure_tape_size(&mut self) {
        while self.dp >= self.tape.len() {
            // Grow tape exponentially
            self.tape.resize(self.tape.len() * 2, 0);
        }
    }

    /// Execute the program
    pub fn run(&mut self) -> Result<()> {
        while self.ip < self.bytecode.instrs.len() {
            self.ensure_tape_size();

            match &self.bytecode.instrs[self.ip] {
                Instr::IncrementPtr => {
                    self.dp += 1;
                    self.ip += 1;
                }
                Instr::DecrementPtr => {
                    if self.dp == 0 {
                        // Wrap around to end of tape (traditional BF behavior)
                        self.ensure_tape_size();
                        self.dp = self.tape.len() - 1;
                    } else {
                        self.dp -= 1;
                    }
                    self.ip += 1;
                }
                Instr::Increment => {
                    self.ensure_tape_size();
                    self.tape[self.dp] = self.tape[self.dp].wrapping_add(1);
                    self.ip += 1;
                }
                Instr::Decrement => {
                    self.ensure_tape_size();
                    self.tape[self.dp] = self.tape[self.dp].wrapping_sub(1);
                    self.ip += 1;
                }
                Instr::Output => {
                    self.ensure_tape_size();
                    print!("{}", self.tape[self.dp] as char);
                    io::stdout().flush()?;
                    self.ip += 1;
                }
                Instr::Input => {
                    self.ensure_tape_size();
                    let mut buf = [0u8; 1];
                    match io::stdin().read_exact(&mut buf) {
                        Ok(_) => self.tape[self.dp] = buf[0],
                        Err(_) => self.tape[self.dp] = 0, // EOF
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

