// AeroFlow Runtime - Supervisor
// Erlang-grade reliability and failure isolation

use crate::actor::{ActorId, ActorCell};
use crate::scheduler::Scheduler;

pub enum Strategy {
    OneForOne,
    AllForOne,
}

pub struct Supervisor {
    children: Vec<ActorId>,
    strategy: Strategy,
}

impl Supervisor {
    pub fn new(strategy: Strategy) -> Self {
        Self {
            children: Vec::new(),
            strategy,
        }
    }

    pub fn handle_failure(&self, actor_id: ActorId, scheduler: &Scheduler) {
        println!("⚠️ Supervisor: Actor {} failed. Applying recovery strategy...", actor_id);
        match self.strategy {
            Strategy::OneForOne => {
                println!("  Restarting actor {}...", actor_id);
                // Implementation would fetch the actor definition and re-spawn
            }
            Strategy::AllForOne => {
                println!("  Restarting all child actors...");
            }
        }
    }
}
