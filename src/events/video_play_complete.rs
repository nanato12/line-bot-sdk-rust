use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct VideoPlayComplete {
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
}
