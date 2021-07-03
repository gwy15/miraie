use super::{api::API, connection::Connection, QQ};
use crate::{messages::Message, Result};
use futures::stream::StreamExt;
use std::{any::Any, net::SocketAddr};
use tokio::sync::{broadcast, mpsc, oneshot};

/// [`Bot`] 用来保存一个 bot 中的状态，如消息队列、跟连接的沟通、数据库连接等。
#[derive(Clone)]
pub struct Bot {
    /// 在 handler 内广播消息，如群消息等
    message_channel: broadcast::Sender<Message>,
    /// 处理主动消息，如发送消息等
    request_channel: mpsc::Sender<(Box<dyn API>, oneshot::Receiver<Box<dyn Any + Send>>)>,
}

impl crate::msg_framework::App for Bot {
    type Message = Message;
    fn event_bus(&self) -> broadcast::Sender<Self::Message> {
        self.message_channel.clone()
    }
}

impl Bot {
    pub async fn new(
        addr: SocketAddr,
        verify_key: impl Into<String>,
        qq: QQ,
    ) -> Result<(Self, Connection)> {
        let verify_key = verify_key.into();
        let (tx, _) = broadcast::channel(4096);

        let url = format!("ws://{}/all?verifyKey={}&qq={}", addr, verify_key, qq);
        debug!("connecting url: {}", url);

        let (request_tx, request_rx) = mpsc::channel(4096);
        let (ws_stream, _) = async_tungstenite::tokio::connect_async(url).await?;
        let (ws_write, ws_read) = ws_stream.split();
        let connection = super::Connection {
            write: ws_write,
            read: ws_read,
            request_receive: request_rx,
        };

        let bot = Bot {
            message_channel: tx,
            request_channel: request_tx,
        };

        Ok((bot, connection))
    }

    // pub async fn request<Request>(request: Request) -> Result<Request::Response>
    // where
    //     Request: crate::bot::API,
    // {
    //     unimplemented!()
    // }
}
