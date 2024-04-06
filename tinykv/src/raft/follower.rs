use super::{error::RaftResult, heartbeat_interval, timer::Timer, Node, NodeReplyReceiver, NodeReplySender, NodeState, RaftMessage, Role};
use async_trait::async_trait;


#[derive(Debug)]
pub(crate) struct Follower {
    pub election_timer: Timer,
}

impl Follower {
    pub fn new(node: &Node) -> Self {
        let election_timer = Timer::new(heartbeat_interval(), node.node_tx.clone(), RaftMessage::ElectionTimeOut);
        Self {
            election_timer,
        }
    }
}

#[async_trait]
impl Role for Follower {
    async fn run(&mut self, node: &mut Node) ->  RaftResult<()> {
        node.election_timer.restart().await?;
        while let Some(msg) = node.node_rx.recv().await  {
            if matches!(msg, RaftMessage::ElectionTimeOut) {
                //switch to candidate & start election
                // node.switch_to_candidate(); 
                println!("EVAN: switch to candidate");
                // node.election_timer.stop().await?;
                break;
            }

            if matches!(msg, RaftMessage::AppendEntries) {
                node.election_timer.reset().await?;
                println!("EVAN: reset timer");
                //TODO: 
                continue;
            }
            // handle message
        }

        println!("exist");

        Ok(())
    }
}


// impl NodeState for Follower {
//     fn handle_message(&self, parent: &RaftNode, msg: RaftMessage, reply_sender: Option<NodeReplySender>) -> RaftResult<Option<Box<dyn NodeState>>> {
//         // reset election timer if we receive heartbeat from leader
//         if matches!(msg, RaftMessage::AppendEntries) {
//             //TODO: check term is ok
//             parent.reset_election_timer()?;
//         }

//         //parent.peers.get("k").unwrap().send(RaftMessage::StartNewElection)?;

//         // parent.reset_election_timer()?;
//         // parent.broadcast_to_peers(RaftMessage::AppendEntries)?;
//         // parent.send_to_peer(peer_id, msg)?;
//         // let answer = parent.ask_peer(peer_id, msg)?;
//         // let answers = parent.ask_peers(msg, wait_for)?;

//         todo!()
//     }
// }


#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    // use tempfile::tempdir;
    use crate::raft::{error::RaftResult, node::Node};
    use crate::raft::{RaftMessage, Role, RoleState};
    use super::Follower;

    #[tokio::test]
    async fn  test_follower() -> RaftResult<()> {
        let mut node  = Node::new(1, vec![]);
        let client = node.transmitter();
        let mut follower = Follower::new(&node);

        // let mut moved_follower = follower.clone();
        tokio::spawn(async move {
            follower.run(&mut node).await
        });

        for _ in 0..5 {
            client.send(RaftMessage::AppendEntries)?;
            tokio::time::sleep(Duration::from_millis(80)).await;
        }
        

        tokio::time::sleep(Duration::from_millis(3000)).await;


        // parent.reset_election_timer()?;
        // parent.broadcast_to_peers(RaftMessage::AppendEntries)?;
        // parent.send_to_peer(peer_id, msg)?;
        // let answer = parent.ask_peer(peer_id, msg)?;
        
        // parent.reset_election_timer()?;
        // parent.broadcast_to_peers(RaftMessage::AppendEntries)?;
        // parent.send_to_peer(peer_id, msg)?;
        // let answer = parent.ask_peer(peer_id, msg)?;

        // let answers = parent.ask_peers(msg, wait_for)?;

        Ok(())
    }
    
}
