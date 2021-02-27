use crate::events::Source;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct JoinEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
}
