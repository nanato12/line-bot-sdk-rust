use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct Action {
    #[serde(flatten)]
    r#type: ActionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum ActionType {
    #[serde(rename = "postback")]
    Postback {
        data: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        display_text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    #[serde(rename = "message")]
    Message { text: String },
    #[serde(rename = "uri")]
    Uri {
        uri: String,
        #[serde(rename = "altUri", skip_serializing_if = "Option::is_none")]
        alt_uri: Option<AltUri>,
    },
    #[serde(rename = "datetimepicker")]
    Datetimepicker {
        data: String,
        mode: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        initial: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        max: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        min: Option<String>,
    },
    #[serde(rename = "camera")]
    Camera {},
    #[serde(rename = "cameraRoll")]
    CameraRoll {},
    #[serde(rename = "location")]
    Location {},
}

#[derive(Serialize, Debug)]
pub struct AltUri {
    desktop: String,
}
