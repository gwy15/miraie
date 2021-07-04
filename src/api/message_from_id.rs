//! 通过messageId获取消息
//!
//! 此方法通过 messageId 获取历史消息, 历史消息的缓存有容量大小, 在配置文件中设置

use crate::{bot::QQ, messages::Message};

#[derive(Serialize)]
pub struct Request {
    pub target: QQ,
}

pub type Response = Message;

crate::api!(command = "messageFromId", Request, Response);
