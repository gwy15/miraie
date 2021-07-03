//! 获取群成员列表
//!
//! 使用此方法获取bot的群列表

use crate::messages::group;

#[derive(Serialize)]
pub struct Request;

pub type Response = Vec<group::Group>;

crate::api!(command = "groupList", Request, Response);
