use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Event {
    pub r#type: String,
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub source: Source,
    pub timestamp: u64,
    pub mode: String,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub r#type: String,
    pub id: String,
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct Source {
    pub r#type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "groupId")]
    pub group_id: String,
}
