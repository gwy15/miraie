//! 包含了所有可能的事件，如接收到加群申请、bot 被禁言等
//!

use chrono::{DateTime, Utc};

use super::{friend, group};
use crate::bot::QQ;

/// 事件，如管理员收到的加群请求等
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Event {
    /// Bot登录成功
    BotOnlineEvent(BotOnlineEvent),
    /// Bot主动离线
    BotOfflineEventActive(BotOfflineEventActive),
    /// Bot被挤下线
    BotOfflineEventForce(BotOfflineEventForce),
    /// Bot被服务器断开或因网络问题而掉线
    BotOfflineEventDropped(BotOfflineEventDropped),
    /// Bot主动重新登录
    BotReloginEvent(BotReloginEvent),
    /// 好友输入状态改变
    FriendInputStatusChangedEvent(FriendInputStatusChangedEvent),
    /// 好友昵称改变
    FriendNickChangedEvent(FriendNickChangedEvent),
    /// Bot在群里的权限被改变. 操作人一定是群主
    BotGroupPermissionChangeEvent(BotGroupPermissionChangeEvent),
    /// Bot被禁言
    BotMuteEvent(BotMuteEvent),
    /// Bot被取消禁言
    BotUnmuteEvent(BotUnmuteEvent),
    /// Bot加入了一个新群
    BotJoinGroupEvent(BotJoinGroupEvent),
    /// Bot主动退出一个群
    BotLeaveEventActive(BotLeaveEventActive),
    /// Bot被踢出一个群
    BotLeaveEventKick(BotLeaveEventKick),
    /// 群消息撤回
    GroupRecallEvent(GroupRecallEvent),
    /// 好友消息撤回
    FriendRecallEvent(FriendRecallEvent),
    /// 某个群名改变
    GroupNameChangeEvent(GroupNameChangeEvent),
    /// 某群入群公告改变
    GroupEntranceAnnouncementChangeEvent(GroupEntranceAnnouncementChangeEvent),
    /// 全员禁言状态改变
    GroupMuteAllEvent(GroupMuteAllEvent),
    /// 匿名聊天状态改变
    GroupAllowAnonymousChatEvent(GroupAllowAnonymousChatEvent),
    /// 坦白说状态改变
    GroupAllowConfessTalkEvent(GroupAllowConfessTalkEvent),
    /// 允许群员邀请好友加群
    GroupAllowMemberInviteEvent(GroupAllowMemberInviteEvent),
    /// 新人入群的事件
    MemberJoinEvent(MemberJoinEvent),
    /// 成员被踢出群（该成员不是Bot）
    MemberLeaveEventKick(MemberLeaveEventKick),
    /// 成员主动离群（该成员不是Bot）
    MemberLeaveEventQuit(MemberLeaveEventQuit),
    /// 群名片改动
    MemberCardChangeEvent(MemberCardChangeEvent),
    /// 群头衔改动（只有群主有操作限权）
    MemberSpecialTitleChangeEvent(MemberSpecialTitleChangeEvent),
    /// 成员权限改变的事件（该成员不是Bot）
    MemberPermissionChangeEvent(MemberPermissionChangeEvent),
    /// 群成员被禁言事件（该成员不是Bot）
    MemberMuteEvent(MemberMuteEvent),
    /// 群成员被取消禁言事件（该成员不是Bot）
    MemberUnmuteEvent(MemberUnmuteEvent),
    /// 群员称号改变
    MemberHonorChangeEvent(MemberHonorChangeEvent),
    /// 添加好友申请
    NewFriendRequestEvent(NewFriendRequestEvent),
    /// 用户入群申请（Bot需要有管理员权限）
    MemberJoinRequestEvent(MemberJoinRequestEvent),
    /// Bot被邀请入群申请
    BotInvitedJoinGroupRequestEvent(BotInvitedJoinGroupRequestEvent),
    /// 命令被执行
    CommandExecutedEvent(CommandExecutedEvent),
}

impl crate::msg_framework::FromRequest<crate::Bot> for Event {
    fn from_request(request: crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
        match request.message {
            crate::messages::Message::Event(e) => Some(e),
            _ => None,
        }
    }
}

/// Bot登录成功
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct BotOnlineEvent {
    qq: QQ,
}
/// Bot主动离线
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct BotOfflineEventActive {
    qq: QQ,
}
/// Bot被挤下线
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct BotOfflineEventForce {
    qq: QQ,
}
/// Bot被服务器断开或因网络问题而掉线
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct BotOfflineEventDropped {
    qq: QQ,
}
/// Bot主动重新登录
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct BotReloginEvent {
    qq: QQ,
}

