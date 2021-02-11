#[macro_use]
use super::{friend, group, utils, Error};
use crate::{DateTime, QQ};
use serde::{ser::SerializeMap, Serialize, Serializer};
use serde_json::Value;
use std::{convert::TryFrom, str::FromStr};

#[derive(Debug, Clone)]
pub enum Message {
    Friend(friend::Message),
    Group(group::Message),
}
impl FromStr for Message {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: Value = s.parse()?;
        let type_ = value
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::format("type field wrong format"))?;
        match type_ {
            "FriendMessage" => Ok(Self::Friend(friend::Message::try_from(value)?)),
            "GroupMessage" => Ok(Self::Group(group::Message::try_from(value)?)),
            _ => Err(Error::UnknownType(type_.to_string())),
        }
    }
}

///
#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    pub id: i64,
    #[serde(deserialize_with = "utils::_parse_dt")]
    pub time: DateTime,
}

/// 接收、发送消息都是走这个结构体
#[derive(Debug, Clone)]
pub enum MessageBlock {
    Plain(String),
    At(QQ),
    Xml(String),
    Image {
        /// 对于发送，设置为空即可
        id: String,
        url: String,
        /// 对于发送，设置为空即可
        path: Option<String>,
    },
    App(String),
}
impl TryFrom<Value> for MessageBlock {
    type Error = Error;
    fn try_from(mut value: Value) -> Result<Self, Self::Error> {
        let type_ = utils::remove_string(&mut value, "type")
            .ok_or_else(|| Error::format("type format wrong"))?;
        let block = match type_.as_str() {
            "Plain" => MessageBlock::Plain(
                utils::remove_string(&mut value, "text")
                    .ok_or_else(|| Error::format("type plain: text format wrong"))?,
            ),
            "Xml" => MessageBlock::Xml(
                utils::remove_string(&mut value, "xml")
                    .ok_or_else(|| Error::format("type xml: xml format wrong"))?,
            ),
            "Image" => MessageBlock::Image {
                id: utils::remove_string(&mut value, "imageId")
                    .ok_or_else(|| Error::format("type image: imageId format wrong"))?,
                url: utils::remove_string(&mut value, "url")
                    .ok_or_else(|| Error::format("type image: url format wrong"))?,
                path: utils::remove_string(&mut value, "path"),
            },
            "App" => MessageBlock::App(
                utils::remove_string(&mut value, "app")
                    .ok_or_else(|| Error::format("type App: app format wrong"))?,
            ),
            "At" => MessageBlock::At(
                utils::remove_i64(&mut value, "target")
                    .ok_or_else(|| Error::format("type At: target format wrong"))?
                    as QQ,
            ),
            t => return Err(Error::format(format!("MessageBlock type unknown: {}", t))),
        };
        Ok(block)
    }
}
impl Serialize for MessageBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        match self {
            MessageBlock::Plain(text) => {
                map.serialize_entry("type", "Plain")?;
                map.serialize_entry("text", text)?;
            }
            MessageBlock::Xml(xml) => {
                map.serialize_entry("type", "Xml")?;
                map.serialize_entry("xml", xml)?;
            }
            MessageBlock::Image { id, url, path } => {
                map.serialize_entry("type", "Image")?;
                map.serialize_entry("url", url)?;
            }
            MessageBlock::App(s) => {
                map.serialize_entry("type", "App")?;
                map.serialize_entry("app", s)?;
            }
            MessageBlock::At(target) => {
                map.serialize_entry("type", "At")?;
                map.serialize_entry("target", target)?;
            }
        }
        map.end()
    }
}

#[cfg(test)]
mod test {
    use serde_json::{from_value, json};

    use super::*;
    #[test]
    fn test_parse_meta() {
        let v = json! ({
            "type": "Source",
            "id": 4006,
            "time": 1612848972
        });
        let meta = from_value::<Meta>(v).unwrap();

        assert_eq!(
            meta.time,
            "2021-02-09 13:36:12 +08:00".parse::<DateTime>().unwrap()
        );
    }

    #[test]
    fn test_parse_message_block() {
        let v = json!({
            "type": "Plain",
            "text": "z？",
        });
        let b = MessageBlock::try_from(v).unwrap();
        assert!(matches!(b, MessageBlock::Plain(_)));

        let v = json!({
            "type": "Image",
            "imageId": "{E8-1547034F3E23}.png",
            "url": "http://256D1547034F3E23/0?term=2",
            "path": null
        });
        let b = MessageBlock::try_from(v).unwrap();
        assert!(matches!(b, MessageBlock::Image{..}));
    }

    #[test]
    fn test_message_from_str() {
        let s = r#"
        {
            "type": "FriendMessage",
            "messageChain": [
                {
                    "type": "Source",
                    "id": 4006,
                    "time": 1612848972
                },
                {
                    "type": "Plain",
                    "text": "z？"
                }
            ],
            "sender": {
                "id": 123456,
                "nickname": "小马",
                "remark": ""
            }
        }"#;
        let msg: Message = s.parse().unwrap();
        assert!(matches!(msg, Message::Friend(_)));
        let friend_msg = match msg {
            Message::Friend(f) => f,
            _ => panic!(),
        };
        assert_eq!(friend_msg.sender.qq, 123456);
        assert_eq!(friend_msg.sender.nickname, "小马");
        assert_eq!(friend_msg.meta.id, 4006);
        assert_eq!(
            friend_msg.meta.time,
            "2021-02-09 13:36:12 +08:00".parse::<DateTime>().unwrap()
        );
        let block = friend_msg.message[0].clone();
        assert!(matches!(block, MessageBlock::Plain(s)));
    }
}
