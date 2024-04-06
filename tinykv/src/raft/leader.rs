use std::collections::{HashMap};

use super::{error::{RaftError, RaftResult}, heartbeat_interval, timer::Timer, Node, NodeId, NodeReplyReceiver, NodeReplySender, NodeSender, NodeState, RaftMessage, Role};
use async_trait::async_trait;


#[derive(Debug)]
pub(crate) struct Leader {
    pub heartbeat_timer: Timer,

    //volatile state on leader

    // For each server, index of the next log entry to send to that server.
    // initialized to leader last log index + 1
    next_index: HashMap<NodeId, usize>,

    // For each server, index of the highest log entry known to replicated on server.
    // initialized to 0, increases monotonically
    match_index: HashMap<NodeId, usize>,
}

impl Leader {
    pub fn new(node: &Node) -> Self {
        let heartbeat_timer = Timer::new(heartbeat_interval(), node.node_tx.clone(), RaftMessage::HeartTimeOut);
        
        let next_index = HashMap::new();
        let match_index = HashMap::new();
        
        Self {
            heartbeat_timer,
            next_index,
            match_index,
        }
    }

    pub async fn send_heartbeat(&mut self, peers: Vec<NodeSender>) ->  RaftResult<()> {
        Ok(())
    }
    
}

#[async_trait]
impl Role for Leader {
    async fn run(&mut self, node: &mut Node) ->  RaftResult<()> {
        self.heartbeat_timer.start();
        let peers: Vec<_> = node.peers.values().cloned().collect();
        while let Some(msg) = node.node_rx.recv().await  {
            //collect votes & messages here
            match msg {
                RaftMessage::HeartTimeOut => {
                    //TODO: increment term
                    self.send_heartbeat(peers.clone()).await?; //start new election
                },
                RaftMessage::AppendEntries => {
                    //check if higher term, then switch to follower & exit
                    println!("EVAN: switch to follower");
                    break;
                },
                _ => (),
            }
            
        }

        println!("exist");

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use crate::raft::{error::RaftResult, node::Node};
    use crate::raft::{RaftMessage, Role, RoleState};
    use super::Leader;

    #[tokio::test]
    async fn  test_leader() -> RaftResult<()> {
        Ok(())
    }
    
}
