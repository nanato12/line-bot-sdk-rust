//! `/flex` command handler.
//!
//! Sends a Flex Message by loading a FlexContainer from an external JSON file.
//! Edit `static/flex_sample.json` to customize the message content.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{FlexContainer, FlexMessage, Message, ReplyMessageRequest},
    },
};

/// Path to the Flex Message JSON file (relative to the working directory).
const FLEX_JSON_PATH: &str = "axum_example/static/flex_sample.json";

/// Loads a FlexContainer from an external JSON file and sends it as a reply.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    // Read the JSON file at runtime so you can edit it without recompiling.
    let json = std::fs::read_to_string(FLEX_JSON_PATH)
        .map_err(|e| format!("Failed to read {FLEX_JSON_PATH}: {e}"))?;

    let contents: FlexContainer =
        serde_json::from_str(&json).map_err(|e| format!("Failed to parse FlexContainer: {e}"))?;

    println!(
        "[flex] payload: {}",
        serde_json::to_string_pretty(&contents).unwrap_or_default()
    );

    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::FlexMessage(FlexMessage::new(
            "Flex Message".to_string(),
            contents,
        ))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
