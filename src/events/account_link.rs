use crate::events::Source;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AccountLinkEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub link: Link,
}

#[derive(Deserialize, Debug)]
pub struct Link {
    pub result: String,
    pub nonce: String,
}
