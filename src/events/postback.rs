use crate::events::Source;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PostBackEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub postback: PostBack,
}

#[derive(Deserialize, Debug)]
pub struct PostBack {
    pub data: String,
    pub params: Option<Params>,
}

#[derive(Deserialize, Debug)]
pub struct Params {
    pub date: Option<String>,
    pub time: Option<String>,
    pub datetime: Option<String>,
}
