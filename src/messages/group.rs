//! 跟群聊、群成员有关的模块
use chrono::{DateTime, Utc};
use futures::{future::ready, Stream, StreamExt};

use super::MessageChain;
use crate::{api, bot::QQ, Bot, Result};

/// 群聊信息
#[derive(Debug, Clone, Deserialize)]
pub struct GroupMessage {
    /// 消息的发送者
    pub sender: GroupMember,

    /// 消息本体
    #[serde(rename = "messageChain")]
    pub message: MessageChain,
}

impl GroupMessage {
    /// 获取当前群的后续群消息
    pub fn followed_group_messages(&self, bot: &Bot) -> impl Stream<Item = GroupMessage> {
        let group_id = self.sender.group.id;
        bot.group_messages()
            .filter(move |msg| ready(msg.sender.group.id == group_id))
    }

    /// 获取消息成员在当前群发送的后续群消息
    pub fn followed_sender_messages(&self, bot: &Bot) -> impl Stream<Item = GroupMessage> {
        let group_id = self.sender.group.id;
        let sender_id = self.sender.id;
        bot.group_messages()
            .filter(move |msg| ready(msg.sender.group.id == group_id && msg.sender.id == sender_id))
    }

    /// 在群里回复这条消息，不产生“引用”。
    pub async fn reply(
        &self,
        message: impl Into<MessageChain>,
        bot: &Bot,
    ) -> Result<api::send_group_message::Response> {
        bot.request(api::send_group_message::Request {
            target: self.sender.group.id,
            quote: None,
            message: message.into(),
        })
        .await
    }

    /// 引用回复这条啊消息
    pub async fn quote_reply(
        &self,
        message: impl Into<MessageChain>,
        bot: &Bot,
    ) -> Result<api::send_group_message::Response> {
        bot.request(api::send_group_message::Request {
            target: self.sender.group.id,
            quote: self.message.message_id(),
            message: message.into(),
        })
        .await
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    /// 群员的 QQ 号
    pub id: QQ,

    /// 群员名字
    pub member_name: String,

    /// 群主给的特殊头衔
    pub special_title: String,

    /// 权限
    pub permission: Permission,

    /// 加群的时间
    #[serde(rename = "joinTimestamp", with = "chrono::serde::ts_seconds_option")]
    pub join: Option<DateTime<Utc>>,

    /// 最后一次发言的时间
    #[serde(
        rename = "lastSpeakTimestamp",
        with = "chrono::serde::ts_seconds_option"
    )]
    pub last_speak: Option<DateTime<Utc>>,

    // /// 剩余的禁言时间
    // #[serde(rename = "muteTimeRemaining", with = "chrono::serde::ts_seconds")]
    // mute_remaining: Duration,
    /// 群的信息
    pub group: Group,
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
    pub id: QQ,
    /// 群名
    pub name: String,
    /// bot 在群里的权限
    pub permission: Permission,
}

impl crate::msg_framework::FromRequest<crate::Bot> for GroupMessage {
    fn from_request(request: crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
        match request.message {
            crate::messages::Message::Group(g) => Some(g),
            _ => None,
        }
    }
}
