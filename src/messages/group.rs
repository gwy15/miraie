//! 跟群聊、群成员有关的模块
use chrono::{DateTime, Utc};
use futures::{future::ready, Stream, StreamExt};
use std::time::{Duration, Instant};

use super::MessageChain;
use crate::{api, bot::QQ, Bot, Error, Result};

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
        bot.group_messages().filter(move |msg| {
            ready({
                // FIXME
                let m = msg.sender.group.id == group_id && msg.sender.id == sender_id;
                info!("msg = {:?}, matches: {}", msg, m);
                m
            })
        })
    }

    /// 在群里回复这条消息，产生“引用”。
    pub async fn reply(
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

    /// 不引用，直接在群里回复这条消息
    pub async fn unquote_reply(
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

    /// 返回一条消息并等待回复，默认超时 10s
    /// # Example
    /// ```plaintext
    /// let msg: GroupMessage;
    /// let confirm = msg.promp("你确定吗？").await?;
    /// if confirm.message.as_confirm().unwrap_or_default() {
    ///     // do something...
    /// }
    /// ```
    pub async fn prompt(&self, message: impl Into<MessageChain>, bot: &Bot) -> Result<Self> {
        self.prompt_timeout(message, bot, Duration::from_secs(10))
            .await
    }

    /// 返回一条消息并等待回复
    pub async fn prompt_timeout(
        &self,
        message: impl Into<MessageChain>,
        bot: &Bot,
        timeout: Duration,
    ) -> Result<Self> {
        let t = Instant::now();
        self.reply(message, bot).await?;
        debug!("prompt sent.");
        let mut followed = self.followed_sender_messages(bot);
        let msg = followed.next();
        let msg = tokio::time::timeout(timeout, msg)
            .await
            .map_err(|_| Error::ResponseTimeout)?;
        info!("prompt 获得了返回，耗时 {} ms", t.elapsed().as_millis());
        msg.ok_or(Error::ConnectionClosed)
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
