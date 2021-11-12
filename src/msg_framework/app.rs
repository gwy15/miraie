use std::future::Future;
use tokio::sync::broadcast;

use super::{func::Func, FromRequest, Request};

pub trait Return {
    fn on_return(self);
}
impl Return for () {
    fn on_return(self) {}
}
impl Return for Result<(), anyhow::Error> {
    fn on_return(self) {
        if let Err(e) = self {
            warn!("handler exec failed: {:?}", e);
        }
    }
}

/// 对一个 App 行为的抽象
///
/// App 需要提供一个 broadcast 类型的通信信道。
///
pub trait App: Sized + Clone + Send + Sync + 'static {
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
    /// 其返回值应该是空（`()`）或 `anyhow::Result<()>`。
    fn handler<F, I, Fut>(self, f: F) -> Self
    where
        F: Func<I, Fut>,
        I: FromRequest<Self> + Send + 'static,
        Fut: Future + Send,
        Fut::Output: Return + Send,
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
                            let fut = async move {
                                let ret = (f).call(input).await;
                                ret.on_return();
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
