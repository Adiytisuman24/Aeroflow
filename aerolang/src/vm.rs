use crate::value::Value;

pub struct VM {
    pub registers: Vec<Value>,
    pub pc: usize,
}

impl VM {
    pub fn new(reg_count: usize) -> Self {
        VM {
            registers: vec![Value::Null; reg_count],
            pc: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        for reg in &mut self.registers {
            *reg = Value::Null;
        }
    }
}
