/// Core data structures for the Brainf*ck interpreter

/// Brainf*ck instruction representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instr {
    /// `>` - Increment data pointer
    IncrementPtr,
    /// `<` - Decrement data pointer
    DecrementPtr,
    /// `+` - Increment value at data pointer
    Increment,
    /// `-` - Decrement value at data pointer
    Decrement,
    /// `.` - Output character at data pointer
    Output,
    /// `,` - Input character and store at data pointer
    Input,
    /// `[` - Jump if zero to matching `]` (index is the matching bracket position)
    JumpIfZero(usize),
    /// `]` - Jump if non-zero to matching `[` (index is the matching bracket position)
    JumpIfNonZero(usize),
}

/// Compiled bytecode representation
#[derive(Debug, Clone)]
pub struct Bytecode {
    pub instrs: Vec<Instr>,
}

