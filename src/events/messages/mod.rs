use crate::events::Source;

pub mod content_provider;
pub mod emoji;
pub mod mention;

pub use content_provider::ContentProvider;
pub use emoji::Emoji;
pub use mention::Mention;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MessageEvent {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    pub mode: String,
    pub timestamp: i64,
    pub source: Source,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    #[serde(flatten)]
    pub r#type: MessageType,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MessageType {
    #[serde(rename = "text")]
    Text {
        id: String,
        text: String,
        emojis: Vec<Emoji>,
        mention: Mention,
    },
    #[serde(rename = "image")]
    Image {
        id: String,
        #[serde(rename = "contentProvider")]
        content_provider: ContentProvider,
    },
    #[serde(rename = "video")]
    Video {
        id: String,
        duration: i64,
        #[serde(rename = "contentProvider")]
        content_provider: ContentProvider,
    },
}

#[derive(Deserialize, Debug)]
pub struct BaseMessage {
    pub id: String,
    pub r#type: String,
    // text
    pub text: Option<String>,
    pub emojis: Option<Vec<Emoji>>,
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
