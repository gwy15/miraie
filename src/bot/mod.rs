mod basic_types;
mod botapp;
mod connection;
mod utils;

pub use basic_types::*;
pub use botapp::Bot;
pub use connection::Connection;

type WebsocketStream = async_tungstenite::WebSocketStream<async_tungstenite::tokio::ConnectStream>;
use async_tungstenite::tungstenite::Message as WsMessage;
use futures::stream::{SplitSink, SplitStream};
