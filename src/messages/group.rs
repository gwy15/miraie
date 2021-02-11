use super::{Error, MessageBlock, Meta};
use crate::{
    client::{Client, Replyable},
    QQ,
};
use serde_json::{from_value, Value};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Message {
    pub meta: Meta,
    pub message: Vec<MessageBlock>,
    pub sender: Sender,
}

impl TryFrom<Value> for Message {
    type Error = Error;
    fn try_from(mut value: Value) -> Result<Self, Self::Error> {
        let mut value = value
            .as_object_mut()
            .ok_or_else(|| Error::format("data is not object."))?;

        // parse sender
        let sender_value = value
            .remove("sender")
            .ok_or_else(|| Error::format("missing sender field"))?;
        let sender: Sender = serde_json::from_value(sender_value)?;

        // parse message & meta info
        let mut message = vec![];
        let mut meta = None;

        let mut message_chain = value
            .remove("messageChain")
            .ok_or_else(|| Error::format("missing messageChain field"))?;
        let message_chain = message_chain
            .as_array_mut()
            .ok_or_else(|| Error::format("messageChain is not array."))?;

        for block in message_chain.drain(..) {
            let block_type = block
                .get("type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| Error::format("message chain item type has wrong format."))?;
            if block_type == "Source" {
                meta = Some(from_value(block)?);
            } else {
                message.push(MessageBlock::try_from(block)?);
            }
        }

        match meta {
            Some(meta) => Ok(Self {
                meta,
                message,
                sender,
            }),
            None => Err(Error::format("Missing source info in message chain")),
        }
    }
}

/// QQ 群里的权限
#[derive(Debug, Clone, Deserialize)]
pub enum Permission {
    #[serde(rename = "OWNER")]
    Owner,
    #[serde(rename = "ADMINISTRATOR")]
    Admin,
    #[serde(rename = "MEMBER")]
    Member,
}

/// 群消息的发送者
#[derive(Debug, Clone, Deserialize)]
pub struct Sender {
    #[serde(rename = "id")]
    qq: QQ,
    #[serde(rename = "memberName")]
    group_card: String,
    permission: Permission,
    group: Group,
}

#[async_trait::async_trait]
impl Replyable for Sender {
    async fn reply_to(
        &self,
        from_qq: QQ,
        message: Vec<MessageBlock>,
        client: Client,
    ) -> crate::Result<i64> {
        client
            .send_group_message(from_qq, self.group.id, message)
            .await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Group {
    id: u64,
    name: String,
    #[serde(rename = "permission")]
    my_permission: Permission,
}

#[cfg(test)]
mod test {
    use crate::DateTime;

    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_group_message() {
        let v = json!({
            "type": "GroupMessage",
            "messageChain": [
                {
                    "type": "Source",
                    "id": 1236,
                    "time": 1612863495
                },
                {
                    "type": "Plain",
                    "text": "hello"
                }
            ],
            "sender": {
                "id": 605957138,
                "memberName": "我",
                "permission": "OWNER",
                "group": {
                    "id": 747136680,
                    "name": "测试群",
                    "permission": "MEMBER"
                }
            }
        });
        let msg = Message::try_from(v).unwrap();
        assert_eq!(
            msg.meta.time,
            "2021-02-09T17:38:15+08:00".parse::<DateTime>().unwrap()
        );
    }
}
