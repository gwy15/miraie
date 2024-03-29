use super::{
    connection::Connection, extensions::Extensions, KeywordCommandHandler, KeywordCommandHandlers,
    QQ,
};
use crate::{
    api::ApiRequest,
    messages::{Event, FriendMessage, GroupMessage, Message},
    msg_framework::{FromRequest, Request, Return},
    App, Error, Result,
};
use futures::{Future, Stream, StreamExt};
use parking_lot::RwLock;
use serde_json::Value;
use std::{
    future::ready,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{broadcast, mpsc};

/// [`Bot`] 代表跟一个 mirai QQ 机器人的链接。
/// 内部保存 bot 中的状态，如消息队列、跟连接的沟通、数据库连接等。
///
/// [`Bot`] 可以用来注册消息处理接口、获取机器人的消息流、主动发起调用等。
///
/// [`Bot`] 会被共享，它可能会被克隆很多份并在多个线程中被访问。它被设计为 [`Send`]。
#[derive(Clone)]
pub struct Bot {
    /// 在 handler 内广播消息，如群消息等
    message_channel: broadcast::Sender<Message>,
    /// 处理主动消息，如发送消息等
    request_channel: mpsc::Sender<(i64, Box<dyn ApiRequest>)>,
    ///
    response_channel: broadcast::Sender<(i64, Value)>,
    ///
    pub(crate) kw_command_handlers: KeywordCommandHandlers,

    pub(crate) extensions: Arc<RwLock<Extensions>>,
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

        let mut bot = Bot {
            message_channel: tx,
            request_channel: request_tx,
            response_channel: response_tx,
            kw_command_handlers: KeywordCommandHandlers::new(),
            extensions: Arc::new(RwLock::new(Extensions::new())),
        };

        // 注册关键词 handler
        bot = bot.handler(Self::process_keyword_command);

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

    /// 通过 **前缀关键词** 来注册一个回调。目前不可取消。
    ///
    /// 只有前缀完全匹配的消息才会进行匹配。
    ///
    /// `command` 方法比 `handler` 的方法相对而言实现更加高效，如果可能，尽量使用 `command` 来注册。
    ///
    /// # Example
    /// ```no_run
    /// # use miraie::prelude::*;
    /// # use anyhow::*;
    /// # tokio_test::block_on(async {
    /// # let (bot, _) = Bot::new("127.0.0.1", "secret", QQ(123456)).await.unwrap();
    /// let bot = bot.command("在吗", |msg: GroupMessage, bot: Bot| async move {
    ///     msg.reply("嘎哈", &bot).await?;
    ///     Result::<(), Error>::Ok(())
    /// });
    ///
    /// // 也可以直接返回 `String` 或者 `Result<String>` 之类的
    /// let bot = bot.command("在吗", |_: GroupMessage| async { "嘎哈" });
    /// # });
    /// ```
    pub fn command<F, I, Fut>(self, command: impl Into<String>, handler: F) -> Self
    where
        F: crate::msg_framework::Func<I, Fut>,
        I: Send + 'static + FromRequest<Bot>,
        Fut: Future + Send + 'static,
        Fut::Output: Return<Bot>,
    {
        let mut handlers = self.kw_command_handlers.0.write();
        let pair = (command.into(), KeywordCommandHandler::new(handler));
        handlers.push(pair);
        std::mem::drop(handlers);

        self
    }

    async fn process_keyword_command(msg: Message, handlers: KeywordCommandHandlers, bot: Bot) {
        debug!("processing keyword command");
        let msg_s = match &msg {
            Message::Friend(f) => f.message.to_string(),
            Message::Group(g) => g.message.to_string(),
            Message::Temp(t) => t.message.to_string(),
            Message::Stranger(s) => s.message.to_string(),
            Message::Event(_ev) => {
                return;
            }
        };
        let handlers = handlers.0.read();
        // TODO: 这里可以用前缀树
        for (command, handler) in handlers.iter() {
            if msg_s.starts_with(command) {
                let app = bot.clone();
                let message = msg.clone();
                let handler = handler.clone();
                let fut = async move {
                    handler.handle(Request { app, message }).await;
                };
                tokio::spawn(fut);
            }
        }
    }

    /// 可以使用 `bot_data` 注册配置/数据库连接池等，并使用 [`crate::Data`] 进行提取。
    ///
    /// ```rust,ignore
    /// bot
    ///     .bot_data(Data::new(Config {
    ///         name: "A-SOUL!".to_string(),
    ///     }))
    ///     .command("配置", |config: Data<Config>| async move {
    ///         // 会在任何聊天中回复【A-SOUL!】
    ///         config.name.clone()
    ///     });
    /// ```
    pub fn bot_data<U: Send + Sync + 'static>(self, ext: U) -> Self {
        self.extensions.write().insert(ext);
        self
    }
}

impl crate::msg_framework::FromRequest<Bot> for Bot {
    fn from_request(request: &crate::msg_framework::Request<Bot>) -> Option<Self> {
        Some(request.app.clone())
    }
}
