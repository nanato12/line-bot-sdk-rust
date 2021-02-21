pub mod text_message;

use serde_derive::Deserialize;
use text_message::Event;

#[derive(Deserialize, Debug)]
pub struct Events {
    pub events: Vec<Event>,
    pub destination: String,
}
