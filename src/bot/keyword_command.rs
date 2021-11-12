//! bot 注册的关键词
//!

use parking_lot::RwLock;
use std::future::Future;
use std::{marker::PhantomData, pin::Pin, sync::Arc};

use crate::{
    msg_framework::{FromRequest, Func, Return},
    Bot,
};
type Request = crate::msg_framework::Request<Bot>;

#[derive(Clone, Default)]
pub(crate) struct KeywordCommandHandlers(
    pub(crate) Arc<RwLock<Vec<(String, KeywordCommandHandler)>>>,
);

impl KeywordCommandHandlers {
    pub fn new() -> Self {
        Self::default()
    }
}
impl FromRequest<Bot> for KeywordCommandHandlers {
    fn from_request(request: &crate::msg_framework::Request<Bot>) -> Option<Self> {
        Some(request.app.kw_command_handlers.clone())
    }
}

/// 关键词消息处理的处理回调，是函数
#[derive(Clone)]
pub struct KeywordCommandHandler(Arc<dyn RequestHandler>);

impl KeywordCommandHandler {
    pub fn new<F, T, Fut>(f: F) -> Self
    where
        F: crate::msg_framework::Func<T, Fut> + Send + Sync,
        T: Send + 'static + FromRequest<Bot>,
        Fut: Future + Send + 'static,
        Fut::Output: Return<Bot>,
    {
        let handler = Callable::<F, T, Fut> {
            f,
            _phantom: PhantomData,
        };
        Self(Arc::new(handler))
    }

    pub(crate) async fn handle(&self, request: Request) {
        self.0.handle_request(request).await;
    }
}

trait RequestHandler: Send + Sync + 'static {
    fn handle_request(&self, request: Request) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

struct Callable<F, T, Fut> {
    f: F,
    _phantom: PhantomData<(T, Fut)>,
}
// 这里需要手动 unsafe impl Send，否则 rust 会要求 F 和 Fut 都是 Sync 的，但是
// 它们都只会在一个线程下被访问，不需要 Sync
unsafe impl<F, T, Fut> Send for Callable<F, T, Fut>
where
    F: Send + Sync + 'static,
    T: Send + 'static,
    Fut: Send + 'static,
{
}
unsafe impl<F, T, Fut> Sync for Callable<F, T, Fut>
where
    F: Send + Sync + 'static,
    T: Send + 'static,
    Fut: Send + 'static,
{
}

impl<F, T, Fut> RequestHandler for Callable<F, T, Fut>
where
    Self: 'static,
    F: Send + Sync + 'static + Func<T, Fut>,
    T: Send + 'static + FromRequest<Bot>,
    Fut: Send + 'static + Future,
    Fut::Output: Return<Bot>,
{
    fn handle_request(&self, request: Request) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        match T::from_request(&request) {
            Some(input) => {
                let fut = self.f.call(input);
                Box::pin(async move {
                    let ret = fut.await;
                    ret.on_return(request).await;
                })
            }
            None => Box::pin(async {}),
        }
    }
}
