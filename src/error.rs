use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Websocket error: {0}")]
    Websocket(#[from] async_tungstenite::tungstenite::Error),

    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("format error: {}", .reason)]
    Format { reason: String },
}

impl Error {
    pub fn format(reason: impl Into<String>) -> Self {
        Self::Format {
            reason: reason.into(),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
