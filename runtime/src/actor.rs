// AeroFlow Runtime - Actor System
// Lightweight isolated execution units

use crate::mailbox::{Message, MessageData};
use crate::arena::Arena;
use std::sync::Arc;

pub type ActorId = String;

pub struct Context {
    pub actor_id: ActorId,
    pub arena: Arena,
}

pub trait Actor: Send + Sync {
    fn receive(&mut self, msg: Message, ctx: &mut Context);
    fn get_state(&self) -> String { "{}".to_string() }
}

pub struct ActorCell {
    pub id: ActorId,
    pub actor: Box<dyn Actor>,
    pub context: Context,
}

impl ActorCell {
    pub fn new(id: ActorId, actor: Box<dyn Actor>) -> Self {
        Self {
            id: id.clone(),
            actor,
            context: Context {
                actor_id: id,
                arena: Arena::new(1024 * 1024), // 1MB arena per actor
            },
        }
    }

    pub fn process_messages(&mut self) {
        // DAS handles message delivery directly now
    }

    pub fn snapshot(&self) -> Vec<u8> {
        // In a real system, we'd use bincode to serialize actor state
        println!("[Snapshot] Freezing actor {}...", self.id);
        Vec::new() // Mock snapshot
    }

    pub fn resume(id: ActorId, snapshot: Vec<u8>, actor: Box<dyn Actor>) -> Self {
        println!("[Resume] Restoring actor {} from snapshot...", id);
        Self::new(id, actor) // Mock resume
    }
}
