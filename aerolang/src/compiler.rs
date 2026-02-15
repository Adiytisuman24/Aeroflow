use crate::ast::*;
use crate::ir::{Instr, Op};
use crate::value::Value;
use std::collections::HashMap;

pub struct Compiler {
    pub code: Vec<Instr>,
    // Map constant values to indices in VM registers (simple MVP strategy)
    // In a real VM, constants would be in a separate pool.
    // Here we'll just emit load instructions or assume pre-loading.
    // For this MVP, we will generate code that assumes specific register usage.
    // Reg 0..N: Locals
    // Reg N+1..M: Temporaries
    next_reg: usize,
    var_map: HashMap<String, usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            code: vec![],
            next_reg: 0,
            var_map: HashMap::new(),
        }
    }

    pub fn compile(&mut self, stmts: &[Stmt]) -> Vec<Instr> {
        for stmt in stmts {
            self.compile_stmt(stmt);
        }
        self.code.clone()
    }

    fn alloc_reg(&mut self) -> usize {
        let r = self.next_reg;
        self.next_reg += 1;
        r
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl { name, expr } => {
                let reg = self.alloc_reg();
                self.var_map.insert(name.clone(), reg);
                self.compile_expr(expr, reg);
            }
            Stmt::Assign { name, expr } => {
                if let Some(&reg) = self.var_map.get(name) {
                    self.compile_expr(expr, reg);
                } else {
                    panic!("Unknown variable: {}", name);
                }
            }
            Stmt::Expr(expr) => {
                let reg = self.alloc_reg(); // temp reg
                self.compile_expr(expr, reg);
            }
            Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    let reg = self.alloc_reg();
                    self.compile_expr(expr, reg);
                    self.emit(Op::Return, reg, 0, 0);
                } else {
                    self.emit(Op::Return, 0, 0, 0); // Return null/void (reg 0 placeholder)
                }
            }
            _ => {
                // TODO: Implement other statements
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr, target_reg: usize) {
        match expr {
            Expr::Int(i) => {
                // In a real VM, we'd have Op::LoadConst w/ index to const pool.
                // For MVP, we pretend 'b' field holds immediate if it fits, or we'd need a different op.
                // Our Value::Int is i64, Instr fields are usize. 
                // We'll hack it: assume small ints for demo or add LoadConstValue to VM.
                // But wait, VM IR is fixed. 
                // Let's add a "LoadImmediate" op to IR or similar?
                // Or just assume the VM has a way to get constants.
                // Let's use a "Const Pool" approach.
                // But IR doesn't support it directly yet.
                // Let's modify VM/IR to support immediate loads or just placeholder.
                
                // Hack for MVP: LoadConst a=target, b=value (cast to usize)
                self.emit(Op::LoadConst, target_reg, *i as usize, 0);
            }
            Expr::Var(name) => {
                if let Some(&src_reg) = self.var_map.get(name) {
                    self.emit(Op::LoadVar, target_reg, src_reg, 0);
                } else {
                    panic!("Unknown variable: {}", name);
                }
            }
            Expr::Binary { left, op, right } => {
                let r_left = self.alloc_reg();
                self.compile_expr(left, r_left);
                
                let r_right = self.alloc_reg();
                self.compile_expr(right, r_right);
                
                let op_code = match op.as_str() {
                    "+" => Op::Add,
                    "-" => Op::Sub,
                    "*" => Op::Mul,
                    "/" => Op::Div,
                    _ => panic!("Unknown op: {}", op),
                };
                
                self.emit(op_code, target_reg, r_left, r_right);
            }
            Expr::Call { name, args } => {
                if name == "send" && args.len() == 2 {
                    // Intrinsic for Actor Send
                    let r_target = self.alloc_reg();
                    self.compile_expr(&args[0], r_target);
                    
                    let r_msg = self.alloc_reg();
                    self.compile_expr(&args[1], r_msg);
                    
                    // Send instruction in IR: Op::Send a=target, b=msg
                    // Use target_reg to store result (null?)
                    self.emit(Op::Send, r_target, r_msg, 0);
                    // Result of send is usually void, but let's clear target_reg
                } else if name == "recv" {
                     self.emit(Op::Recv, target_reg, 0, 0);
                } else {
                    // Function call
                    self.emit(Op::Call, target_reg, 0, 0);
                }
            }
            _ => {}
        }
    }

    fn emit(&mut self, op: Op, a: usize, b: usize, c: usize) {
        self.code.push(Instr { op, a, b, c });
    }
}
