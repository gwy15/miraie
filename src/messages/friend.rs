//! 跟私聊、好友有关的模块

use futures::{future::ready, StreamExt};

use super::{stream::MessageStream, traits::Conversation, MessageChain};
use crate::{api, bot::QQ, Bot, Result};

/// 私聊消息的发送者
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct FriendMember {
    pub id: QQ,
    /// 好友昵称
    pub nickname: String,
    /// 好友备注
    pub remark: String,
}

/// 好友私聊信息
#[derive(Debug, Clone, Deserialize)]
pub struct FriendMessage {
    pub sender: FriendMember,
    #[serde(rename = "messageChain")]
    pub message: MessageChain,
}

#[async_trait]
impl Conversation for FriendMessage {
    type ReplyResponse = api::send_friend_message::Response;

    fn followed_group_message(&self, bot: &Bot) -> MessageStream<Self> {
        let sender_id = self.sender.id;

        MessageStream::new(
            bot.friend_messages()
                .filter(move |msg| ready(msg.sender.id == sender_id)),
        )
    }

    fn followed_sender_messages(&self, bot: &Bot) -> MessageStream<Self> {
        self.followed_group_message(bot)
    }

    async fn reply(
        &self,
        message: impl Into<MessageChain> + Send + 'static,
        bot: &Bot,
    ) -> Result<Self::ReplyResponse> {
        bot.request(api::send_friend_message::Request {
            target: self.sender.id,
            quote: self.message.message_id(),
            message: message.into(),
        })
        .await
    }

    async fn reply_unquote(
        &self,
        message: impl Into<MessageChain> + Send + 'static,
        bot: &Bot,
    ) -> Result<Self::ReplyResponse> {
        bot.request(api::send_friend_message::Request {
            target: self.sender.id,
            quote: None,
            message: message.into(),
        })
        .await
    }
}

impl crate::msg_framework::FromRequest<crate::Bot> for FriendMessage {
    fn from_request(request: crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
        match request.message {
            crate::messages::Message::Friend(f) => Some(f),
            _ => None,
        }
    }
}
