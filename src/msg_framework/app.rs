use std::future::Future;
use tokio::sync::broadcast;

use super::{func::Func, FromRequest, Request};

/// 对一个 App 行为的抽象
///
/// App 需要提供一个 broadcast 类型的通信信道。
///
pub trait App: Sized + Clone + Send + Sync + 'static {
    type Message: Clone + Send + 'static;

    fn event_bus(&self) -> broadcast::Sender<Self::Message>;

    fn handler<F, I, Fut, O>(self, f: F) -> Self
    where
        F: Func<I, Fut>,
        Fut: Future<Output = O> + Send,
        I: FromRequest<Self> + Send + 'static,
    {
        let receiver = self.event_bus().subscribe();
        let app = self.clone();
        let task = async move {
            let mut receiver = receiver;
            loop {
                let recv = receiver.recv().await;
                match recv {
                    Ok(message) => {
                        // convert message to request
                        let request = Request::<Self> {
                            // carries message & context
                            message,
                            // carries data, e.g., database connections, etc.
                            app: app.clone(),
                        };
                        if let Some(input) = I::from_request(request) {
                            let fut = async move {
                                let _ret = (f).call(input).await;
                                // ignore O for now
                                // TODO: add a Return trait
                            };
                            tokio::spawn(fut);
                        };
                    }
                    Err(broadcast::error::RecvError::Lagged(i)) => {
                        warn!("broadcast lagged {} messages.", i);
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        };
        tokio::spawn(task);
        self
    }
}
