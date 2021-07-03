use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Websocket error: {0}")]
    Websocket(#[from] async_tungstenite::tungstenite::Error),

    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
