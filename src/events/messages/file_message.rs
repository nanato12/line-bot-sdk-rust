use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FileMessage {
    id: String,
    #[serde(rename = "fileName")]
    file_name: String,
    #[serde(rename = "fileSize")]
    file_size: i64,
}
