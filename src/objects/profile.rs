use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Profile {
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "pictureUrl")]
    pub picture_url: Option<String>,
    #[serde(rename = "statusMessage")]
    pub status_message: Option<String>,
    pub language: Option<String>,
}
