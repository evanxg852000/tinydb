use super::{error::RaftResult, NodeState, RaftMessage, RaftNode};



#[derive(Debug)]
pub(crate) struct Follower {
    election_timer: u32,
    election_time_out: u32,
}

impl Follower {
    pub fn new() -> Self {
        Self { 
            election_timer: 0,
            election_time_out: 12,
        }
    }
}

impl NodeState for Follower {
    fn handle_message(&self, parent: &RaftNode, msg: RaftMessage) -> RaftResult<Option<Box<dyn NodeState>>> {
        parent.peers.get("k").unwrap().send(RaftMessage::StartNewElection)?;

        // if matches!(msg, RaftMessage::ResetElectionTimer) {
        //     self.election_timer = 0
        // }

        

        // parent.reset_election_timer()?;
        // parent.broadcast_to_peers(RaftMessage::AppendEntries)?;
        // parent.send_to_peer(peer_id, msg)?;
        // let answer = parent.ask_peer(peer_id, msg)?;

        // let answers = parent.ask_peers(msg, wait_for)?;

        todo!()
    }
}
