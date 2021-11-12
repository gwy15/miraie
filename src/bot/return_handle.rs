use std::fmt::{Debug, Display};

use crate::messages::MessageChain;
use crate::msg_framework::{Request, Return};
use crate::prelude::{Bot, Conversation};

#[async_trait]
impl<T> Return<Bot> for T
where
    T: Into<MessageChain> + Send + 'static,
{
    async fn on_return(self, request: Request<Bot>) {
        // send back the string
        let response = match request.message {
            crate::messages::Message::Friend(f) => f.reply(self, &request.app).await,
            crate::messages::Message::Group(g) => g.reply(self, &request.app).await,
            _ => {
                warn!("Unsupported message type has return value string.");
                return;
            }
            // TODO
            // crate::messages::Message::Temp(t) => t.reply(self, &request.app).await,
            // crate::messages::Message::Stranger(_) => todo!(),
            // crate::messages::Message::Event(_) => todo!(),
        };
        if let Err(e) = response {
            error!(
                "Error happened when trying to send response to conversation: {}",
                e
            );
        }
    }
}

#[async_trait]
impl<T, E> Return<Bot> for Result<T, E>
where
    T: Into<MessageChain> + Send + 'static,
    E: Send + Display + Debug,
{
    async fn on_return(self, request: Request<Bot>) {
        match self {
            Ok(s) => s.on_return(request).await,
            Err(e) => {
                error!("Error handling request: {}", e);
                debug!("backtrace: {:?}", e);
            }
        }
    }
}

#[async_trait]
impl<T> Return<Bot> for Option<T>
where
    T: Into<MessageChain> + Send + 'static,
{
    async fn on_return(self, request: Request<Bot>) {
        match self {
            Some(msg) => msg.on_return(request).await,
            None => {
                debug!("return is none");
            }
        };
    }
}

#[async_trait]
impl<T, E> Return<Bot> for Result<Option<T>, E>
where
    T: Into<MessageChain> + Send + 'static,
    E: Send + Display + Debug,
{
    async fn on_return(self, request: Request<Bot>) {
        match self {
            Ok(Some(msg)) => msg.on_return(request).await,
            Ok(None) => {
                debug!("return is none");
            }
            Err(e) => {
                error!("Error handling request: {}", e);
                debug!("backtrace: {:?}", e);
            }
        };
    }
}
