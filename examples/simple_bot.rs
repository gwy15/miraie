use anyhow::*;
use futures::StreamExt;
use log::*;
use miraie::{messages::events::Approvable, prelude::*};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug)]
pub struct Config {
    name: String,
}

/// 实现一个最简单的 ping-pong 服务，它会对消息 ping 回复 pong，并在五秒后撤回该 pong。
async fn ping_pong_handler<T: Conversation>(msg: T, bot: Bot) -> Result<()> {
    if msg.as_message().to_string().trim() == "ping" {
        let resp = msg.reply("pong", &bot).await?;
        // 五秒后撤回
        sleep(Duration::from_secs(5)).await;
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
            info!("开始复读，等待下一句");
            // 等待下一句
            let mut next = group_msg
                .followed_sender_messages(&bot)
                .next()
                .await
                .context("连接断开了哦")?;
            info!("复读这一句话：{:?}", next);
            // 进行一个读的复
            let msg = next.message.take();
            next.reply_unquote(msg, &bot).await?;
        } else {
            group_msg.reply("确认失败", &bot).await?;
        }
    }
    Ok(())
}

/// 在日志打印所有的事件（Event 事件，如管理员收到的加群请求等）
async fn on_event(event: Event) {
    info!("event: {:?}", event);
}

async fn on_group_invite(event: events::BotInvitedJoinGroupRequestEvent, bot: Bot) -> Result<()> {
    info!("邀请我进去群: {:?}", event);
    info!("加入群！");
    event.approve(&bot).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init_timed();
    let (bot, con) = miraie::Bot::new(
        "127.0.0.1:18418",
        // 我开发时使用的，你应该换成自己的
        "dZujVWpnxxXXE5b",
        std::env::var("MIRAIE_BOT_QQ")
            .expect("请设置 MIRAIE_BOT_QQ 环境变量")
            .parse()
            .expect("无效的 MIRAIE_BOT_QQ 环境变量"),
    )
    .await?;
    info!("bot connected.");

    // 可以直接使用 `command` 注册指令，可以直接返回字符串以进行回复
    bot.command("你好", |_: GroupMessage| async { "你好" })
        // 返回值也可以是 `Result<String>`
        .command("在吗", |_: GroupMessage| async {
            Result::<&'static str>::Ok("嗯嗯")
        })
        // 或者手动调用 `Bot` 的 `reply` 接口
        .command("在干什么", |msg: GroupMessage, bot: Bot| async move {
            msg.reply("你有事吗", &bot).await?;
            Result::<(), Error>::Ok(())
        })
        // 返回 `Option<String>`，注意这里的参数是 FriendMessage，因此这个接口只会对好友私聊消息生效。
        // 如果想要对所有的消息都生效，可以将 `FriendMessage` 修改为 `Message`。
        .command("嘉然我真的好喜欢你啊", |_: FriendMessage| async {
            Some("...")
        })
        // 或者 `Result<Option<String>>` 等
        .command("晚晚我真的好喜欢你啊", |_: Message| async {
            Result::<_, Error>::Ok(Some("mua"))
        })
        // 下面的几个例子可以对所有的事件都进行注册，不会被关键词过滤
        // ping pong 服务对群聊和私聊都进行注册
        .handler(ping_pong_handler::<GroupMessage>)
        .handler(ping_pong_handler::<FriendMessage>)
        .handler(on_group_msg_confirm)
        .handler(on_event)
        .handler(on_group_invite)
        // 可以使用 `bot_data` 注册配置/数据库连接池等，并使用 `Data` 进行提取。
        .bot_data(Data::new(Config {
            name: "A-SOUL!".to_string(),
        }))
        .command("配置", |config: Data<Config>| async move {
            // 会在任何聊天中回复【A-SOUL!】
            config.name.clone()
        });

    con.run().await?;
    Ok(())
}
