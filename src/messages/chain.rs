use crate::bot::QQ;
use chrono::{DateTime, Utc};
use std::{
    env,
    fmt::{self, Write},
};

/// 消息的一个分块，见
/// <https://github.com/project-mirai/mirai-api-http/blob/master/docs/api/MessageType.md>
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum MessageBlock {
    /// Source类型永远为chain的第一个元素
    Source {
        /// 消息的识别号，用于引用回复
        id: i64,
        #[serde(with = "chrono::serde::ts_seconds")]
        time: DateTime<Utc>,
    },
    /// 引用回复
    Quote {
        /// 原消息的messageId
        id: i64,

        /// 原消息所接收的群号，当为好友消息时为0
        #[serde(rename = "groupId")]
        group_id: QQ,

        /// 原消息的发送者的QQ号
        #[serde(rename = "senderId")]
        sender_id: QQ,

        /// 原消息的接收者者的QQ号（或群号）
        #[serde(rename = "targetId")]
        target_id: QQ,

        /// 原消息的消息链对象
        origin: MessageChain,
    },
    /// @ 人
    At {
        /// 群员QQ号
        target: QQ,
        // At时显示的文字，发送消息时无效，自动使用群名片
        display: String,
    },
    /// @全体成员
    AtAll,

    /// QQ表情
    Face {
        /// QQ表情编号，可选，优先高于name
        #[serde(rename = "faceId")]
        face_id: i32,
        /// QQ表情拼音，可选
        name: String,
    },

    /// 文字消息
    #[serde(rename = "Plain")]
    Text { text: String },

    /// 图片消息
    ///
    /// 三个参数任选其一，出现多个参数时，按照imageId > url > path > base64的优先级
    Image {
        /// 图片的imageId，群图片与好友图片格式不同。不为空时将忽略url属性
        /// 群图片格式   "{01E9451B-70ED-EAE3-B37C-101F1EEBF5B5}.mirai"
        /// 好友图片格式 "/f8f1ab55-bf8e-4236-b55e-955848d7069f"
        #[serde(rename = "imageId")]
        image_id: String,
        /// 图片的URL，发送时可作网络图片的链接；接收时为腾讯图片服务器的链接，可用于图片下载
        url: String,
        // /// 图片的路径，发送本地图片，相对路径于 plugins/MiraiAPIHTTP/images
        // path: Option<String>,
        /// 图片的 Base64 编码
        base64: Option<String>,
    },

    /// 闪照
    ///
    /// 三个参数任选其一，出现多个参数时，按照imageId > url > path > base64的优先级
    FlushImage {
        /// 图片的imageId，群图片与好友图片格式不同。不为空时将忽略url属性
        /// 群图片格式   "{01E9451B-70ED-EAE3-B37C-101F1EEBF5B5}.mirai"
        /// 好友图片格式 "/f8f1ab55-bf8e-4236-b55e-955848d7069f"
        #[serde(rename = "imageId")]
        image_id: String,
        /// 图片的URL，发送时可作网络图片的链接；接收时为腾讯图片服务器的链接，可用于图片下载
        url: String,
        // /// 图片的路径，发送本地图片，相对路径于 plugins/MiraiAPIHTTP/images
        // path: Option<String>,
        /// 图片的 Base64 编码
        base64: Option<String>,
    },

    /// 音频消息
    ///
    /// 三个参数任选其一，出现多个参数时，按照voiceId > url > path > base64的优先级
    Voice {
        /// 语音的voiceId，不为空时将忽略url属性
        #[serde(rename = "voiceId")]
        voice_id: Option<String>,
        /// 语音的URL，发送时可作网络语音的链接；接收时为腾讯语音服务器的链接，可用于语音下载
        url: Option<String>,
        // /// 语音的路径，发送本地语音，相对路径于 plugins/MiraiAPIHTTP/voices
        // path: Option<String>,
        /// 语音的 Base64 编码
        base64: Option<String>,
    },

    /// XML
    Xml { xml: String },
    // /// 转发消息，暂时不支持
    // #[serde(rename = "ForwardMessage")]
    // ForwardMessage {
    //     #[serde(rename = "nodeList")]
    //     nodes: Vec<ForwardMessageNode>,
    // }
    /// 文件消息
    File {
        /// 文件识别id
        id: String,
        /// 文件名
        name: String,
        /// 文件大小
        size: usize,
    },
}

