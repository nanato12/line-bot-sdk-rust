//! `/flex` command handler.
//!
//! Sends a Flex Message by loading a FlexContainer from an external JSON file.
//! Edit `static/flex_sample.json` to customize the message content.

use http_body_util::BodyExt;
use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::{Error, MessagingApiApi},
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

    match line.messaging_api_client.reply_message(req).await {
        Ok(_) => Ok(()),
        Err(Error::Api(api_err)) => {
            // Read the response body to get the detailed error message from LINE API.
            let status = api_err.code;
            let body_bytes = api_err
                .body
                .collect()
                .await
                .map(|c| c.to_bytes())
                .unwrap_or_default();
            let body_str = String::from_utf8_lossy(&body_bytes);
            Err(format!("reply_message failed: {status}\n{body_str}"))
        }
        Err(e) => Err(format!("reply_message failed: {e}")),
    }
}
