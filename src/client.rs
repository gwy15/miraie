use crate::{Error, MessageBlock, Result, QQ};
use parking_lot::RwLock;
use request::{Client as RClient, RequestBuilder};
use std::{collections::HashMap, sync::Arc};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, WebSocketStream};

#[derive(Debug, Clone)]
pub struct Client {
    client: RClient,
    base_url: Arc<String>,
    sessions: Arc<RwLock<HashMap<QQ, String>>>,
}

#[allow(unused)]
impl Client {
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = RClient::new();
        let base_url = Arc::new(base_url.into());

        Self {
            client,
            base_url,
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    pub fn post(&self, path: &str) -> RequestBuilder {
        self.client.post(&self.url(path))
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        self.client.get(&self.url(path))
    }

    pub fn bound_accounts(&self) -> Vec<(QQ, String)> {
        let s = self.sessions.read();
        s.iter().map(|(k, v)| (*k, v.clone())).collect()
    }

    fn session_key(&self, qq: QQ) -> Result<String> {
        self.sessions
            .read()
            .get(&qq)
            .ok_or(Error::NotBounded(qq))
            .map(|s| s.clone())
    }

    async fn auth(&self, auth_key: String) -> Result<String> {
        info!("POST /auth");
        def_req! {
            #[serde(rename = "authKey")]
            auth_key: String,
        }
        def_resp! {
            session: String,
        }
        let r: Response = self
            .post("/auth")
            .json(&Request { auth_key })
            .send()
            .await?
            .json()
            .await?;
        r.ok()?;
        info!("auth success");
        Ok(r.session)
    }

    async fn verify(&self, session_key: String, qq: QQ) -> Result<()> {
        info!("verify qq {}", qq);
        def_req! {
            #[serde(rename = "sessionKey")]
            session_key: String,
            qq: QQ,
        }
        def_resp! {
            msg: String,
        }

        let r: Response = self
            .post("/verify")
            .json(&Request {
                session_key: session_key.clone(),
                qq,
            })
            .send()
            .await?
            .json()
            .await?;
        r.ok()?;
        info!("verify: {}", r.msg);
        self.sessions.write().insert(qq, session_key);
        Ok(())
    }

    async fn release(&mut self, qq: QQ) -> Result<()> {
        def_req! {
            #[serde(rename = "sessionKey")]
            session_key: String,
            qq: QQ,
        }
        def_resp! {
            msg: String,
        }

        let r: Response = self
            .post("/release")
            .json(&Request {
                qq,
                session_key: self.session_key(qq)?,
            })
            .send()
            .await?
            .json()
            .await?;
        r.ok()?;
        self.sessions.write().remove(&qq);
        Ok(())
    }

    /// 将一个 QQ 绑定到这个 client 上
    pub async fn bind(&mut self, auth_key: String, qq: u64) -> Result<()> {
        let session_key = self.auth(auth_key).await?;
        self.verify(session_key, qq).await?;
        Ok(())
    }

    /// 发送私聊消息，返回消息 id
    pub async fn send_friend_message(
        &self,
        from_qq: QQ,
        to_qq: QQ,
        message: Vec<MessageBlock>,
    ) -> Result<i64> {
        def_req! {
            #[serde(rename = "sessionKey")]
            session_key: String,
            target: QQ,
            #[serde(rename = "messageChain")]
            message: Vec<MessageBlock>,
        }
        def_resp! {
            msg: String,
            #[serde(rename = "messageId")]
            message_id: i64,
        }

        let r: Response = self
            .post("/sendFriendMessage")
            .json(&Request {
                target: to_qq,
                session_key: self.session_key(from_qq)?,
                message,
            })
            .send()
            .await?
            .json()
            .await?;
        r.ok()?;
        Ok(r.message_id)
    }

    pub async fn ws_connect(&self, qq: QQ) -> Result<WebSocketStream<TcpStream>> {
        let session = self.session_key(qq)?;
        let base_url = self.base_url.replace("https", "wss").replace("http", "ws");
        let url = format!("{}/all?sessionKey={}", base_url, session);
        debug!("ws connecting to url {}", url);
        let url = url::Url::parse(&url).expect("Bad WS addr");
        let (ws_stream, resp) = connect_async(url).await?;
        Ok(ws_stream)
    }
}

#[async_trait::async_trait]
pub trait Replyable {
    async fn reply_to(&self, from_qq: QQ, msg: Vec<MessageBlock>, client: Client) -> Result<i64>;
}
