use anyhow::*;
use log::*;
use miraie::{
    api,
    bot::QQ,
    messages::{Event, FriendMessage, GroupMessage},
    App, Bot,
};

async fn on_group_msg(group_msg: GroupMessage) {
    info!("group: {:?}", group_msg);
}

async fn on_private_msg(private_msg: FriendMessage, bot: Bot) {
    info!("private: {:?}", private_msg);

    let response = bot.request(api::friend_list::Request).await.unwrap();
    info!("当前好友: {:#?}", response);

    let response = bot.request(api::group_list::Request).await.unwrap();
    info!("加的群：{:#?}", response);

    let response = bot
        .request(api::member_list::Request {
            target: QQ(570197155),
        })
        .await
        .unwrap();
    info!("群里成员：{:#?}", response);
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
