//! `/webhook` command handler.
//!
//! Displays the current webhook endpoint configuration.

use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
};

/// Fetches and displays the current webhook endpoint URL and active status.
pub async fn handle(line: &LINE, reply_token: String) -> Result<(), String> {
    let text = match line.messaging_api_client.get_webhook_endpoint().await {
        Ok(info) => format!("{info:#?}"),
        Err(e) => format!("Failed to get webhook endpoint: {e}"),
    };

    let req = ReplyMessageRequest {
        reply_token,
        messages: vec![Message::TextMessage(TextMessage::new(text))],
        notification_disabled: Some(false),
    };

    line.messaging_api_client
        .reply_message(req)
        .await
        .map_err(|e| format!("reply_message failed: {e}"))?;

    Ok(())
}
