use crate::messages::friend;

#[derive(Serialize)]
pub struct Request;

pub type Response = Vec<friend::FriendMember>;

crate::api!(command = "friendList", Request, Response);
