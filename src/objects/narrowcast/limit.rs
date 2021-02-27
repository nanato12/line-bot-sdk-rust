use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct Limit {
    pub max: i64,
    #[serde(rename = "upToRemainingQuota")]
    pub up_to_remaining_quota: bool,
}
