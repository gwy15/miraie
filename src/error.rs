use thiserror::Error;

use crate::QQ;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Network error: {0}")]
    Network(#[from] request::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Mirai returned code: {0}")]
    Status(i32),

    #[error("The QQ {0} is not bound with this client.")]
    NotBounded(QQ),

    #[error("mpsc channel error: {0}")]
    ChannelError(Box<dyn std::error::Error + Send>),
}

pub type Result<T> = std::result::Result<T, Error>;
