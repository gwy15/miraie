//! 错误模块，提供 [`enum@Error`] 类型
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Websocket error: {0}")]
    Websocket(#[from] async_tungstenite::tungstenite::Error),

    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("format error: {}", .reason)]
    Format { reason: String },

    #[error("The connection to mirai bot is closed.")]
    ConnectionClosed,

    #[error("Request to mirai bot has timeout.")]
    RequestTimeout,

    #[error("Request error: code = {}, msg = {}", .code, msg)]
    Request { code: i32, msg: String },
}

impl Error {
    pub fn format(reason: impl Into<String>) -> Self {
        Self::Format {
            reason: reason.into(),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
