//! 获取群列表
//!
//! 使用此方法获取bot的群列表

use crate::{bot::QQ, messages::group};

#[derive(Serialize)]
pub struct Request {
    pub target: QQ,
}

pub type Response = Vec<group::GroupMember>;

crate::api!(command = "memberList", Request, Response);
