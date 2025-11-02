use crate::types::{Bytecode, Instr};
use anyhow::{anyhow, Result};

/// Parse Brainfuck source code into bytecode
///
/// Strips all non-BF characters and matches brackets.
/// Returns an error if brackets are unmatched.
pub fn parse(src: &str) -> Result<Bytecode> {
    // Filter to only BF characters
    let filtered: Vec<char> = src
        .chars()
        .filter(|&c| matches!(c, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
        .collect();

    // First pass: match brackets and build instruction vector with placeholder indices
    let mut instrs = Vec::new();
    let mut bracket_stack: Vec<usize> = Vec::new(); // Stack of instruction indices for '['
    let mut bracket_pairs: Vec<(usize, usize)> = Vec::new(); // (open_instr_idx, close_instr_idx) pairs

    for &ch in &filtered {
        let instr_idx = instrs.len();
        
        match ch {
            '>' => instrs.push(Instr::IncrementPtr),
            '<' => instrs.push(Instr::DecrementPtr),
            '+' => instrs.push(Instr::Increment),
            '-' => instrs.push(Instr::Decrement),
            '.' => instrs.push(Instr::Output),
            ',' => instrs.push(Instr::Input),
            '[' => {
                bracket_stack.push(instr_idx);
                instrs.push(Instr::JumpIfZero(0)); // Placeholder, will be fixed
            }
            ']' => {
                let open_idx = bracket_stack.pop().ok_or_else(|| {
                    anyhow!("Unmatched closing bracket")
                })?;
                bracket_pairs.push((open_idx, instr_idx));
                instrs.push(Instr::JumpIfNonZero(0)); // Placeholder, will be fixed
            }
            _ => {} // Should never happen after filtering, but satisfies exhaustiveness
        }
    }

    if !bracket_stack.is_empty() {
        return Err(anyhow!("Unmatched opening brackets: {} remaining", bracket_stack.len()));
    }

    // Second pass: fix bracket indices
    for (open_idx, close_idx) in bracket_pairs {
        match &mut instrs[open_idx] {
            Instr::JumpIfZero(ref mut target) => *target = close_idx,
            _ => unreachable!(),
        }
        match &mut instrs[close_idx] {
            Instr::JumpIfNonZero(ref mut target) => *target = open_idx,
            _ => unreachable!(),
        }
    }

    Ok(Bytecode { instrs })
}

