// AeroFlow Compiler - IR
// Flat & Brutal

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Instr {
    LoadConst(Value),
    LoadVar(String),
    StoreVar(String),
    LoadEnv(String),
    LoadTime,           // Logical time from scheduler
    LoadRand,           // Deterministic PRNG
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Gt,
    Lt,
    Call(String, usize), // name, arg_count
    Spawn(usize),        // arg_count
    Jump(usize),         // exact instruction index
    JumpIfFalse(usize),
    Return,
    Render,              // Context-aware output
    RenderTimeline,      // Emit distributed event graph
    RenderState,         // Emit engine state snapshot
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub instrs: Vec<Instr>,
}

impl Chunk {
    pub fn new() -> Self {
        Self { instrs: Vec::new() }
    }

    pub fn emit(&mut self, instr: Instr) {
        self.instrs.push(instr);
    }
}
