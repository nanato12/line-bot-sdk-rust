use crate::events::messages::content_provider::ContentProvider;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VideoMessage {
    id: String,
    duration: i64,
    #[serde(rename = "contentProvider")]
    content_provider: ContentProvider,
}
