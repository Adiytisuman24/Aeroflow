use std::collections::VecDeque;
use std::collections::HashMap;
use crate::fiber::Fiber;
use crate::step_executor::{step, SchedulerContext};
use crate::value::Value;

pub struct Scheduler {
    actors: HashMap<usize, Fiber>,
    run_queue: VecDeque<usize>,
    next_id: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            actors: HashMap::new(),
            run_queue: VecDeque::new(),
            next_id: 0,
        }
    }

    pub fn spawn(&mut self, mut fiber: Fiber) -> usize {
        let id = self.next_id;
        fiber.id = id;
        self.next_id += 1;
        self.actors.insert(id, fiber);
        self.run_queue.push_back(id);
        id
    }

    pub fn send(&mut self, target: usize, msg: Value) {
        if let Some(actor) = self.actors.get_mut(&target) {
            actor.mailbox.send(msg);
        }
    }

    pub fn run(&mut self) {
        let mut loop_count = 0;
        
        while let Some(id) = self.run_queue.pop_front() {
            if let Some(mut actor) = self.actors.remove(&id) {
                let mut ctx = Context::new(id);
                
                step(&mut actor, 50, &mut ctx);

                // Process messages
                for (t, m) in ctx.msgs {
                    if t == id {
                        actor.mailbox.send(m);
                    } else if let Some(target) = self.actors.get_mut(&t) {
                        target.mailbox.send(m);
                    } else {
                        // Actor might be in queue but removed temporarily - wait.
                        // Can't solve this with current ownership model easily.
                        // For MVP: Messages might be lost if target is executing concurrently (not possible single thread)
                        // Ah, if target is ID, it is self. Target IS ID.
                        // If target IS current "actor", we just send to mailbox.
                        // If target is ANOTHER actor, it IS in the map because only ONE is popped.
                        // Correct!
                    }
                }

                if !actor.finished {
                    self.actors.insert(id, actor);
                    self.run_queue.push_back(id);
                } else {
                    println!("Actor {} finished.", id);
                }
            }
            loop_count += 1;
            if loop_count > 10_000 { break; } 
        }
    }
}

// Private context implementation
struct Context {
    source_id: usize,
    msgs: Vec<(usize, Value)>,
}

impl Context {
    fn new(id: usize) -> Self {
        Context { source_id: id, msgs: vec![] }
    }
}

impl SchedulerContext for Context {
    fn send_message(&mut self, target_id: usize, msg: Value) {
        // println!("Actor {} sending to {}", self.source_id, target_id);
        self.msgs.push((target_id, msg));
    }
}
