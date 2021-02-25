use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct TextMessage {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emojis: Option<Vec<Emoji>>,
}

#[derive(Serialize, Debug)]
pub struct Emoji {
    pub index: i64,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "emojiId")]
    pub emoji_id: String,
}
