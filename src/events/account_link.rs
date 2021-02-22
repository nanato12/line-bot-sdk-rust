use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Link {
    pub result: String,
    pub nonce: String,
}
