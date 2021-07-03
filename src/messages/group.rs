//! 跟群聊、群成员有关的模块
use chrono::{DateTime, Utc};

use super::MessageChain;
use crate::bot::QQ;

/// 群聊信息
#[derive(Debug, Clone, Deserialize)]
pub struct GroupMessage {
    sender: GroupMember,

    #[serde(rename = "messageChain")]
    message: MessageChain,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    id: QQ,

    member_name: String,

    /// 群主给的特殊头衔
    special_title: String,

    /// 权限
    permission: Permission,

    /// 加群的时间
    #[serde(rename = "joinTimestamp", with = "chrono::serde::ts_seconds_option")]
    join: Option<DateTime<Utc>>,

    /// 最后一次发言的时间
    #[serde(
        rename = "lastSpeakTimestamp",
        with = "chrono::serde::ts_seconds_option"
    )]
    last_speak: Option<DateTime<Utc>>,

    // /// 剩余的禁言时间
    // #[serde(rename = "muteTimeRemaining", with = "chrono::serde::ts_seconds")]
    // mute_remaining: Duration,
    /// 群的信息
    group: Group,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Permission {
    /// 群员
    Member,
    /// 管理员
    Administrator,
    /// 群主
    Owner,
}

/// 群信息
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Group {
    id: QQ,
    /// 群名
    name: String,
    /// bot 在群里的权限
    permission: Permission,
}

impl crate::msg_framework::FromRequest<crate::Bot> for GroupMessage {
    fn from_request(request: crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
        match request.message {
            crate::messages::Message::Group(g) => Some(g),
            _ => None,
        }
    }
}
