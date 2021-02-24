use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Emoji {
    pub index: i64,
    pub length: i64,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "emojiId")]
    pub emoji_id: String,
}
