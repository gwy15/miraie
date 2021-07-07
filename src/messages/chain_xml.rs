use super::{MessageBlock, MessageChain};

use regex::{Regex, RegexBuilder};

impl MessageChain {
    /// 从简单的 xml 解析出 [`MessageChain`]
    ///
    /// # 标签
    /// 标签内容都是路径，且是相对于 env:MIRAIE_RESOURCE_ROOT/{voices|images} 的相对路径。
    /// - `i`, `img`, `image`：图片
    /// - `v`, `voice`：音频
    ///
    /// # Example
    /// ```
    /// # use miraie::prelude::*;
    /// # std::env::set_var("MIRAIE_RESOURCE_ROOT", ".");
    /// let chain = MessageChain::from_xml("下面是图片：<img> image.jpg </img>");
    /// ```
    ///
    pub fn from_xml(xml: &str) -> Self {
        lazy_static::lazy_static! {
            static ref START_TAG_PATTERN: Regex = RegexBuilder::new(r"<(i|v|voice|img|image)>")
                .case_insensitive(true)
                .build()
                .unwrap();
        }
        let mut blocks = vec![];

        let mut rest = xml;
        while let Some(cap) = START_TAG_PATTERN.captures(rest) {
            let matches = cap.get(0).unwrap();
            let (tag_start, tag_end) = (matches.start(), matches.end());
            let tag = &cap[1];
            // 处理这个标签之前的文本
            if tag_start > 0 {
                let head = rest[..tag_start].trim();
                if !head.is_empty() {
                    blocks.push(MessageBlock::text(head));
                }
            }

            let after_tag = &rest[tag_end..];

            let end_pat = format!("</{}>", tag);
            if let Some(end_tag_left) = after_tag.find(&end_pat) {
                let body = after_tag[..end_tag_left].trim();
                match tag {
                    "v" | "voice" => blocks.push(MessageBlock::voice_path(body)),
                    "i" | "img" | "image" => blocks.push(MessageBlock::image_path(body)),
                    _ => unreachable!(),
                }
                rest = &after_tag[end_tag_left + end_pat.len()..];
            } else {
                // 没找到结束标签，剩下的全部处理成字符串
                blocks.push(MessageBlock::text(rest));
                // 清空
                rest = &rest[rest.len()..];
            }
        }
        // 处理剩下的文本
        rest = rest.trim();
        if !rest.is_empty() {
            blocks.push(MessageBlock::text(rest));
        }

        Self(blocks)
    }
}

#[test]
fn test_parse_from_xml() {
    std::env::set_var("MIRAIE_RESOURCE_ROOT", ".");
    let s = "hello, world";
    assert_eq!(MessageChain::from_xml(s), MessageChain::new().text(s));

    let s = "<v> filename.silk </v>";
    assert_eq!(
        MessageChain::from_xml(s),
        MessageChain::new().voice_path("filename.silk")
    );

    let s = "prefix <v> filename.silk </v> postfix";
    assert_eq!(
        MessageChain::from_xml(s),
        MessageChain::new()
            .text("prefix")
            .voice_path("filename.silk")
            .text("postfix")
    );

    let s = "prefix <v> filename.silk </v> <image> image.jpg </image> postfix   ";
    assert_eq!(
        MessageChain::from_xml(s),
        MessageChain::new()
            .text("prefix")
            .voice_path("filename.silk")
            .image_path("image.jpg")
            .text("postfix")
    );
    let s = "prefix <v> 1.silk </v> <voice>2.silk</voice>      \n";
    assert_eq!(
        MessageChain::from_xml(s),
        MessageChain::new()
            .text("prefix")
            .voice_path("1.silk")
            .voice_path("2.silk")
    );
}
