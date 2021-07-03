use std::sync::{
    atomic::{AtomicBool, Ordering::Relaxed},
    Arc,
};

use log::*;
use miraie::msg_framework::{App, FromRequest, Request};
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum Msg {
    Text(String),
    Number(i64),
}

async fn handler(msg: Msg, app: Application) {
    info!("msg: {:?}", msg);
    match msg {
        Msg::Number(_i) => app.num_received.store(true, Relaxed),
        Msg::Text(_s) => app.msg_received.store(true, Relaxed),
    }
}

impl FromRequest<Application> for Msg {
    fn from_request(request: Request<Application>) -> Option<Self> {
        Some(request.message)
    }
}
impl FromRequest<Application> for Application {
    fn from_request(request: Request<Application>) -> Option<Self> {
        Some(request.app)
    }
}

#[derive(Debug, Clone)]
struct Application {
    channel: broadcast::Sender<Msg>,
    msg_received: Arc<AtomicBool>,
    num_received: Arc<AtomicBool>,
}
impl Application {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(1_000);
        let msg_received = Default::default();
        let num_received = Default::default();
        Self {
            channel: tx,
            msg_received,
            num_received,
        }
    }
}
impl App for Application {
    type Message = Msg;
    fn event_bus(&self) -> broadcast::Sender<Self::Message> {
        self.channel.clone()
    }
}

#[tokio::test]
async fn test_simple_server() {
    // pretty_env_logger::init();
    let app = Application::new().handler(handler);
    let event_bus = app.event_bus();
    event_bus.send(Msg::Text("test".to_string())).unwrap();
    event_bus.send(Msg::Number(123)).unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    assert_eq!(app.msg_received.load(Relaxed), true);
    assert_eq!(app.num_received.load(Relaxed), true);
}
