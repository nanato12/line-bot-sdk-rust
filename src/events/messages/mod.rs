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
    #[serde(rename = "audio")]
    Audio {
        id: String,
        duration: i64,
        #[serde(rename = "contentProvider")]
        content_provider: ContentProvider,
    },
    #[serde(rename = "file")]
    File {
        id: String,
        #[serde(rename = "fileName")]
        file_name: String,
        #[serde(rename = "fileSize")]
        file_size: i64,
    },
    #[serde(rename = "location")]
    Location {
        id: String,
        title: String,
        address: String,
        latitude: f32,
        longitude: f32,
    },
    #[serde(rename = "sticker")]
    Sticker {
        id: String,
        #[serde(rename = "stickerId")]
        sticker_id: String,
        #[serde(rename = "packageId")]
        package_id: String,
        #[serde(rename = "stickerResourceType")]
        sticker_resource_type: String,
        keywords: Vec<String>,
    },
}
