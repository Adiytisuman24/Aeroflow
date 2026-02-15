// AeroFlow Compiler - AST (v1.0 Locked Spec)
// Using smallvec for performance, Box for indirection

use smallvec::SmallVec;
use crate::lexer::TokenKind;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Ident(String),
    Binary {
        left: Box<Expr>,
        op: TokenKind,
        right: Box<Expr>,
    },
    Call {
        name: String,
        args: SmallVec<[Box<Expr>; 4]>,
    },
    Tensor {
        shape: Vec<usize>,
        data: Vec<f64>,
    },
    Env(String),
    Time,
    Rand,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Fn {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
        is_pure: bool,
    },
    Actor {
        name: String,
        body: Vec<Stmt>,
    },
    Agent {
        name: String,
        model: String,
        body: Vec<Stmt>,
    },
    Model {
        name: String,
        source: String,
        body: Vec<Stmt>,
    },
    FromImport {
        package: String,
        layer: String,
    },
    VarDecl {
        name: String,
        value: Expr,
    },
    Render(Expr),
    Spawn(Expr),
    If { condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>> },
    While { condition: Expr, body: Vec<Stmt> },
    Return(Expr),
    Expr(Expr),
}
