use crate::events::Source;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BeaconEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub beacon: Beacon,
}

#[derive(Deserialize, Debug)]
pub struct Beacon {
    pub hwid: String,
    pub r#type: String,
    pub dm: Option<String>,
}
