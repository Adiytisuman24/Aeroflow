// AeroFlow Runtime - Event Tracing & Time-Travel Debugging
// Recording the deterministic path of execution

use crate::mailbox::Message;
use crate::actor::ActorId;
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEvent {
    pub logical_time: u64,
    pub actor_id: ActorId,
    pub input: Message,
    pub state_snapshot: String, // JSON representation of state
}

pub struct Tracer {
    events: Mutex<Vec<TraceEvent>>,
}

impl Tracer {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::with_capacity(10000)),
        }
    }

    pub fn record(&self, event: TraceEvent) {
        self.events.lock().unwrap().push(event);
    }

    pub fn export_json(&self) -> String {
        serde_json::to_string_pretty(&*self.events.lock().unwrap()).unwrap()
    }
}

pub static GLOBAL_TRACER: std::sync::OnceLock<Tracer> = std::sync::OnceLock::new();

pub fn get_tracer() -> &'static Tracer {
    GLOBAL_TRACER.get_or_init(Tracer::new)
}
