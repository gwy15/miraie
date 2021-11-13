//! 实现 mirai 提供的 API 接口，如拉取群列表等
//!
pub mod common;
pub mod friend_list;
pub mod group_list;
pub mod member_list;
pub mod message_from_id;
pub mod recall;
pub mod send_friend_message;
pub mod send_group_message;

// 私有 mod，用 approve 方法隐藏实现
#[allow(non_snake_case)]
pub(crate) mod resp_botInvitedJoinGroupRequestEvent;

use serde::Serialize;

use crate::Result;

/// 所有发往 mirai 的请求都实现这个 trait
pub trait Api: ApiRequest {
    /// 请求返回的类型
    type Response: serde::de::DeserializeOwned;

    fn process_response(value: serde_json::Value) -> Result<Self::Response>;
}

/// 对应将请求序列化成 ws packet 的行为
pub trait ApiRequest: Send {
    fn command(&self) -> &'static str;

    fn sub_command(&self) -> Option<&'static str>;

    fn encode(&self, sync_id: i64) -> String;
}

#[derive(Serialize)]
pub struct ApiRequestData<T: Serialize> {
    #[serde(rename = "syncId")]
    pub sync_id: i64,

    pub command: &'static str,

    #[serde(rename = "subCommand")]
    pub sub_command: Option<&'static str>,

    pub content: T,
}

/// 用于自动实现跟 mirai-http-api 的 binding code
///
/// 如果你有需要的跟 mirai-http-api 的 API 请求但是本 crate 没实现，你可以使用这个宏在非侵入的形势下自动实现。
///
/// # Example
/// ```
/// use miraie::prelude::*;
/// use miraie::messages::group;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// pub struct Request {
///     pub target: QQ,
/// }
///
/// pub type Response = Vec<group::GroupMember>;
///
/// miraie::api!(command = "memberList", Request, Response);
/// ```

#[macro_export]
macro_rules! api {
    (
        command = $cmd:literal,
        $req:path,
        $rsp:path
    ) => {
        $crate::api!(
            command = $cmd,
            subcommand = None,
            field = "data",
            $req,
            $rsp
        );
    };
    (
        command = $cmd:literal,
        subcommand = $sub_cmd:expr,
        field = $field:tt,
        $req:path,
        $rsp:path
    ) => {
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
        // 定义返回的类型
        $crate::api!(@def_resp field = $field);

        impl $crate::api::Api for $req {
            type Response = $rsp;
            fn process_response(value: serde_json::Value) -> $crate::Result<Self::Response> {
                log::trace!("process value {:?} as response", value);
                let resp: ApiResponseData::<$rsp> = serde_json::from_value(value)?;
                if resp.code != 0 {
                    return Err($crate::Error::Request {
                        code: resp.code,
                        msg: resp.msg,
                    });
                }
                Ok(resp.data)
            }
        }
    };
    (@def_resp field = "data") => {
        #[derive(serde::Deserialize)]
        struct ApiResponseData<T> {
            code: i32,
            msg: String,
            data: T,
        }
    };
    (@def_resp field = "flatten") => {
        #[derive(serde::Deserialize)]
        struct ApiResponseData<T> {
            code: i32,
            msg: String,
            #[serde(flatten)]
            data: T,
        }
    };
    (@def_resp field = "default") => {
        #[derive(serde::Deserialize)]
        struct ApiResponseData<T> {
            code: i32,
            msg: String,
            #[serde(default)]
            data: T,
        }
    }
}
