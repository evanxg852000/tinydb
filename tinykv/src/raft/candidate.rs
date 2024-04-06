use std::collections::{HashMap, HashSet};

use super::{error::{RaftError, RaftResult}, heartbeat_interval, timer::Timer, Node, NodeId, NodeReplyReceiver, NodeReplySender, NodeSender, NodeState, RaftMessage, Role};
use async_trait::async_trait;
use futures::stream::FuturesUnordered;


#[derive(Debug)]
pub(crate) struct Candidate {
    votes_received: HashMap<NodeId, bool>, 
    election_timer: Timer,
}

impl Candidate {
    pub fn new(node: &Node) -> Self {
        let election_timer = Timer::new(heartbeat_interval(), node.node_tx.clone(), RaftMessage::ElectionTimeOut);
        Self {
            election_timer,
            votes_received: HashMap::new(),
        }
    }

    pub async fn send_requests_vote(&mut self, peers: Vec<NodeSender>) ->  RaftResult<()> {
        self.votes_received.clear();
        let tasks = peers.into_iter().map(|peer| tokio::spawn(async move {
            peer.send(RaftMessage::RequestVote)
                .map_err(RaftError::from)
        })).collect::<FuturesUnordered<_>>();

        futures::future::join_all(tasks)
            .await
            .into_iter().collect::<Result<Vec<_>, _>>()?
            .into_iter().collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }
}

#[async_trait]
impl Role for Candidate {
    async fn run(&mut self, node: &mut Node) ->  RaftResult<()> {
        node.election_timer.restart().await?;
        let peers: Vec<_> = node.peers.values().cloned().collect();
        self.send_requests_vote(peers.clone()).await?;
        while let Some(msg) = node.node_rx.recv().await  {
            //collect votes & messages here
            match msg {
                RaftMessage::RequestVoteResponse(granted, node_id) => {
                    self.votes_received.insert(node_id, granted);
                    
                },
                RaftMessage::ElectionTimeOut => {
                    //TODO: increment term
                    self.send_requests_vote(peers.clone()).await?; //start new election
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
    use super::Candidate;

    #[tokio::test]
    async fn  test_candidate() -> RaftResult<()> {
        Ok(())
    }
    
}
