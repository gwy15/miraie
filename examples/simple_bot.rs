use anyhow::*;
use log::*;
use miraie::{
    api,
    messages::{Event, FriendMessage, GroupMessage, MessageChain},
    App, Bot,
};
use std::time::Duration;

async fn on_group_msg(group_msg: GroupMessage) {
    info!("group: {:?}", group_msg);
}

async fn on_private_msg(private_msg: FriendMessage, bot: Bot) -> anyhow::Result<()> {
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
    pretty_env_logger::init_timed();
    let (bot, con) = miraie::Bot::new(
        "127.0.0.1:18418".parse().unwrap(),
        "dZujVWpnxxXXE5b",
        2394345431u64.into(),
    )
    .await?;

    bot.handler(on_group_msg)
        .handler(on_private_msg)
        .handler(on_event);

    con.run().await?;
    Ok(())
}
