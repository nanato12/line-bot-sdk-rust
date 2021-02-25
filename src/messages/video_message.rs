use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct VideoMessage {
    #[serde(rename = "originalContentUrl")]
    pub original_content_url: String,
    #[serde(rename = "previewImageUrl")]
    pub preview_image_url: String,
    #[serde(rename = "trackingId", skip_serializing_if = "Option::is_none")]
    pub track_id: Option<String>,
}
