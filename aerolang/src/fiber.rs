use crate::vm::VM;
use crate::ir::Instr;
use crate::actor::Mailbox;

pub struct Fiber {
    pub id: usize,
    pub vm: VM,
    pub code: Vec<Instr>,
    pub mailbox: Mailbox,
    pub finished: bool,
}

impl Fiber {
    pub fn new(id: usize, code: Vec<Instr>, reg_count: usize) -> Self {
        Self {
            id,
            vm: VM::new(reg_count),
            code,
            mailbox: Mailbox::new(),
            finished: false,
        }
    }
}
