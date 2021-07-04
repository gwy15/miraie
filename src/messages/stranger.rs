use super::MessageChain;

/// 陌生人消息，跟好友消息差不多
#[derive(Debug, Clone, Deserialize)]
pub struct StrangerMessage {
    pub sender: super::friend::FriendMember,
    #[serde(rename = "messageChain")]
    pub message: MessageChain,
}

impl crate::msg_framework::FromRequest<crate::Bot> for StrangerMessage {
    fn from_request(request: crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
        match request.message {
            crate::messages::Message::Stranger(msg) => Some(msg),
            _ => None,
        }
    }
}
