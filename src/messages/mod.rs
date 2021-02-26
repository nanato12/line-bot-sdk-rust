//! SendMessage Instances

pub mod audio_message;
pub mod flex_message;
pub mod image_map_message;
pub mod image_message;
pub mod location_message;
pub mod sticker_message;
pub mod template_message;
pub mod text_message;
pub mod video_message;

pub use audio_message::AudioMessage;
pub use flex_message::FlexMessage;
pub use image_map_message::ImagemapMessage;
pub use image_message::ImageMessage;
pub use location_message::LocationMessage;
pub use sticker_message::StickerMessage;
pub use template_message::TemplateMessage;
pub use text_message::{Emoji, TextMessage};
pub use video_message::VideoMessage;

use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct SendMessage {
    #[serde(flatten)]
    pub r#type: SendMessageType,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum SendMessageType {
    #[serde(rename = "text")]
    TextMessage(TextMessage),
    #[serde(rename = "sticker")]
    StickerMessage(StickerMessage),
    #[serde(rename = "image")]
    ImageMessage(ImageMessage),
    #[serde(rename = "video")]
    VideoMessage(VideoMessage),
    #[serde(rename = "audio")]
    AudioMessage(AudioMessage),
    #[serde(rename = "location")]
    LocationMessage(LocationMessage),
    #[serde(rename = "imagemap")]
    ImagemapMessage(ImagemapMessage),
    #[serde(rename = "template")]
    TemplateMessage(TemplateMessage),
    // TODO: FlexMessage Component
    #[serde(rename = "flex")]
    FlexMessage(FlexMessage),
}
