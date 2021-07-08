//! 基于 mirai 和 mirai-api-http 的 QQ 机器人框架
//!
//! # features
//! ## native-tls
//! 使用 native tls 作为 backend
//!
//! ## rustls
//! 使用 rustls 作为 backend

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

pub mod api;

pub mod bot;
pub mod error;
pub mod messages;
mod msg_framework;

pub use api::Api;
pub use bot::Bot;
pub use error::{Error, Result};
pub use msg_framework::App;

pub mod prelude {
    //! prelude 模块提供一些常用的导入

    pub use super::{
        api,
        bot::QQ,
        messages::{FriendMessage, GroupMessage, Message, MessageBlock, MessageChain},
        Api, App, Bot,
    };
}
