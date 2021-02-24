use crate::events::Source;

pub mod audio_message;
pub mod content_provider;
pub mod emoji;
pub mod file_message;
pub mod image_message;
pub mod location_message;
pub mod mention;
pub mod sticker_message;
pub mod text_message;
pub mod video_message;

pub use audio_message::AudioMessage;
pub use file_message::FileMessage;
pub use image_message::ImageMessage;
pub use location_message::LocationMessage;
pub use sticker_message::StickerMessage;
pub use text_message::TextMessage;
pub use video_message::VideoMessage;

pub use content_provider::ContentProvider;
pub use emoji::Emoji;
pub use mention::{Mention, Mentionee};

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
    TextMessage(TextMessage),
    #[serde(rename = "image")]
    ImageMessage(ImageMessage),
    #[serde(rename = "video")]
    VideoMessage(VideoMessage),
    #[serde(rename = "audio")]
    AudioMessage(AudioMessage),
    #[serde(rename = "file")]
    FileMessage(FileMessage),
    #[serde(rename = "location")]
    LocationMessage(LocationMessage),
    #[serde(rename = "sticker")]
    StickerMessage(StickerMessage),
}
