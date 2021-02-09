Miraie
===

开发中……基于 mirai-api-http 的 QQ 机器人框架。

# Example

下面的示例会对接收到的消息进行计数

```rust
#[macro_use]
extern crate log;

use std::{error::Error, sync::Mutex};

use miraie::{App, Context, Message};

pub async fn message_handler(msg: Message, ctx: Context) -> bool {
    info!("got msg: {:?}", msg);
    if let Some(counter) = ctx.data::<Mutex<u64>>() {
        *counter.lock().unwrap() += 1;
    }
    false
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();
    let base_url = "http://127.0.0.1:19191";
    let qq = 123456;
    let auth_key = "ldsfgnsdofjgnsdjfn";

    let counter = Mutex::new(0u64);

    App::builder()
        .data(counter)
        .bind(base_url, auth_key, &[qq])
        .await?
        .handler(message_handler)
        .build()
        .run()
        .await?;
    Ok(())
}
```
