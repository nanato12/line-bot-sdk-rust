use serde_derive::Deserialize;

// TEXT MESSAGE

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
    pub mentionees: Vec<Mentionee>,
}

#[derive(Deserialize, Debug)]
pub struct Mentionee {
    pub index: i64,
    pub length: i64,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}
