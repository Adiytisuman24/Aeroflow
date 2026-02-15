use std::collections::VecDeque;
use crate::value::Value;

pub struct Mailbox {
    queue: VecDeque<Value>,
}

impl Mailbox {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn send(&mut self, msg: Value) {
        self.queue.push_back(msg);
    }

    pub fn recv(&mut self) -> Option<Value> {
        self.queue.pop_front()
    }
}
