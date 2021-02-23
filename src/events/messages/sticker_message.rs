use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StickerMessage {
    pub id: String,
    #[serde(rename = "stickerId")]
    pub sticker_id: String,
    #[serde(rename = "packageId")]
    pub package_id: String,
    #[serde(rename = "stickerResourceType")]
    pub sticker_resource_type: String,
    pub keywords: Vec<String>,
}
