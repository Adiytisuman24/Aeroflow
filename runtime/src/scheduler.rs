// AeroFlow Runtime - Deterministic Actor Scheduler (DAS)
// Concurrency without nondeterminism

use crate::actor::{ActorCell, ActorId};
use crate::mailbox::Message;
use crate::vm::{VM, VMContext};
use std::collections::{HashMap, BinaryHeap};
use std::sync::Arc;
use parking_lot::Mutex;
use std::cmp::Ordering;

#[derive(Debug)]
struct ScheduledMessage {
    message: Message,
    target: ActorId,
}

impl PartialEq for ScheduledMessage {
    fn eq(&self, other: &Self) -> bool {
        self.message.logical_time == other.message.logical_time &&
        self.message.sequence_id == other.message.sequence_id &&
        self.target == other.target
    }
}

impl Eq for ScheduledMessage {}

impl Ord for ScheduledMessage {
    fn cmp(&self, other: &Self) -> Ordering {
        other.message.logical_time.cmp(&self.message.logical_time)
            .then_with(|| other.target.cmp(&self.target))
            .then_with(|| other.message.sequence_id.cmp(&self.message.sequence_id))
    }
}

impl PartialOrd for ScheduledMessage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Scheduler {
    actors: Mutex<HashMap<ActorId, ActorCell>>,
    queue: Mutex<BinaryHeap<ScheduledMessage>>,
    pub(crate) logical_clock: Mutex<u64>,
    pub(crate) sequence_counter: Mutex<u64>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            actors: Mutex::new(HashMap::new()),
            queue: Mutex::new(BinaryHeap::new()),
            logical_clock: Mutex::new(0),
            sequence_counter: Mutex::new(0),
        }
    }

    pub fn spawn(&self, actor_cell: ActorCell) {
        let id = actor_cell.id.clone();
        self.actors.lock().insert(id, actor_cell);
    }

    pub fn send(&self, target: ActorId, message_data: crate::mailbox::MessageData, sender: ActorId) {
        let mut clock = self.logical_clock.lock();
        let mut seq = self.sequence_counter.lock();
        
        *clock += 1;
        *seq += 1;

        let msg = Message::new(sender, message_data, *clock, *seq);
        self.queue.lock().push(ScheduledMessage { message: msg, target });
    }

    pub fn send_with_time(&self, target: ActorId, message_data: crate::mailbox::MessageData, sender: ActorId, time: u64, seq: u64) {
        let mut clock = self.logical_clock.lock();
        if time > *clock {
            *clock = time;
        }

        let msg = Message::new(sender, message_data, time, seq);
        self.queue.lock().push(ScheduledMessage { message: msg, target });
    }

    pub fn step(&self) -> bool {
        let scheduled = {
            let mut q = self.queue.lock();
            q.pop()
        };

        if let Some(s) = scheduled {
            let mut actors = self.actors.lock();
            if let Some(actor_cell) = actors.get_mut(&s.target) {
                // In DAS, time is explicit
                let ctx = VMContext {
                    logical_time: s.message.logical_time,
                    rand_seed: 0xDEADBEEF ^ s.message.logical_time,
                };
                
                // Record event (Step 2: Tracing)
                let trace_state = actor_cell.actor.get_state();
                let tracer = crate::get_tracer();
                tracer.record(crate::trace::TraceEvent {
                    logical_time: s.message.logical_time,
                    actor_id: s.target.clone(),
                    input: s.message.clone(),
                    state_snapshot: trace_state,
                });

                actor_cell.actor.receive(s.message, &mut actor_cell.context);
                return true;
            }
        }
        false
    }

    pub fn run_deterministic_loop(&self) {
        println!("🌀 DAS: Starting Deterministic Scheduler Loop...");
        loop {
            if !self.step() {
                std::thread::sleep(std::time::Duration::from_micros(10));
            }
        }
    }
}
