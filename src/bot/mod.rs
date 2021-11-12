//! bot 的实现
mod basic_types;
mod botapp;
mod connection;
mod keyword_command;
mod return_handle;
mod utils;

pub use basic_types::*;
pub use botapp::Bot;
pub use connection::Connection;
pub(crate) use keyword_command::{KeywordCommandHandler, KeywordCommandHandlers};

type WebsocketStream = async_tungstenite::WebSocketStream<async_tungstenite::tokio::ConnectStream>;
use async_tungstenite::tungstenite::Message as WsMessage;
use futures::stream::{SplitSink, SplitStream};
