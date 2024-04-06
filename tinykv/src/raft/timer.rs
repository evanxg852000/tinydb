use std::time::Duration;

use tokio::{sync::mpsc::{channel, Sender}, task::JoinHandle, time};

use super::{error::{RaftError, RaftResult}, NodeSender, RaftMessage};

enum TimerEvent {
    ResetClock,
    StopClock,
}


#[derive(Debug)]
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
        Self{
            timeout, 
            sender,
            timeout_message,
            timer_event_sender: None,
            clock_task_handle: None,
        }
    }

    #[cfg(test)]
    pub fn dummy(sender: NodeSender, timeout_message: RaftMessage) -> Self {
        Self::new(Duration::ZERO, sender, timeout_message)
    }

    pub fn start(&mut self) {
        if self.timeout.is_zero()  {
            return; // zero timeout is for dummy clock, no need to setup a task.
        }

        let event_sender = self.sender.clone();
        let (timer_event_sender, mut timer_event_receiver) = channel::<TimerEvent>(10);
        let moved_timeout_message = self.timeout_message.clone();
        let timeout = self.timeout.clone();
        let handle = tokio::spawn(async move {
            let mut interval = time::interval(timeout);
            // Get rid of first immediate tick
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

        self.clock_task_handle = Some(handle);
        self.timer_event_sender = Some(timer_event_sender);
    }

    pub async fn restart(&mut self) -> RaftResult<()> { 
        if self.clock_task_handle.is_none() {
            self.start();
            return Ok(())
        }
        self.reset().await
    }

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

    pub fn timeout(&self)  -> RaftResult<()> {
        self.sender.send(self.timeout_message.clone())
            .map_err(RaftError::from)
    }

}
