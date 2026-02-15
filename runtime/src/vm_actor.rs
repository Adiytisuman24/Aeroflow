use crate::actor::{Actor, Context};
use crate::mailbox::Message;
use crate::vm::{VM, VMContext};
use aeroflow_compiler::ir::Chunk;

pub struct VMActor {
    vm: VM,
    chunk: Chunk,
}

impl VMActor {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            vm: VM::new(),
            chunk,
        }
    }
}

impl Actor for VMActor {
    fn receive(&mut self, msg: Message, _ctx: &mut Context) {
        let vm_ctx = VMContext {
            logical_time: msg.logical_time,
            rand_seed: 0xDEADBEEF ^ msg.logical_time,
        };
        self.vm.execute(&self.chunk, &vm_ctx);
    }

    fn get_state(&self) -> String {
        serde_json::to_string(self.vm.get_globals()).unwrap_or_else(|_| "{}".to_string())
    }
}
