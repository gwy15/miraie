use super::MessageChain;
use crate::bot::QQ;

/// 好友私聊信息
#[derive(Debug, Clone, Deserialize)]
pub struct FriendMessage {
    sender: FriendMember,
    #[serde(rename = "messageChain")]
    message: MessageChain,
}

/// 私聊消息的发送者
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct FriendMember {
    id: QQ,
    /// 好友昵称
    nickname: String,
    /// 好友备注
    remark: String,
}

impl crate::msg_framework::FromRequest<crate::Bot> for FriendMessage {
    fn from_request(request: crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
        match request.message {
            crate::messages::Message::Friend(f) => Some(f),
            _ => None,
        }
    }
}
