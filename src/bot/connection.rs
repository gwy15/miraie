use crate::{
    api::ApiRequest,
    bot::{SplitSink, SplitStream, WebsocketStream, WsMessage},
    messages::Message,
    Result,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::BTreeMap, convert::TryFrom, sync::atomic::AtomicI64};
use tokio::sync::{broadcast, mpsc, oneshot};

static SYNC_ID: AtomicI64 = AtomicI64::new(10);

type RequestResponseChannel = oneshot::Sender<Value>;

/// Connection 负责使用 ws 协议跟 mirai 沟通。
///
/// 当接收到 mirai 通过 ws 发过来的包时，它会判断是命令的返回值（response）
/// 还是接收到的推送消息。
///
/// - 如果是接收到的推送消息，它会将其解析成 [`Message`] 并将其发布到 message_channel。
/// - 如果是命令的返回值，它会通过 syncId 找到对应的 oneshot channel 并塞进去。
///
pub struct Connection {
    /// 发布消息的 channel
    message_channel: broadcast::Sender<Message>,

    /// 接收 request 的通道
    request_receive: mpsc::Receiver<(Box<dyn ApiRequest>, RequestResponseChannel)>,
    /// 保存返回 request 结果的 channel
    request_callback_channel: BTreeMap<i64, RequestResponseChannel>,

    /// 向 mirai 发送消息
    write: SplitSink<WebsocketStream, WsMessage>,
    /// 从 mirai 接收消息
    read: SplitStream<WebsocketStream>,
}

/// 从 mirai 接收到的 ws 包
#[derive(Debug, Deserialize)]
struct MiraiPacket {
    #[serde(
        rename = "syncId",
        deserialize_with = "super::utils::from_string_ignore_error"
    )]
    sync_id: Option<i64>,
    data: Value,
}

impl Connection {
    pub(crate) fn new(
        ws: WebsocketStream,
        request_receive: mpsc::Receiver<(Box<dyn ApiRequest>, RequestResponseChannel)>,
        message_channel: broadcast::Sender<Message>,
    ) -> Self {
        let (write, read) = ws.split();
        Self {
            request_receive,
            write,
            read,
            request_callback_channel: BTreeMap::default(),
            message_channel,
        }
    }

    pub async fn run(mut self) -> Result<()> {
        loop {
            tokio::select! {
                ws_msg = self.read.next() => {
                    match ws_msg {
                        // 忽略错误
                        Some(ws_msg) => {
                            self.on_ws_msg(ws_msg?).await.ok();
                        },
                        // ws 已停止
                        None => break,
                    }
                },
                request = self.request_receive.recv() => {
                    match request {
                        // 忽略错误
                        Some(request) => {
                            self.on_request(request).await.ok();
                        },
                        // API 请求通道被关闭
                        None => break,
                    }

                }
            }
        }
        Ok(())
    }

    async fn on_ws_msg(&mut self, msg: WsMessage) -> Result<()> {
        let packet: MiraiPacket = match msg {
            WsMessage::Text(json) => serde_json::from_str(&json)?,
            _ => return Ok(()),
        };
        trace!("received ws packet: {:?}", packet);
        match packet.sync_id {
            // 如果是 request 的 response
            Some(sync_id) if sync_id > 0 => {
                debug!("received packet with sync_id = {}", sync_id);
                match self.request_callback_channel.remove(&sync_id) {
                    Some(ch) => {
                        if ch.send(packet.data).is_err() {
                            warn!("receiver is already closed.");
                        }
                    }
                    None => {
                        warn!("channel not found.");
                    }
                };
                return Ok(());
            }
            // 否则尝试按照消息解析
            _ => {
                let data = packet.data;
                let message = Message::try_from(data).map_err(|e| {
                    warn!("Failed to parse as message: {:?}", e);
                    e
                })?;
                debug!("message = {:?}", message);
                if self.message_channel.send(message).is_err() {
                    warn!("no active receiver to receive message.");
                }
                Ok(())
            }
        }
    }

    async fn on_request(
        &mut self,
        request: (Box<dyn ApiRequest>, RequestResponseChannel),
    ) -> Result<()> {
        let (payload, ch) = request;
        // 生成 sync_id
        let sync_id = SYNC_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let payload_s = payload.encode(sync_id);
        debug!(
            "sending request, sync_id = {}, payload = {}",
            sync_id, payload_s
        );
        // 把 request 发给 mirai
        self.write.send(WsMessage::Text(payload_s)).await?;
        // 保存返回结果的 channel
        self.request_callback_channel.insert(sync_id, ch);

        Ok(())
    }
}
