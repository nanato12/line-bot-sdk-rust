use crate::events::messages::content_provider::ContentProvider;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ImageMessage {
    pub id: String,
    #[serde(rename = "contentProvider")]
    pub content_provider: ContentProvider,
}
