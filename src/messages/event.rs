use chrono::{DateTime, Utc};

use super::{friend, group};
use crate::bot::QQ;

/// 事件，如管理员收到的加群请求等
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Event {
    /// Bot登录成功
    BotOnlineEvent { qq: QQ },
    /// Bot主动离线
    BotOfflineEventActive { qq: QQ },
    /// Bot被挤下线
    BotOfflineEventForce { qq: QQ },
    /// Bot被服务器断开或因网络问题而掉线
    BotOfflineEventDropped { qq: QQ },
    /// Bot主动重新登录
    BotReloginEvent { qq: QQ },

    /// 好友输入状态改变
    FriendInputStatusChangedEvent {
        friend: friend::FriendMember,
        /// 当前输出状态是否正在输入
        inputting: bool,
    },
    /// 好友昵称改变
    FriendNickChangedEvent {
        friend: friend::FriendMember,
        from: String,
        to: String,
    },
    /// Bot在群里的权限被改变. 操作人一定是群主
    BotGroupPermissionChangeEvent {
        origin: group::Permission,
        current: group::Permission,
        group: group::Group,
    },
    /// Bot被禁言
    BotMuteEvent {
        /// 禁言时长，单位为秒
        #[serde(rename = "durationSeconds")]
        seconds: u32,
        operator: group::GroupMember,
    },
    /// Bot被取消禁言
    BotUnmuteEvent { operator: group::GroupMember },
    /// Bot加入了一个新群
    BotJoinGroupEvent { group: group::Group },
    /// Bot主动退出一个群
    BotLeaveEventActive { group: group::Group },
    /// Bot被踢出一个群
    BotLeaveEventKick { group: group::Group },
    /// 群消息撤回
    GroupRecallEvent {
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
    },
    /// 好友消息撤回
    FriendRecallEvent {
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
    },
    /// 某个群名改变
    GroupNameChangeEvent {
        /// 原群名
        origin: String,
        /// 新群名
        current: String,
        group: group::Group,
        /// 操作的管理员或群主信息，当null时为Bot操作
        operator: Option<group::GroupMember>,
    },
    /// 某群入群公告改变
    GroupEntranceAnnouncementChangeEvent {
        /// 原公告
        origin: String,
        /// 新公告
        current: String,
        group: group::Group,
        /// 操作的管理员或群主信息，当null时为Bot操作
        operator: Option<group::GroupMember>,
    },
    /// 全员禁言状态改变
    GroupMuteAllEvent {
        /// 原本是否处于全员禁言
        origin: bool,
        /// 现在是否处于全员禁言
        current: bool,
        group: group::Group,
        /// 操作的管理员或群主信息，当null时为Bot操作
        operator: Option<group::GroupMember>,
    },
    /// 匿名聊天状态改变
    GroupAllowAnonymousChatEvent {
        /// 原本是否处于全员禁言
        origin: bool,
        /// 现在是否处于全员禁言
        current: bool,
        group: group::Group,
        /// 操作的管理员或群主信息，当null时为Bot操作
        operator: Option<group::GroupMember>,
    },
    /// 坦白说状态改变
    GroupAllowConfessTalkEvent {
        /// 原本坦白说是否开启
        origin: bool,
        /// 现在坦白说是否开启
        current: bool,
        group: group::Group,
        /// 是否Bot进行该操作
        #[serde(rename = "isByBot")]
        is_by_bot: bool,
    },
    /// 允许群员邀请好友加群
    GroupAllowMemberInviteEvent {
        /// 原本是否允许群员邀请好友加群
        origin: bool,
        /// 现在是否允许群员邀请好友加群
        current: bool,
        group: group::Group,
        /// 操作的管理员或群主信息，当null时为Bot操作
        operator: Option<group::GroupMember>,
    },
    /// 新人入群的事件
    MemberJoinEvent {
        /// 新人信息
        member: group::GroupMember,
    },
    /// 成员被踢出群（该成员不是Bot）
    MemberLeaveEventKick {
        member: group::GroupMember,
        operator: Option<group::GroupMember>,
    },
    /// 成员主动离群（该成员不是Bot）
    MemberLeaveEventQuit { member: group::GroupMember },
    /// 群名片改动
    MemberCardChangeEvent {
        origin: String,
        current: String,
        member: group::GroupMember,
    },
    /// 群头衔改动（只有群主有操作限权）
    MemberSpecialTitleChangeEvent {
        origin: String,
        current: String,
        member: group::GroupMember,
    },
    /// 成员权限改变的事件（该成员不是Bot）
    MemberPermissionChangeEvent {
        origin: group::Permission,
        current: group::Permission,
        member: group::GroupMember,
    },
    /// 群成员被禁言事件（该成员不是Bot）
    MemberMuteEvent {
        /// 禁言时长，单位为秒
        #[serde(rename = "durationSeconds")]
        seconds: u32,
        member: group::GroupMember,
        operator: Option<group::GroupMember>,
    },
    /// 群成员被取消禁言事件（该成员不是Bot）
    MemberUnmuteEvent {
        member: group::GroupMember,
        operator: Option<group::GroupMember>,
    },
    /// 群员称号改变
    MemberHonorChangeEvent {
        member: group::GroupMember,
        /// 称号变化行为：achieve获得成好，lose失去称号
        action: String,
        /// 称号名称, e.g., 龙王
        honor: String,
    },
    /// 添加好友申请
    NewFriendRequestEvent {
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
    },
    /// 用户入群申请（Bot需要有管理员权限）
    MemberJoinRequestEvent {
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
    },
    /// Bot被邀请入群申请
    BotInvitedJoinGroupRequestEvent {
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
    },

    /// 命令被执行
    CommandExecutedEvent {
        #[serde(rename = "eventId")]
        event_id: i64,
        name: String,
        friend: Option<friend::FriendMember>,
        member: Option<group::GroupMember>,
        args: Vec<serde_json::Value>,
    },
}

impl crate::msg_framework::FromRequest<crate::Bot> for Event {
    fn from_request(request: crate::msg_framework::Request<crate::Bot>) -> Option<Self> {
        match request.message {
            crate::messages::Message::Event(e) => Some(e),
            _ => None,
        }
    }
}

#[test]
fn test_parse_event() {
    let s = r#"{
        "type": "BotOnlineEvent",
        "qq": 123
    }"#;
    let evt: Event = serde_json::from_str(s).unwrap();
    assert_eq!(evt, Event::BotOnlineEvent { qq: QQ(123) });
}
