//! 使用此方法处理Bot被邀请入群申请

use crate::bot::QQ;

#[derive(Debug, Serialize)]
pub struct Request {
    #[serde(rename = "sessionKey")]
    pub session_key: String,
    #[serde(rename = "eventId")]
    pub event_id: i64,
    #[serde(rename = "fromId")]
    pub from_id: QQ,
    #[serde(rename = "groupId")]
    pub group_id: QQ,
    /// 0 同意邀请；1 拒绝邀请
    pub operate: i32,

    pub message: String,
}

crate::api!(
    command = "resp_botInvitedJoinGroupRequestEvent",
    subcommand = None,
    field = "flatten",
    Request,
    serde_json::Value
);
