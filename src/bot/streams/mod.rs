use futures::{FutureExt, Stream};
use std::{pin, task};
use tokio::sync::broadcast;

use crate::messages::Message;

pub struct MessageStream {
    message_receiver: broadcast::Receiver<Message>,
}

impl MessageStream {
    pub fn new(message_receiver: broadcast::Receiver<Message>) -> Self {
        Self { message_receiver }
    }
}

impl Stream for MessageStream {
    type Item = Message;

    fn poll_next(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        let task = self.message_receiver.recv();
        match Box::pin(task).poll_unpin(cx) {
            task::Poll::Ready(r) => task::Poll::Ready(r.ok()),
            task::Poll::Pending => task::Poll::Pending,
        }
    }
}

impl Unpin for MessageStream {}