impl fmt::Display for MessageBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageBlock::Source { .. } => Ok(()),
            MessageBlock::Quote { origin, .. } => {
                writeln!(f, "> {}", origin)
            }
            MessageBlock::At {
                target, display, ..
            } => {
                if display.is_empty() {
                    write!(f, "@{}", target)
                } else {
                    f.write_str(display.as_str())
                }
            }
            MessageBlock::AtAll => f.write_str("@全体成员"),
            MessageBlock::Face { name, .. } => write!(f, "[{}]", name),
            MessageBlock::Text { text } => f.write_str(text),
            MessageBlock::Image { .. } => f.write_str("[图片]"),
            MessageBlock::FlushImage { .. } => f.write_str("[闪照]"),
            MessageBlock::Voice { .. } => f.write_str("[语音消息]"),
            MessageBlock::Xml { .. } => f.write_str("[XML消息]"),
            MessageBlock::File { .. } => f.write_str("[文件消息]"),
        }
    }
}

/// 一条发送的消息，其可能由几个 [`MessageBlock`] 构成。
///
/// 注意第一个 Block 一定是 Source
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Default)]
pub struct MessageChain(pub Vec<MessageBlock>);

impl fmt::Display for MessageChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let filtered = self
            .0
            .iter()
            .filter(|block| !matches!(block, MessageBlock::Source { .. }))
            .collect::<Vec<_>>();

        for (i, block) in filtered.iter().enumerate() {
            block.fmt(f)?;
            if i != filtered.len() - 1 {
                f.write_char(' ')?;
            }
        }
        Ok(())
    }
}

impl MessageChain {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn at(mut self, qq: QQ) -> Self {
        self.0.push(MessageBlock::At {
            target: qq,
            display: String::new(),
        });
        self
    }
    pub fn at_all(mut self) -> Self {
        self.0.push(MessageBlock::AtAll);
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.0.push(MessageBlock::Text { text: text.into() });
        self
    }

    pub fn image_url(mut self, url: impl Into<String>) -> Self {
        self.0.push(MessageBlock::Image {
            image_id: String::new(),
            url: url.into(),
            base64: None,
        });
        self
    }
    /// 图片的路径，发送本地图片，相对路径于 env:RESOURCE_ROOT/images
    pub fn image_path(self, path: impl AsRef<str>) -> Self {
        self.image_url(format!(
            "file:///{}/images/{}",
            env::var("RESOURCE_ROOT").unwrap(),
            path.as_ref()
        ))
    }

    pub fn voice_url(mut self, url: impl Into<String>) -> Self {
        self.0.push(MessageBlock::Voice {
            voice_id: None,
            url: Some(url.into()),
            base64: None,
        });
        self
    }
    /// 语音的路径，发送本地语音，相对路径于 env:RESOURCE_ROOT/voices
    pub fn voice_path(self, path: impl AsRef<str>) -> Self {
        self.voice_url(format!(
            "file:///{}/voices/{}",
            env::var("RESOURCE_ROOT").unwrap(),
            path.as_ref()
        ))
    }

    /// 获取消息的 message id
    pub fn message_id(&self) -> Option<i64> {
        if self.0.is_empty() {
            return None;
        }
        match self.0[0] {
            MessageBlock::Source { id, .. } => Some(id),
            _ => None,
        }
    }

    /// 尝试把 message 按照确认取消匹配
    pub fn as_confirm(&self) -> Option<bool> {
        match self.to_string().to_lowercase().trim() {
            "好" | "好的" | "是" | "确认" | "真的" | "ok" | "yes" | "y" | "√" | "1" => {
                Some(true)
            }
            "不" | "否" | "算了" | "取消" | "no" | "n" | "x" | "0" => Some(false),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    use super::*;
    #[test]
    fn test_message_block_source() {
        let s = r#"{
            "type": "Source",
            "id": 123,
            "time": 123  
        }"#;
        assert_eq!(
            serde_json::from_str::<MessageBlock>(s).unwrap(),
            MessageBlock::Source {
                id: 123,
                time: DateTime::from_utc(NaiveDateTime::from_timestamp(123, 0), Utc)
            }
        );
    }

    #[test]
    fn test_message_block_face() {
        let s = r#"{
            "type": "Face",
            "name": "惊讶",
            "faceId": 0
        }"#;
        assert_eq!(
            serde_json::from_str::<MessageBlock>(s).unwrap(),
            MessageBlock::Face {
                face_id: 0,
                name: "惊讶".to_string()
            }
        );
    }
}