/// 好友输入状态改变
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct FriendInputStatusChangedEvent {
    friend: friend::FriendMember,
    /// 当前输出状态是否正在输入
    inputting: bool,
}
/// 好友昵称改变
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct FriendNickChangedEvent {
    friend: friend::FriendMember,
    from: String,
    to: String,
}
/// Bot在群里的权限被改变. 操作人一定是群主
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct BotGroupPermissionChangeEvent {
    origin: group::Permission,
    current: group::Permission,
    group: group::Group,
}
/// Bot被禁言
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct BotMuteEvent {
    /// 禁言时长，单位为秒
    #[serde(rename = "durationSeconds")]
    seconds: u32,
    operator: group::GroupMember,
}
/// Bot被取消禁言
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct BotUnmuteEvent {
    operator: group::GroupMember,
}
/// Bot加入了一个新群
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct BotJoinGroupEvent {
    group: group::Group,
}
/// Bot主动退出一个群
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct BotLeaveEventActive {
    group: group::Group,
}
/// Bot被踢出一个群
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct BotLeaveEventKick {
    group: group::Group,
}
/// 群消息撤回
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct GroupRecallEvent {
    /// 原消息发送者的QQ号
    #[serde(rename = "authorId")]
    author: QQ,
    /// 原消息messageId
    #[serde(rename = "messageId")]
    message_id: i64,
    /// 原消息发送时间
    #[serde(with = "chrono::serde::ts_seconds")]
    time: DateTime<Utc>,
    /// 消息撤回所在的群
    group: group::Group,
    /// 撤回消息的操作人，当null时为bot操作
    operator: Option<group::GroupMember>,
}
/// 好友消息撤回
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct FriendRecallEvent {
    /// 原消息发送者的QQ号
    #[serde(rename = "authorId")]
    author: QQ,
    /// 原消息messageId
    #[serde(rename = "messageId")]
    message_id: i64,
    /// 原消息发送时间
    #[serde(with = "chrono::serde::ts_seconds")]
    time: DateTime<Utc>,
    /// 好友QQ号或BotQQ号
    operator: QQ,
}
/// 某个群名改变
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct GroupNameChangeEvent {
    /// 原群名
    origin: String,
    /// 新群名
    current: String,
    group: group::Group,
    /// 操作的管理员或群主信息，当null时为Bot操作
    operator: Option<group::GroupMember>,
}
/// 某群入群公告改变
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct GroupEntranceAnnouncementChangeEvent {
    /// 原公告
    origin: String,
    /// 新公告
    current: String,
    group: group::Group,
    /// 操作的管理员或群主信息，当null时为Bot操作
    operator: Option<group::GroupMember>,
}
/// 全员禁言状态改变
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct GroupMuteAllEvent {
    /// 原本是否处于全员禁言
    origin: bool,
    /// 现在是否处于全员禁言
    current: bool,
    group: group::Group,
    /// 操作的管理员或群主信息，当null时为Bot操作
    operator: Option<group::GroupMember>,
}
/// 匿名聊天状态改变
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct GroupAllowAnonymousChatEvent {
    /// 原本是否处于全员禁言
    origin: bool,
    /// 现在是否处于全员禁言
    current: bool,
    group: group::Group,
    /// 操作的管理员或群主信息，当null时为Bot操作
    operator: Option<group::GroupMember>,
}
/// 坦白说状态改变
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct GroupAllowConfessTalkEvent {
    /// 原本坦白说是否开启
    origin: bool,
    /// 现在坦白说是否开启
    current: bool,
    group: group::Group,
    /// 是否Bot进行该操作
    #[serde(rename = "isByBot")]
    is_by_bot: bool,
}
/// 允许群员邀请好友加群
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct GroupAllowMemberInviteEvent {
    /// 原本是否允许群员邀请好友加群
    origin: bool,
    /// 现在是否允许群员邀请好友加群
    current: bool,
    group: group::Group,
    /// 操作的管理员或群主信息，当null时为Bot操作
    operator: Option<group::GroupMember>,
}
/// 新人入群的事件
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MemberJoinEvent {
    /// 新人信息
    member: group::GroupMember,
}
/// 成员被踢出群（该成员不是Bot）
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MemberLeaveEventKick {
    member: group::GroupMember,
    operator: Option<group::GroupMember>,
}
/// 成员主动离群（该成员不是Bot）
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MemberLeaveEventQuit {
    member: group::GroupMember,
}
/// 群名片改动
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MemberCardChangeEvent {
    origin: String,
    current: String,
    member: group::GroupMember,
}
/// 群头衔改动（只有群主有操作限权）
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MemberSpecialTitleChangeEvent {
    origin: String,
    current: String,
    member: group::GroupMember,
}
/// 成员权限改变的事件（该成员不是Bot）
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MemberPermissionChangeEvent {
    origin: group::Permission,
    current: group::Permission,
    member: group::GroupMember,
}
/// 群成员被禁言事件（该成员不是Bot）
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MemberMuteEvent {
    /// 禁言时长，单位为秒
    #[serde(rename = "durationSeconds")]
    seconds: u32,
    member: group::GroupMember,
    operator: Option<group::GroupMember>,
}
/// 群成员被取消禁言事件（该成员不是Bot）
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MemberUnmuteEvent {
    member: group::GroupMember,
    operator: Option<group::GroupMember>,
}
/// 群员称号改变
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MemberHonorChangeEvent {
    member: group::GroupMember,
    /// 称号变化行为：achieve获得成好，lose失去称号
    action: String,
    /// 称号名称, e.g., 龙王
    honor: String,
}
/// 添加好友申请
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct NewFriendRequestEvent {
    /// 事件标识，响应该事件时的标识
    #[serde(rename = "eventId")]
    event_id: i64,
    /// 申请人QQ号
    #[serde(rename = "fromId")]
    from_id: QQ,
    /// 申请人如果通过某个群添加好友，该项为该群群号；否则为0
    #[serde(rename = "groupId")]
    group_id: QQ,
    /// 申请人的昵称或群名片
    nick: String,
    /// 申请消息
    message: String,
}
/// 用户入群申请（Bot需要有管理员权限）
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MemberJoinRequestEvent {
    /// 事件标识，响应该事件时的标识
    #[serde(rename = "eventId")]
    event_id: i64,

    /// 申请人QQ号
    #[serde(rename = "fromId")]
    from_id: QQ,

    /// 申请人申请入群的群号
    #[serde(rename = "groupId")]
    group_id: QQ,

    /// 申请人申请入群的群名称
    #[serde(rename = "groupName")]
    group_name: String,

    /// 申请人的昵称或群名片
    nick: String,

    /// 申请消息
    message: String,
}
/// Bot被邀请入群申请
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct BotInvitedJoinGroupRequestEvent {
    /// 事件标识，响应该事件时的标识
    #[serde(rename = "eventId")]
    event_id: i64,

    /// 邀请人（好友）的QQ号
    #[serde(rename = "fromId")]
    from_id: QQ,

    /// 被邀请进入群的群号
    #[serde(rename = "groupId")]
    group_id: QQ,

    /// 被邀请进入群的群名称
    #[serde(rename = "groupName")]
    group_name: String,

    /// 邀请人（好友）的昵称
    nick: String,

    /// 命令被执行
    message: String,
}

