//! 撤回消息

#[derive(Debug, Serialize)]
pub struct Request {
    /// 需要撤回的消息的messageId
    #[serde(rename = "target")]
    pub message_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct Response;

crate::api!(command = "recall", Request, Response);
