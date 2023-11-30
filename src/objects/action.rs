//! Action objects
//! # Note
//! These are types of actions for your bot to take when a user taps a button or an image in a message.
//! <https://developers.line.biz/en/reference/messaging-api/#action-objects>
use serde_derive::Serialize;

/// Action object
/// # Note
#[derive(Serialize, Debug)]
pub struct Action {
    #[serde(flatten)]
    pub r#type: ActionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Action object types
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
    #[serde(rename = "richmenuswitch")]
    RichMenuSwitch {
        #[serde(rename = "richMenuAliasId")]
        rich_menu_alias_id: String,
        data: String,
    },
}

/// Alt uri object
/// # Note
/// URI opened on LINE for macOS and Windows when the action is performed.
#[derive(Serialize, Debug)]
pub struct AltUri {
    pub desktop: String,
}
