/// 好友私聊信息
#[derive(Debug, Clone)]
pub struct FriendMessage(String);

impl std::convert::TryFrom<serde_json::Value> for FriendMessage {
    type Error = crate::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
