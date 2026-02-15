#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    LoadConst,
    LoadVar,
    StoreVar,
    Add,
    Sub,
    Mul,
    Div,
    Jump,
    JumpIfFalse,
    Call,
    Send,
    Recv,
    Yield,
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instr {
    pub op: Op,
    pub a: usize,
    pub b: usize,
    pub c: usize,
}
