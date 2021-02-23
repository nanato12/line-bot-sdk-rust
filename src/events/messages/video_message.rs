use crate::events::messages::content_provider::ContentProvider;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VideoMessage {
    pub id: String,
    pub duration: i64,
    #[serde(rename = "contentProvider")]
    pub content_provider: ContentProvider,
}
