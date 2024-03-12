use std::{collections::HashMap, sync::mpsc::channel};


use super::{error::{RaftError, RaftResult}, follower::Follower, NodeId, NodeReceiver, NodeSender, NodeState, RaftMessage};


pub(crate) struct RaftNode {
    id: NodeId,
    transport: (NodeSender, NodeReceiver),
    pub peers: HashMap<NodeId, NodeSender>,
    node_state: Box<dyn NodeState>,
}



impl RaftNode {
    pub fn new(id: String, peers: Vec<(NodeId, NodeSender)>) -> Self {
        Self { 
            id, 
            transport: channel(),
            peers: peers.into_iter().collect(), 
            node_state: Box::new(Follower::new()),
        }
    }

    pub fn client(&self) -> NodeSender {
        return  self.transport.0.clone();
    }

    pub fn step(&mut self) -> RaftResult<()> {
        // receive one message
        let msg = self.transport.1.recv()?;
        // process message, 
        // - may send messages to others
        // - may returns a new state to notify a transition
        let next_state_opt = self.node_state.handle_message(self, msg)?;
        if let Some(next_sate) = next_state_opt {
            self.node_state = next_sate;
        }

        Ok(())
    }


    pub fn broadcast_to_peers(&self, msg: RaftMessage) -> RaftResult<()> {
        for (_, sender) in &self.peers {
            sender.send(msg.clone())?;
        }
        Ok(()) 
    }

    pub fn send_to_peer(&self, peer_id: &NodeId, msg: RaftMessage) -> RaftResult<()> {
        self.peers
            .get(peer_id)
            .ok_or(RaftError::new(format!("unknown peer id: {}", peer_id)))?
            .send(msg)?;
        Ok(())
    }

    pub fn reset_election_timer(&self) -> RaftResult<()> {
        // self.election_timer.reset()
        Ok(())
    }
    

}
