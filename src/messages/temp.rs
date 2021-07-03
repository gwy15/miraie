use super::MessageChain;

/// 群临时消息，跟群消息差不多
#[derive(Debug, Clone, Deserialize)]
pub struct TempMessage {
    sender: super::group::GroupMember,

    #[serde(rename = "messageChain")]
    message: MessageChain,
}
