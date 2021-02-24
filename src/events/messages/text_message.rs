use crate::events::messages::emoji::Emoji;
use crate::events::messages::mention::Mention;

use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TextMessage {
    pub id: String,
    pub text: String,
    pub emojis: Option<Vec<Emoji>>,
    pub mention: Option<Mention>,
}
