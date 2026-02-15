// AeroFlow Runtime - Messages (Deterministic)
// Core sequence and timestamping for DAS

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageData {
    Text(String),
    Json(String),
    Binary(Vec<u8>),
    Signal(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub sender: String,
    pub data: MessageData,
    pub logical_time: u64,
    pub sequence_id: u64,
}

impl Message {
    pub fn new(sender: String, data: MessageData, time: u64, seq: u64) -> Self {
        Self {
            sender,
            data,
            logical_time: time,
            sequence_id: seq,
        }
    }
}
