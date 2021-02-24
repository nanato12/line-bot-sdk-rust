use crate::events::Member;
use crate::events::Source;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MemberJoinEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub joined: Joined,
}

#[derive(Deserialize, Debug)]
pub struct Joined {
    pub members: Vec<Member>,
}
