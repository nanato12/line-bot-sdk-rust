use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Events {
    pub events: Vec<BaseEvent>,
    pub destination: String,
}

#[derive(Deserialize, Debug)]
pub struct BaseEvent {
    pub r#type: String,
    pub mode: String,
    pub timestamp: u64,
    pub source: BaseSource,
    #[serde(rename = "replyToken")]
    pub reply_token: Option<String>,
    pub message: Option<BaseMessage>,
}

#[derive(Deserialize, Debug)]
pub struct BaseSource {
    pub r#type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "roomId")]
    pub room_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BaseMessage {
    pub id: String,
    pub r#type: String,
    pub text: String,
    pub emojis: Option<Vec<Emojis>>,
    pub mention: Option<Mention>,
}

#[derive(Deserialize, Debug)]
pub struct Emojis {
    pub index: i64,
    pub length: i64,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "emojiId")]
    pub emoji_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Mention {
    mentionees: Vec<Mentionee>,
}

#[derive(Deserialize, Debug)]
pub struct Mentionee {
    index: i64,
    length: i64,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}
