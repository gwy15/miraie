use crate::messages::friend;

pub type Request = ();

pub type Response = Vec<friend::FriendMember>;

crate::api!(command = "friendList", Request, Response);
