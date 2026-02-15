// AeroFlow Module (.afm) Binary Format
// Designed for memory mapping and zero-copy loading

use serde::{Serialize, Deserialize};
use crate::ir::Chunk;

pub const AFM_MAGIC: [u8; 4] = *b"AFM1";

#[repr(C)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AfmHeader {
    pub magic: [u8; 4],
    pub version: u16,
    pub arch: u16,
    pub flags: u32,
    pub metadata_offset: u32,
    pub metadata_len: u32,
    pub ir_offset: u32,
    pub ir_len: u32,
    pub signature_offset: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AfmMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AfmModule {
    pub metadata: AfmMetadata,
    pub chunk: Chunk,
}

impl AfmModule {
    pub fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let bytes = bincode::serialize(self)?;
        Ok(bytes)
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        let module = bincode::deserialize(bytes)?;
        Ok(module)
    }
}
