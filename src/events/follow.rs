use crate::events::Source;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FollowEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: Option<String>,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
}
