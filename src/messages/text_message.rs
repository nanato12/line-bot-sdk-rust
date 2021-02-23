use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct TextMessage {
    pub r#type: String,
    pub text: String,
}

impl TextMessage {
    pub fn new(text: &str) -> TextMessage {
        TextMessage {
            r#type: String::from("text"),
            text: String::from(text),
        }
    }
}
