pub mod emoji;
pub mod image_map;
pub mod template;

use emoji::Emoji;
use image_map::{Actions, BaseSize, Video};
use template::Template;

use serde_derive::Serialize;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum SendMessageType {
    #[serde(rename = "text")]
    TextMessage {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        emojis: Option<Vec<Emoji>>,
    },
    #[serde(rename = "sticker")]
    StickerMessage {
        #[serde(rename = "packageId")]
        package_id: String,
        #[serde(rename = "stickerId")]
        sticker_id: String,
    },
    #[serde(rename = "image")]
    ImageMessage {
        #[serde(rename = "originalContentUrl")]
        original_content_url: String,
        #[serde(rename = "previewImageUrl")]
        preview_image_url: String,
    },
    #[serde(rename = "video")]
    VideoMessage {
        #[serde(rename = "originalContentUrl")]
        original_content_url: String,
        #[serde(rename = "previewImageUrl")]
        preview_image_url: String,
        #[serde(rename = "trackingId", skip_serializing_if = "Option::is_none")]
        track_id: Option<String>,
    },
    #[serde(rename = "audio")]
    AudioMessage {
        #[serde(rename = "originalContentUrl")]
        original_content_url: String,
        duration: u64,
    },
    #[serde(rename = "location")]
    LocationMessage {
        title: String,
        address: String,
        latitude: f64,
        longitude: f64,
    },
    #[serde(rename = "imagemap")]
    ImagemapMessage {
        #[serde(rename = "baseUrl")]
        base_url: String,
        #[serde(rename = "altText")]
        alt_text: String,
        #[serde(rename = "baseSize")]
        base_size: BaseSize,
        #[serde(skip_serializing_if = "Option::is_none")]
        video: Option<Video>,
        actions: Vec<Actions>,
    },
    #[serde(rename = "template")]
    TemplateMessage {
        #[serde(rename = "altText")]
        alt_text: String,
        template: Template,
    },
    // jsonで実装すべき
    #[serde(rename = "flex")]
    Flex {},
}
