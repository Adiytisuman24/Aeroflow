use crate::fiber::Fiber;
use crate::ir::Op;
use crate::value::Value;

// Reuse the trait or redefine locally if simple
pub trait SchedulerContext {
    fn send_message(&mut self, target_id: usize, msg: Value);
}

pub fn step(fiber: &mut Fiber, max_steps: usize, ctx: &mut dyn SchedulerContext) {
    let mut steps = 0;

    while steps < max_steps && !fiber.finished {
        if fiber.vm.pc >= fiber.code.len() {
            fiber.finished = true;
            break;
        }

        let instr = fiber.code[fiber.vm.pc];
        fiber.vm.pc += 1;

        match instr.op {
            Op::LoadConst => {
                // For MVP, we pass 'b' as source const index if we had a const table, 
                // but let's assume 'b' is the value for now in this demo (not possible with usize directly unless encoded)
                // Actually, let's just use `LoadVar` logic for simplicity or modify `Instr` to carry `Value`?
                // The IR definition says `Instr` carries `usize`.
                // Let's assume registers are pre-populated for this demo or use `Op::LoadConst` to copy from reg `b` to `a` (like move).
                fiber.vm.registers[instr.a] = fiber.vm.registers[instr.b].clone();
            }
            
            Op::LoadVar => {
                fiber.vm.registers[instr.a] = fiber.vm.registers[instr.b].clone();
            }

            Op::Add => {
                let left = &fiber.vm.registers[instr.b];
                let right = &fiber.vm.registers[instr.c];
                fiber.vm.registers[instr.a] = match (left, right) {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                    (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
                    _ => Value::Null, // panic!("Type error: Add"),
                };
            }

            Op::Send => {
                let target_reg = instr.a; 
                // Wait, instr.a is target register index? Or target ID?
                // Use registers for target ID.
                let target_id = match fiber.vm.registers[target_reg] {
                    Value::Int(id) => id as usize,
                    _ => 0, // Invalid target
                };
                
                let value_reg = instr.b;
                let value = fiber.vm.registers[value_reg].clone();
                ctx.send_message(target_id, value);
            }

            Op::Recv => {
                if let Some(msg) = fiber.mailbox.recv() {
                    fiber.vm.registers[instr.a] = msg;
                } else {
                    fiber.vm.pc -= 1; // Retry later
                    break; // Yield
                }
            }

            Op::Return => {
                fiber.finished = true;
                break;
            }

            _ => {}
        }

        steps += 1;
    }
}
