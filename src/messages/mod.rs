//! mirai 传回的消息，群聊、私聊、事件等
mod chain;
mod event;
pub mod friend;
pub mod group;
mod stranger;
mod temp;

use std::convert::TryFrom;

pub use chain::{MessageBlock, MessageChain};
pub use event::Event;
pub use friend::FriendMessage;
pub use group::GroupMessage;
use serde::Deserialize;
use serde_json::Value;
pub use stranger::StrangerMessage;
pub use temp::TempMessage;

/// 接收到的消息，可能是群消息、私聊消息、事件等
#[derive(Debug, Clone)]
pub enum Message {
    Friend(FriendMessage),
    Group(GroupMessage),
    Temp(TempMessage),
    Stranger(StrangerMessage),
    Event(Event),
}

impl crate::msg_framework::FromRequest<crate::Bot> for Message {
    fn from_request(request: crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
        Some(request.message)
    }
}

impl TryFrom<Value> for Message {
    type Error = serde_json::Error;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        use crate::serde::de::Error;
        let t = value
            .get("type")
            .and_then(|s| s.as_str())
            .ok_or_else(|| serde_json::Error::custom("type not found."))?;
        let msg = match t {
            "FriendMessage" => Message::Friend(serde_json::from_value(value)?),
            "GroupMessage" => Message::Group(serde_json::from_value(value)?),
            "TempMessage" => Message::Temp(serde_json::from_value(value)?),
            "StrangerMessage" => Message::Stranger(serde_json::from_value(value)?),
            "OtherClientMessage" => {
                return Err(serde_json::Error::custom(
                    "Unsupported type: `OtherClientMessage`",
                ));
            }
            _event_type => Message::Event(serde_json::from_value(value)?),
        };
        Ok(msg)
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let value = Value::deserialize(deserializer)?;
        let msg = Message::try_from(value).map_err(|e| D::Error::custom(e.to_string()))?;

        Ok(msg)
    }
}
