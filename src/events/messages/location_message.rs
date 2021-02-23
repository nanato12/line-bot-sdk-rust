use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LocationMessage {
    pub id: String,
    pub title: String,
    pub address: String,
    pub latitude: f32,
    pub longitude: f32,
}
