use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ContentProvider {
    pub r#type: String,
    #[serde(rename = "originalContentUrl")]
    pub original_content_url: Option<String>,
    #[serde(rename = "previewImageUrl")]
    pub preview_image_url: Option<String>,
}
