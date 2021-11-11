# miraie 未来へ

[![Build](https://github.com/gwy15/miraie/actions/workflows/build.yml/badge.svg)](https://github.com/gwy15/miraie/actions/workflows/build.yml)
[![docs](https://docs.rs/miraie/badge.svg)](https://docs.rs/miraie)

Miraie 是一个基于 [mirai](https://github.com/mamoe/mirai) 和 [mirai-api-http](https://github.com/project-mirai/mirai-api-http) 的 Rust 机器人框架。

# 特性
- 灵活、自然的对话式写法
- 基于 mirai-api-http，可基于 docker 灵活部署
- 支持 rustls，编译出的机器人可不依赖于 openssl

# Demo
```rust,no_run
use miraie::prelude::*;
use anyhow::*;
use futures::*;

/// 实现一个最简单的 ping-pong 服务，它会对消息 ping 回复 pong，并在五秒后撤回该 pong。
async fn ping_pong_handler<T: Conversation>(msg: T, bot: Bot) -> Result<()> {
    if msg.as_message().to_string().trim() == "ping" {
        let resp = msg.reply("pong", &bot).await?;
        // 五秒后撤回
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        bot.request(api::recall::Request {
            message_id: resp.message_id,
        })
        .await?;
        return Ok(());
    }
    Ok(())
}

/// 这个例子说明了交互式的对话是怎么进行的。该 handler 只对群聊消息生效。
///
/// 消息过程：
/// ```text
///                                     复读一下 <
/// > 真的要复读吗？请在 10 秒内进行确认
///                                        确认 <
/// > 确认成功，复读下一句
///                                Are you ok? <
/// > Are you ok?
/// ```
async fn on_group_msg_confirm(group_msg: GroupMessage, bot: Bot) -> Result<()> {
    if group_msg.message.to_string() == "复读一下" {
        let next_msg = group_msg
            .prompt("真的要复读吗？请在 10 秒内进行确认", &bot)
            .await?;

        if next_msg.message.as_confirm() == Some(true) {
            group_msg.reply("确认成功，复读下一句", &bot).await?;
            log::info!("开始复读，等待下一句");
            // 等待下一句
            let next = group_msg
                .followed_sender_messages(&bot)
                .next()
                .await
                .context("连接断开了哦")?;
            log::info!("复读这一句话：{:?}", next);
            // 进行一个读的复
            next.reply_unquote(next.message.clone(), &bot).await?;
        } else {
            group_msg.reply("确认失败", &bot).await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let (bot, con) = miraie::Bot::new("127.0.0.1:18418", "VERIFY_KEY", QQ(123456)).await?;

    // ping pong 服务对群聊和私聊都进行注册
    bot.handler(ping_pong_handler::<GroupMessage>)
        .handler(ping_pong_handler::<FriendMessage>)
        .handler(on_group_msg_confirm);

    // 取消注释下面一行以运行bot
    // con.run().await?;
    Ok(())
}
```

上面的机器人完整代码可以在 [simple_bot.rs 示例代码](https://github.com/gwy15/miraie/blob/main/examples/simple_bot.rs) 中找到。

一个更完整的、带有管理员、配置文件的机器人示例可以在 [avabot](https://github.com/gwy15/avabot) 找到。


# Rust features
## native-tls
使用 native tls 作为 backend
 
## rustls
使用 rustls 作为 backend

# 后续规划
- [ ] 更多提取器（AtMe，UserAt，Keyword）
- [ ] 补全 `Approve` trait


# 运行时需要提供的环境变量
- `MIRAIE_RESOURCE_ROOT`：资源的根目录，这个需要是 mirai 运行时的目录，如果 mirai 运行在机器 A 上，rust bot 运行在机器 B 上，这个需要是机器 A 上的路径。
