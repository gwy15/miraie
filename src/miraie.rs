use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::Arc,
};

use crate::{client::Client, messages::Message, Error, Result, QQ};
use futures_util::StreamExt;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite;

pub struct Context {
    app: Arc<App>,
}
impl Context {
    pub fn new(app: Arc<App>) -> Self {
        Self { app }
    }
    pub fn data<T: 'static>(&self) -> Option<Arc<T>> {
        let any_obj = self.app.data.get(&TypeId::of::<Arc<T>>())?.clone();
        any_obj.downcast_ref::<Arc<T>>().cloned()
    }
}

pub trait Handler: Send + Sync {
    fn call(&self, message: Message, ctx: Context) -> Pin<Box<dyn Future<Output = bool> + Send>>;
}

impl<F, R> Handler for F
where
    F: Fn(Message, Context) -> R + Send + Sync,
    R: Future<Output = bool> + Send + 'static,
{
    fn call(&self, message: Message, ctx: Context) -> Pin<Box<dyn Future<Output = bool> + Send>> {
        Box::pin((self)(message, ctx))
    }
}

pub struct AppBuilder {
    clients: Vec<Client>,
    data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
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

    pub fn handler<F>(mut self, f: F) -> Self
    where
        F: Handler + Send + 'static,
    {
        self.handlers.push(Box::new(f));
        self
    }

    pub fn data<T>(mut self, data: T) -> Self
    where
        T: Send + Sync + 'static,
    {
        self.data
            .insert(TypeId::of::<Arc<T>>(), Box::new(Arc::new(data)));
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
    data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    handlers: Vec<Box<dyn Handler>>,
}
impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::new()
    }

    pub async fn run(self) -> Result<()> {
        let app = Arc::new(self);
        let (tx, mut rx) = mpsc::channel(8192);
        let mut handlers = vec![];
        for client in app.clients.iter() {
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
            let app = app.clone();
            let msg = msg.clone();
            // 每条消息单独起一个 task
            tokio::spawn(async move {
                for handler in app.handlers.iter() {
                    if handler.call(msg.clone(), Context::new(app.clone())).await {
                        break;
                    }
                }
            });
        }
        Ok(())
    }
}
