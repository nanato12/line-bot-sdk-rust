use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StickerMessage {
    id: String,
    #[serde(rename = "stickerId")]
    sticker_id: String,
    #[serde(rename = "packageId")]
    package_id: String,
    #[serde(rename = "stickerResourceType")]
    sticker_resource_type: String,
    keywords: Vec<String>,
}
