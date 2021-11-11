#![doc = include_str!("../README.md")]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate async_trait;

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
        messages::{
            events, Conversation, Event, FriendMessage, GroupMessage, Message, MessageBlock,
            MessageChain,
        },
        Api, App, Bot,
    };
}
