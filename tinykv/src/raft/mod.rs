mod node;
mod follower;
mod error;
mod timer;
mod log;

use self::{error::RaftResult, node::RaftNode};

type NodeId = String;

type NodeSender = std::sync::mpsc::Sender<RaftMessage>;
type NodeReceiver = std::sync::mpsc::Receiver<RaftMessage>;

#[derive(Debug, Clone)]
enum RaftMessage {
    StartNewElection,
    ResetElectionTimer,

    AppendEntries,
}


trait NodeState { 
    fn handle_message(&self, parent: &RaftNode, msg: RaftMessage) -> RaftResult<Option<Box<dyn NodeState>>>;
}


#[cfg(test)]
mod tests {

    #[test]
    fn node_setup() {
        
    }
}
