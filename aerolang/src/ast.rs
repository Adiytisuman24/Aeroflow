#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Var(String),
    Call {
        name: String,
        args: Vec<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl {
        name: String,
        expr: Expr,
    },
    Assign {
        name: String,
        expr: Expr,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_block: Vec<Stmt>,
        elif_blocks: Vec<(Expr, Vec<Stmt>)>,
        else_block: Option<Vec<Stmt>>,
    },
    Return(Option<Expr>),
    Expr(Expr),
}
