#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

pub mod api;

pub mod bot;
pub mod error;
pub mod messages;
mod msg_framework;

pub use api::API;
pub use bot::Bot;
pub use error::{Error, Result};
pub use msg_framework::App;

pub mod prelude {
    //! prelude 模块提供一些常用的导入

    pub use super::{
        api,
        bot::QQ,
        messages::{self, FriendMessage, GroupMessage, Message},
        App, Bot, API,
    };
}
