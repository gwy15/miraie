#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

pub mod bot;
pub mod error;
pub mod messages;
pub mod msg_framework;

pub use bot::Bot;
pub use error::{Error, Result};
pub use msg_framework::App;
