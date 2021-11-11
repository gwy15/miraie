//! 核心事件框架
mod app;
mod func;
mod requests;
#[cfg(test)]
mod test_msg_framework;

pub use app::{App, Return};
pub use func::Func;
pub use requests::{FromRequest, Request};
