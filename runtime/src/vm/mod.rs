// AeroFlow Runtime - VM
// High performance bytecode execution

use aeroflow_compiler::ir::{Instr, Chunk, Value};
use std::collections::HashMap;

pub struct VMContext {
    pub logical_time: u64,
    pub rand_seed: u64,
}

pub struct VM {
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
    rng: u64, // Simple XorShift seed
}

impl VM {
    pub fn new() -> Self {
        Self { 
            stack: Vec::with_capacity(256),
            globals: HashMap::new(),
            rng: 0xACE1,
        }
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.rng = seed;
    }

    pub fn get_globals(&self) -> &HashMap<String, Value> {
        &self.globals
    }

    pub fn execute(&mut self, chunk: &Chunk, ctx: &VMContext) {
        let mut ip = 0;
        
        while ip < chunk.instrs.len() {
            let instr = &chunk.instrs[ip];
            match instr {
                Instr::LoadConst(val) => {
                    self.stack.push(val.clone());
                }
                Instr::LoadVar(name) => {
                    let val = self.globals.get(name).cloned().unwrap_or(Value::Nil);
                    self.stack.push(val);
                }
                Instr::StoreVar(name) => {
                    if let Some(val) = self.stack.pop() {
                        self.globals.insert(name.clone(), val);
                    }
                }
                Instr::LoadEnv(key) => {
                    let val = std::env::var(key).unwrap_or_else(|_| "".to_string());
                    self.stack.push(Value::String(val));
                }
                Instr::LoadTime => {
                    // SECURE: Use logical time from scheduler context, NOT wall-clock
                    self.stack.push(Value::Number(ctx.logical_time as f64));
                }
                Instr::LoadRand => {
                    // SECURE: Deterministic XorShift PRNG
                    self.rng ^= self.rng << 13;
                    self.rng ^= self.rng >> 17;
                    self.rng ^= self.rng << 5;
                    let val = (self.rng as f64) / (u64::MAX as f64);
                    self.stack.push(Value::Number(val));
                }
                Instr::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => self.stack.push(Value::Number(a + b)),
                        (Value::String(a), Value::String(b)) => {
                            let mut res = a;
                            res.push_str(&b);
                            self.stack.push(Value::String(res));
                        }
                        (Value::String(a), Value::Number(b)) => {
                            let mut res = a;
                            res.push_str(&b.to_string());
                            self.stack.push(Value::String(res));
                        }
                        _ => self.stack.push(Value::Nil),
                    }
                }
                Instr::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => self.stack.push(Value::Number(a - b)),
                        _ => self.stack.push(Value::Nil),
                    }
                }
                Instr::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => self.stack.push(Value::Number(a * b)),
                        _ => self.stack.push(Value::Nil),
                    }
                }
                Instr::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => self.stack.push(Value::Number(a / b)),
                        _ => self.stack.push(Value::Nil),
                    }
                }
                Instr::Eq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(format!("{:?}", a) == format!("{:?}", b)));
                }
                Instr::Gt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => self.stack.push(Value::Bool(a > b)),
                        _ => self.stack.push(Value::Bool(false)),
                    }
                }
                Instr::Lt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a, b) {
                        (Value::Number(a), Value::Number(b)) => self.stack.push(Value::Bool(a < b)),
                        _ => self.stack.push(Value::Bool(false)),
                    }
                }
                Instr::Jump(target) => {
                    ip = *target;
                    continue;
                }
                Instr::JumpIfFalse(target) => {
                    if let Some(val) = self.stack.pop() {
                        let is_true = match val {
                            Value::Bool(b) => b,
                            Value::Nil => false,
                            Value::Number(n) => n != 0.0,
                            _ => true,
                        };
                        if !is_true {
                            ip = *target;
                            continue;
                        }
                    }
                }
                Instr::Call(name, _) => {
                    if name == "len" {
                        if let Some(Value::String(s)) = self.stack.pop() {
                            self.stack.push(Value::Number(s.len() as f64));
                        }
                    }
                }
                Instr::Spawn(_) => {
                    self.stack.pop();
                    println!("[Runtime] Spawned task");
                }
                Instr::Return => {
                    return;
                }
                Instr::Render => {
                    if let Some(val) = self.stack.pop() {
                        match val {
                            Value::Number(n) => println!("{}", n),
                            Value::String(s) => println!("{}", s),
                            Value::Bool(b) => println!("{}", b),
                            Value::Nil => println!("nil"),
                        }
                    }
                }
                Instr::RenderTimeline => {
                    println!("🌀 [DAS] EMITTING DISTRIBUTED TIMELINE DAG...");
                }
                Instr::RenderState => {
                    println!("🧩 [DAS] EMITTING DETERMINISTIC STATE SNAPSHOT...");
                }
                Instr::RenderUI => {
                    println!("📱 [DAS] DISPATCHING DECLARATIVE UI UPDATE...");
                }
            }
            ip += 1;
        }
    }
}
