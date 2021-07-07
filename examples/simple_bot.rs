use anyhow::*;
use futures::StreamExt;
use log::*;
use miraie::{
    api,
    messages::{Event, FriendMessage, GroupMessage, MessageChain},
    App, Bot,
};
use std::time::Duration;
use tokio::time::{sleep, timeout};

async fn on_group_msg_ping_pong(group_msg: GroupMessage, bot: Bot) -> Result<()> {
    if group_msg.message.to_string() == "ping" {
        let resp = bot
            .request(api::send_group_message::Request {
                target: group_msg.sender.group.id,
                message: MessageChain::new().text("pong"),
                quote: group_msg.message.message_id(),
            })
            .await?;
        sleep(Duration::from_secs(5)).await;
        bot.request(api::recall::Request {
            message_id: resp.message_id,
        })
        .await?;
        return Ok(());
    }
    if group_msg.message.to_string() == "谁要喜欢你啊" {
        group_msg
            .reply(MessageChain::new().voice_path(r"谁要喜欢你啊.silk"), &bot)
            .await?;
        info!("请求语音发送成功");
        return Ok(());
    }
    if group_msg.message.to_string() == "起床" {
        group_msg
            .reply(MessageChain::new().voice_path(r"起床.silk"), &bot)
            .await?;
        info!("请求语音发送成功");
        return Ok(());
    }
    Ok(())
}

async fn on_group_msg_confirm(group_msg: GroupMessage, bot: Bot) -> Result<()> {
    if group_msg.message.to_string() == "打嗝" {
        group_msg
            .reply(MessageChain::new().text("真的要打嗝吗？"), &bot)
            .await?;

        let mut reply = group_msg
            .followed_sender_messages(&bot)
            .filter_map(|msg| async move { msg.message.as_confirm() })
            .boxed();
        match timeout(Duration::from_secs(5), reply.next()).await {
            Ok(Some(confirm)) => {
                if confirm {
                    group_msg
                        .reply(MessageChain::new().text("嗝~"), &bot)
                        .await?;
                } else {
                    group_msg
                        .reply(MessageChain::new().text("那就不打了"), &bot)
                        .await?;
                }
            }
            _ => {
                group_msg
                    .reply(MessageChain::new().text("算了不打了"), &bot)
                    .await?;
            }
        }
    }
    Ok(())
}

async fn on_private_msg(private_msg: FriendMessage, bot: Bot) -> Result<()> {
    info!("private: {:?}", private_msg);

    let id = bot
        .request(api::send_friend_message::Request {
            target: private_msg.sender.id,
            quote: None,
            message: MessageChain::new().text("在在在"),
        })
        .await?;
    let message_id = id.message_id;
    info!("response message id: {:?}", message_id);

    tokio::time::sleep(Duration::from_secs(5)).await;
    info!("开始撤回");

    bot.request(api::recall::Request { message_id }).await?;
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
        .handler(on_private_msg)
        .handler(on_event);

    con.run().await?;
    Ok(())
}
