//! 获取好友列表
//!
//! 使用此方法获取bot的好友列表

use crate::messages::friend;

#[derive(Serialize)]
pub struct Request;

pub type Response = Vec<friend::FriendMember>;

crate::api!(command = "friendList", Request, Response);
