use std::{
    fmt::{Debug, Display},
    future::Future,
};
use tokio::sync::broadcast;

use super::{func::Func, FromRequest, Request};

/// 描述回调返回值的处理方式
#[async_trait]
pub trait Return<A>
where
    Self: Send,
    A: App,
{
    async fn on_return(self, request: Request<A>);
}
#[async_trait]
impl<A> Return<A> for ()
where
    A: App,
{
    async fn on_return(self, _request: Request<A>) {
        {}
    }
}

#[async_trait]
impl<A, E> Return<A> for Result<(), E>
where
    A: App,
    E: Send + Display + Debug,
{
    async fn on_return(self, _request: Request<A>) {
        if let Err(e) = self {
            warn!("handler exec failed: {}", e);
            debug!("handler backtrace: {:?}", e);
        }
    }
}

/// 对一个 App 行为的抽象
///
/// App 需要提供一个 broadcast 类型的通信信道。
///
pub trait App: Sized + Clone + Send + 'static {
    /// App 内广播的消息类型。对于 [`Bot`](crate::Bot) 来说，传递的是 [`Message`](crate::prelude::Message)。
    type Message: Clone + Send + 'static;

    /// 获取 App 内传递的消息广播通道。
    fn event_bus(&self) -> broadcast::Sender<Self::Message>;

    /// 注册一个新的消息广播处理 handler。注册之后将会永远存在，无法取消订阅。
    ///
    /// # 参数
    /// - `f`: 一个回调接口，其入参均实现了 [`FromRequest`](`crate::msg_framework::FromRequest`)，
    /// 如 [`Message`](crate::prelude::Message), [`FriendMessage`](crate::prelude::FriendMessage),
    /// [`Bot`](crate::Bot) 等。
    /// 其返回值应该是空（`()`）或 `Result<()>` 或 `Return<T>` 等，其中 T 可以被转换为 [`MessageChain`](crate::prelude::MessageChain`)。
    fn handler<F, I, Fut>(self, f: F) -> Self
    where
        F: Func<I, Fut>,
        I: FromRequest<Self> + Send + 'static,
        Fut: Future + Send + 'static,
        Fut::Output: Return<Self>,
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
                        if let Some(input) = I::from_request(&request) {
                            let fut = (f).call(input);
                            let fut = async move {
                                fut.await.on_return(request).await;
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
