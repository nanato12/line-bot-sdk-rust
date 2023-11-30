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
#[serde(untagged)]
pub enum Params {
    RichMenuSwitch {
        #[serde(rename = "newRichMenuAliasId")]
        new_rich_menu_alias_id: String,
        status: String,
    },
    DatetimePicker {
        date: Option<String>,
        time: Option<String>,
        datetime: Option<String>,
    },
}
