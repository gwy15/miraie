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
