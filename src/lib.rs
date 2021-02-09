#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

#[macro_use]
pub mod utils;

pub mod client;
pub mod error;
pub mod messages;
pub mod mirai;

pub use error::{Error, Result};

pub type QQ = u64;
pub type DateTime = chrono::DateTime<chrono::FixedOffset>;
