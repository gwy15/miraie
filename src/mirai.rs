use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
    pin::Pin,
};

use crate::{client::Client, messages::Message, Error, Result, QQ};
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite;

pub trait Handler {
    fn handle(&self, msg: Message) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

impl<Func, Fut> Handler for Func
where
    Func: Fn(Message) -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    fn handle(&self, msg: Message) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin((self)(msg))
    }
}

pub struct AppBuilder {
    clients: Vec<Client>,
    data: HashMap<TypeId, Box<dyn Any>>,
    handlers: Vec<Box<dyn Handler>>,
}
impl AppBuilder {
    pub fn new() -> Self {
        Self {
            clients: vec![],
            data: HashMap::new(),
            handlers: vec![],
        }
    }

    pub async fn bind(
        mut self,
        addr: impl Into<String>,
        auth_key: impl Into<String>,
        qq_list: &[QQ],
    ) -> Result<Self> {
        let auth_key = auth_key.into();
        let mut client = Client::new(addr);
        for &qq in qq_list.iter() {
            client.bind(auth_key.clone(), qq).await?;
        }
        self.clients.push(client);
        Ok(self)
    }

    pub fn handler(mut self, f: impl Handler + 'static) -> Self {
        self.handlers.push(Box::new(f));
        self
    }

    pub fn build(self) -> App {
        App {
            data: self.data,
            clients: self.clients,
            handlers: self.handlers,
        }
    }
}

pub struct App {
    clients: Vec<Client>,
    data: HashMap<TypeId, Box<dyn Any>>,
    handlers: Vec<Box<dyn Handler>>,
}
impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::new()
    }

    pub async fn run(self) -> Result<()> {
        let (tx, mut rx) = mpsc::channel(8192);
        let mut handlers = vec![];
        for client in self.clients {
            for (qq, _session) in client.bound_accounts() {
                let mut ws_stream = client.ws_connect(*qq).await?;
                let tx = tx.clone();
                let handler = tokio::spawn(async move {
                    while let Some(msg) = ws_stream.next().await {
                        let msg = msg?;
                        match msg {
                            tungstenite::Message::Text(text) => {
                                debug!("received text: {}...", &text[..30]);
                                match text.parse::<Message>() {
                                    Ok(msg) => {
                                        tx.send(msg)
                                            .await
                                            .map_err(|e| Error::ChannelError(Box::new(e)))?;
                                    }
                                    Err(e) => {
                                        debug!("error parsing msg: {}, raw msg = {}", e, text);
                                    }
                                }
                            }
                            tungstenite::Message::Binary(bin) => {
                                debug!("got binary ({} bytes), ignore.", bin.len());
                            }
                            tungstenite::Message::Ping(_) => {}
                            tungstenite::Message::Pong(_) => {}
                            tungstenite::Message::Close(_) => {}
                        }
                    }
                    Result::Ok(())
                });
                handlers.push(handler);
            }
        }
        std::mem::drop(tx);
        // start receiver
        while let Some(msg) = rx.recv().await {
            info!("msg recv: {:?}", msg);
            for handler in self.handlers.iter() {
                tokio::spawn(handler.handle(msg.clone()));
            }
        }
        Ok(())
    }
}
