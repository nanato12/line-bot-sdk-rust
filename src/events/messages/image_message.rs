use crate::events::messages::content_provider::ContentProvider;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ImageMessage {
    id: String,
    #[serde(rename = "contentProvider")]
    content_provider: ContentProvider,
}
