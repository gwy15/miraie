use crate::{
    bot::{api::API, Bot, SplitSink, SplitStream, WebsocketStream, WsMessage},
    Result,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use serde_json::Value;
use std::any::Any;
use tokio::sync::{mpsc, oneshot};

/// Connection 负责使用 ws 协议跟 mirai 沟通。
///
/// 当接收到 mirai 通过 ws 发过来的包时，它会判断是命令的返回值（response）
/// 还是接收到的推送消息。
///
/// - 如果是接收到的推送消息，它会将其解析成 [`Message`] 并将其发布到 message_channel。
/// - 如果是命令的返回值，它会通过 syncId 找到对应的 oneshot channel 并塞进去。
///
pub struct Connection {
    pub(crate) request_receive:
        mpsc::Receiver<(Box<dyn API>, oneshot::Receiver<Box<dyn Any + Send>>)>,

    pub(crate) write: SplitSink<WebsocketStream, WsMessage>,
    pub(crate) read: SplitStream<WebsocketStream>,
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
    pub async fn run(mut self) -> Result<()> {
        loop {
            tokio::select! {
                ws_msg = self.read.next() => {
                    match ws_msg {
                        Some(ws_msg) => self.on_ws_msg(ws_msg?).await?,
                        // ws 已停止
                        None => break,
                    }
                },
                request = self.request_receive.recv() => {
                    match request {
                        Some(request) => self.on_request(request).await?,
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
        info!("received ws packet: {:?}", packet);
        // TODO:
        Ok(())
    }

    async fn on_request(
        &mut self,
        request: (Box<dyn API>, oneshot::Receiver<Box<dyn Any + Send>>),
    ) -> Result<()> {
        let (payload, ch) = request;

        unimplemented!()
    }
}
