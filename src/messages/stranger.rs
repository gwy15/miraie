use super::MessageChain;

/// 陌生人消息，跟好友消息差不多
#[derive(Debug, Clone, Deserialize)]
pub struct StrangerMessage {
    sender: super::friend::FriendMember,
    #[serde(rename = "messageChain")]
    message: MessageChain,
}
