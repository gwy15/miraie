use crate::messages::{self, Message};
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub struct Bot {
    message_channel: broadcast::Sender<Message>,
}

impl crate::msg_framework::App for Bot {
    type Message = Message;
    fn event_bus(&self) -> broadcast::Sender<Self::Message> {
        self.message_channel.clone()
    }
}

impl Bot {
    pub fn new(buffer_size: usize) -> Self {
        let (tx, _) = broadcast::channel(buffer_size);
        Self {
            message_channel: tx,
        }
    }

    pub async fn start(self) {
        // FIXME
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
