pub mod content;
pub mod text;

use content::ContentProvider;
use serde_derive::Deserialize;
use text::{Emojis, Mention};

#[derive(Deserialize, Debug)]
pub struct BaseMessage {
    pub id: String,
    pub r#type: String,
    // text
    pub text: Option<String>,
    pub emojis: Option<Vec<Emojis>>,
    pub mention: Option<Mention>,
    // image or video
    #[serde(rename = "contentProvider")]
    pub content_provider: Option<ContentProvider>,
    // audio
    pub duration: Option<i64>,
    // file
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "fileSize")]
    pub file_size: Option<i64>,
    // location
    pub title: Option<String>,
    pub address: Option<String>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    // sticker
    #[serde(rename = "stickerId")]
    pub sticker_id: Option<String>,
    #[serde(rename = "packageId")]
    pub package_id: Option<String>,
    #[serde(rename = "stickerResourceType")]
    pub sticker_resource_type: Option<String>,
    pub keywords: Option<Vec<String>>,
}
