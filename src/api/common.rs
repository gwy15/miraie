/// 发送消息的返回
#[derive(Debug, Deserialize)]
pub struct SendMessageResponse {
    /// 标识本条消息，用于撤回和引用回复
    #[serde(rename = "messageId")]
    pub message_id: i64,
}
