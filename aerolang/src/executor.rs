use crate::ir::*;
use crate::value::*;
use crate::vm::*;

pub fn execute(code: &[Instr], vm: &mut VM) -> Value {
    loop {
        if vm.pc >= code.len() {
            return Value::Null;
        }
        
        let instr = code[vm.pc];
        vm.pc += 1;

        match instr.op {
            Op::LoadConst => {
                // In a real implementation, constants are in a separate pool
                // For this MVP, we'll assume constants are pre-loaded into registers for LoadConst
                // or passed via a 'c' field if it were an index.
                // We'll treat 'b' as the source register for simplicity in this demo.
                vm.registers[instr.a] = vm.registers[instr.b].clone();
            }

            Op::Add => {
                let left = &vm.registers[instr.b];
                let right = &vm.registers[instr.c];
                vm.registers[instr.a] = match (left, right) {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                    (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
                    _ => panic!("Type error: Add"),
                };
            }
            
            Op::Sub => {
                let left = &vm.registers[instr.b];
                let right = &vm.registers[instr.c];
                vm.registers[instr.a] = match (left, right) {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
                    _ => panic!("Type error: Sub"),
                };
            }

            Op::Jump => {
                vm.pc = instr.a;
            }

            Op::JumpIfFalse => {
                if !vm.registers[instr.a].is_truthy() {
                    vm.pc = instr.b;
                }
            }

            Op::Return => {
                return vm.registers[instr.a].clone();
            }

            _ => {
                // Other ops handled by step executor in fiber context
            }
        }
    }
}
