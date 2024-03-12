use std::time::Duration;

use tokio::{sync::mpsc::{channel, Sender}, task::JoinHandle, time};

use super::{error::RaftResult, NodeSender, RaftMessage};

enum TimerEvent {
    ResetClock,
    StopClock,
}


pub(crate) struct Timer {
    // clock timeout value
    timeout: Duration,
    // channel on which to fire the timeout event
    sender: NodeSender,
    // event fired (message sent) when the timer has timed-out.
    timeout_message: RaftMessage,
    // reset channel
    timer_event_sender: Option<Sender<TimerEvent>>,

    clock_task_handle: Option<JoinHandle<()>>,
}

impl Timer {

    pub fn new(timeout: Duration, sender: NodeSender, timeout_message: RaftMessage) -> Self {
        // zero timeout is for dummy clock, no need to setup a task.
        let (clock_task_handle, timer_event_sender) = if !timeout.is_zero() {
            let event_sender = sender.clone();
            let (timer_event_sender, mut timer_event_receiver) = channel::<TimerEvent>(10);
            let moved_timeout_message = timeout_message.clone();
            let handle = tokio::spawn(async move {
                let mut interval = time::interval(timeout);
                // get rid of first immediate tick
                interval.tick().await;
    
                loop {
                    tokio::select! {
                        event_opt = timer_event_receiver.recv() => {
                            if let Some(event) = event_opt {
                                match event {
                                    TimerEvent::ResetClock => interval.reset(),
                                    TimerEvent::StopClock => break,
                                }
                            }
                        }
                        _ = interval.tick() => {
                            event_sender.send(moved_timeout_message.clone()).unwrap();
                        }
                    }
                }
            });
            (Some(handle), Some(timer_event_sender))
        } else {
            (None, None)
        };
        
        Self{
            timeout, 
            sender,
            timeout_message,
            timer_event_sender,
            clock_task_handle,
        }
    }

    #[cfg(test)]
    pub fn dummy(sender: NodeSender, timeout_message: RaftMessage) -> Self {
        Self::new(Duration::ZERO, sender, timeout_message)
    }

    // pub fn start(&self) {
    //     let timeout = self.timeout.clone();
    //     let timer_event_receiver = self.timer_event_channel.1.clone();
    //     let event_sender = self.sender.clone();
    //     let timeout_message = self.timeout_message.clone();
    //     self.clock_task_handle  = 
    // }

    pub async fn reset(&self) -> RaftResult<()> {
        if let Some(sender) = &self.timer_event_sender {
            sender.send(TimerEvent::ResetClock).await?;
        }
        Ok(())
    }

    pub async fn stop(self) -> RaftResult<()> {
        if let Some(sender) = &self.timer_event_sender {
            sender.send(TimerEvent::StopClock).await?;
        }
        if let Some(handle) = self.clock_task_handle {
            handle.await?;
        }
        Ok(())
    }

    pub fn timeout(&self) {
        self.sender.send(self.timeout_message.clone()).unwrap()
    }

}
