use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct ImageMessage {
    #[serde(rename = "originalContentUrl")]
    pub original_content_url: String,
    #[serde(rename = "previewImageUrl")]
    pub preview_image_url: String,
}
