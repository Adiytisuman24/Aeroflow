// AeroFlow Compiler - AST (v1.0 Locked Spec)
// Using smallvec for performance, Box for indirection

use smallvec::SmallVec;
use crate::lexer::TokenKind;

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    List(Box<Type>),
    Dict(Box<Type>, Box<Type>),
    Void,
}

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
    Block(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum RenderExpression {
    Expr(Expr),
    Timeline(TimelineBlock),
    DistributedState(DistributedStateBlock),
}

#[derive(Debug, Clone)]
pub struct TimelineBlock {
    pub events: Vec<TimelineEvent>,
}

#[derive(Debug, Clone)]
pub struct TimelineEvent {
    pub from: String,
    pub to: String,
    pub at_ms: u64,
    pub payload: Expr,
}

#[derive(Debug, Clone)]
pub struct DistributedStateBlock {
    pub state_refs: Vec<NodeStateRef>,
}

#[derive(Debug, Clone)]
pub struct NodeStateRef {
    pub node: String,
    pub field: String,
}

#[derive(Debug, Clone)]
pub struct EventHandler {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Fn {
        name: String,
        params: Vec<(String, Type)>,
        body: Vec<Stmt>,
        return_type: Type,
        is_pure: bool,
    },
    Actor {
        name: String,
        body: Vec<Stmt>,
    },
    Agent {
        name: String,
        model: Option<String>,
        handlers: Vec<EventHandler>,
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
        r#type: Type,
        value: Expr,
    },
    Render(RenderExpression),
    Spawn(Expr),
    If { condition: Expr, then_branch: Vec<Stmt>, else_branch: Option<Vec<Stmt>> },
    While { condition: Expr, body: Vec<Stmt> },
    Return(Expr),
    Expr(Expr),
}
