use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct Emoji {
    pub index: i64,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "emojiId")]
    pub emoji_id: String,
}
