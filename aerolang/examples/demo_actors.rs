fn main() {
    let mut scheduler = Scheduler::new();
    
    // Spawn a producer actor (ID 0)
    let producer = Fiber {
        id: 0,
        vm: VM::new(4),
        code: vec![
            Instr { op: Op::LoadConst, a: 0, b: 1 }, // Reg[0] = 1 (Target ID)
            Instr { op: Op::LoadConst, a: 1, b: 42 }, // Reg[1] = 42 (Message)
            Instr { op: Op::Send, a: 0, b: 1 },      // Send Reg[1] to Actor(Reg[0])
            Instr { op: Op::Return, a: 0, b: 0 },
        ],
        mailbox: Mailbox::new(),
        finished: false,
    };
    
    // Spawn a consumer actor (ID 1)
    let consumer = Fiber {
        id: 1,
        vm: VM::new(4),
        code: vec![
            Instr { op: Op::Recv, a: 0, b: 0 },      // Receive into Reg[0]
            // Process (Add 10)
            Instr { op: Op::LoadConst, a: 1, b: 10 }, 
            Instr { op: Op::Add, a: 2, b: 0, c: 1 },
            // Print result? (Return for now)
            Instr { op: Op::Return, a: 2, b: 0 },
        ],
        mailbox: Mailbox::new(),
        finished: false,
    };
    
    scheduler.spawn(producer);
    scheduler.spawn(consumer);
    
    scheduler.run();
}
