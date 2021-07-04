use super::{connection::Connection, QQ};
use crate::{api::ApiRequest, messages::Message, Error, Result};
use serde_json::Value;
use std::{net::SocketAddr, time::Duration};
use tokio::sync::{broadcast, mpsc, oneshot};

/// [`Bot`] 用来保存一个 bot 中的状态，如消息队列、跟连接的沟通、数据库连接等。
#[derive(Clone)]
pub struct Bot {
    /// 在 handler 内广播消息，如群消息等
    message_channel: broadcast::Sender<Message>,
    /// 处理主动消息，如发送消息等
    request_channel: mpsc::Sender<(Box<dyn ApiRequest>, oneshot::Sender<Value>)>,
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
        debug!("bot {} connected.", qq);
        let connection = super::Connection::new(ws_stream, request_rx, tx.clone());

        let bot = Bot {
            message_channel: tx,
            request_channel: request_tx,
        };

        Ok((bot, connection))
    }

    /// 对 mirai bot 发送一个请求，默认超时 10s，如果需要调整超时，使用 [`Self::request_timeout`]。
    pub async fn request<Request>(&self, request: Request) -> Result<Request::Response>
    where
        Request: crate::API + 'static,
    {
        self.request_timeout(request, Duration::from_secs(10)).await
    }

    pub async fn request_timeout<Request>(
        &self,
        request: Request,
        timeout: Duration,
    ) -> Result<Request::Response>
    where
        Request: crate::API + 'static,
    {
        let (tx, rx) = oneshot::channel::<Value>();
        let boxed_request: Box<dyn ApiRequest> = Box::new(request);
        let msg = (boxed_request, tx);
        if self.request_channel.send(msg).await.is_err() {
            return Err(Error::ConnectionClosed);
        }
        // timeout
        let rx = tokio::time::timeout(timeout, rx);
        let value = match rx.await {
            Ok(Ok(v)) => Ok(v),
            Ok(Err(_)) => Err(Error::ConnectionClosed),
            Err(_elapsed) => Err(Error::RequestTimeout),
        }?;
        // 这里拿到的是 { code, msg, data }
        let response = Request::process_response(value)?;
        Ok(response)
    }
}

impl crate::msg_framework::FromRequest<Bot> for Bot {
    fn from_request(request: crate::msg_framework::Request<Bot>) -> Option<Self> {
        Some(request.app)
    }
}
