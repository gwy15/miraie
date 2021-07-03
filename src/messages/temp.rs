/// 群临时消息
#[derive(Debug, Clone)]
pub struct TempMessage(String);

impl std::convert::TryFrom<serde_json::Value> for TempMessage {
    type Error = crate::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
