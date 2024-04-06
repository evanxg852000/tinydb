mod node;
mod follower;
mod candidate;
mod leader;
mod error;
mod timer;
mod log;

use std::time::Duration;

use rand::Rng;
use async_trait::async_trait;

use self::{error::RaftResult, node::Node};



/// A node ID.
type NodeId = u8;

/// A leader term.
pub type Term = u64;

/// A logical clock interval as number of ticks.
pub type Ticks = u8;

/// The interval between leader heartbeats, in milliseconds.
const HEARTBEAT_INTERVAL: u64 = 300;

/// The randomized election timeout range (min-max), in milliseconds. This is
/// randomized per node to avoid ties.
const ELECTION_TIMEOUT_RANGE: std::ops::Range<u64> = 1000..2000;

/// Generates a randomized election timeout.
fn rand_election_timeout() -> Duration {
    let delay = rand::thread_rng().gen_range(ELECTION_TIMEOUT_RANGE);
    Duration::from_millis(delay)
}

fn heartbeat_interval() -> Duration {
    Duration::from_millis(HEARTBEAT_INTERVAL)
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RoleState {
    Follower,
    Candidate,
    Leader,
}

/// A Raft role: leader, follower, or candidate.
#[async_trait]
pub trait Role {
    async fn run(&mut self, node: &mut Node) -> RaftResult<()>;
}





















type NodeReplySender = tokio::sync::oneshot::Sender<RaftMessage>;
type NodeReplyReceiver = tokio::sync::oneshot::Receiver<RaftMessage>;

type NodeSender = tokio::sync::mpsc::UnboundedSender<RaftMessage>;
type NodeReceiver = tokio::sync::mpsc::UnboundedReceiver<RaftMessage>;


#[derive(Debug, Clone)]
pub enum RaftMessage {
    ElectionTimeOut,
    HeartTimeOut,

    StartNewElection,
    IncrementTimer,
    ResetElectionTimer,

    AppendEntries,
    AppendEntriesResponse(u16),

    RequestVote,
    RequestVoteResponse(bool, u8),
}

// #[derive(Debug,Clone)]
// enum RaftResponse {
//     Void,
// }


trait NodeState { 
    fn handle_message(&self, parent: &Node, msg: RaftMessage, reply_sender: Option<NodeReplySender>) -> RaftResult<Option<Box<dyn NodeState>>>;
}





#[cfg(test)]
mod tests {

    #[test]
    fn node_setup() {
        
    }
}
