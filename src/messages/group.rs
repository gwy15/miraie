//! 跟群聊、群成员有关的模块
use chrono::{DateTime, Utc};
use futures::{future::ready, StreamExt};

use super::{stream::MessageStream, MessageChain};
use crate::{api, bot::QQ, Bot, Result};

/// 一个群里的某个成员
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

impl From<GroupMember> for QQ {
    fn from(src: GroupMember) -> Self {
        src.id
    }
}

impl AsRef<QQ> for GroupMember {
    fn as_ref(&self) -> &QQ {
        &self.id
    }
}

/// 成员在群里的权限，可能为群员、管理员或群主
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
impl PartialOrd for Permission {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        if self.eq(other) {
            return Some(Equal);
        }
        Some(match (self, other) {
            (Permission::Member, _) => Less,
            (Permission::Owner, _) => Greater,
            (Permission::Administrator, Permission::Member) => Greater,
            (Permission::Administrator, &Permission::Owner) => Less,
            _ => unreachable!(),
        })
    }
}

/// 群的信息
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Group {
    pub id: QQ,
    /// 群名
    pub name: String,
    /// bot 在群里的权限
    pub permission: Permission,
}

/// 群聊消息
#[derive(Debug, Clone, Deserialize)]
pub struct GroupMessage {
    /// 消息的发送者
    pub sender: GroupMember,

    /// 消息本体
    #[serde(rename = "messageChain")]
    pub message: MessageChain,
}

#[async_trait]
impl super::traits::Conversation for GroupMessage {
    type Sender = GroupMember;

    fn sender(&self) -> &Self::Sender {
        &self.sender
    }

    fn as_message(&self) -> &MessageChain {
        &self.message
    }

    fn followed_group_message(&self, bot: &Bot) -> MessageStream<Self> {
        let group_id = self.sender.group.id;
        MessageStream::new(
            bot.group_messages()
                .filter(move |msg| ready(msg.sender.group.id == group_id)),
        )
    }

    fn followed_sender_messages(&self, bot: &Bot) -> MessageStream<Self> {
        let group_id = self.sender.group.id;
        let sender_id = self.sender.id;
        MessageStream::new(bot.group_messages().filter(move |msg| {
            ready(msg.sender.group.id == group_id && msg.sender.id == sender_id)
        }))
    }

    async fn reply(
        &self,
        message: impl Into<MessageChain> + Send + 'static,
        bot: &Bot,
    ) -> Result<api::common::SendMessageResponse> {
        bot.request(api::send_group_message::Request {
            target: self.sender.group.id,
            quote: self.message.message_id(),
            message: message.into(),
        })
        .await
    }

    async fn reply_unquote(
        &self,
        message: impl Into<MessageChain> + Send + 'static,
        bot: &Bot,
    ) -> Result<api::common::SendMessageResponse> {
        bot.request(api::send_group_message::Request {
            target: self.sender.group.id,
            quote: None,
            message: message.into(),
        })
        .await
    }
}

impl crate::msg_framework::FromRequest<crate::Bot> for GroupMessage {
    fn from_request(request: &crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
        match &request.message {
            crate::messages::Message::Group(g) => Some(g.clone()),
            _ => None,
        }
    }
}
