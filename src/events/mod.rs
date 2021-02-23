pub mod account_link;
pub mod beacon;
pub mod follow;
pub mod join;
pub mod leave;
pub mod member;
pub mod member_join;
pub mod member_leave;
pub mod messages;
pub mod postback;
pub mod source;
pub mod things;
pub mod unfollow;
pub mod unsend;
pub mod video_play_complete;

pub use follow::FollowEvent;
pub use join::JoinEvent;
pub use leave::LeaveEvent;
pub use member::Member;
pub use member_join::MemberJoinEvent;
pub use member_leave::MemberLeaveEvent;
pub use postback::PostBackEvent;
pub use source::Source;
pub use unfollow::UnFollowEvent;
pub use unsend::UnsendEvent;
use video_play_complete::VideoPlayCompleteEvent;

use serde_derive::Deserialize;

use account_link::Link;
use beacon::Beacon;
use member_leave::Left;
use messages::BaseMessage;
use postback::PostBack;
use things::Things;
use unsend::Unsend;
use video_play_complete::VideoPlayComplete;

#[derive(Deserialize, Debug)]
pub struct Events {
    pub events: Vec<Event>,
    pub destination: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum EventType {
    #[serde(rename = "unsend")]
    UnsendEvent(UnsendEvent),
    #[serde(rename = "follow")]
    FollowEvent(FollowEvent),
    #[serde(rename = "unfollow")]
    UnFollowEvent(UnFollowEvent),
    #[serde(rename = "join")]
    JoinEvent(JoinEvent),
    #[serde(rename = "leave")]
    LeaveEvent(LeaveEvent),
    #[serde(rename = "memberJoined")]
    MemberJoinEvent(MemberJoinEvent),
    #[serde(rename = "memberLeft")]
    MemberLeaveEvent(MemberLeaveEvent),
    #[serde(rename = "postback")]
    PostBackEvent(PostBackEvent),
    #[serde(rename = "videoPlayComplete")]
    VideoPlayCompleteEvent(VideoPlayCompleteEvent),
    Other,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(flatten)]
    pub r#type: EventType,
}

#[derive(Deserialize, Debug)]
pub struct BaseEvent {
    pub r#type: String,
    pub mode: String,
    pub timestamp: u64,
    pub source: Source,
    #[serde(rename = "replyToken")]
    pub reply_token: Option<String>,
    pub message: Option<BaseMessage>,
    // unsend
    pub unsend: Option<Unsend>,
    // member leave_
    pub left: Option<Left>,
    // postback
    pub postback: Option<PostBack>,
    // videoPlayComplete
    #[serde(rename = "videoPlayComplete")]
    pub video_play_complete: Option<VideoPlayComplete>,
    // beacon
    pub beacon: Option<Beacon>,
    // accountLink
    pub link: Option<Link>,
    // things
    pub things: Option<Things>,
}
