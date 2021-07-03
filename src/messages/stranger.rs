/// 陌生人消息
#[derive(Debug, Clone)]
pub struct StrangerMessage;

impl std::convert::TryFrom<serde_json::Value> for StrangerMessage {
    type Error = crate::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
