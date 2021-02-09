use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("format error: {0}")]
    MsgFormat(String),

    #[error("Type {0} not supported.")]
    UnknownType(String),
}

impl Error {
    pub fn format(s: impl Into<String>) -> Self {
        Self::MsgFormat(s.into())
    }
}
