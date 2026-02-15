use crate::ir::*;
use crate::fiber::Fiber;
use crate::scheduler::Scheduler;
use crate::value::Value;

pub fn main() {
    println!("🚀 Starting AeroLang Actor Runtime Demo...");
    
    let mut scheduler = Scheduler::new();
    
    // Actor 0: Repeater
    // Code
    let code0 = vec![
        // Regs pre-filled: r0=1 (Target), r1=42 (Msg)
        Instr { op: Op::Send, a: 0, b: 1, c: 0 }, // Send(dest=r0, val=r1)
        Instr { op: Op::Recv, a: 2, b: 0, c: 0 }, // Recv into r2
        Instr { op: Op::Return, a: 2, b: 0, c: 0 },
    ];
    
    // Actor 1: Adder
    // Code
    let code1 = vec![
        // Regs pre-filled: r1=10 (Add), r3=0 (Reply Target)
        Instr { op: Op::Recv, a: 0, b: 0, c: 0 }, // Recv into r0
        Instr { op: Op::Add, a: 2, b: 0, c: 1 },  // r2 = r0 + r1
        Instr { op: Op::Send, a: 3, b: 2, c: 0 }, // Send(dest=r3, val=r2)
        Instr { op: Op::Return, a: 2, b: 0, c: 0 },
    ];
    
    let mut fiber0 = Fiber::new(0, code0, 5);
    fiber0.vm.registers[0] = Value::Int(1);
    fiber0.vm.registers[1] = Value::Int(42);
    
    let mut fiber1 = Fiber::new(1, code1, 5);
    fiber1.vm.registers[1] = Value::Int(10);
    fiber1.vm.registers[3] = Value::Int(0);
    
    println!("Spawned Actor 0 (Repeater)");
    scheduler.spawn(fiber0);
    
    println!("Spawned Actor 1 (Multiplier)");
    scheduler.spawn(fiber1);
    
    scheduler.run();
    println!("✅ Demo Complete.");
}
