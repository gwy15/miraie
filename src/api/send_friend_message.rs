//! 发送好友消息
//!
//! 使用此方法向指定好友发送消息

use crate::{bot::QQ, messages::MessageChain};

#[derive(Debug, Serialize)]
pub struct Request {
    /// 发送消息目标好友的QQ号
    pub target: QQ,
    /// 引用一条消息的messageId进行回复
    pub quote: Option<i64>,
    /// 消息链，是一个消息对象构成的数组
    #[serde(rename = "messageChain")]
    pub message: MessageChain,
}

crate::api!(
    command = "sendFriendMessage",
    subcommand = None,
    field = "flatten",
    Request,
    super::common::SendMessageResponse
);
