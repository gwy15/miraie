use futures::Stream;

#[pin_project::pin_project]
pub struct MessageStream<Message> {
    #[pin]
    inner: Box<dyn Stream<Item = Message> + 'static + Unpin + Send>,
}

impl<M> MessageStream<M> {
    pub fn new(stream: impl Stream<Item = M> + 'static + Unpin + Send) -> Self {
        Self {
            inner: Box::new(stream),
        }
    }
}

impl<M> Stream for MessageStream<M> {
    type Item = M;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project().inner.poll_next(cx)
    }
}
