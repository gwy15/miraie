/// 群聊信息
#[derive(Debug, Clone)]
pub struct GroupMessage(String);

impl crate::msg_framework::FromRequest<crate::Bot> for GroupMessage {
    fn from_request(request: crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
        match request.message {
            crate::messages::Message::Group(g) => Some(g),
            _ => None,
        }
    }
}

impl std::convert::TryFrom<serde_json::Value> for GroupMessage {
    type Error = crate::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
