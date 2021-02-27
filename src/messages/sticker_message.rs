use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct StickerMessage {
    #[serde(rename = "packageId")]
    pub package_id: String,
    #[serde(rename = "stickerId")]
    pub sticker_id: String,
}
