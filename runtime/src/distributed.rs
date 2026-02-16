use crate::scheduler::Scheduler;
use crate::mailbox::{Message, MessageData};
use crate::actor::ActorId;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use parking_lot::Mutex;

pub type NodeId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedMessage {
    pub from_node: NodeId,
    pub target_actor: ActorId,
    pub sender_actor: ActorId,
    pub logical_time: u64,
    pub sequence_id: u64,
    pub data: MessageData,
}

pub struct DistributedScheduler {
    pub node_id: NodeId,
    inner: Arc<Scheduler>,
    peers: Mutex<Vec<NodeId>>, // In a real system, these would be network connections
}

impl DistributedScheduler {
    pub fn new(node_id: NodeId, inner: Arc<Scheduler>) -> Self {
        Self {
            node_id,
            inner,
            peers: Mutex::new(Vec::new()),
        }
    }

    pub fn add_peer(&self, peer_id: NodeId) {
        self.peers.lock().push(peer_id);
    }

    /// Broadcast a message to all nodes in the cluster (Deterministic Broadcast)
    pub fn broadcast(&self, target_actor: ActorId, sender_actor: ActorId, data: MessageData) {
        let logical_time = {
            let mut clock = self.inner.logical_clock.lock();
            *clock += 1;
            *clock
        };

        let seq = {
            let mut s = self.inner.sequence_counter.lock();
            *s += 1;
            *s
        };

        let dist_msg = DistributedMessage {
            from_node: self.node_id.clone(),
            target_actor: target_actor.clone(),
            sender_actor: sender_actor.clone(),
            logical_time,
            sequence_id: seq,
            data,
        };

        // In a real distributed system, we would send this over TCP/UDP.
        // For the spec, we log it and simulate delivery.
        println!("🌐 [D-DAS] Node {} Broadcasting Message at T={}...", self.node_id, logical_time);
        
        // Self-delivery
        self.receive_remote(dist_msg);
    }

    /// Receive a message from another node and insert it into the local DAS queue
    pub fn receive_remote(&self, msg: DistributedMessage) {
        // Enforce determinism: The message is placed into the DAS priority queue.
        // Since the queue is ordered by (logical_time, target, seq), 
        // all nodes with the same messages will process them in the exact same order.
        
        // This is the heart of AeroFlow's multi-node determinism.
        self.inner.send_with_time(
            msg.target_actor, 
            msg.data, 
            msg.sender_actor, 
            msg.logical_time, 
            msg.sequence_id
        );
    }
}
