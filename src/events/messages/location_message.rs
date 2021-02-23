use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LocationMessage {
    id: String,
    title: String,
    address: String,
    latitude: f32,
    longitude: f32,
}
