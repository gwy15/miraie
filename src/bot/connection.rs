use crate::{
    api::ApiRequest,
    bot::{SplitSink, SplitStream, WebsocketStream, WsMessage},
    messages::Message,
    Error, Result,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use serde_json::Value;
use std::sync::atomic::AtomicI64;
use tokio::sync::{broadcast, mpsc};

pub static SYNC_ID: AtomicI64 = AtomicI64::new(10);

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
    request_receive: mpsc::Receiver<(i64, Box<dyn ApiRequest>)>,
    /// 返回 request 结果的 channel
    response_channel: broadcast::Sender<(i64, Value)>,

    /// 向 mirai 发送消息
    write: SplitSink<WebsocketStream, WsMessage>,
    /// 从 mirai 接收消息
    read: SplitStream<WebsocketStream>,

    /// 用来消除掉接收到的第一个 packet 的 warning
    inited: bool,
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
        request_receive: mpsc::Receiver<(i64, Box<dyn ApiRequest>)>,
        message_channel: broadcast::Sender<Message>,
        response_channel: broadcast::Sender<(i64, Value)>,
    ) -> Self {
        let (write, read) = ws.split();
        Self {
            message_channel,

            request_receive,
            response_channel,

            write,
            read,

            inited: false,
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
            WsMessage::Close(Some(close_frame)) => {
                error!("mirai 主动关闭了连接：{}", close_frame.reason);
                return Err(Error::ConnectionClosed);
            }
            _ => return Ok(()),
        };
        // debug!("received ws packet: {:?}", packet);
        match packet.sync_id {
            // 如果是 request 的 response
            Some(sync_id) if sync_id > 0 => {
                debug!("received packet with sync_id = {}", sync_id);
                if let Err(e) = self.response_channel.send((sync_id, packet.data)) {
                    // 这里可能失败，可能超时，接收方已经关闭了，甚至根本没有发送这个请求
                    warn!("send request response failed: {:?}", e);
                }
            }
            // 否则尝试按照消息解析
            _ => {
                let data = packet.data;
                let message: Message = match serde_json::from_value(data) {
                    Ok(msg) => msg,
                    Err(e) => {
                        return if self.inited {
                            Err(e.into())
                        } else {
                            self.inited = true;
                            Ok(())
                        };
                    }
                };
                debug!("message = {:?}", message);
                if self.message_channel.send(message).is_err() {
                    warn!("no active receiver to receive message.");
                }
            }
        }
        self.inited = true;
        Ok(())
    }

    async fn on_request(&mut self, request: (i64, Box<dyn ApiRequest>)) -> Result<()> {
        let (sync_id, payload) = request;

        let cmd = payload.command();

        info!("发送 API 请求 `{}`, sync_id = {}", cmd, sync_id);
        let payload_s = payload.encode(sync_id);
        debug!(
            "sending request, sync_id = {}, payload = {}",
            sync_id, payload_s
        );
        // 把 request 发给 mirai
        self.write.send(WsMessage::Text(payload_s)).await?;

        Ok(())
    }
}
