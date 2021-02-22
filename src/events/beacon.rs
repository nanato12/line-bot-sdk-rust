use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Beacon {
    pub hwid: String,
    pub r#type: String,
    pub dm: Option<String>,
}
