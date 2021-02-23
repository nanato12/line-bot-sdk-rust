use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ContentProvider {
    #[serde(flatten)]
    pub r#type: ContentProviderType,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ContentProviderType {
    #[serde(rename = "external")]
    External {
        #[serde(rename = "originalContentUrl")]
        original_content_url: String,
        #[serde(rename = "previewImageUrl")]
        preview_image_url: String,
    },
    #[serde(other)]
    Other,
}
