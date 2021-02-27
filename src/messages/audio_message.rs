use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct AudioMessage {
    #[serde(rename = "originalContentUrl")]
    pub original_content_url: String,
    pub duration: i64,
}
