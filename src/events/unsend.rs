use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Unsend {
    #[serde(rename = "messageId")]
    pub message_id: String,
}
