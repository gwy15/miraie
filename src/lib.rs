#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

#[macro_use]
pub mod api;

pub mod bot;
pub mod error;
pub mod messages;
mod msg_framework;

pub use api::API;
pub use bot::Bot;
pub use error::{Error, Result};
pub use msg_framework::App;
