use super::{connection::Connection, QQ};
use crate::{
    api::ApiRequest,
    messages::{Event, FriendMessage, GroupMessage, Message},
    Error, Result,
};
use futures::{Stream, StreamExt};
use serde_json::Value;
use std::{
    future::ready,
    time::{Duration, Instant},
};
use tokio::sync::{broadcast, mpsc};

/// [`Bot`] 代表跟一个 mirai QQ 机器人的链接。
/// 内部保存 bot 中的状态，如消息队列、跟连接的沟通、数据库连接等。
///
/// [`Bot`] 可以用来注册消息处理接口、获取机器人的消息流、主动发起调用等。
#[derive(Clone)]
pub struct Bot {
    /// 在 handler 内广播消息，如群消息等
    message_channel: broadcast::Sender<Message>,
    /// 处理主动消息，如发送消息等
    request_channel: mpsc::Sender<(i64, Box<dyn ApiRequest>)>,
    ///
    response_channel: broadcast::Sender<(i64, Value)>,
}

impl crate::msg_framework::App for Bot {
    type Message = Message;
    fn event_bus(&self) -> broadcast::Sender<Self::Message> {
        self.message_channel.clone()
    }
}

impl Bot {
    /// 建立一个 bot，会在这里建立跟服务器的 websocket 连接。
    ///
    /// # 参数
    /// - addr: mirai 服务器的地址，需要开启 websocket 的 adapter
    /// - verify_key: 鉴权 key
    /// - qq：机器人的 qq 号
    ///
    /// # Example
    /// ```no_run
    /// # use miraie::prelude::*;
    /// # tokio_test::block_on(async {
    /// let (bot, conn) = Bot::new(
    ///     "127.0.0.1:8080",
    ///     "verify_key",
    ///     QQ(12345)
    /// ).await?;
    ///
    /// async fn handler(msg: Message) {}
    ///
    /// bot.handler(handler);
    /// conn.run().await?;
    /// # Result::<(), miraie::Error>::Ok(()) });
    /// ```
    pub async fn new(
        addr: impl AsRef<str>,
        verify_key: impl Into<String>,
        qq: QQ,
    ) -> Result<(Self, Connection)> {
        let verify_key = verify_key.into();
        let (tx, _) = broadcast::channel(4096);

        let url = format!(
            "ws://{}/all?verifyKey={}&qq={}",
            addr.as_ref(),
            verify_key,
            qq
        );
        debug!("connecting url: {}", url);

        let (ws_stream, _) = async_tungstenite::tokio::connect_async(url).await?;
        let (request_tx, request_rx) = mpsc::channel(4096);
        let (response_tx, _) = broadcast::channel(4096);
        debug!("bot {} connected.", qq);
        let connection =
            super::Connection::new(ws_stream, request_rx, tx.clone(), response_tx.clone());

        let bot = Bot {
            message_channel: tx,
            request_channel: request_tx,
            response_channel: response_tx,
        };

        Ok((bot, connection))
    }

    /// 对 mirai bot 发送一个请求，默认超时 10s，如果需要调整超时，使用 [`Self::request_timeout`]。
    pub async fn request<Request>(&self, request: Request) -> Result<Request::Response>
    where
        Request: crate::Api + 'static,
    {
        self.request_timeout(request, Duration::from_secs(10)).await
    }

    /// 对 mirai bot 发送一个请求，带有自定义超时
    pub async fn request_timeout<Request>(
        &self,
        request: Request,
        timeout: Duration,
    ) -> Result<Request::Response>
    where
        Request: crate::Api + 'static,
    {
        let sync_id = super::connection::SYNC_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let cmd = request.command();
        let boxed_request: Box<dyn ApiRequest> = Box::new(request);
        let t = Instant::now();
        if self
            .request_channel
            .send((sync_id, boxed_request))
            .await
            .is_err()
        {
            return Err(Error::ConnectionClosed);
        }

        let mut response_rx = self.response_channel.subscribe();
        let wait_task = async move {
            loop {
                match response_rx.recv().await {
                    Ok((r_sync_id, resp)) => {
                        if r_sync_id == sync_id {
                            break Result::Ok(resp);
                        }
                    }
                    Err(e) => {
                        warn!("recv error: {:?}", e);
                        return Err(Error::ConnectionClosed);
                    }
                }
            }
        };
        // timeout
        let rx = tokio::time::timeout(timeout, wait_task);
        let value = match rx.await {
            Ok(Ok(v)) => Ok(v),
            Ok(Err(_)) => Err(Error::ConnectionClosed),
            Err(_elapsed) => Err(Error::RequestTimeout),
        }?;
        // 这里拿到的是 { code, msg, data? }
        let response = Request::process_response(value)?;
        info!(
            "API 请求 `{}` 成功，耗时 {} ms",
            cmd,
            t.elapsed().as_millis()
        );
        Ok(response)
    }

    /// 获取一个全部消息的 stream
    pub fn messages(&self) -> impl Stream<Item = Message> + Unpin + Send {
        let mut ch = self.message_channel.subscribe();

        let s = async_stream::stream! {
            while let Ok(msg) = ch.recv().await {
                yield msg;
            }
        };
        Box::pin(s)
    }

    /// 获取一个全部群聊消息的 stream
    pub fn group_messages(&self) -> impl Stream<Item = GroupMessage> + Unpin + Send {
        self.messages().filter_map(|msg| {
            ready(match msg {
                Message::Group(msg) => Some(msg),
                _ => None,
            })
        })
    }

    /// 获取一个私聊消息的 stream
    pub fn friend_messages(&self) -> impl Stream<Item = FriendMessage> + Unpin + Send {
        self.messages().filter_map(|msg| {
            ready(match msg {
                Message::Friend(msg) => Some(msg),
                _ => None,
            })
        })
    }

    /// 获取一个事件的 stream
    pub fn events(&self) -> impl Stream<Item = Event> + Unpin + Send {
        self.messages().filter_map(|msg| {
            ready(match msg {
                Message::Event(evt) => Some(evt),
                _ => None,
            })
        })
    }
}

impl crate::msg_framework::FromRequest<Bot> for Bot {
    fn from_request(request: crate::msg_framework::Request<Bot>) -> Option<Self> {
        Some(request.app)
    }
}
