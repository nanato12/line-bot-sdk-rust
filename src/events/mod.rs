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

pub use account_link::AccountLinkEvent;
pub use beacon::BeaconEvent;
pub use follow::FollowEvent;
pub use join::JoinEvent;
pub use leave::LeaveEvent;
pub use member::Member;
pub use member_join::MemberJoinEvent;
pub use member_leave::MemberLeaveEvent;
pub use messages::MessageEvent;
pub use postback::PostBackEvent;
pub use source::Source;
pub use things::ThingsEvent;
pub use unfollow::UnFollowEvent;
pub use unsend::UnsendEvent;
pub use video_play_complete::VideoPlayCompleteEvent;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Events {
    pub events: Vec<Event>,
    pub destination: String,
}

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(flatten)]
    pub r#type: EventType,
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
    #[serde(rename = "beacon")]
    BeaconEvent(BeaconEvent),
    #[serde(rename = "accountLink")]
    AccountLinkEvent(AccountLinkEvent),
    #[serde(rename = "things")]
    ThingsEvent(ThingsEvent),
    #[serde(rename = "message")]
    MessageEvent(MessageEvent),
    #[serde(other)]
    Other,
}
