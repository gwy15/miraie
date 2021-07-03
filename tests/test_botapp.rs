use std::sync::atomic::{AtomicUsize, Ordering};

use miraie::{messages::GroupMessage, App, Bot};

static COUNT: AtomicUsize = AtomicUsize::new(0);

async fn on_group_message(_msg: GroupMessage) {
    COUNT.fetch_add(1, Ordering::SeqCst);
}

#[tokio::test]
async fn test_bot_app() {
    let bot = Bot::new(10)
        .handler(on_group_message)
        .handler(on_group_message);
    std::mem::drop(bot);
}
