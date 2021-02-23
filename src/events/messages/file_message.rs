use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FileMessage {
    pub id: String,
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "fileSize")]
    pub file_size: i64,
}
