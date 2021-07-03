mod event;
mod friend;
mod group;
mod stranger;
mod temp;

use std::convert::TryFrom;

pub use event::Event;
pub use friend::FriendMessage;
pub use group::GroupMessage;
use serde_json::Value;
pub use stranger::StrangerMessage;
pub use temp::TempMessage;

use crate::Error;

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
    type Error = Error;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let t = value
            .get("type")
            .and_then(|s| s.as_str())
            .ok_or_else(|| Error::format("type not found."))?;
        let msg = match t {
            "FriendMessage" => Message::Friend(FriendMessage::try_from(value)?),
            "GroupMessage" => Message::Group(GroupMessage::try_from(value)?),
            "TempMessage" => Message::Temp(TempMessage::try_from(value)?),
            "StrangerMessage" => Message::Stranger(StrangerMessage::try_from(value)?),
            "OtherClientMessage" => {
                return Err(Error::format("Unsupported type: `OtherClientMessage`"));
            }
            typename => Message::Event(Event::try_from((typename.to_string(), value))?),
        };
        Ok(msg)
    }
}
