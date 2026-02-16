use wasm_bindgen::prelude::*;
use crate::scheduler::Scheduler;
use crate::actor::{ActorCell, ActorId, VMActor};
use crate::mailbox::MessageData;
use std::sync::Arc;

#[wasm_bindgen]
pub struct WasmScheduler {
    scheduler: Arc<Scheduler>,
}

#[wasm_bindgen]
impl WasmScheduler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            scheduler: Arc::new(Scheduler::new()),
        }
    }

    pub fn spawn_vm_actor(&self, id: String, bytecode: Vec<u8>) {
        let chunk: aeroflow_compiler::ir::Chunk = bincode::deserialize(&bytecode).unwrap();
        let actor = VMActor::new(chunk);
        let actor_cell = ActorCell {
            id: id.clone(),
            actor: Box::new(actor),
            context: crate::actor::Context {
                actor_id: id,
                arena: crate::Arena::new(1024 * 1024), // 1MB Arena
            },
        };
        self.scheduler.spawn(actor_cell);
    }

    pub fn send_message(&self, target: String, sender: String, data: String) {
        let msg_data = MessageData::String(data);
        self.scheduler.send(target, msg_data, sender);
    }

    pub fn step(&self) -> bool {
        self.scheduler.step()
    }

    pub fn get_logical_time(&self) -> u64 {
        *self.scheduler.logical_clock.lock()
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
