//! 跟私聊、好友有关的模块
use std::time::Duration;

use futures::{future::ready, Stream, StreamExt};

use super::MessageChain;
use crate::{api, bot::QQ, Bot, Error, Result};

/// 好友私聊信息
#[derive(Debug, Clone, Deserialize)]
pub struct FriendMessage {
    pub sender: FriendMember,
    #[serde(rename = "messageChain")]
    pub message: MessageChain,
}

impl FriendMessage {
    /// 返回一个消息的后续回复的 stream
    pub fn followed_messages(&self, bot: &Bot) -> impl Stream<Item = FriendMessage> + Unpin {
        let sender_id = self.sender.id;
        bot.friend_messages()
            .filter(move |msg| ready(msg.sender.id == sender_id))
    }

    /// 回复这条消息
    pub async fn reply(
        &self,
        message: impl Into<MessageChain>,
        bot: &Bot,
    ) -> Result<api::send_friend_message::Response> {
        bot.request(api::send_friend_message::Request {
            target: self.sender.id,
            quote: self.message.message_id(),
            message: message.into(),
        })
        .await
    }

    /// 返回一条消息并等待回复，默认超时 10s
    /// # Example
    /// ```plaintext,no_run
    /// let msg: FriendMessage;
    /// let confirm = msg.prompt("你确定吗？").await?;
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
        self.reply(message, bot).await?;
        let mut followed = self.followed_messages(bot);
        let msg = followed.next();
        let msg = tokio::time::timeout(timeout, msg)
            .await
            .map_err(|_| Error::ResponseTimeout)?;
        msg.ok_or(Error::ConnectionClosed)
    }
}

/// 私聊消息的发送者
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct FriendMember {
    pub id: QQ,
    /// 好友昵称
    pub nickname: String,
    /// 好友备注
    pub remark: String,
}

impl crate::msg_framework::FromRequest<crate::Bot> for FriendMessage {
    fn from_request(request: crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
        match request.message {
            crate::messages::Message::Friend(f) => Some(f),
            _ => None,
        }
    }
}
