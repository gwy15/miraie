pub mod friend_list;

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

pub trait API: ApiRequest {
    type Response: serde::de::DeserializeOwned;

    fn process_response(value: serde_json::Value) -> Result<Self::Response> {
        let resp: ApiResponseData<Self::Response> = serde_json::from_value(value)?;
        if resp.code != 0 {
            return Err(Error::Request {
                code: resp.code,
                msg: resp.msg,
            });
        }
        Ok(resp.data)
    }
}

/// 对应将请求序列化成 ws packet 的行为
pub trait ApiRequest: Send {
    fn command(&self) -> &'static str;

    fn sub_command(&self) -> Option<&'static str>;

    fn encode(&self, sync_id: i64) -> String;
}

#[derive(Serialize)]
struct ApiRequestData<T: Serialize> {
    #[serde(rename = "syncId")]
    sync_id: i64,

    command: &'static str,

    #[serde(rename = "subCommand")]
    sub_command: Option<&'static str>,

    content: T,
}

#[derive(Deserialize)]
struct ApiResponseData<T> {
    code: i32,
    msg: String,
    data: T,
}

#[macro_export]
macro_rules! api {
    (command=$cmd:literal, $req:path, $rsp:path) => {
        $crate::api!(command = $cmd, subcommand = None, $req, $rsp);
    };
    (command=$cmd:literal, subcommand=$sub_cmd:expr, $req:path, $rsp:path) => {
        impl $crate::api::ApiRequest for $req {
            fn command(&self) -> &'static str {
                $cmd
            }
            fn sub_command(&self) -> Option<&'static str> {
                $sub_cmd
            }
            fn encode(&self, sync_id: i64) -> String {
                let request = $crate::api::ApiRequestData {
                    command: self.command(),
                    sub_command: self.sub_command(),
                    sync_id,
                    content: &self,
                };
                serde_json::to_string(&request).unwrap()
            }
        }
        impl $crate::api::API for $req {
            type Response = $rsp;
            // fn request(self, )
        }
    };
}
