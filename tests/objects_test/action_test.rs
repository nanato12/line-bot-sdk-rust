#[cfg(test)]
extern crate line_bot_sdk_rust as line;

use line::objects::action::{ActionType, AltUri};
use line::objects::Action;

use serde_json::{json, Value};

#[test]
fn test_postback_action_valid() {
    let expected_str: &str = r#"
        {
            "type": "postback",
            "label": "Buy",
            "data": "action=buy&itemid=111",
            "text": "Buy"
        }
    "#;
    let expected_json: Value = serde_json::from_str(expected_str).unwrap();

    let postback_action = Action {
        r#type: ActionType::Postback {
            data: String::from("action=buy&itemid=111"),
            text: Some(String::from("Buy")),
            display_text: None,
        },
        label: Some(String::from("Buy")),
    };
    assert_eq!(json!(postback_action), expected_json);
}

#[test]
fn test_message_action_valid() {
    let expected_str: &str = r#"
        {
            "type":"message",
            "label":"Yes",
            "text":"Yes"
        }
    "#;
    let expected_json: Value = serde_json::from_str(expected_str).unwrap();

    let message_action = Action {
        r#type: ActionType::Message {
            text: String::from("Yes"),
        },
        label: Some(String::from("Yes")),
    };
    assert_eq!(json!(message_action), expected_json);
}

#[test]
fn test_uri_action_valid() {
    let expected_str: &str = r#"
        {
            "type":"uri",
            "label":"View details",
            "uri":"http://example.com/page/222",
            "altUri": {
                "desktop" : "http://example.com/pc/page/222"
            }
        }
    "#;
    let expected_json: Value = serde_json::from_str(expected_str).unwrap();

    let uri_action = Action {
        r#type: ActionType::Uri {
            uri: String::from("http://example.com/page/222"),
            alt_uri: Some(AltUri {
                desktop: String::from("http://example.com/pc/page/222"),
            }),
        },
        label: Some(String::from("View details")),
    };
    assert_eq!(json!(uri_action), expected_json);
}

#[test]
fn test_datetimepicker_action_valid() {
    let expected_str: &str = r#"
        {
            "type":"datetimepicker",
            "label":"Select date",
            "data":"storeId=12345",
            "mode":"datetime",
            "initial":"2017-12-25t00:00",
            "max":"2018-01-24t23:59",
            "min":"2017-12-25t00:00"
        }
    "#;
    let expected_json: Value = serde_json::from_str(expected_str).unwrap();

    let datetimepicker_action = Action {
        r#type: ActionType::Datetimepicker {
            data: String::from("storeId=12345"),
            mode: String::from("datetime"),
            initial: Some(String::from("2017-12-25t00:00")),
            max: Some(String::from("2018-01-24t23:59")),
            min: Some(String::from("2017-12-25t00:00")),
        },
        label: Some(String::from("Select date")),
    };
    assert_eq!(json!(datetimepicker_action), expected_json);
}

#[test]
fn test_camera_action_valid() {
    let expected_str: &str = r#"
        {
            "type":"camera",
            "label":"Camera"
        }
    "#;
    let expected_json: Value = serde_json::from_str(expected_str).unwrap();

    let camera_action = Action {
        r#type: ActionType::Camera {},
        label: Some(String::from("Camera")),
    };
    assert_eq!(json!(camera_action), expected_json);
}

#[test]
fn test_camera_roll_action_valid() {
    let expected_str: &str = r#"
        {
            "type":"cameraRoll",
            "label":"Camera roll"
        }
    "#;
    let expected_json: Value = serde_json::from_str(expected_str).unwrap();

    let camera_roll_action = Action {
        r#type: ActionType::CameraRoll {},
        label: Some(String::from("Camera roll")),
    };
    assert_eq!(json!(camera_roll_action), expected_json);
}

#[test]
fn test_location_action_valid() {
    let expected_str: &str = r#"
        {
            "type":"location",
            "label":"Location"
        }
    "#;
    let expected_json: Value = serde_json::from_str(expected_str).unwrap();

    let location_action = Action {
        r#type: ActionType::Location {},
        label: Some(String::from("Location")),
    };
    assert_eq!(json!(location_action), expected_json);
}
