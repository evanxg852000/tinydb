use std::{collections::HashMap, time::Duration};

use super::{candidate::Candidate, error::{RaftError, RaftResult}, follower::Follower, heartbeat_interval, leader::Leader, log::RaftLog, timer::Timer, NodeId, NodeReceiver, NodeSender, RaftMessage, Role, RoleState};


pub(crate) struct Node {
    id: NodeId,
    pub node_tx: NodeSender,
    pub node_rx: NodeReceiver,
    pub peers: HashMap<NodeId, NodeSender>,
    pub role_state: RoleState,
    pub election_timer: Timer,

    // persisted state on all servers
    current_term: u64,
    voted_for: Option<NodeId>,
    log: Vec<u8>, //TODO: change
    
    // volatile state on all servers
    commit_index: usize, // initialized at 0 & increases monotonically
    last_applied: usize, // initialized at 0 & increases monotonically

}


impl Node {
    pub fn new(
        id: NodeId, 
        peers: Vec<(NodeId, NodeSender)>,
    ) -> Self {
        let (node_tx, node_rx) = tokio::sync::mpsc::unbounded_channel();
        let election_timer = Timer::new(heartbeat_interval(), node_tx.clone(), RaftMessage::ElectionTimeOut);
        Self { 
            id, 
            node_tx,
            node_rx,
            peers: peers.into_iter().collect(), 
            role_state: RoleState::Follower,
            election_timer,

            current_term: 0,
            voted_for: None,
            log: vec![],

            commit_index: 0,
            last_applied: 0,
        }
    }

    pub async fn run(&mut self) -> RaftResult<()> {
        loop {
            match self.role_state {
                RoleState::Follower => Follower::new(self).run(self).await?,
                RoleState::Candidate => Candidate::new(self).run(self).await?,
                RoleState::Leader => Leader::new(self).run(self).await?,
            }
        }
        // Ok(())
    }


    pub async fn stop(self) -> RaftResult<()> {
        self.election_timer.stop().await?;
        // self.stop_channel.0.send(())?;
        Ok(())
    }


    pub fn transmitter(&self) -> NodeSender {
        return  self.node_tx.clone();
    }


/*


    pub async fn tick(&mut self) -> RaftResult<()> {
        // receive one message
        let (msg, reply_sender) = self.transport.1.recv()
            .await
            .ok_or(RaftError::new("channel closed".to_string()))?;
        // process message, 
        // - may send messages to others
        // - may returns a new state to notify a transition
        let next_state_opt = self.node_state.handle_message(self, msg, reply_sender)?;
        if let Some(next_sate) = next_state_opt {
            self.node_state = next_sate;
        }

        Ok(())
    }

    pub async fn send_to_peer(&self, peer_id: &NodeId, msg: RaftMessage) -> RaftResult<()> {
        self.peers
            .get(peer_id)
            .ok_or(RaftError::new(format!("unknown peer id: {}", peer_id)))?
            .send((msg, None)).await?;
        Ok(())
    }

    pub async fn send_to_peers(&self, msg: RaftMessage) -> RaftResult<()> {
        for (_, sender) in &self.peers {
            sender.send((msg.clone(), None)).await?;
        }
        Ok(()) 
    }

    pub async fn ask_peer(&self, peer_id: &NodeId, msg: RaftMessage) -> RaftResult<RaftResponse> {
        let (reply_sender, reply_receiver) = tokio::sync::oneshot::channel::<RaftResponse>();
        self.peers
            .get(peer_id)
            .ok_or(RaftError::new(format!("unknown peer id: {}", peer_id)))?
            .send((msg, Some(reply_sender))).await?;

        let reply = reply_receiver.await?;
        Ok(reply)
    }

    pub async fn ask_peers(&self, msg: RaftMessage, num_replies_to_wait_for: usize) -> RaftResult<Vec<RaftResponse>> {
        let mut set = JoinSet::new();

        for (_, sender) in &self.peers {
            let (reply_sender, reply_receiver) = tokio::sync::oneshot::channel::<RaftResponse>();
            sender.send((msg.clone(), Some(reply_sender))).await?;
            set.spawn(async move { reply_receiver.await });
        }

        let mut responses = Vec::with_capacity(num_replies_to_wait_for);
        while let Some(result) = set.join_next().await {
            let response = result??;
            responses.push(response);
            if responses.len() == num_replies_to_wait_for {
                set.abort_all();
                break;
            }
        }

        Ok(responses)
    }

    pub fn reset_election_timer(&self) -> RaftResult<()> {
        self.election_timer.reset();
        Ok(())
    }

    */


    

}

