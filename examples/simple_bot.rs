use anyhow::*;
use futures::StreamExt;
use log::*;
use miraie::{
    api,
    messages::{Event, GroupMessage},
    App, Bot,
};
use std::time::Duration;
use tokio::time::sleep;

async fn on_group_msg_ping_pong(group_msg: GroupMessage, bot: Bot) -> Result<()> {
    if group_msg.message.to_string() == "ping" {
        let resp = group_msg.reply("pong", &bot).await?;
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

async fn on_group_msg_confirm(group_msg: GroupMessage, bot: Bot) -> Result<()> {
    if group_msg.message.to_string() == "复读一下" {
        let response = group_msg
            .prompt("真的要复读吗？请在 10 秒内进行确认", &bot)
            .await?;

        if response.message.as_confirm() == Some(true) {
            group_msg.reply("确认成功，复读下一句", &bot).await?;
            info!("开始复读，等待下一句");
            // 等待下一句
            let next = group_msg
                .followed_sender_messages(&bot)
                .next()
                .await
                .context("连接断开了哦")?;
            info!("复读这一句话：{:?}", next);
            // 进行一个读的复
            next.unquote_reply(next.message.clone(), &bot).await?;
        } else {
            group_msg.reply("确认失败", &bot).await?;
        }
    }
    Ok(())
}

async fn on_event(event: Event) {
    info!("event: {:?}", event);
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init_timed();
    let (bot, con) = miraie::Bot::new(
        "127.0.0.1:18418".parse().unwrap(),
        "dZujVWpnxxXXE5b",
        std::env::var("MIRAIE_BOT_QQ")
            .expect("请设置 MIRAIE_BOT_QQ 环境变量")
            .parse()
            .unwrap(),
    )
    .await?;
    info!("bot connected.");

    bot.handler(on_group_msg_ping_pong)
        .handler(on_group_msg_confirm)
        .handler(on_event);

    con.run().await?;
    Ok(())
}
