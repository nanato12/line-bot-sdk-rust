use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Things {
    #[serde(rename = "deviceId")]
    pub device_id: String,
    pub r#type: String,
    pub result: Option<ThingsResult>,
}

#[derive(Deserialize, Debug)]
pub struct ThingsResult {
    #[serde(rename = "scenarioId")]
    pub scenario_id: String,
    pub revision: i32,
    #[serde(rename = "startTime")]
    pub start_time: i32,
    #[serde(rename = "endTime")]
    pub end_time: i32,
    #[serde(rename = "resultCode")]
    pub result_code: String,
    #[serde(rename = "bleNotificationPayload")]
    pub ble_notification_payload: String,
    #[serde(rename = "actionResults")]
    pub action_results: Vec<ActionResult>,
}

#[derive(Deserialize, Debug)]
pub struct ActionResult {
    pub r#type: String,
    pub data: String,
}
