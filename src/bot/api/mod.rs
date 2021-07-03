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

    fn sub_command(&self) -> Option<&'static str> {
        None
    }

    fn encode(&self, sync_id: i64) -> String;
    //     let request = APIRequest {
    //         command: self.command(),
    //         sub_command: self.sub_command(),
    //         sync_id,
    //         content: &self,
    //     };
    //     serde_json::to_string(&request).unwrap()
    // }
}
