pub mod friend_list;

use serde::Serialize;

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

    fn sub_command(&self) -> Option<&'static str>;

    fn encode(&self, sync_id: i64) -> String;
}

#[macro_export]
macro_rules! api {
    (command=$cmd:literal, $req:path, $rsp:path) => {
        $crate::api!(command = $cmd, subcommand = None, $req, $rsp);
    };
    (command=$cmd:literal, subcommand=$sub_cmd:expr, $req:path, $rsp:path) => {
        impl $crate::api::API for $req {
            fn command(&self) -> &'static str {
                $cmd
            }
            fn sub_command(&self) -> Option<&'static str> {
                $sub_cmd
            }
            fn encode(&self, sync_id: i64) -> String {
                let request = $crate::api::APIRequest {
                    command: self.command(),
                    sub_command: self.sub_command(),
                    sync_id,
                    content: &self,
                };
                serde_json::to_string(&request).unwrap()
            }
        }
    };
}