/// 命令被执行
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct CommandExecutedEvent {
    #[serde(rename = "eventId")]
    event_id: i64,
    name: String,
    friend: Option<friend::FriendMember>,
    member: Option<group::GroupMember>,
    args: Vec<serde_json::Value>,
}

/// 自动实现 FromRequest
macro_rules! auto_impl {
    ($($event:tt,)*) => {
        $(auto_impl!(@impl $event);)*
    };
    (@impl $event:tt) => {
        impl crate::msg_framework::FromRequest<crate::Bot> for $event {
            fn from_request(request: crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
                match request.message {
                    crate::messages::Message::Event(
                        Event::$event(e)
                    ) => Some(e),
                    _ => None,
                }
            }
        }
    };
}
auto_impl! {
    BotOnlineEvent,
    BotOfflineEventActive,
    BotOfflineEventForce,
    BotOfflineEventDropped,
    BotReloginEvent,
    FriendInputStatusChangedEvent,
    FriendNickChangedEvent,
    BotGroupPermissionChangeEvent,
    BotMuteEvent,
    BotUnmuteEvent,
    BotJoinGroupEvent,
    BotLeaveEventActive,
    BotLeaveEventKick,
    GroupRecallEvent,
    FriendRecallEvent,
    GroupNameChangeEvent,
    GroupEntranceAnnouncementChangeEvent,
    GroupMuteAllEvent,
    GroupAllowAnonymousChatEvent,
    GroupAllowConfessTalkEvent,
    GroupAllowMemberInviteEvent,
    MemberJoinEvent,
    MemberLeaveEventKick,
    MemberLeaveEventQuit,
    MemberCardChangeEvent,
    MemberSpecialTitleChangeEvent,
    MemberPermissionChangeEvent,
    MemberMuteEvent,
    MemberUnmuteEvent,
    MemberHonorChangeEvent,
    NewFriendRequestEvent,
    MemberJoinRequestEvent,
    BotInvitedJoinGroupRequestEvent,
    CommandExecutedEvent,
}

#[test]
fn test_parse_event() {
    let s = r#"{
        "type": "BotOnlineEvent",
        "qq": 123
    }"#;
    let evt: Event = serde_json::from_str(s).unwrap();
    assert_eq!(evt, Event::BotOnlineEvent(BotOnlineEvent { qq: QQ(123) }));
}
