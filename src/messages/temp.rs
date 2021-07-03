use super::MessageChain;

/// 群临时消息，跟群消息差不多
#[derive(Debug, Clone, Deserialize)]
pub struct TempMessage {
    sender: super::group::GroupMember,

    #[serde(rename = "messageChain")]
    message: MessageChain,
}

impl crate::msg_framework::FromRequest<crate::Bot> for TempMessage {
    fn from_request(request: crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
        match request.message {
            crate::messages::Message::Temp(msg) => Some(msg),
            _ => None,
        }
    }
}
