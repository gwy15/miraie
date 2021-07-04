//! 跟私聊、好友有关的模块
use super::MessageChain;
use crate::bot::QQ;

/// 好友私聊信息
#[derive(Debug, Clone, Deserialize)]
pub struct FriendMessage {
    pub sender: FriendMember,
    #[serde(rename = "messageChain")]
    pub message: MessageChain,
}

/// 私聊消息的发送者
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct FriendMember {
    pub id: QQ,
    /// 好友昵称
    pub nickname: String,
    /// 好友备注
    pub remark: String,
}

impl crate::msg_framework::FromRequest<crate::Bot> for FriendMessage {
    fn from_request(request: crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
        match request.message {
            crate::messages::Message::Friend(f) => Some(f),
            _ => None,
        }
    }
}
