// AeroFlow Runtime - System Manager
// Entry point for the distributed actor system

use crate::scheduler::Scheduler;
use std::sync::OnceLock;

static SCHEDULER: OnceLock<Scheduler> = OnceLock::new();

pub fn get_scheduler() -> &'static Scheduler {
    SCHEDULER.get_or_init(Scheduler::new)
}

pub fn boot() {
    println!("🌀 AeroFlow Runtime Booting...");
    let scheduler = get_scheduler();
    
    // In a real system, we'd start worker threads here.
    // For MVP, we'll run a single-threaded loop in a background thread if needed.
    std::thread::spawn(move || {
        scheduler.run_deterministic_loop();
    });
}
