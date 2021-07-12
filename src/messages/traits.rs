use super::{stream::MessageStream, MessageChain};
use crate::{Bot, Error, Result};
use futures::StreamExt;
use std::time::{Duration, Instant};

/// 消息流，实现了消息流的类型（群聊消息和私聊消息）可以：
/// - 获取后续的消息
/// - 对本条消息进行回复
#[async_trait]
pub trait Conversation: Sized {
    // TODO: 群聊和私聊返回的消息类型都是一样的，直接合并成一个
    type ReplyResponse;

    /// 获取本聊天的后续消息。
    /// 如果是群聊，则返回当前群聊的任意后续消息；
    /// 如果是私聊，则返回当前私聊的任意后续消息。
    fn followed_group_message(&self, bot: &Bot) -> MessageStream<Self>;

    /// 获取这条消息发送者在本聊天中发送的后续消息
    fn followed_sender_messages(&self, bot: &Bot) -> MessageStream<Self>;

    // TODO
    // fn followed_quote_messages(&self, bot: &Bot) -> MessageStream<Self>;

    /// 回复这条消息，产生“引用”。
    async fn reply(
        &self,
        message: impl Into<MessageChain> + Send + 'static,
        bot: &Bot,
    ) -> Result<Self::ReplyResponse>;

    /// 不引用，直接回复这条消息
    async fn reply_unquote(
        &self,
        message: impl Into<MessageChain> + Send + 'static,
        bot: &Bot,
    ) -> Result<Self::ReplyResponse>;

    /// 返回一条消息并等待回复，默认超时 10s
    /// # Example
    /// ```plaintext
    /// let msg: GroupMessage;
    /// let confirm = msg.promp("你确定吗？").await?;
    /// if confirm.message.as_confirm().unwrap_or_default() {
    ///     // do something...
    /// }
    /// ```
    async fn prompt(
        &self,
        message: impl Into<MessageChain> + Send + 'static,
        bot: &Bot,
    ) -> Result<Self> {
        self.prompt_timeout(message, bot, Duration::from_secs(10))
            .await
    }

    /// 返回一条消息并等待回复
    async fn prompt_timeout(
        &self,
        message: impl Into<MessageChain> + Send + 'static,
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
