// AeroFlow Compiler - Codegen
// AST -> IR

use crate::ast::{Expr, Stmt};
use crate::ir::{Instr, Chunk, Value};
use crate::lexer::TokenKind;

pub struct Codegen {
    chunk: Chunk,
}

impl Codegen {
    pub fn new() -> Self {
        Self { chunk: Chunk::new() }
    }

    pub fn compile(mut self, stmts: Vec<Stmt>) -> Chunk {
        for stmt in stmts {
            self.compile_stmt(stmt);
        }
        self.chunk
    }

    fn compile_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Fn { .. } => {}
            Stmt::Actor { .. } => {}
            Stmt::Agent { .. } => {}
            Stmt::Model { .. } => {}
            Stmt::FromImport { .. } => {}
            Stmt::VarDecl { name, r#type: _, value } => {
                self.compile_expr(&value);
                self.chunk.emit(Instr::StoreVar(name));
            }
            Stmt::Render(render_expr) => {
                match render_expr {
                    crate::ast::RenderExpression::Expr(expr) => {
                        self.compile_expr(&expr);
                        self.chunk.emit(Instr::Render);
                    }
                    crate::ast::RenderExpression::Timeline(_) => {
                        // For now, Timeline is a hint for the IDE/Scheduler
                        self.chunk.emit(Instr::RenderTimeline);
                    }
                    crate::ast::RenderExpression::DistributedState(_) => {
                        self.chunk.emit(Instr::RenderState);
                    }
                }
            }
            Stmt::Spawn(expr) => {
                self.compile_expr(&expr);
                self.chunk.emit(Instr::Spawn(0));
            }
            Stmt::If { condition, then_branch, else_branch } => {
                self.compile_expr(&condition);
                let jump_if_false_idx = self.chunk.instrs.len();
                self.chunk.emit(Instr::JumpIfFalse(0)); // Placeholder
                
                for s in then_branch { self.compile_stmt(s); }
                
                if let Some(else_stmts) = else_branch {
                    let jump_idx = self.chunk.instrs.len();
                    self.chunk.emit(Instr::Jump(0)); // Placeholder
                    
                    self.chunk.instrs[jump_if_false_idx] = Instr::JumpIfFalse(self.chunk.instrs.len());
                    
                    for s in else_stmts { self.compile_stmt(s); }
                    self.chunk.instrs[jump_idx] = Instr::Jump(self.chunk.instrs.len());
                } else {
                    self.chunk.instrs[jump_if_false_idx] = Instr::JumpIfFalse(self.chunk.instrs.len());
                }
            }
            Stmt::While { condition, body } => {
                let start_idx = self.chunk.instrs.len();
                self.compile_expr(&condition);
                let jump_if_false_idx = self.chunk.instrs.len();
                self.chunk.emit(Instr::JumpIfFalse(0));
                
                for s in body { self.compile_stmt(s); }
                self.chunk.emit(Instr::Jump(start_idx));
                self.chunk.instrs[jump_if_false_idx] = Instr::JumpIfFalse(self.chunk.instrs.len());
            }
            Stmt::Return(expr) => {
                self.compile_expr(&expr);
                self.chunk.emit(Instr::Return);
            }
            Stmt::Expr(expr) => {
                self.compile_expr(&expr);
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Number(n) => self.chunk.emit(Instr::LoadConst(Value::Number(*n))),
            Expr::String(s) => self.chunk.emit(Instr::LoadConst(Value::String(s.clone()))),
            Expr::Bool(b) => self.chunk.emit(Instr::LoadConst(Value::Bool(*b))),
            Expr::Ident(name) => self.chunk.emit(Instr::LoadVar(name.clone())),
            Expr::Env(key) => self.chunk.emit(Instr::LoadEnv(key.clone())),
            Expr::Time => self.chunk.emit(Instr::LoadTime),
            Expr::Rand => self.chunk.emit(Instr::LoadRand),
            Expr::Binary { left, op, right } => {
                self.compile_expr(left);
                self.compile_expr(right);
                match op {
                    TokenKind::Plus => self.chunk.emit(Instr::Add),
                    TokenKind::Minus => self.chunk.emit(Instr::Sub),
                    TokenKind::Star => self.chunk.emit(Instr::Mul),
                    TokenKind::Slash => self.chunk.emit(Instr::Div),
                    TokenKind::EqualEqual => self.chunk.emit(Instr::Eq),
                    TokenKind::RAngle => self.chunk.emit(Instr::Gt),
                    TokenKind::LAngle => self.chunk.emit(Instr::Lt),
                    TokenKind::And => self.chunk.emit(Instr::Add), // Placeholder: Use same logic as Add or Eq for now
                    TokenKind::Or => self.chunk.emit(Instr::Add),  // Placeholder
                    _ => {}
                }
            }
            Expr::Call { name, args } => {
                let arg_count = args.len();
                for arg in args {
                    self.compile_expr(arg);
                }
                self.chunk.emit(Instr::Call(name.clone(), arg_count));
            }
            Expr::Tensor { .. } => {}
            Expr::Block(exprs) => {
                for expr in exprs {
                    self.compile_expr(expr);
                }
            }
        }
    }
}
