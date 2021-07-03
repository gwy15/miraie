/// 事件，如管理员收到的加群请求等
#[derive(Debug, Clone)]
pub struct Event(String);

impl std::convert::TryFrom<(String, serde_json::Value)> for Event {
    type Error = crate::Error;
    fn try_from((typename, value): (String, serde_json::Value)) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
