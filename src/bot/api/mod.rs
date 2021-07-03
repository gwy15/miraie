use crate::Result;
use serde::Serialize;
use std::{any::Any, future::Future, pin::Pin, sync::atomic::AtomicI64};

pub type FutureResponse<T> = Pin<Box<dyn Future<Output = Result<T>>>>;

static SYNC_ID: AtomicI64 = AtomicI64::new(10);

#[derive(Serialize)]
struct APIRequest<T: Serialize> {
    #[serde(rename = "syncId")]
    sync_id: i64,

    command: &'static str,

    #[serde(rename = "subCommand")]
    sub_command: Option<&'static str>,

    content: T,
}

pub trait API: Send {
    fn command(&self) -> &'static str;

    fn sub_command(&self) -> Option<&'static str> {
        None
    }

    fn encode(&self) -> (i64, String) {
        unimplemented!()
        // let sync_id = SYNC_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // let request = APIRequest {
        //     command: self.command(),
        //     sub_command: self.sub_command(),
        //     sync_id,
        //     content: &self,
        // };
        // (sync_id, serde_json::to_string(&request).unwrap())
    }
}
