mod event;
mod friend;
mod group;
mod stranger;
mod temp;

pub use event::Event;
pub use friend::FriendMessage;
pub use group::GroupMessage;
pub use stranger::StrangerMessage;
pub use temp::TempMessage;

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
